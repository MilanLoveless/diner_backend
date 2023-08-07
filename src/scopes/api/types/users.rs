use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UserFormData {
    pub username: String,
    pub avatar: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserRecord {
    pub id: Uuid,
    pub username: String,
    pub avatar: String,
}
