CREATE TABLE ledger.listings (
    item_id String,
    name String,
    league String,
    normalized_price Float64,
    listed_price Float64,
    listed_currency String,
    implicit_mods Array(String),
    explicit_mods Array(String),
    created_at DateTime
) ENGINE = MergeTree PRIMARY KEY (name, created_at) ORDER BY (name, created_at);
