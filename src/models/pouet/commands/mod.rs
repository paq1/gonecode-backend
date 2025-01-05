use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum PouetCommands {
    Create(CreatePouetCommand),
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreatePouetCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
