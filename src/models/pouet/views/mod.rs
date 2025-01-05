use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "statusType")]
pub enum PouetViewState {
    #[serde(rename = "pouet")]
    Create(PouetView),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PouetView {
    #[serde(flatten)]
    pub data: PouetDataView,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct PouetDataView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
#[serde(tag = "eventType")]
pub enum PouetViewEvent {
    #[serde(rename = "created")]
    Created(PouetCreatedView),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct PouetCreatedView {
    #[serde(flatten)]
    pub data: PouetDataView,
    pub by: String,
    pub at: DateTime<Utc>,
}
