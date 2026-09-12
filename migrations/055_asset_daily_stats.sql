-- Daily views and downloads per asset, so the marketplace can draw a series
-- rather than only a running total.
--
-- Neither existing counter can answer "what happened last Tuesday".
-- `assets.views` / `assets.downloads` are scalars: once incremented, the history
-- is gone. `page_views` looks like a log but is not one, because its unique
-- index is (entity_type, entity_id, ip_hash) and a repeat visit rewrites the
-- existing row's timestamp in place. So the series needs its own rollup,
-- written at the same moments those counters move.
--
-- A daily rollup rather than an event log: one row per asset per day it was
-- actually touched. That keeps the table proportional to activity instead of to
-- traffic, and weeks and months are a date_trunc over the days rather than a
-- second table to keep in step.
CREATE TABLE IF NOT EXISTS asset_stats_daily (
    asset_id  UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    day       DATE NOT NULL,
    views     BIGINT NOT NULL DEFAULT 0,
    downloads BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (asset_id, day)
);

-- The creator dashboard asks "every asset of mine, over the last N buckets",
-- which is a range over `day` across a set of assets. The primary key leads with
-- asset_id and cannot serve that scan.
CREATE INDEX IF NOT EXISTS idx_asset_stats_daily_day
    ON asset_stats_daily (day);
