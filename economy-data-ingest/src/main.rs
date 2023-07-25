use std::env;

use poeledger_economy_data::CurrencyPriceRecord;

use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::Instant;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let surreal_host = env::var("SURREAL_HOST").unwrap_or("127.0.0.1:8000".to_string());
    let surreal_user = env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let surreal_password = env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD must be set");

    let db = Surreal::new::<Ws>(surreal_host).await?;
    db.signin(Root {
        username: &surreal_user,
        password: &surreal_password,
    })
    .await?;

    db.use_ns("economy").use_db("economy").await?;

    let file = File::open("Sanctum.currency.csv").await?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    lines.next_line().await?;

    let start_time = Instant::now();

    let mut line_count = 1;
    while let Some(line) = lines.next_line().await? {
        create_price_record(&db, line).await?;

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

async fn create_price_record(db: &Surreal<Client>, line: String) -> anyhow::Result<Record> {
    let record: CurrencyPriceRecord = CurrencyPriceRecord::try_from(line)?;

    let created: Record = db.create("prices").content(record).await?;

    Ok(created)
}
