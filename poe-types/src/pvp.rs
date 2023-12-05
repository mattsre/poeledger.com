use serde::{Deserialize, Serialize};

use super::account::Account;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub enum PvpStyle {
    Blitz,
    Swiss,
    #[default]
    Arena,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PvpMatch {
    pub id: String,
    pub realm: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub url: Option<String>,
    pub description: String,
    pub glicko_ratings: bool,
    pub pvp: bool,
    pub style: PvpStyle,
    pub register_at: Option<String>,
    pub complete: Option<bool>,
    pub upcoming: Option<bool>,
    pub in_progress: Option<bool>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PvpLadderTeamEntry {
    pub rank: usize,
    pub rating: Option<usize>,
    pub points: Option<usize>,
    pub games_played: Option<usize>,
    pub cumulative_opponent_points: Option<usize>,
    pub last_game_time: Option<String>,
    pub members: Vec<PvpLadderTeamMember>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PvpLadderTeamMember {
    pub account: Account,
    pub character: PvpCharacter,
    pub public: Option<bool>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PvpCharacter {
    pub id: String,
    pub name: String,
    pub level: usize,
    pub class: String,
    pub league: Option<String>,
    pub score: Option<usize>,
}
