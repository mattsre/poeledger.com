use poeledger_economy_data::CurrencyPriceRecord;
use std::env;
use surrealdb::engine::any::Any;

use serde::Deserialize;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::Instant;

static DB: Surreal<Any> = Surreal::init();

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let file = File::open("Sanctum.currency.csv").await?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    lines.next_line().await?;

    let start_time = Instant::now();

    let mut line_count = 1;
    while let Some(line) = lines.next_line().await? {
        create_price_record(DB.clone(), line).await?;

        line_count += 1;
        if line_count % 500 == 0 {
            println!("Processed {line_count} price records");
        }
    }

    let duration = start_time.elapsed();

    println!("Processed {line_count} price records");
    println!("Spent {:?} processing records", duration);

    Ok(())
}

async fn create_price_record(db_client: Surreal<Any>, line: String) -> anyhow::Result<Record> {
    let record: CurrencyPriceRecord = CurrencyPriceRecord::try_from(line)?;

    let created: Record = db_client.create("prices").content(record).await?;

    Ok(created)
}
