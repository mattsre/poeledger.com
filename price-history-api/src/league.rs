use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use time::OffsetDateTime;
use time_macros::datetime;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueInfo {
    name: String,
    start_time: Option<i64>,
    end_time: Option<i64>,
}

impl LeagueInfo {
    pub fn new(name: &str, start: Option<OffsetDateTime>, end: Option<OffsetDateTime>) -> Self {
        let mut info = Self {
            name: name.to_owned(),
            start_time: Default::default(),
            end_time: Default::default(),
        };

        if let Some(s) = start {
            let st = s.unix_timestamp();
            info.start_time = Some(st);
        }

        if let Some(e) = end {
            let et = e.unix_timestamp();
            info.end_time = Some(et);
        }

        info
    }
}

pub async fn league_info() -> anyhow::Result<impl IntoResponse, StatusCode> {
    let leagues: Vec<LeagueInfo> = vec![
        LeagueInfo::new("Standard", None, None),
        LeagueInfo::new(
            "Affliction",
            Some(datetime!(2023-12-08 19:00 UTC)),
            Some(datetime!(2023-03-26 21:00 UTC)),
        ),
        LeagueInfo::new("Necropolis", Some(datetime!(2024-03-29 18:00 UTC)), None),
    ];

    Ok(Json(leagues))
}

#[cfg(test)]
mod tests {
    use crate::league::LeagueInfo;
    use time_macros::datetime;

    #[test]
    fn league_start_timestamp() {
        let league = LeagueInfo::new("Affliction", Some(datetime!(2023-12-08 19:00 UTC)), None);

        assert_eq!(league.name, "Affliction");
        assert_eq!(league.start_time, Some(1702062000));
        assert_eq!(league.end_time, None);
    }

    #[test]
    fn league_end_timestamp() {
        let league = LeagueInfo::new(
            "Affliction",
            Some(datetime!(2023-12-08 19:00 UTC)),
            Some(datetime!(2024-03-25 00:00 UTC)),
        );

        assert_eq!(league.name, "Affliction");
        assert_eq!(league.start_time, Some(1702062000));
        assert_eq!(league.end_time, Some(1711324800));
    }
}
