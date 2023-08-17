use axum::{extract::State, Json};
use poeledger_economy_data::PriceRecord;

use crate::ApiState;

pub async fn filters_handler(State(state): State<ApiState>) -> Json<Vec<String>> {
    let economy_collection = state.db.collection::<PriceRecord>("economy");

    match economy_collection.distinct("name", None, None).await {
        Ok(res) => {
            let filters: Vec<String> = res
                .iter()
                .map(|f| f.to_string().replace("\"", ""))
                .collect();

            Json(filters)
        }
        Err(e) => {
            eprintln!("failed querying mongo for filters: {:#?}", e);

            Json(vec![])
        }
    }
}
