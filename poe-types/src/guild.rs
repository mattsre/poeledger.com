use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub id: usize,
    pub name: String,
    pub tag: String,
    pub points: Option<usize>,
    pub status_message: Option<String>,
    pub created_at: String,
}
