-- Enable the pg_trgm extension before app migrations run.
--
-- Migration 027 creates a GIN trigram index (gin_trgm_ops) before its own
-- CREATE EXTENSION statement, so on a fresh database the index would fail.
-- Enabling the extension here (during Postgres first-time init, before the
-- app connects) makes migration 027's CREATE EXTENSION IF NOT EXISTS a no-op.
CREATE EXTENSION IF NOT EXISTS pg_trgm;
