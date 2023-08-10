use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("an unknown league was encountered")]
    UnknownLeague,
    #[error("an unknown confidence value was encountered")]
    UnknownConfidence,
    #[error("failed parsing a currency price record")]
    InvalidPriceRecord,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum League {
    #[default]
    Sanctum,
}

impl FromStr for League {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sanctum" => Ok(League::Sanctum),
            _ => Err(Error::UnknownLeague),
        }
    }
}

impl ToString for League {
    fn to_string(&self) -> String {
        match self {
            League::Sanctum => "Sanctum".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum Confidence {
    High,
    #[default]
    Medium,
    Low,
}

impl FromStr for Confidence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "High" => Ok(Confidence::High),
            "Medium" => Ok(Confidence::Medium),
            "Low" => Ok(Confidence::Low),
            _ => Err(Error::UnknownConfidence),
        }
    }
}

impl ToString for Confidence {
    fn to_string(&self) -> String {
        match self {
            Confidence::High => "High".to_owned(),
            Confidence::Medium => "Medium".to_owned(),
            Confidence::Low => "Low".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct CurrencyPriceRecord {
    #[serde(rename = "League")]
    pub league: League,
    #[serde(rename = "Date")]
    pub date: NaiveDate,
    #[serde(rename = "Get")]
    pub get: String,
    #[serde(rename = "Pay")]
    pub pay: String,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Confidence")]
    pub confidence: Confidence,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ItemPriceRecord {
    #[serde(rename = "League")]
    pub league: League,
    #[serde(rename = "Date")]
    pub date: NaiveDate,
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Type")]
    pub item_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "BaseType")]
    pub base_type: Option<String>,
    #[serde(rename = "Variant")]
    pub variant: Option<String>,
    #[serde(rename = "Links")]
    pub links: Option<String>,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Confidence")]
    pub confidence: Confidence,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PriceRecord {
    CurrencyPriceRecord(CurrencyPriceRecord),
    ItemPriceRecord(ItemPriceRecord),
}
