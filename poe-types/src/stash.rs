use serde::{Deserialize, Serialize};

use super::item::Item;

#[derive(Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicStashChange {
    pub id: String,
    pub public: bool,
    pub account_name: Option<String>,
    pub stash: Option<String>,
    pub stash_type: String,
    pub league: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct StashTab {
    pub id: String,
    pub parent: Option<String>,
    pub name: String,
    pub r#type: String,
    pub index: Option<usize>,
    pub metadata: StashTabMetadata,
    pub children: Option<Vec<Self>>,
    pub items: Option<Vec<Item>>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct StashTabMetadata {
    pub public: Option<bool>,
    pub folder: Option<bool>,
    pub colour: Option<String>,
}
