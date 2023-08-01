use std::env;

use axum::extract::State;
use axum::{extract::Path, routing::get, Json, Router};
use poeledger_economy_data::CurrencyPriceRecord;
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
        .route("/", get(handler))
        .route("/hello", get(hello_world))
        .route("/hello/:name", get(hello_name))
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

async fn handler(State(state): State<ApiState>) -> Json<Vec<CurrencyPriceRecord>> {
    let query = state.db.query("SELECT * FROM prices WHERE get = 'Divine Orb' AND league = 'Sanctum' AND pay = 'Chaos Orb' ORDER BY date LIMIT 50").await;
    match query {
        Ok(mut res) => {
            let records: Vec<CurrencyPriceRecord> = res.take(0).unwrap();
            Json(records)
        }
        Err(_) => Json(vec![]),
    }
}

async fn hello_world() -> String {
    "Hello World".to_string()
}

async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello {name}")
}
