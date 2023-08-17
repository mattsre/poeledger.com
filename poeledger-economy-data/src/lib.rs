use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typeshare::typeshare;

#[derive(Error, Debug)]
pub enum Error {
    #[error("an unknown league was encountered")]
    UnknownLeague,
    #[error("an unknown confidence value was encountered")]
    UnknownConfidence,
    #[error("an unknown item link value was encountered")]
    UnknownItemLinks,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum League {
    Crucible,
    #[default]
    Sanctum,
    Kalandra,
    Sentinel,
    Archnemesis,
    Scourge,
    Expedition,
    Ultimatum,
    Ritual,
    Heist,
}

impl FromStr for League {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Crucible" => Ok(League::Crucible),
            "Sanctum" => Ok(League::Sanctum),
            "Kalandra" => Ok(League::Kalandra),
            "Sentinel" => Ok(League::Sentinel),
            "Archnemesis" => Ok(League::Archnemesis),
            "Scourge" => Ok(League::Scourge),
            "Expedition" => Ok(League::Expedition),
            "Ultimatum" => Ok(League::Ultimatum),
            "Ritual" => Ok(League::Ritual),
            "Heist" => Ok(League::Heist),
            _ => Err(Error::UnknownLeague),
        }
    }
}

impl ToString for League {
    fn to_string(&self) -> String {
        match self {
            League::Crucible => "Crucible".to_owned(),
            League::Sanctum => "Sanctum".to_owned(),
            League::Kalandra => "Kalandra".to_owned(),
            League::Sentinel => "Sentinel".to_owned(),
            League::Archnemesis => "Archnemesis".to_owned(),
            League::Scourge => "Scourge".to_owned(),
            League::Expedition => "Expedition".to_owned(),
            League::Ultimatum => "Ultimatum".to_owned(),
            League::Ritual => "Ritual".to_owned(),
            League::Heist => "Heist".to_owned(),
        }
    }
}

#[typeshare]
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

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum ItemLinks {
    #[default]
    #[serde(rename = "1-4 links")]
    OneToFour,
    #[serde(rename = "5 links")]
    Five,
    #[serde(rename = "6 links")]
    Six,
}

impl FromStr for ItemLinks {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1-4 links" => Ok(ItemLinks::OneToFour),
            "5 links" => Ok(ItemLinks::Five),
            "6 links" => Ok(ItemLinks::Six),
            _ => Err(Error::UnknownItemLinks),
        }
    }
}

impl ToString for ItemLinks {
    fn to_string(&self) -> String {
        match self {
            ItemLinks::OneToFour => "1-4 links".to_owned(),
            ItemLinks::Five => "5 links".to_owned(),
            ItemLinks::Six => "6 links".to_owned(),
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PriceRecord {
    pub league: League,
    pub confidence: Confidence,
    pub date: NaiveDate,
    pub value: f32,
    pub name: String,
    #[serde(rename = "itemId")]
    pub item_id: Option<i32>,
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,
    #[serde(rename = "baseType")]
    pub base_type: Option<String>,
    #[serde(rename = "itemVariant")]
    pub item_variant: Option<String>,
    #[serde(rename = "itemLinks")]
    pub item_links: Option<ItemLinks>,
}
