use chrono::DateTime;
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::TimeZone;
use chrono::Utc;
use futures::prelude::*;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use poeledger_economy_data::CurrencyPriceRecord;
use poeledger_economy_data::ItemPriceRecord;
use poeledger_economy_data::PriceRecord;
use std::env;
use std::process::exit;

use tokio::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let influx_host = env::var("INFLUX_HOST").expect("INFLUX_HOST must be set");
    let influx_token = env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN must be set");
    let influx_org = env::var("INFLUX_ORG").unwrap_or("poeledger".to_string());
    let influx_bucket = env::var("INFLUX_BUCKET").unwrap_or("economy".to_string());

    let influx_client = Client::new(influx_host, influx_org, influx_token);

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("Sanctum.currency.csv")?;

    let start_time = Instant::now();

    let mut line_count = 0;
    let max_batch_size = 1000;
    let mut write_batch: Vec<DataPoint> = Vec::new();

    for result in csv_reader.deserialize() {
        let record: PriceRecord = result?;

        let datapoint = match record {
            PriceRecord::CurrencyPriceRecord(r) => match create_datapoint_from_cpr(r) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("encountered error parsing line: {e}");
                    exit(1);
                }
            },
            PriceRecord::ItemPriceRecord(r) => match create_datapoint_from_ipr(r) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("encountered error parsing line: {e}");
                    exit(1);
                }
            },
        };

        write_batch.push(datapoint);

        if write_batch.len() >= max_batch_size {
            match influx_client
                .write(&influx_bucket, stream::iter(write_batch.clone()))
                .await
            {
                Ok(_) => write_batch.clear(),
                Err(e) => {
                    eprintln!("failed writing batch to influx with error: {e}");
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
        if let Err(e) = influx_client
            .write(&influx_bucket, stream::iter(write_batch.clone()))
            .await
        {
            eprintln!("failed writing batch to influx with error: {e}");
            exit(1);
        }
    }

    let duration = start_time.elapsed();

    println!("Processed {line_count} price records");
    println!("Spent {:?} processing records", duration);

    Ok(())
}

fn create_datapoint_from_cpr(record: CurrencyPriceRecord) -> anyhow::Result<DataPoint> {
    let timestamp = get_nano_timestamp(record.date);

    let datapoint = DataPoint::builder("currency_price_record")
        .timestamp(timestamp)
        .tag("league", record.league.to_string())
        .tag("get", &record.get)
        .tag("pay", &record.pay)
        .tag("confidence", record.confidence.to_string())
        .field("value", record.value)
        .build()?;

    Ok(datapoint)
}

fn create_datapoint_from_ipr(record: ItemPriceRecord) -> anyhow::Result<DataPoint> {
    let timestamp = get_nano_timestamp(record.date);

    let datapoint = DataPoint::builder("item_price_record")
        .timestamp(timestamp)
        .tag("league", record.league.to_string())
        .tag("id", record.id.to_string())
        .tag("type", &record.item_type)
        .tag("name", &record.name)
        .tag("baseType", &record.base_type.unwrap_or("".to_string()))
        .tag("variant", &record.variant.unwrap_or("".to_string()))
        .tag("links", &record.links.unwrap_or("".to_string()))
        .tag("confidence", &record.confidence.to_string())
        .field("value", record.value)
        .build()?;

    Ok(datapoint)
}

fn get_nano_timestamp(date: NaiveDate) -> i64 {
    let tz_offset = FixedOffset::east_opt(5 * 3600).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let datetime = NaiveDateTime::new(date, time);
    let dt_with_tz: DateTime<FixedOffset> = tz_offset.from_local_datetime(&datetime).unwrap();
    let dt_with_tz_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());

    dt_with_tz_utc.timestamp_nanos()
}
