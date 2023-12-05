use serde::{Deserialize, Serialize};

use super::guild::Guild;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub realm: Option<String>,
    pub guild: Option<Guild>,
    pub challenges: Option<ChallengeSet>,
    pub twitch: Option<TwitchAccount>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ChallengeSet {
    pub set: String,
    pub completed: usize,
    pub max: usize,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TwitchAccount {
    pub name: String,
    pub stream: Option<TwitchStream>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TwitchStream {
    pub name: String,
    pub image: String,
    pub status: String,
}
