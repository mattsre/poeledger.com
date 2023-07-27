use std::{env, net::SocketAddr};

use axum::extract::State;
use axum::{extract::Path, routing::get, Json, Router};
use poeledger_economy_data::CurrencyPriceRecord;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = env::var("PORT")
        .unwrap_or("5000".to_string())
        .parse::<u16>()
        .expect("PORT should parse to u16");

    let surreal_host = env::var("SURREAL_HOST").unwrap_or("surrealdb:8000".to_string());
    let surreal_user = env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let surreal_password = env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD must be set");

    let db_client = match Surreal::new::<Ws>(surreal_host).await {
        Ok(c) => c,
        Err(e) => {
            panic!("Failed to get DB connection with error: {:#?}", e);
        }
    };

    db_client
        .signin(Root {
            username: &surreal_user,
            password: &surreal_password,
        })
        .await?;

    db_client.use_ns("economy").use_db("economy").await?;

    let app = Router::new()
        .route("/", get(handler))
        .route("/hello", get(hello_world))
        .route("/hello/:name", get(hello_name))
        .with_state(db_client);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handler(State(db_client): State<Surreal<Client>>) -> Json<Vec<CurrencyPriceRecord>> {
    let query = db_client.query("SELECT * FROM prices WHERE get = 'Divine Orb' AND league = 'Sanctum' AND pay = 'Chaos Orb' ORDER BY date LIMIT 50").await;
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
