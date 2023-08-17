mod filters;
mod leagues;
mod prices;
mod utils;

use std::env;

use axum::{routing::get, Router};
use mongodb::{options::ClientOptions, Client as MongoClient, Database};

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
    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid port number");

    let mongo_connection_string = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mongo_client = match create_mongo_client(mongo_connection_string).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{:#?}", e);
            panic!("MongoDB client failed to connect, connection is required for API to function. Exiting!");
        }
    };

    let mongodb = mongo_client.database("economy");

    let app = Router::new()
        .route("/prices", get(prices::prices_handler))
        .route("/filters", get(filters::filters_handler))
        .route("/leagues", get(leagues::leagues_handler))
        .with_state(ApiState::new(mongodb));

    let addr = format!("[::]:{port}").parse()?;
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn create_mongo_client(uri: String) -> anyhow::Result<MongoClient> {
    let mut options = ClientOptions::parse(uri).await?;
    options.app_name = Some("economy-data-api".to_owned());
    options.default_database = Some("economy".to_owned());

    let client = MongoClient::with_options(options)?;

    Ok(client)
}
