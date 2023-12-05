use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PassiveGroup {
    pub x: f64,
    pub y: f64,
    pub orbits: Vec<usize>,
    pub is_proxy: Option<bool>,
    pub proxy: Option<String>,
    pub nodes: Vec<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PassiveNode {
    pub skill: Option<usize>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub is_keystone: Option<bool>,
    pub is_notable: Option<bool>,
    pub is_mastery: Option<bool>,
    pub inactive_icon: Option<String>,
    pub active_icon: Option<String>,
    pub active_effect_image: Option<String>,
    pub mastery_effects: Option<Vec<MasteryEffect>>,
    pub is_blighted: Option<bool>,
    pub is_tattoo: Option<bool>,
    pub is_proxy: Option<bool>,
    pub is_jewel_socket: Option<bool>,
    pub expansion_jewel: Option<ClusterJewel>,
    pub recipe: Option<Vec<String>>,
    pub granted_strength: Option<usize>,
    pub granted_dexterity: Option<usize>,
    pub granted_intelligence: Option<usize>,
    pub ascendancy_name: Option<String>,
    pub is_ascendancy_start: Option<bool>,
    pub is_multiple_choice: Option<bool>,
    pub is_multiple_choice_option: Option<bool>,
    pub granted_passive_points: Option<usize>,
    pub stats: Option<Vec<String>>,
    pub reminder_text: Option<Vec<String>>,
    pub flavour_text: Option<Vec<String>>,
    pub class_start_index: Option<usize>,
    pub group: Option<usize>,
    pub orbit: Option<usize>,
    pub orbit_index: Option<usize>,
    pub out: Vec<String>,
    pub r#in: Vec<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct MasteryEffect {
    pub effect: usize,
    pub stats: Vec<String>,
    pub reminder_text: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ClusterJewel {
    pub size: Option<usize>,
    pub index: Option<usize>,
    pub proxy: Option<usize>,
    pub parent: Option<usize>,
}
