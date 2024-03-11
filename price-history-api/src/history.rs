use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use serde::Deserialize;
use time::{Duration, OffsetDateTime};

use crate::{
    db::{are_valid_quantiles, ChInterval, ChTimeframe},
    AppState,
};

const DEFAULT_INTERVAL: ChInterval = ChInterval::Hour(3);
const DEFAULT_HISTORY_DURATION: Duration = Duration::days(7);
const DEFAULT_LEAGUE: &str = "Affliction";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHistoryQuery {
    item: String,
    league: Option<String>,
    interval_amount: Option<i64>,
    interval_unit: Option<String>,
    quantiles: Option<Vec<String>>,
    start_time: Option<i64>,
    end_time: Option<i64>,
}

pub async fn history_by_name(
    Query(params): Query<PriceHistoryQuery>,
    State(state): State<AppState>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let league = params.league.unwrap_or(DEFAULT_LEAGUE.to_owned());

    let interval = match (params.interval_amount, params.interval_unit) {
        (None, None) => Ok(DEFAULT_INTERVAL),
        (None, Some(unit)) => ChInterval::try_from(format!("{} {unit}", 1)),
        (Some(amt), None) => ChInterval::try_from(format!("{amt} {}", "hour")),
        (Some(amt), Some(unit)) => ChInterval::try_from(format!("{amt} {unit}")),
    }
    .unwrap_or(DEFAULT_INTERVAL);

    let quantiles = match params.quantiles {
        Some(q) => {
            let fq = q.iter().map(|v| v.parse::<f64>().unwrap()).collect();
            if !are_valid_quantiles(&fq) {
                return Err(StatusCode::BAD_REQUEST);
            }

            fq
        }
        None => vec![0.1],
    };

    let timeframe = match (params.start_time, params.end_time) {
        (None, None) => ChTimeframe {
            start: OffsetDateTime::now_utc()
                .saturating_sub(DEFAULT_HISTORY_DURATION)
                .unix_timestamp(),
            end: OffsetDateTime::now_utc().unix_timestamp(),
        },
        (None, Some(end)) => ChTimeframe {
            start: OffsetDateTime::now_utc()
                .saturating_sub(DEFAULT_HISTORY_DURATION)
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
        .query_ledger_by_name(&params.item, &league, interval, quantiles, timeframe)
        .await
    {
        Ok(results) => Ok(Json(results)),
        Err(e) => {
            tracing::error!("failed querying ledger: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
