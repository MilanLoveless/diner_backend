use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct GameFormData {
    pub name: String,
    pub description: String,
    pub link: String,
}

#[derive(Deserialize, Serialize)]
pub struct GameRecord {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub link: String,
}
