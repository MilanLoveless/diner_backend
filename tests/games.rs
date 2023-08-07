// //! tests/subscribe.rs
// // `tokio::test` is the testing equivalent of `tokio::main`.
// // It also spares you from having to specify the `#[test]` attribute. //
// // You can inspect what code gets generated using
// // `cargo expand --test health_check` (<- name of the test file)
// use rstest::rstest;
// use sqlx::query;
// mod common;
// use common::spawn_app;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct GameForm {
//     name: String,
//     description: String,
//     link: String,
// }

// #[tokio::test]
// async fn post_game_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();
//     // Act
//     let body = "name=Blades%20in%20the%20Dark&description=You’re%20in%20ahaunted Victorian-era city trapped inside a wall of lightning powered by demon blood.%40gmail.com";
//     let response = client
//         .post(&format!("{}/subscriptions", &app.address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     // Assert
//     assert_eq!(200, response.status().as_u16());
//     assert_eq!(Some(0), response.content_length());
//     let saved = query!("SELECT name, description, link FROM games",)
//         .fetch_one(&app.pool)
//         .await
//         .expect("Failed to fetch saved subscription.");
//     assert_eq!(saved.name, "Blades in the Dark");
//     assert_eq!(saved.description, "You’re in a haunted Victorian-era city trapped inside a wall of lightning powered by demon blood.");
//     assert_eq!(
//         saved.link,
//         "https://evilhat.com/product/blades-in-the-dark/"
//     );
// }

// #[rstest]
// #[case("name=le%20guin", "missing the email")]
// #[case("email=ursula_le_guin%40gmail.com", "missing the name")]
// #[case("", "missing both name and email")]
// #[tokio::test]
// async fn subscribe_returns_a_400_when_data_is_missing(
//     #[case] invalid_body: String,
//     #[case] error_message: String,
// ) {
//     // Arrange
//     let app = spawn_app().await;
//     let client = reqwest::Client::new();
//     // Act
//     let response = client
//         .post(&format!("{}/subscriptions", &app.address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(invalid_body)
//         .send()
//         .await
//         .expect("Failed to execute request.");
//     // Assert
//     assert_eq!(
//         400,
//         response.status().as_u16(),
//         // Additional customised error message on test failure
//         "The API did not fail with 400 Bad Request when the payload was {}.",
//         error_message
//     );
// }
