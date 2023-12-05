use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub enum FilterType {
    #[default]
    Normal,
    Ruthless,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ItemFilter {
    pub id: String,
    pub filter_name: String,
    pub realm: String,
    pub description: String,
    pub version: String,
    pub r#type: FilterType,
    pub public: Option<bool>,
    pub filter: Option<String>,
    pub validation: Option<FilterValidation>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct FilterValidation {
    pub valid: bool,
    pub version: Option<String>,
    pub validated: Option<String>,
}
