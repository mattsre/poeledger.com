use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::AppState;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHistory {
    item_name: String,
    league: String,
    events: Vec<PriceHistoryEvent>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHistoryEvent {
    listed_currency: ListingCurrency,
    listed_price: f64,
    listed_date: Datetime,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ItemListing {
    pub name: String,
    pub item_id: String,
    pub league: String,
    pub price: ComplexPrice,
    pub implicit_mods: Vec<String>,
    pub explicit_mods: Vec<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ComplexPrice {
    /// TODO(stash-processor): value of item normalized to chaos equivalent
    pub normalized_value: f64,
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

#[derive(Deserialize)]
pub struct HistoryQuery {
    item: String,
}

pub async fn history_by_name(
    Query(params): Query<HistoryQuery>,
    State(state): State<AppState>,
) -> Json<PriceHistory> {
    let stripped_name = params
        .item
        .trim()
        .replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "");

    let sql = "SELECT * FROM type::table($table) WHERE name = type::string($name)";
    let mut query_response = state
        .db
        .query(sql)
        .bind(("table", "item_listings"))
        .bind(("name", &stripped_name))
        .await
        .unwrap();

    let listings: Vec<ItemListing> = query_response.take(0).unwrap();

    let mut price_history = PriceHistory {
        item_name: stripped_name,
        league: "Affliction".to_owned(),
        events: Vec::new(),
    };

    for listing in listings {
        let price_event = PriceHistoryEvent {
            listed_currency: listing.price.listed_currency,
            listed_price: listing.price.listed_price,
            listed_date: listing.updated_at,
        };

        price_history.events.push(price_event);
    }

    Json(price_history)
}
