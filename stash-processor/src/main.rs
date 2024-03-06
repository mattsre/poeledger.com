pub mod db;
pub mod listing;

use std::env;

use anyhow::Context;
use async_nats::jetstream::{self, consumer::PullConsumer};
use poe_types::{item::FrameType, stash::PublicStashChange};
use tokio_stream::StreamExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::listing::Listing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger();

    let nats_url = env::var("NATS_URL").unwrap_or("nats://localhost:4222".to_string());
    let nats = async_nats::connect(&nats_url)
        .await
        .context(format!("failed to connect to NATS_URL: {nats_url}"))?;
    let jetstream = jetstream::new(nats);

    let ch_db = db::ClickhouseDatabase::new();

    let stream_name = "PublicStashStream";
    let consumer_name = "StashProcessor";
    let consumer: PullConsumer = jetstream
        .get_consumer_from_stream(consumer_name, stream_name)
        .await
        .unwrap();

    let messages = consumer.messages().await?;

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        match msg {
            Ok(m) => {
                let stash = match serde_json::from_slice::<PublicStashChange>(&m.payload) {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::error!("failed parsing a stash change: {e}");
                        if let Err(e) = jetstream
                            .publish("river.failed_stashes", m.payload.to_owned())
                            .await
                        {
                            tracing::error!("couldn't push unprocessable stash change to failed stashes stream: {e}");
                        }

                        if let Err(e) = m.ack_with(jetstream::AckKind::Term).await {
                            tracing::error!("failed to ack unprocessable stash message: {e}");
                        }

                        continue;
                    }
                };

                let mut listings_batch = Vec::new();
                for raw_item in stash.items {
                    let is_priced = raw_item.note.is_some();
                    let is_unique = raw_item.frame_type.as_ref().is_some_and(|f| {
                        matches!(
                            f,
                            FrameType::Unique | FrameType::Foil | FrameType::SupporterFoil
                        )
                    });
                    let name_exists = !raw_item.name.is_empty();
                    let has_item_id = raw_item.id.is_some();

                    if is_priced && is_unique && name_exists && has_item_id {
                        match Listing::try_from(raw_item) {
                            Ok(listing) => {
                                listings_batch.push(listing);
                            }
                            Err(e) => {
                                tracing::error!("failed converting item to a listing: {e}")
                            }
                        };
                    }
                }

                if let Err(e) = ch_db.create_batch(listings_batch).await {
                    tracing::error!("failed to create listing: {e}");
                }

                if let Err(e) = m.ack().await {
                    tracing::error!("couldn't ack message: {e}");
                }
            }
            Err(e) => {
                tracing::error!("failed to read item: {e}");
            }
        }
    }

    Ok(())
}

fn setup_logger() {
    let logger = tracing_subscriber::fmt::layer().json();
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("failed to create logger");
    let exporter = Registry::default().with(logger).with(env_filter);

    tracing::subscriber::set_global_default(exporter).expect("failed to set log exporter");
}
