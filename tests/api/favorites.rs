use crate::helpers::TestApp;
use serde::Serialize;
use uuid::Uuid;

const FAVORITES: &str = "api/favorites";

impl TestApp {
    pub async fn post_favorites<T>(&self, payload: &T) -> reqwest::Response
    where
        T: Serialize + ?Sized,
    {
        self.post(FAVORITES, payload).await
    }

    pub async fn get_favorites(&self) -> reqwest::Response {
        self.get(FAVORITES).await
    }

    pub async fn delete_favorites(&self, id: Uuid) -> reqwest::Response {
        self.delete(FAVORITES, id).await
    }
}
