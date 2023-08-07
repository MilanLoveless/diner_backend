use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct UserFormData {
    username: String,
    avatar: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserRecord {
    id: Uuid,
    username: String,
    avatar: String,
}
