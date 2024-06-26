use std::env;

use clickhouse::{error::Error::RowNotFound, Row};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::listing::Listing;

pub struct ClickhouseDatabase {
    client: clickhouse::Client,
}

#[derive(Row, Serialize, Deserialize)]
pub struct ListingChRow {
    pub item_id: String,
    pub name: String,
    pub league: String,
    pub normalized_price: f64,
    pub listed_price: f64,
    pub listed_currency: String,
    pub implicit_mods: Vec<String>,
    pub explicit_mods: Vec<String>,
    #[serde(with = "clickhouse::serde::time::datetime")]
    pub created_at: OffsetDateTime,
}

impl From<&Listing> for ListingChRow {
    fn from(l: &Listing) -> Self {
        Self {
            item_id: l.item_id.clone(),
            name: l.name.clone(),
            league: l.league.clone(),
            normalized_price: l.price.normalized_price,
            listed_price: l.price.listed_price,
            listed_currency: l.price.listed_currency.to_string(),
            implicit_mods: l.implicit_mods.clone(),
            explicit_mods: l.explicit_mods.clone(),
            created_at: l.created_at,
        }
    }
}

impl ClickhouseDatabase {
    pub async fn new() -> Self {
        let url = env::var("CLICKHOUSE_URL").unwrap_or("http://localhost:8123".to_owned());

        let user = env::var("CLICKHOUSE_USER");
        let password = env::var("CLICKHOUSE_PASSWORD");

        let dbname = "ledger";
        let mut client = clickhouse::Client::default()
            .with_url(url)
            .with_database(dbname);

        if let (Ok(u), Ok(p)) = (user, password) {
            client = client.with_user(u).with_password(p);
        }

        // ensure client connects to Clickhouse
        if let Err(e) = client.query("SELECT 1").execute().await {
            tracing::error!("failed to connect to Clickhouse: {e}");
            tracing::error!("exiting!");

            std::process::exit(1);
        }

        tracing::info!("connected to Clickhouse!");

        Self { client }
    }

    pub async fn exists(&self, id: &str) -> anyhow::Result<bool> {
        let qr = self
            .client
            .query("SELECT item_id FROM listings WHERE item_id = ?")
            .bind(id)
            .fetch_one::<ListingChRow>()
            .await;

        match qr {
            Ok(_) => Ok(true),
            Err(e) => match e {
                RowNotFound => Ok(false),
                _ => {
                    tracing::error!("failed to fetch listing with ID: {id} - {e}");

                    Err(e.into())
                }
            },
        }
    }

    pub async fn create_batch(&self, listings: &Vec<Listing>) -> anyhow::Result<()> {
        let mut insert = self.client.insert("listings")?;

        for l in listings {
            let ch_row = ListingChRow::from(l);
            insert.write(&ch_row).await?;
        }

        insert.end().await?;

        Ok(())
    }
}
