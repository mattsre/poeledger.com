use core::fmt;

use anyhow::Context;
use once_cell::sync::Lazy;
use poe_types::item::Item;
use regex::Regex;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub name: String,
    pub item_id: String,
    pub league: String,
    pub price: ComplexPrice,
    pub implicit_mods: Vec<String>,
    pub explicit_mods: Vec<String>,
    pub created_at: OffsetDateTime,
}

impl Listing {
    pub fn new() -> Self {
        let timestamp = OffsetDateTime::now_utc();

        Self {
            name: Default::default(),
            item_id: Default::default(),
            league: Default::default(),
            price: Default::default(),
            implicit_mods: Default::default(),
            explicit_mods: Default::default(),
            created_at: timestamp,
        }
    }
}

impl Default for Listing {
    fn default() -> Self {
        Self::new()
    }
}

/// ComplexPrice contains a normalized value of a listing at the current time of the chaos to divine conversion
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ComplexPrice {
    /// TODO(stash-processor): value of item normalized to chaos equivalent
    pub normalized_price: f64,
    /// raw listed price
    pub listed_price: f64,
    /// raw listed currency
    pub listed_currency: ListingCurrency,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ListingCurrency {
    Chaos,
    Divine,
    Exalt,
    #[default]
    Unknown,
}

impl From<&str> for ListingCurrency {
    fn from(value: &str) -> Self {
        match value {
            "exa" => ListingCurrency::Exalt,
            "divine" => ListingCurrency::Divine,
            "chaos" => ListingCurrency::Chaos,
            _ => ListingCurrency::Unknown,
        }
    }
}

impl fmt::Display for ListingCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ListingCurrency::Chaos => write!(f, "chaos"),
            ListingCurrency::Divine => write!(f, "divine"),
            ListingCurrency::Exalt => write!(f, "exa"),
            ListingCurrency::Unknown => write!(f, "unknown"),
        }
    }
}

impl TryFrom<Item> for Listing {
    type Error = anyhow::Error;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        let id = item.id.context("items are expected to have an id")?;
        let note = item.note.context("items must have a note to be priced")?;
        let price = note_to_complex_price(&note);

        let timestamp = OffsetDateTime::now_utc();
        Ok(Self {
            name: item.name,
            item_id: id,
            league: item.league.unwrap_or("Affliction".to_owned()),
            price: price.unwrap_or_default(),
            implicit_mods: item.implicit_mods.unwrap_or_default(),
            explicit_mods: item.explicit_mods.unwrap_or_default(),
            created_at: timestamp,
        })
    }
}

pub fn note_to_complex_price(note: &str) -> Option<ComplexPrice> {
    static PRICE_REGEXP: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"~(price|b/o) ([\d\.]+(?:/[\d\.]+)?) ([\w-]+)").expect("price regex must parse")
    });

    match PRICE_REGEXP.captures(note) {
        Some(caps) => {
            if caps.len() == 4 {
                let mut raw_value = 0 as f64;

                if let Some((num, denom)) = caps.get(2).unwrap().as_str().split_once('/') {
                    let raw_num = num.parse::<f64>().unwrap();
                    let raw_denom = denom.parse::<f64>().unwrap();

                    if raw_denom > 0 as f64 {
                        raw_value = raw_num / raw_denom;
                    }
                } else {
                    raw_value = caps.get(2).unwrap().as_str().parse::<f64>().unwrap();
                }

                let currency = ListingCurrency::from(caps.get(3).unwrap().as_str());

                return Some(ComplexPrice {
                    normalized_price: 0 as f64,
                    listed_price: raw_value,
                    listed_currency: currency,
                });
            }

            None
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::listing::ListingCurrency;

    use super::note_to_complex_price;

    #[test]
    fn simple_chaos_note() {
        let note = "~price 70 chaos";
        let price = note_to_complex_price(note);

        let p = price.expect("should unwrap");
        assert_eq!(70 as f64, p.listed_price);
        assert_eq!(ListingCurrency::Chaos, p.listed_currency);
    }

    #[test]
    fn simple_exalt_note() {
        let note = "~price 20 exa";
        let price = note_to_complex_price(note);

        let p = price.expect("should unwrap");
        assert_eq!(20 as f64, p.listed_price);
        assert_eq!(ListingCurrency::Exalt, p.listed_currency);
    }

    #[test]
    fn simple_divine_note() {
        let note = "~b/o 10 divine";
        let price = note_to_complex_price(note);

        let p = price.expect("should unwrap");
        assert_eq!(10 as f64, p.listed_price);
        assert_eq!(ListingCurrency::Divine, p.listed_currency);
    }

    #[test]
    fn fractional_chaos_note() {
        let note = "~price 100/10 chaos";
        let price = note_to_complex_price(note);

        let p = price.expect("should unwrap");
        assert_eq!(10 as f64, p.listed_price);
        assert_eq!(ListingCurrency::Chaos, p.listed_currency);
    }

    #[test]
    fn fractional_divine_note() {
        let note = "~price 5/20 divine";
        let price = note_to_complex_price(note);

        let p = price.expect("should unwrap");
        assert_eq!(0.25 as f64, p.listed_price);
        assert_eq!(ListingCurrency::Divine, p.listed_currency);
    }

    #[test]
    fn float_divine_note() {
        let note = "~price 0.8 divine";
        let price = note_to_complex_price(note);

        let p = price.expect("should unwrap");
        assert_eq!(0.8 as f64, p.listed_price);
        assert_eq!(ListingCurrency::Divine, p.listed_currency);
    }

    #[test]
    fn unknown_currency_note() {
        let note = "~price 3 alch";
        let price = note_to_complex_price(note);

        let p = price.expect("should unwrap");
        assert_eq!(3 as f64, p.listed_price);
        assert_eq!(ListingCurrency::Unknown, p.listed_currency);
    }

    #[test]
    fn invalid_note() {
        let note = "random note on item";
        let price = note_to_complex_price(note);

        assert!(price.is_none());
    }
}
