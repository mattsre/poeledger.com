use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use serde::Deserialize;
use time::{Duration, OffsetDateTime};

use crate::{
    db::{are_valid_quantiles, ChInterval, ChTimeframe},
    AppState,
};

#[derive(Deserialize)]
pub struct PriceHistoryQuery {
    item_name: String,
    interval: Option<String>,
    quantiles: Option<Vec<f64>>,
    start_time: Option<i64>,
    end_time: Option<i64>,
}

pub async fn history_by_name(
    Query(params): Query<PriceHistoryQuery>,
    State(state): State<AppState>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let name = params.item_name;
    let interval = match params.interval {
        Some(i) => ChInterval::try_from(i).map_err(|_| StatusCode::BAD_REQUEST)?,
        None => ChInterval::Hour(1),
    };

    let quantiles = match params.quantiles {
        Some(q) => {
            if !are_valid_quantiles(&q) {
                return Err(StatusCode::BAD_REQUEST);
            }

            q
        }
        None => vec![0.1],
    };

    let timeframe = match (params.start_time, params.end_time) {
        (None, None) => ChTimeframe {
            start: OffsetDateTime::now_utc()
                .saturating_sub(Duration::days(7))
                .unix_timestamp(),
            end: OffsetDateTime::now_utc().unix_timestamp(),
        },
        (None, Some(end)) => ChTimeframe {
            start: OffsetDateTime::now_utc()
                .saturating_sub(Duration::days(7))
                .unix_timestamp(),
            end,
        },
        (Some(start), None) => ChTimeframe {
            start,
            end: OffsetDateTime::now_utc().unix_timestamp(),
        },
        (Some(start), Some(end)) => ChTimeframe::new(start, end),
    };

    match state
        .db
        .query_ledger_by_name(&name, interval, quantiles, timeframe)
        .await
    {
        Ok(results) => Ok(Json(results)),
        Err(e) => {
            tracing::error!("failed querying ledger: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
