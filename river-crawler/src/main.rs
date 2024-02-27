mod limiter;

use std::{env, str::from_utf8};

use anyhow::Context;
use async_nats::jetstream::{self, consumer::PullConsumer, AckKind};
use futures::StreamExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::limiter::NatsRateLimiter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID should be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET should be set");
    let user_agent = env::var("USER_AGENT").expect("USER_AGENT should be set");

    let nats_url = env::var("NATS_URL").unwrap_or("nats://localhost:4222".to_string());
    let nats_client = async_nats::connect(&nats_url)
        .await
        .context(format!("failed to connect to NATS_URL: {nats_url}"))?;
    let limiter = NatsRateLimiter::new(nats_client.clone()).await?;

    let mut poe_client = poe_api_client::Client::new(&user_agent, limiter)?;
    poe_client.authorize(&client_id, &client_secret).await?;

    let stream_name = "PublicStashChangeIds";
    let consumer_name = "RiverCrawler";
    let jetstream = jetstream::new(nats_client.clone());
    let consumer: PullConsumer = jetstream
        .get_consumer_from_stream(&consumer_name, &stream_name)
        .await
        .expect(&format!(
            "failed to get consumer: {consumer_name} for stream: {stream_name}"
        ));

    let messages = consumer.messages().await?;

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        match msg {
            Ok(m) => {
                let change_id = match from_utf8(&m.payload) {
                    Ok(s) => s,
                    Err(_) => {
                        tracing::error!("recieved invalid utf8 changeid in message, skipping");
                        if let Err(e) = m.ack_with(AckKind::Term).await {
                            tracing::error!(
                                "failed to ack the failed utf8 changeid message with error: {e}"
                            );
                        }
                        continue;
                    }
                };

                match poe_client.get_public_stashes(Some(change_id)).await {
                    Ok((changes, _)) => {
                        let next_change_id = &changes.next_change_id;
                        if let Err(e) = jetstream
                            .publish(
                                "river.changeids".to_owned(),
                                next_change_id.to_owned().into(),
                            )
                            .await
                        {
                            tracing::error!(
                                "failed publishing next_change_id: {next_change_id} with error: {e}"
                            );
                        }

                        if !changes.stashes.is_empty() {
                            for stash_change in changes.stashes {
                                if stash_change.public && stash_change.league.is_some() {
                                    if let Ok(sc_json) = serde_json::to_string(&stash_change) {
                                        if let Err(e) =
                                            jetstream.publish("river.stashes", sc_json.into()).await
                                        {
                                            tracing::error!(
                                                "failed publishing a json stash with error: {e}"
                                            );
                                        }
                                    }
                                }
                            }
                        }

                        if let Err(e) = m.ack().await {
                            tracing::error!("couldn't ack message: {e}");
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            "failed getting public stashes for change_id: {change_id} with error: {e}"
                        );
                    }
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
