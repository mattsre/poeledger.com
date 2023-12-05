pub mod item;

use std::env;

use async_nats::jetstream::{self, consumer::PullConsumer};
use poe_types::{item::FrameType, stash::PublicStashChange};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio_stream::StreamExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::item::{ItemListing, ItemRecord};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger();

    let nats_url = env::var("NATS_URL").unwrap_or("nats://localhost:4222".to_string());
    let nats = async_nats::connect(&nats_url).await?;
    let jetstream = jetstream::new(nats);

    let surreal_url = env::var("SURREAL_URL").unwrap_or("localhost:8000".to_string());
    let surreal = Surreal::new::<Ws>(surreal_url).await?;
    surreal
        .signin(Root {
            username: "admin",
            password: "password",
        })
        .await?;
    surreal.use_ns("poeledger").use_db("river").await?;

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
                let stash = serde_json::from_slice::<PublicStashChange>(&m.payload)?;

                let table_name = "item_listings";
                for raw_item in stash.items {
                    let is_priced = raw_item.note.is_some();
                    let is_unique = raw_item.frame_type.as_ref().is_some_and(|f| {
                        matches!(
                            f,
                            FrameType::Unique | FrameType::Foil | FrameType::SupporterFoil
                        )
                    });
                    let name_is_empty = raw_item.name.is_empty();
                    let current_league = "Ancestor";
                    let is_current_league = raw_item
                        .league
                        .as_ref()
                        .is_some_and(|l| l == current_league);

                    if is_priced && is_unique && !name_is_empty && is_current_league {
                        match raw_item.id.clone() {
                            Some(raw_id) => {
                                let priced_item = ItemListing::from(raw_item);

                                let query: Result<Option<ItemRecord>, surrealdb::Error> = surreal
                                    .update((table_name, raw_id))
                                    .content(&priced_item)
                                    .await;

                                if let Err(e) = query {
                                    tracing::error!(
                                        "failed to upsert item: {:#?} with error: {e}",
                                        &priced_item
                                    );
                                }
                            }
                            None => tracing::warn!("ignoring item with null item ID"),
                        }
                    }
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
