use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub realm: Option<String>,
    pub description: Option<String>,
    pub rules: Option<Vec<LeagueRule>>,
    pub register_at: Option<String>,
    pub event: Option<bool>,
    pub url: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub timed_event: Option<bool>,
    pub score_event: Option<bool>,
    pub delve_event: Option<bool>,
    pub ancestor_event: Option<bool>,
    pub league_event: Option<bool>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LeagueRule {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LeagueAccount {
    pub atlas_passives: Option<AtlasPassives>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AtlasPassives {
    pub hashes: Vec<usize>,
}
