use std::env;
use std::process::exit;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use futures::prelude::*;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use poeledger_economy_data::{CurrencyPriceRecord, ItemPriceRecord, PriceRecord};
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

    let mut raw_record = csv::StringRecord::new();
    let headers = csv_reader.headers()?.clone();

    while csv_reader.read_record(&mut raw_record)? {
        let record: PriceRecord = match raw_record.len() {
            6 => {
                let r: CurrencyPriceRecord = raw_record.deserialize(Some(&headers))?;
                PriceRecord::CurrencyPriceRecord(r)
            }
            10 => {
                let r: ItemPriceRecord = raw_record.deserialize(Some(&headers))?;
                PriceRecord::ItemPriceRecord(r)
            }
            _ => {
                eprintln!(
                    "failed to determine whether this file contains currency or item price records"
                );
                exit(1);
            }
        };

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

    let builder = DataPoint::builder("currency_price_record")
        .timestamp(timestamp)
        .tag("league", record.league.to_string())
        .tag("get", &record.get)
        .tag("pay", &record.pay)
        .tag("confidence", record.confidence.to_string())
        .field("value", record.value);

    Ok(builder.build()?)
}

fn create_datapoint_from_ipr(record: ItemPriceRecord) -> anyhow::Result<DataPoint> {
    let timestamp = get_nano_timestamp(record.date);

    let mut builder = DataPoint::builder("item_price_record")
        .timestamp(timestamp)
        .tag("league", record.league.to_string())
        .tag("id", record.id.to_string())
        .tag("type", &record.item_type)
        .tag("name", &record.name)
        .tag("confidence", record.confidence.to_string())
        .field("value", record.value);

    if !&record.base_type.is_empty() {
        builder = builder.tag("baseType", &record.base_type);
    }
    if !&record.variant.is_empty() {
        builder = builder.tag("variant", &record.variant);
    }
    if !&record.links.is_empty() {
        builder = builder.tag("links", &record.links);
    }

    Ok(builder.build()?)
}

fn get_nano_timestamp(date: NaiveDate) -> i64 {
    let tz_offset = FixedOffset::east_opt(5 * 3600).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let datetime = NaiveDateTime::new(date, time);
    let dt_with_tz: DateTime<FixedOffset> = tz_offset.from_local_datetime(&datetime).unwrap();
    let dt_with_tz_utc: DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());

    dt_with_tz_utc.timestamp_nanos()
}

#[cfg(test)]
mod tests {
    use poeledger_economy_data::{CurrencyPriceRecord, ItemPriceRecord, PriceRecord};
    use serde::{Deserialize, Serialize};

    #[test]
    fn deserialize_currency_price_record() -> anyhow::Result<()> {
        let data = "\
League;Date;Get;Pay;Value;Confidence
Sanctum;2022-12-09;Sacrifice at Dusk;Chaos Orb;1;High
Sanctum;2022-12-10;Sacrifice at Dusk;Chaos Orb;0.9787;High
Sanctum;2022-12-11;Sacrifice at Dusk;Chaos Orb;0.4618;High
Sanctum;2022-12-12;Sacrifice at Dusk;Chaos Orb;0.27212;High
Sanctum;2022-12-13;Sacrifice at Dusk;Chaos Orb;0.182;High
";

        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(data.as_bytes());

        let mut raw_record = csv::StringRecord::new();
        let headers = csv_reader.headers()?.clone();

        let mut count = 0;
        while csv_reader.read_record(&mut raw_record)? {
            let _: CurrencyPriceRecord = raw_record
                .deserialize(Some(&headers))
                .expect("should deserialize to CurrencyPriceRecord");

            count += 1;
        }

        let expected_deser_count = 5;
        assert_eq!(count, expected_deser_count);

        Ok(())
    }

    #[test]
    fn deserialize_item_price_record() -> anyhow::Result<()> {
        let data = "\
League;Date;Id;Type;Name;BaseType;Variant;Links;Value;Confidence
Sanctum;2022-12-09;2673;SkillGem;Dark Pact;;1/20;;1;Low
Sanctum;2022-12-10;2673;SkillGem;Dark Pact;;1/20;;1.42;Medium
Sanctum;2022-12-11;2673;SkillGem;Dark Pact;;1/20;;1.61;High
Sanctum;2022-12-12;2673;SkillGem;Dark Pact;;1/20;;1.99;High
Sanctum;2022-12-13;2673;SkillGem;Dark Pact;;1/20;;2;High
";

        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(data.as_bytes());

        let mut raw_record = csv::StringRecord::new();
        let headers = csv_reader.headers()?.clone();

        let mut count = 0;
        while csv_reader.read_record(&mut raw_record)? {
            let _: ItemPriceRecord = raw_record
                .deserialize(Some(&headers))
                .expect("should deserialize to CurrencyPriceRecord");

            count += 1;
        }

        let expected_deser_count = 5;
        assert_eq!(count, expected_deser_count);

        Ok(())
    }
}
