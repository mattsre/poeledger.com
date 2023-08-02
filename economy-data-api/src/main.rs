use std::env;

use axum::extract::{Query, State};
use axum::{routing::get, Json, Router};
use poeledger_economy_data::CurrencyPriceRecord;
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: Surreal<Any> = Surreal::init();

#[derive(Clone)]
struct ApiState {
    db: Surreal<Any>,
}

impl ApiState {
    pub fn new() -> Self {
        Self { db: DB.clone() }
    }
}

#[derive(Deserialize)]
struct PricesQuery {
    get: String,
    pay: String,
    league: String,
}

#[derive(Serialize, Deserialize)]
enum RatesWindow {
    #[serde(rename = "7D")]
    SevenDay,
    #[serde(rename = "14D")]
    FourteenDay,
    #[serde(rename = "30D")]
    ThirtyDay,
    #[serde(rename = "90D")]
    NinentyDay,
}

impl RatesWindow {
    pub fn value(&self) -> u16 {
        match self {
            RatesWindow::SevenDay => 7,
            RatesWindow::FourteenDay => 14,
            RatesWindow::ThirtyDay => 30,
            RatesWindow::NinentyDay => 90,
        }
    }
}

#[derive(Deserialize)]
struct RatesQuery {
    get: String,
    pay: String,
    league: String,
    window: RatesWindow,
}

#[derive(Serialize, Deserialize)]
struct RatesResponse {
    rate: f32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid port number");

    let surreal_host = env::var("SURREAL_HOST")
        .expect("SURREAL_HOST must be set and contain the connection protocol (ws | wss)");
    let surreal_user = env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let surreal_pass = env::var("SURREAL_PASS").expect("SURREAL_PASS must be set");

    DB.connect(surreal_host).await?;

    DB.signin(Root {
        username: &surreal_user,
        password: &surreal_pass,
    })
    .await?;

    DB.use_ns("economy").use_db("economy").await?;

    let app = Router::new()
        .route("/prices", get(prices_handler))
        .route("/rates", get(rates_handler))
        .with_state(ApiState::new());

    // run it
    let addr = format!("[::]:{port}")
        .parse()
        .expect("address should be available");
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn prices_handler(
    State(state): State<ApiState>,
    query: Query<PricesQuery>,
) -> Json<Vec<CurrencyPriceRecord>> {
    let query = state.db.query("SELECT * FROM prices WHERE get = $get AND pay = $pay AND league = $league ORDER BY date LIMIT 120").bind(("get", &query.get)).bind(("pay", &query.pay)).bind(("league", &query.league)).await;
    match query {
        Ok(mut res) => {
            let records: Vec<CurrencyPriceRecord> = res.take(0).unwrap();
            Json(records)
        }
        Err(_) => Json(vec![]),
    }
}

async fn rates_handler(
    State(state): State<ApiState>,
    query: Query<RatesQuery>,
) -> Json<RatesResponse> {
    let price_history_limit = query.window.value();

    let query = state.db.query("SELECT * FROM prices WHERE get = $get AND pay = $pay AND league = $league ORDER BY date LIMIT $limit").bind(("get", &query.get)).bind(("pay", &query.pay)).bind(("league", &query.league)).bind(("limit", &price_history_limit)).await;
    match query {
        Ok(mut res) => {
            let records: Vec<CurrencyPriceRecord> = res.take(0).unwrap();

            if !records.is_empty() {
                let initial_value = &records.first().unwrap().value;
                let end_value = &records.last().unwrap().value;

                let change_percentage = ((end_value - initial_value) / initial_value) * 100 as f32;

                return Json(RatesResponse {
                    rate: change_percentage,
                });
            }

            Json(RatesResponse { rate: 0 as f32 })
        }
        Err(_) => Json(RatesResponse { rate: 0 as f32 }),
    }
}
