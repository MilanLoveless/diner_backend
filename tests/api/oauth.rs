//! tests/api/oauth.rs
use crate::helpers::{spawn_app, TestApp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

impl TestApp {
    async fn get_url(&self) -> reqwest::Response {
        self.get("oauth2/url").await
    }

    async fn get_redirect(&self, query: &RedirectQueryParams) -> reqwest::Response {
        self.get_with_query("oauth2/redirect", query).await
    }

    async fn get_revoke(&self, query: &RedirectQueryParams) -> reqwest::Response {
        self.get_with_query::<RedirectQueryParams>("oauth2/revoke", query)
            .await
    }
}

#[derive(Deserialize)]
struct UrlResponse {
    pub url: String,
}

#[derive(Serialize)]
struct DiscordUserResponse {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
    pub verified: bool,
    pub flags: u32,
    pub banner: String,
    pub accent_color: u32,
    pub premium_type: u32,
    pub public_flags: u32,
}

#[derive(Serialize)]
struct DiscordTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    refresh_token: String,
    scope: String,
}

#[derive(Serialize)]
struct RedirectQueryParams {
    pub code: String,
    pub state: String,
}

#[tokio::test]
async fn get_url_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_url().await;
    // Assert
    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn oauth_calls_discord() {
    // Arrange
    let app = spawn_app().await;
    // Get 3rd Party Oauth2 Provider Auth url
    let url_response = app.get_url().await.json::<UrlResponse>().await.unwrap();
    // Query Params from parsed Url
    let mut query_info: HashMap<_, _> = Url::parse(&url_response.url)
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect();
    // State Query Param
    let state = query_info.remove("state").unwrap();
    // // Location for 302 Redirect expected from 3rd Party Oauth 2 Provider
    // let mut location = Url::parse(query_info.remove("redirect_uri").unwrap().as_str()).unwrap();
    // location.set_query(Some(format!("state={}&code=12345", state).as_str()));
    // // Location Header
    // let location_header =
    //     wiremock::http::HeaderValue::from_bytes(location.to_string().as_bytes().to_vec()).unwrap();
    // // Discord Oauth Authorization
    // Mock::given(path("/oauth2/authorize"))
    //     .and(method("GET"))
    //     .respond_with(ResponseTemplate::new(302).append_header("Location", location_header))
    //     .expect(1)
    //     .mount(&app.discord_oauth)
    //     .await;
    // Discord Oauth2 Token Response
    let token_body = DiscordTokenResponse {
        access_token: "6qrZcUqja7812RVdnEKjpzOL4CvHBFG".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 604800,
        refresh_token: "D43f5y0ahjqew82jZ4NViEr2YafMKhue".to_string(),
        scope: "identify".to_string(),
    };

    // Discord Api User Response
    let user_body = DiscordUserResponse {
        id: "80351110224678912".to_string(),
        username: "Nelly".to_string(),
        discriminator: "1337".to_string(),
        avatar: "8342729096ea3675442027381ff50dfe".to_string(),
        verified: true,
        flags: 64,
        banner: "06c16474723fe537c283b8efa61a30c8".to_string(),
        accent_color: 16711680,
        premium_type: 1,
        public_flags: 64,
    };
    // Discord Oauth Token
    Mock::given(path("/api/oauth2/token"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_body))
        .expect(1)
        .mount(&app.discord_oauth)
        .await;
    // Discord Api
    Mock::given(path("/api/users/@me"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_body))
        .expect(1)
        .mount(&app.discord_api)
        .await;
    // Act
    let query = RedirectQueryParams {
        state,
        code: "12345".to_string(),
    };
    app.get_redirect(&query).await;
    // Assert
    // Mock asserts on drop
}
