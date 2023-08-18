use axum::{
    extract::{Query as AxumQuery, State},
    Json,
};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use poeledger_economy_data::PriceRecord;
use serde::Deserialize;

use crate::{
    utils::{empty_string_as_none, sanitize_mongo_string},
    ApiState,
};

#[derive(Debug, Deserialize, Clone)]
pub struct PricesQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    name: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    league: Option<String>,
}

pub async fn prices_handler(
    State(state): State<ApiState>,
    query: AxumQuery<PricesQuery>,
) -> Json<Vec<PriceRecord>> {
    tracing::debug!(
        "handling request for prices with query params: {:#?}",
        &query
    );

    let economy_collection = state.db.collection::<PriceRecord>("economy");

    let name = match query.name.clone() {
        Some(n) => sanitize_mongo_string(n),
        None => "Divine Orb".to_owned(),
    };

    let league = match query.league.clone() {
        Some(l) => sanitize_mongo_string(l),
        None => "Sanctum".to_owned(),
    };

    let filter = doc! { "name": name, "league": league };
    match economy_collection.find(filter, None).await {
        Ok(mut cursor) => {
            let mut records: Vec<PriceRecord> = Vec::new();
            while let Some(r) = cursor.try_next().await.unwrap() {
                records.push(r);
            }

            tracing::debug!(
                "returning {} documents for this prices query",
                records.len()
            );

            Json(records)
        }
        Err(e) => {
            tracing::error!("failed querying mongo for prices: {:#?}", e);

            Json(vec![])
        }
    }
}
