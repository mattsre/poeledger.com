use std::{env, net::SocketAddr};

use axum::{routing::get, Json, Router};
use poeledger_economy_data::CurrencyPriceRecord;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse::<u16>()
        .expect("PORT should parse to u16");

    let surreal_host = env::var("SURREAL_HOST").unwrap_or("127.0.0.1:8000".to_string());
    let surreal_user = env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let surreal_password = env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD must be set");

    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Json<CurrencyPriceRecord> {
    Json(CurrencyPriceRecord::default())
}
