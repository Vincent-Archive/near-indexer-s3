CREATE TABLE IF NOT EXISTS near_indexer_raw_202208 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-08-01') TO ('2022-09-01');

CREATE TABLE IF NOT EXISTS near_indexer_raw_202209 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-09-01') TO ('2022-10-01');

CREATE TABLE IF NOT EXISTS near_indexer_raw_202210 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-10-01') TO ('2022-11-01');

CREATE TABLE IF NOT EXISTS near_indexer_raw_202211 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-11-01') TO ('2022-12-01');

CREATE TABLE IF NOT EXISTS near_indexer_raw_202212 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-12-01') TO ('2023-01-01');