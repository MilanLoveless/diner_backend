use super::super::configuration::OauthSettings;
use delay_map::HashMapDelay;
use oauth2::basic::BasicClient;
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse, BasicTokenType,
};
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl, Scope, StandardRevocableToken,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use secrecy::{ExposeSecret, Secret};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct OauthClient {
    client_id: String,
    client_secret: Secret<String>,
    auth_url: String,
    token_url: String,
    revoke_url: String,
    redirect_url: String,
    pkce_hash_map: Arc<Mutex<HashMapDelay<String, PkceCodeVerifier>>>,
}

impl OauthClient {
    pub fn new(config: &OauthSettings) -> Self {
        Self {
            client_id: config.client_id.clone(),
            client_secret: config.client_secret.clone(),
            auth_url: config.discord_auth_url.clone(),
            token_url: config.discord_token_url.clone(),
            revoke_url: config.discord_token_url.clone(),
            redirect_url: config.redirect_url.clone(),
            pkce_hash_map: Arc::new(Mutex::new(HashMapDelay::new(
                std::time::Duration::from_secs(15 * 60),
            ))),
        }
    }

    fn get_pkce(&self, key: &String) -> Option<PkceCodeVerifier> {
        self.pkce_hash_map.lock().unwrap().remove(key)
    }

    fn set_pkce(&self, key: String, value: PkceCodeVerifier) {
        self.pkce_hash_map.lock().unwrap().insert(key, value);
    }

    pub fn get_oauth_url(&self) -> Result<Url, String> {
        if let Ok(client) = &self.get_client() {
            // Generate a PKCE challenge.
            let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
            // Generate the full authorization URL.
            let (auth_url, csrf_token) = client
                .authorize_url(CsrfToken::new_random)
                // Set the desired scopes.
                .add_scope(Scope::new("identify".to_string()))
                // Set the PKCE code challenge.
                .set_pkce_challenge(pkce_challenge)
                .url();
            // Set Pkce Verifier
            self.set_pkce(csrf_token.secret().to_string(), pkce_verifier);
            return Ok(auth_url);
        }
        Err("Nope!".to_string())
    }

    pub async fn get_token(
        &mut self,
        code: String,
        state: String,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, String> {
        let auth_code = AuthorizationCode::new(code.clone());
        // Once the user has been redirected to the redirect URL, you'll have access to the
        // authorization code. For security reasons, your code should verify that the `state`
        // parameter returned by the server matches `csrf_state`.
        if let Some(pkce_verifier) = self.get_pkce(&state) {
            let client = self.get_client();
            // Now you can trade it for an access token.
            let token_result = client
                .expect("fail")
                .exchange_code(auth_code)
                // Set the PKCE code verifier.
                .set_pkce_verifier(pkce_verifier)
                .request_async(async_http_client)
                .await;
            // Unwrapping token_result will either produce a Token or a RequestTokenError.
            if let Ok(final_token) = token_result {
                return Ok(final_token);
            }
        }
        Err("Nope!".to_string())
    }

    pub async fn revoke(
        &self,
        token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) {
        print!("revoke url: {}", self.revoke_url);
        let client = self.get_client().expect("Nope!");
        // Revoke the obtained token
        let token_to_revoke = match token.refresh_token() {
            Some(token) => token.into(),
            None => token.access_token().into(),
        };

        client
            .revoke_token(token_to_revoke)
            .unwrap()
            .request_async(async_http_client)
            .await
            .expect("shenanigans");
    }

    fn get_client(
        &self,
    ) -> Result<
        Client<
            BasicErrorResponse,
            BasicTokenResponse,
            BasicTokenType,
            BasicTokenIntrospectionResponse,
            StandardRevocableToken,
            BasicRevocationErrorResponse,
        >,
        String,
    > {
        let client = BasicClient::new(
            ClientId::new(self.client_id.clone()),
            Some(ClientSecret::new(
                self.client_secret.expose_secret().to_string(),
            )),
            AuthUrl::new(self.auth_url.clone()).expect("Invalid auth endpoint URL"),
            Some(TokenUrl::new(self.token_url.clone()).expect("Invalid token endpoint URL")),
        )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(
            RedirectUrl::new(self.redirect_url.clone()).expect("Invalid redirect endpoint URL"),
        )
        .set_revocation_uri(
            RevocationUrl::new(self.revoke_url.clone()).expect("Invalid revocation endpoint URL"),
        );
        Ok(client)
    }
}
