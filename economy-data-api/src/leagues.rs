use axum::{extract::State, Json};
use poeledger_economy_data::PriceRecord;

use crate::ApiState;

pub async fn leagues_handler(State(state): State<ApiState>) -> Json<Vec<String>> {
    tracing::debug!("handling request for leagues");

    let economy_collection = state.db.collection::<PriceRecord>("economy");

    match economy_collection.distinct("league", None, None).await {
        Ok(res) => {
            let leagues: Vec<String> = res
                .iter()
                .map(|f| f.to_string().replace("\"", ""))
                .collect();

            tracing::debug!("returning {} leagues", leagues.len());

            Json(leagues)
        }
        Err(e) => {
            tracing::error!("failed querying mongo for leagues: {:#?}", e);

            Json(vec![])
        }
    }
}
