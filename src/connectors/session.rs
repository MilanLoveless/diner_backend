use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct SessionRecord {
    pub username: String,
    pub avatar: String,
}

#[derive(Clone)]
pub struct SessionStore {
    url: String,
}

impl SessionStore {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn create(&self, id: Uuid) -> Result<SessionRecord, String> {
        Ok(SessionRecord {
            username: "@10xmilan".to_string(),
            avatar: "https://example.com/images/milan".to_string(),
        })
    }
}
