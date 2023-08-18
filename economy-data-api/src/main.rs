mod filters;
mod leagues;
mod prices;
mod utils;

use std::{env, process::exit};

use axum::{extract::MatchedPath, http::Request, routing::get, Router};
use mongodb::{bson::doc, options::ClientOptions, Client as MongoClient, Database};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::prelude::*;

#[derive(Clone)]
pub struct ApiState {
    db: Database,
}

impl ApiState {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "economy_data_api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid port number");

    let mongo_connection_string = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mongo_client = match create_mongo_client(mongo_connection_string).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("{:#?}", e);
            tracing::error!("MongoDB client failed to connect, connection is required for API to function. Exiting!");
            exit(1);
        }
    };

    let mongodb = mongo_client.database("economy");

    let app = Router::new()
        .route("/prices", get(prices::prices_handler))
        .route("/filters", get(filters::filters_handler))
        .route("/leagues", get(leagues::leagues_handler))
        .with_state(ApiState::new(mongodb))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    let addr = format!("[::]:{port}").parse()?;
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn create_mongo_client(uri: String) -> anyhow::Result<MongoClient> {
    let mut options = ClientOptions::parse(uri).await?;
    options.app_name = Some("economy-data-api".to_owned());

    let client = MongoClient::with_options(options)?;

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    tracing::info!("connected to mongodb!");

    Ok(client)
}
