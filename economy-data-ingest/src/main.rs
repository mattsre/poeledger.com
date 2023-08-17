mod ninja;

use std::env;
use std::process::exit;

use mongodb::{options::ClientOptions, Client as MongoClient};
use poeledger_economy_data::PriceRecord;
use tokio::time::Instant;

use clap::Parser;

use crate::ninja::{NinjaCurrencyRecord, NinjaItemRecord};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mongo_connection_string = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let mongo_client = match create_mongo_client(mongo_connection_string).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{:#?}", e);
            panic!("MongoDB client failed to connect, connection is required for API to function. Exiting!");
        }
    };

    let mongodb = mongo_client.database("economy");
    let economy_collection = mongodb.collection::<PriceRecord>("economy");

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(cli.file)?;

    let start_time = Instant::now();

    let mut line_count = 0;
    let max_batch_size = 500;
    let mut write_batch: Vec<PriceRecord> = Vec::new();

    let mut raw_record = csv::StringRecord::new();
    let headers = csv_reader.headers()?.clone();

    while csv_reader.read_record(&mut raw_record)? {
        let record: PriceRecord = match raw_record.len() {
            6 => {
                // parse as currency record
                let r: NinjaCurrencyRecord = raw_record.deserialize(Some(&headers))?;
                r.to_price_record()
            }
            10 => {
                // parse as item record
                let r: NinjaItemRecord = raw_record.deserialize(Some(&headers))?;
                r.to_price_record()
            }
            _ => {
                eprintln!(
                    "failed to determine whether this file contains currency or item price records"
                );
                exit(1);
            }
        };

        write_batch.push(record);

        if write_batch.len() >= max_batch_size {
            match economy_collection.insert_many(&write_batch, None).await {
                Ok(_) => write_batch.clear(),
                Err(e) => {
                    eprintln!("failed writing batch to mongodb with error: {e}");
                    exit(1);
                }
            }
        }

        line_count += 1;
        if line_count % max_batch_size == 0 {
            println!("Processed {line_count} price records");
        }
    }

    if !write_batch.is_empty() {
        if let Err(e) = economy_collection.insert_many(&write_batch, None).await {
            eprintln!("failed writing batch to influx with error: {e}");
            exit(1);
        }
    }

    let duration = start_time.elapsed();

    println!("Processed {line_count} price records");
    println!("Spent {:?} processing records", duration);

    Ok(())
}

async fn create_mongo_client(uri: String) -> anyhow::Result<MongoClient> {
    let mut options = ClientOptions::parse(uri).await?;
    options.app_name = Some("economy-data-ingest".to_owned());
    options.default_database = Some("economy".to_owned());

    let client = MongoClient::with_options(options)?;

    Ok(client)
}
