use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    item::{Item, ItemJewelData},
    passives::PassiveNode,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BanditChoice {
    Kraityn,
    Alira,
    Oak,
    Eramir,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PantheonMajor {
    TheBrineKing,
    Arakaali,
    Solaris,
    Lunaris,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PantheonMinor {
    Abberath,
    Grukthul,
    Yugul,
    Shakari,
    Tukohama,
    Ralakesh,
    Garukhan,
    Ryslatha,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub realm: String,
    pub class: String,
    pub league: Option<String>,
    pub level: usize,
    pub experience: usize,
    pub ruthless: Option<bool>,
    pub expired: Option<bool>,
    pub deleted: Option<bool>,
    pub current: Option<bool>,
    pub equipment: Option<Vec<Item>>,
    pub inventory: Option<Vec<Item>>,
    pub jewels: Option<Vec<Item>>,
    pub passives: Option<CharacterPassives>,
    pub metadata: Option<CharacterMetadata>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct CharacterPassives {
    pub hashes: Vec<usize>,
    pub hashes_ex: Vec<usize>,
    pub mastery_effects: HashMap<String, i64>,
    pub skill_overrides: HashMap<String, PassiveNode>,
    pub bandit_choice: Option<BanditChoice>,
    pub pantheon_major: Option<PantheonMajor>,
    pub pantheon_minor: Option<PantheonMinor>,
    pub jewel_data: HashMap<String, ItemJewelData>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CharacterMetadata {
    pub version: String,
}
