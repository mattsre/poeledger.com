CREATE TABLE ledger.listings (
    name String,
    item_id String,
    league String,
    normalized_value Float64,
    listed_price Float64,
    listed_currency String,
    implicit_mods Array(String),
    explicit_mods Array(String),
    created_at DateTime
) ENGINE = MergeTree PRIMARY KEY (item_id, name) ORDER BY (item_id, name, created_at);
