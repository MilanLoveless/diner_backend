use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UserFormData {
    pub username: String,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub global_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UserRecord {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub global_name: Option<String>,
}
