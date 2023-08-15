//! tests/subscribe.rs
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
use crate::helpers::{spawn_app, TestApp};
use diner_backend::types::GameFormData;
use rstest::rstest;
use sqlx::query;
use uuid::Uuid;

const GAMES: &str = "api/games";

impl TestApp {
    pub async fn post_games(&self, payload: &GameFormData) -> reqwest::Response {
        self.post(GAMES, payload).await
    }

    pub async fn get_games(&self) -> reqwest::Response {
        self.get(GAMES).await
    }

    pub async fn get_games_by_id(&self, id: Uuid) -> reqwest::Response {
        self.get_by_id(GAMES, id).await
    }

    pub async fn put_games(&self, payload: &GameFormData, id: Uuid) -> reqwest::Response {
        self.put(GAMES, payload, id).await
    }

    pub async fn delete_games(&self, id: Uuid) -> reqwest::Response {
        self.delete(GAMES, id).await
    }
}

const GAME_NAME: &str = "Blades in the Dark";
const GAME_DESCRIPTION: &str = "You’re in a haunted Victorian-era city trapped inside a wall of lightning powered by demon blood.";
const GAME_LINK: &str = "https://bladesinthedark.com/greetings-scoundrel";

#[tokio::test]
async fn post_game_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;

    let payload = GameFormData {
        name: GAME_NAME.to_string(),
        description: GAME_DESCRIPTION.to_string(),
        link: GAME_LINK.to_string(),
    };

    // Act
    let response = app.post_games(&payload).await;
    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
    let saved = query!("SELECT name, description, link FROM games",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved game.");
    assert_eq!(saved.name, GAME_NAME);
    assert_eq!(saved.description, GAME_DESCRIPTION);
    assert_eq!(saved.link, GAME_LINK);
}

#[rstest]
#[case("", GAME_DESCRIPTION, GAME_LINK, "missing the name")]
#[case(GAME_NAME, "", GAME_LINK, "missing the decription")]
#[case(GAME_NAME, GAME_DESCRIPTION, "", "missing the link")]
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(
    #[case] name: &str,
    #[case] description: &str,
    #[case] link: &str,
    #[case] error_message: String,
) {
    // Arrange
    let app = spawn_app().await;
    let payload = GameFormData {
        name: name.to_string(),
        description: description.to_string(),
        link: link.to_string(),
    };

    // Act
    let response = app.post_games(&payload).await;
    // Assert
    assert_eq!(
        400,
        response.status().as_u16(),
        // Additional customised error message on test failure
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message
    );
}
