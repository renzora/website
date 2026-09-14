//! Content-addressed storage: one object per distinct set of bytes.
//!
//! A release used to upload its own copy of every file it contained, under a
//! key with a fresh UUID in it, so two releases differing in one line stored two
//! full copies of everything else. Keying on the SHA-256 of the contents makes
//! that impossible: the same bytes are the same object, and a release costs only
//! what is actually new in it.
//!
//! Nothing here deletes an object on its own. Whether a blob is still needed is
//! a question about `asset_files`, and it is asked in one statement at deletion
//! time so that a concurrent publish cannot slip between the question and the
//! answer. See [`release_if_unreferenced`].

use sqlx::PgPool;

/// A stored object, identified by what is in it.
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct StorageBlob {
    pub sha256: String,
    pub storage_key: String,
    pub byte_size: i64,
    pub content_type: String,
    /// `None` until the bytes are known to be in storage. A caller that finds
    /// this unset must upload rather than assume, because it means another
    /// publish of the same new file is mid-upload right now.
    pub uploaded_at: Option<time::OffsetDateTime>,
}

/// What [`claim`] decided the caller has to do.
#[derive(Debug, PartialEq, Eq)]
pub enum Claim {
    /// These bytes are not in storage, or not known to be yet. Upload them to
    /// `storage_key`, then call [`mark_uploaded`].
    Upload { storage_key: String },
    /// Already stored. Reference `storage_key` and upload nothing.
    Reuse { storage_key: String },
}

/// Hex SHA-256 of some bytes. The identity of a blob.
pub fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

impl StorageBlob {
    /// Work out whether these bytes still need uploading, reserving the row if
    /// they do.
    ///
    /// The insert and the check are one statement on purpose. Two publishes of
    /// the same new file race otherwise: both find nothing, both upload, and one
    /// overwrites the other's row. `ON CONFLICT DO NOTHING RETURNING` gives
    /// exactly one of them the row back, and it is the one that uploads.
    ///
    /// The loser still uploads when the winner has not finished, which is what
    /// `uploaded_at` is for. It costs a duplicate upload of identical bytes to
    /// the same key in a rare race, and it buys never handing out a key to an
    /// object that is not there.
    pub async fn claim(
        pool: &PgPool,
        sha256: &str,
        proposed_key: &str,
        byte_size: i64,
        content_type: &str,
    ) -> Result<Claim, sqlx::Error> {
        let inserted: Option<(String,)> = sqlx::query_as(
            "INSERT INTO storage_blobs (sha256, storage_key, byte_size, content_type)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (sha256) DO NOTHING
             RETURNING storage_key",
        )
        .bind(sha256)
        .bind(proposed_key)
        .bind(byte_size)
        .bind(content_type)
        .fetch_optional(pool)
        .await?;

        if let Some((key,)) = inserted {
            return Ok(Claim::Upload { storage_key: key });
        }

        // Somebody else owns the row. Take their key, and upload anyway if they
        // have not finished with it.
        let row: (String, Option<time::OffsetDateTime>) = sqlx::query_as(
            "SELECT storage_key, uploaded_at FROM storage_blobs WHERE sha256 = $1",
        )
        .bind(sha256)
        .fetch_one(pool)
        .await?;

        Ok(match row.1 {
            Some(_) => Claim::Reuse { storage_key: row.0 },
            None => Claim::Upload { storage_key: row.0 },
        })
    }

    /// Record that the bytes are in storage.
    pub async fn mark_uploaded(pool: &PgPool, sha256: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE storage_blobs SET uploaded_at = NOW() WHERE sha256 = $1")
            .bind(sha256)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Drop the blob row if nothing references it any more, and say which object
    /// the caller should now delete.
    ///
    /// `None` means something else still points at these bytes and the object
    /// must be left alone. That is the whole hazard of sharing storage: the
    /// obvious "delete the file's object when the file goes" is, once two
    /// releases can share one object, a way to delete a file another release is
    /// still serving.
    ///
    /// One statement, so a publish that takes a reference cannot land between
    /// the check and the delete. If it commits first, the `NOT EXISTS` fails and
    /// the blob stays. If it commits after, its foreign key has nothing to point
    /// at and the publish fails outright, which is the safe direction to fail:
    /// a rejected publish rather than a release referencing bytes that are gone.
    ///
    /// Call this AFTER the `asset_files` rows are deleted, or it will always
    /// find the reference it is asking about.
    pub async fn release_if_unreferenced(
        pool: &PgPool,
        sha256: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        let row: Option<(String,)> = sqlx::query_as(
            "DELETE FROM storage_blobs b
             WHERE b.sha256 = $1
               AND NOT EXISTS (SELECT 1 FROM asset_files f WHERE f.blob_sha = b.sha256)
             RETURNING b.storage_key",
        )
        .bind(sha256)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| r.0))
    }

    /// Total bytes stored, and what the same files would have cost stored per
    /// release. For the admin storage view.
    pub async fn savings(pool: &PgPool) -> Result<(i64, i64), sqlx::Error> {
        let row: (Option<i64>, Option<i64>) = sqlx::query_as(
            "SELECT
                (SELECT SUM(byte_size) FROM storage_blobs),
                (SELECT SUM(b.byte_size) FROM asset_files f
                 JOIN storage_blobs b ON b.sha256 = f.blob_sha)",
        )
        .fetch_one(pool)
        .await?;
        Ok((row.0.unwrap_or(0), row.1.unwrap_or(0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The hash is the identity, so identical bytes must land on it regardless
    /// of where they came from, and any change must move it.
    #[test]
    fn identical_bytes_hash_alike() {
        assert_eq!(sha256_hex(b"crate-type = [\"dylib\"]"), sha256_hex(b"crate-type = [\"dylib\"]"));
        assert_ne!(sha256_hex(b"version = \"1.0.0\""), sha256_hex(b"version = \"1.0.1\""));
        assert_eq!(sha256_hex(b"").len(), 64);
    }

    /// Known vector, so a change of hash function cannot pass unnoticed: it
    /// would silently orphan every object already stored under the old one.
    #[test]
    fn hashes_are_sha256() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
