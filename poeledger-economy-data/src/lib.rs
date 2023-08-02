use std::str::FromStr;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed parsing a currency price record")]
    InvalidPriceRecord,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum League {
    #[default]
    Sanctum,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum Confidence {
    High,
    #[default]
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct CurrencyPriceRecord {
    pub league: League,
    pub date: NaiveDate,
    pub get: String,
    pub pay: String,
    pub value: f32,
    pub confidence: Confidence,
}

impl TryFrom<String> for CurrencyPriceRecord {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        let parts: Vec<&str> = value.split(";").collect();

        let league = match parts.get(0).unwrap() {
            &"Sanctum" => League::Sanctum,
            _ => League::default(),
        };

        let date = NaiveDate::from_str(parts.get(1).unwrap()).unwrap();

        let confidence = match parts.get(5).unwrap() {
            &"High" => Confidence::High,
            &"Medium" => Confidence::Medium,
            &"Low" => Confidence::Low,
            _ => Confidence::default(),
        };

        Ok(Self {
            league,
            date,
            get: parts.get(2).unwrap().to_string(),
            pay: parts.get(3).unwrap().to_string(),
            value: parts.get(4).unwrap().parse::<f32>().unwrap(),
            confidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_price_record_try_from() {
        let line = "Sanctum;2022-12-10;Sacrifice at Dusk;Chaos Orb;0.9787;High".to_owned();
        let expected_record = CurrencyPriceRecord {
            league: League::Sanctum,
            date: NaiveDate::from_ymd_opt(2022, 12, 10).unwrap(),
            get: "Sacrifice at Dusk".to_owned(),
            pay: "Chaos Orb".to_owned(),
            value: "0.9787".parse::<f32>().unwrap(),
            confidence: Confidence::High,
        };

        let parsed_line = CurrencyPriceRecord::try_from(line).expect("line should parse");
        assert_eq!(expected_record, parsed_line);
    }
}
