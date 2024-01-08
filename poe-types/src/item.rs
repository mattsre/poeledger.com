use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::passives::{PassiveGroup, PassiveNode};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ColourAttr {
    /// Strength
    S,
    /// Dexterity
    D,
    /// Intelligence
    I,
    /// Generic
    G,
    /// Abyss
    A,
    /// Delve
    Dv,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SocketColour {
    /// Red
    R,
    /// Green
    G,
    /// Blue
    B,
    /// White
    W,
    /// Abyss
    A,
    /// Delve
    Dv,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FrameType {
    /// Normal
    Normal = 0,
    Magic = 1,
    Rare = 2,
    Unique = 3,
    Gem = 4,
    Currency = 5,
    DivinationCard = 6,
    Quest = 7,
    Prophecy = 8,
    Foil = 9,
    SupporterFoil = 10,
}

#[derive(Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub verified: bool,
    pub w: usize,
    pub h: usize,
    pub icon: String,
    pub support: Option<bool>,
    pub stack_size: Option<i64>,
    pub max_stack_size: Option<i64>,
    pub stack_size_text: Option<String>,
    pub league: Option<String>,
    pub id: Option<String>,
    pub influences: Option<HashMap<String, bool>>,
    pub elder: Option<bool>,
    pub shaper: Option<bool>,
    pub searing: Option<bool>,
    pub tangled: Option<bool>,
    pub abyss_jewel: Option<bool>,
    pub delve: Option<bool>,
    pub fractured: Option<bool>,
    pub synthesized: Option<bool>,
    pub sockets: Option<Vec<ItemSocket>>,
    pub socketed_items: Option<Vec<Self>>,
    pub name: String,
    pub type_line: String,
    pub base_type: String,
    pub identified: bool,
    pub item_level: Option<i32>,
    pub note: Option<String>,
    pub forum_note: Option<String>,
    pub locked_to_character: Option<bool>,
    pub locked_to_account: Option<bool>,
    pub duplicated: Option<bool>,
    pub split: Option<bool>,
    pub corrupted: Option<bool>,
    pub unmodifiable: Option<bool>,
    pub cis_race_reward: Option<bool>,
    pub sea_race_reward: Option<bool>,
    pub th_race_reward: Option<bool>,
    pub properties: Option<Vec<ItemProperty>>,
    pub notable_properties: Option<Vec<ItemProperty>>,
    pub requirements: Option<Vec<ItemProperty>>,
    pub additional_properties: Option<Vec<ItemProperty>>,
    pub next_level_requirements: Option<Vec<ItemProperty>>,
    pub talisman_tier: Option<i64>,
    pub rewards: Option<Vec<ItemRewards>>,
    pub sec_descr_text: Option<String>,
    pub utility_mods: Option<Vec<String>>,
    pub logbook_mods: Option<Vec<LogbookArea>>,
    pub enchant_mods: Option<Vec<String>>,
    pub scourge_mods: Option<Vec<String>>,
    pub implicit_mods: Option<Vec<String>>,
    pub ultimatum_mods: Option<Vec<UltimatumMod>>,
    pub explicit_mods: Option<Vec<String>>,
    pub crafted_mods: Option<Vec<String>>,
    pub fractured_mods: Option<Vec<String>>,
    pub crucible_mods: Option<Vec<String>>,
    pub cosmetic_mods: Option<Vec<String>>,
    pub veiled_mods: Option<Vec<String>>,
    pub veiled: Option<bool>,
    pub descr_text: Option<String>,
    pub flavour_text: Option<Vec<String>>,
    pub flavour_text_note: Option<String>,
    pub prophecy_text: Option<String>,
    pub is_relic: Option<bool>,
    pub foil_variation: Option<i64>,
    pub replica: Option<bool>,
    pub foreseeing: Option<bool>,
    pub incubated_item: Option<Incubator>,
    pub scourged: Option<ScourgeStatus>,
    #[serde(skip)]
    pub crucible: Option<CrucibleTree>,
    pub ruthless: Option<bool>,
    pub frame_type: Option<FrameType>,
    pub art_filename: Option<String>,
    pub hybrid: Option<HybridValues>,
    pub extended: Option<ExtendedValues>,
    pub x: Option<usize>,
    pub y: Option<usize>,
    pub inventory_id: Option<String>,
    pub socket: Option<usize>,
    pub colour: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSocket {
    pub group: usize,
    pub attr: Option<ColourAttr>,
    pub s_colour: Option<SocketColour>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemProperty {
    pub name: String,
    pub values: Vec<(String, usize)>,
    pub display_mode: usize,
    pub progress: Option<f64>,
    pub r#type: Option<usize>,
    pub suffix: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ItemRewards {
    pub label: String,
    pub rewards: HashMap<String, i64>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LogbookArea {
    pub name: String,
    pub faction: LogbookFaction,
    pub mods: Vec<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct LogbookFaction {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct UltimatumMod {
    pub r#type: String,
    pub tier: usize,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Incubator {
    pub name: String,
    pub level: usize,
    pub progress: usize,
    pub total: usize,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ScourgeStatus {
    pub tier: usize,
    pub level: Option<usize>,
    pub progress: Option<usize>,
    pub total: Option<usize>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct CrucibleTree {
    pub layout: String,
    pub nodes: Vec<CrucibleNode>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrucibleNode {
    pub skill: Option<usize>,
    pub tier: Option<usize>,
    pub icon: Option<String>,
    pub allocated: Option<bool>,
    pub is_notable: Option<bool>,
    pub is_reward: Option<bool>,
    pub stats: Option<Vec<String>>,
    pub reminder_text: Option<Vec<String>>,
    pub orbit: Option<usize>,
    pub orbit_index: Option<usize>,
    pub out: Vec<String>,
    pub r#in: Vec<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridValues {
    pub is_vaal_gem: Option<bool>,
    pub base_type_name: String,
    pub properties: Option<Vec<ItemProperty>>,
    pub explicit_mods: Option<Vec<String>>,
    pub sec_descr_text: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ExtendedValues {
    pub category: Option<String>,
    pub subcategories: Option<Vec<String>>,
    pub prefixes: Option<usize>,
    pub suffixes: Option<usize>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemJewelData {
    pub r#type: String,
    pub radius: Option<usize>,
    pub radius_min: Option<usize>,
    pub radius_visual: Option<String>,
    pub subgraph: Option<ItemSubgraph>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct ItemSubgraph {
    pub groups: HashMap<String, PassiveGroup>,
    pub nodes: HashMap<String, PassiveNode>,
}
