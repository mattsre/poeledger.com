use std::env;

use anyhow::anyhow;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub fn are_valid_quantiles(quantiles: &Vec<f64>) -> bool {
    for q in quantiles {
        if !(&0.0..&1.0).contains(&q) {
            return false;
        }
    }

    true
}

pub enum ChInterval {
    Minute(i32),
    Hour(i32),
    Week(i32),
    Month(i32),
    Year(i32),
}

impl TryFrom<String> for ChInterval {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut iter = value.trim().split(' ');
        let x = iter.next();
        let unit = iter.next();

        if let (Some(x), Some(unit)) = (x, unit) {
            let xi = x.parse::<i32>().unwrap();

            let interval = match unit {
                "minute" => ChInterval::Minute(xi),
                "hour" => ChInterval::Hour(xi),
                "week" => ChInterval::Week(xi),
                "month" => ChInterval::Month(xi),
                "year" => ChInterval::Year(xi),
                _ => {
                    tracing::warn!("recieved invalid interval unit: {}", unit);
                    return Err(anyhow!("invalid interval unit"));
                }
            };

            return Ok(interval);
        }

        Err(anyhow!("invalid interval"))
    }
}

impl ToString for ChInterval {
    fn to_string(&self) -> String {
        match self {
            ChInterval::Minute(x) => format!("{x} minute"),
            ChInterval::Hour(x) => format!("{x} hour"),
            ChInterval::Week(x) => format!("{x} week"),
            ChInterval::Month(x) => format!("{x} month"),
            ChInterval::Year(x) => format!("{x} year"),
        }
    }
}

#[derive(Default)]
pub struct ChTimeframe {
    pub start: i64,
    pub end: i64,
}

impl ChTimeframe {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

#[derive(Clone)]
pub struct ClickhouseDatabase {
    client: clickhouse::Client,
}

#[derive(Serialize, Deserialize, Row)]
pub struct PriceHistoryBucketRow {
    pub item_name: String,
    #[serde(with = "clickhouse::serde::time::datetime")]
    pub interval_bucket: OffsetDateTime,
    pub price_by_quantile: Vec<(f64, f64)>,
    pub listed_currency: String,
}

impl ClickhouseDatabase {
    pub fn new() -> Self {
        let url = env::var("CLICKHOUSE_URL").unwrap_or("http://localhost:8123".to_string());
        let dbname = "ledger";

        let client = clickhouse::Client::default()
            .with_url(url)
            .with_database(dbname);

        Self { client }
    }

    pub async fn query_ledger_by_name(
        &self,
        name: &str,
        interval: ChInterval,
        quantiles: Vec<f64>,
        timeframe: ChTimeframe,
    ) -> anyhow::Result<Vec<PriceHistoryBucketRow>> {
        let quants = quantiles
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let ChTimeframe { start, end } = timeframe;

        let raw_query = format!(
            "SELECT
                name as item_name,
                toStartOfInterval(created_at, INTERVAL {}) AS interval_bucket,
                arrayZip([{quants}], quantiles({quants})(listed_price)) AS price_by_quantile,
                listed_currency
            FROM ledger.listings
            WHERE name = ? AND created_at BETWEEN {start} AND {end}
            GROUP BY interval_bucket, name, listed_currency
            ORDER BY interval_bucket",
            interval.to_string(),
        );

        let rows = self
            .client
            .query(&raw_query)
            .bind(name)
            .fetch_all::<PriceHistoryBucketRow>()
            .await?;

        Ok(rows)
    }
}

impl Default for ClickhouseDatabase {
    fn default() -> Self {
        Self::new()
    }
}
