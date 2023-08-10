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
        .route("/filters", get(filter_handler))
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

async fn filter_handler() -> Json<Vec<String>> {
    let filters = vec![
        "Mirror of Kalandra".to_string(),
        "Mirror Shard".to_string(),
        "Fracturing Orb".to_string(),
        "Tempering Orb".to_string(),
        "Tainted Divine Teardrop".to_string(),
        "Tailoring Orb".to_string(),
        "Blessing of Chayula".to_string(),
        "Sacred Crystallised Lifeforce".to_string(),
        "Orb of Dominance".to_string(),
        "Blessing of Esh".to_string(),
        "Blessing of Tul".to_string(),
        "Divine Orb".to_string(),
        "Hunter's Exalted Orb".to_string(),
        "Otherworldly Scouting Report".to_string(),
        "Secondary Regrading Lens".to_string(),
        "Blessing of Xoph".to_string(),
        "Blessing of Uul-Netol".to_string(),
        "Sacred Orb".to_string(),
        "Prime Regrading Lens".to_string(),
        "Crusader's Exalted Orb".to_string(),
        "Fracturing Shard".to_string(),
        "Redeemer's Exalted Orb".to_string(),
        "Orb of Conflict".to_string(),
        "Elevated Sextant".to_string(),
        "Awakener's Orb".to_string(),
        "Exceptional Eldritch Ichor".to_string(),
        "Comprehensive Scouting Report".to_string(),
        "Tainted Blessing".to_string(),
        "Exceptional Eldritch Ember".to_string(),
        "Tainted Exalted Orb".to_string(),
        "Eldritch Orb of Annulment".to_string(),
        "Eldritch Chaos Orb".to_string(),
        "Tainted Orb of Fusing".to_string(),
        "Warlord's Exalted Orb".to_string(),
        "Exalted Orb".to_string(),
        "Tainted Mythic Orb".to_string(),
        "Charged Compass".to_string(),
        "Eldritch Exalted Orb".to_string(),
        "Oil Extractor".to_string(),
        "Veiled Chaos Orb".to_string(),
        "Prismatic Catalyst".to_string(),
        "Ancient Orb".to_string(),
        "Orb of Annulment".to_string(),
        "Blighted Scouting Report".to_string(),
        "Tainted Chaos Orb".to_string(),
        "Unstable Catalyst".to_string(),
        "Grand Eldritch Ember".to_string(),
        "Grand Eldritch Ichor".to_string(),
        "Fertile Catalyst".to_string(),
        "Influenced Scouting Report".to_string(),
        "Ritual Vessel".to_string(),
        "Awakened Sextant".to_string(),
        "Tainted Chromatic Orb".to_string(),
        "Delirious Scouting Report".to_string(),
        "Stacked Deck".to_string(),
        "Accelerating Catalyst".to_string(),
        "Singular Scouting Report".to_string(),
        "Surveyor's Compass".to_string(),
        "Harbinger's Orb".to_string(),
        "Exalted Shard".to_string(),
        "Tainted Jeweller's Orb".to_string(),
        "Orb of Scouring".to_string(),
        "Greater Eldritch Ember".to_string(),
        "Tainted Armourer's Scrap".to_string(),
        "Orb of Unmaking".to_string(),
        "Greater Eldritch Ichor".to_string(),
        "Vaal Orb".to_string(),
        "Tempering Catalyst".to_string(),
        "Gemcutter's Prism".to_string(),
        "Explorer's Scouting Report".to_string(),
        "Intrinsic Catalyst".to_string(),
        "Vaal Scouting Report".to_string(),
        "Noxious Catalyst".to_string(),
        "Regal Orb".to_string(),
        "Orb of Regret".to_string(),
        "Blessed Orb".to_string(),
        "Enkindling Orb".to_string(),
        "Turbulent Catalyst".to_string(),
        "Lesser Eldritch Ichor".to_string(),
        "Abrasive Catalyst".to_string(),
        "Orb of Fusing".to_string(),
        "Glassblower's Bauble".to_string(),
        "Instilling Orb".to_string(),
        "Orb of Horizons".to_string(),
        "Orb of Chance".to_string(),
        "Cartographer's Chisel".to_string(),
        "Engineer's Orb".to_string(),
        "Imbued Catalyst".to_string(),
        "Lesser Eldritch Ember".to_string(),
        "Tainted Blacksmith's Whetstone".to_string(),
        "Chromatic Orb".to_string(),
        "Orb of Alteration".to_string(),
        "Jeweller's Orb".to_string(),
        "Orb of Augmentation".to_string(),
        "Vivid Crystallised Lifeforce".to_string(),
        "Orb of Alchemy".to_string(),
        "Portal Scroll".to_string(),
        "Blacksmith's Whetstone".to_string(),
        "Orb of Binding".to_string(),
        "Wild Crystallised Lifeforce".to_string(),
        "Primal Crystallised Lifeforce".to_string(),
        "Orb of Transmutation".to_string(),
        "Armourer's Scrap".to_string(),
        "Scroll of Wisdom".to_string(),
        "Maven's Orb".to_string(),
    ];
    return Json(filters);
}
