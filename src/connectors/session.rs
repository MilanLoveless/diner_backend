use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SessionRecord {
    pub username: String,
    pub avatar: String,
}

#[derive(Clone)]
pub struct SessionStore {}

impl SessionStore {
    pub fn create(&self) -> Result<SessionRecord, String> {
        Ok(SessionRecord {
            username: "@10xmilan".to_string(),
            avatar: "https://example.com/images/milan".to_string(),
        })
    }
}
