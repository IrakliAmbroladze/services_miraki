use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DessertDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DessertsResponse {
    pub items: Vec<DessertDto>,
}
