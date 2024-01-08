use std::env;

use anyhow::Context;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

use crate::listing::{ItemListing, ItemListingPriceUpdate};

#[async_trait]
pub trait Datastore {
    /// Open a connection to the remote DB
    async fn connect(&self) -> anyhow::Result<()>;
    /// Check if an item listing exists with a given ID
    async fn exists(&self, id: &str) -> anyhow::Result<bool>;
    /// Create a new item listing
    async fn create(&self, listing: ItemListing) -> anyhow::Result<Option<ItemListing>>;
    /// Update an item listing's price and updated_at timestamp
    async fn update(
        &self,
        id: &str,
        data: ItemListingPriceUpdate,
    ) -> anyhow::Result<Option<ItemListing>>;
}

static SURREAL_DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub struct SurrealDatastore {
    listings_table: String,
}

#[derive(Debug, Deserialize)]
pub struct ListingRecord {
    pub id: Thing,
}

impl SurrealDatastore {
    pub fn new() -> Self {
        Self {
            listings_table: "item_listings".to_owned(),
        }
    }
}

#[async_trait]
impl Datastore for SurrealDatastore {
    async fn connect(&self) -> anyhow::Result<()> {
        let url = env::var("SURREAL_URL").unwrap_or("localhost:8000".to_string());
        SURREAL_DB
            .connect::<Ws>(&url)
            .await
            .context("failed connecting to surreal instance at {url}")?;

        let username = env::var("SURREAL_USER").unwrap_or("admin".to_string());
        let password = env::var("SURREAL_PASS").unwrap_or("password".to_string());
        SURREAL_DB
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await
            .context("failed authenticating for user: {username}")?;

        let ns_name = "poeledger";
        let db_name = "river";
        SURREAL_DB
            .use_ns(ns_name)
            .use_db(db_name)
            .await
            .context("failed to use NS {ns_name} and DB {db_name}")?;

        Ok(())
    }

    async fn exists(&self, id: &str) -> anyhow::Result<bool> {
        let listing: Option<ItemListing> = SURREAL_DB.select((&self.listings_table, id)).await?;

        Ok(listing.is_some())
    }

    async fn create(&self, listing: ItemListing) -> anyhow::Result<Option<ItemListing>> {
        let record: Option<ItemListing> = SURREAL_DB
            .create((&self.listings_table, &listing.item_id))
            .content(listing)
            .await?;

        Ok(record)
    }

    async fn update(
        &self,
        id: &str,
        data: ItemListingPriceUpdate,
    ) -> anyhow::Result<Option<ItemListing>> {
        let record: Option<ItemListing> = SURREAL_DB
            .update((&self.listings_table, id))
            .merge(data)
            .await?;

        Ok(record)
    }
}
