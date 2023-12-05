use serde::{Deserialize, Serialize};

use super::account::Account;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LadderEntry {
    pub rank: usize,
    pub dead: Option<bool>,
    pub retired: Option<bool>,
    pub ineligible: Option<bool>,
    pub public: Option<bool>,
    pub character: LadderCharacter,
    pub account: Option<Account>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LadderCharacter {
    pub id: String,
    pub name: String,
    pub level: usize,
    pub class: String,
    pub time: Option<usize>,
    pub score: Option<usize>,
    pub experience: Option<usize>,
    pub depth: Option<DelveLadderEntry>,
    pub account: Option<Account>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct DelveLadderEntry {
    pub default: Option<usize>,
    pub solo: Option<usize>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct EventLadderEntry {
    pub rank: usize,
    pub ineligible: Option<bool>,
    pub time: Option<usize>,
    pub private_league: Option<EventPrivateLeague>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct EventPrivateLeague {
    pub name: String,
    pub url: String,
}
