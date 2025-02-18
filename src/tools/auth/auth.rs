use {
    chrono::{DateTime, Utc},
    oauth2::{ExtraTokenFields, TokenType},
    serde::{Deserialize, Serialize},
    std::{any::Any, collections::HashMap, error::Error},
};

pub type MyErr = Box<dyn Error + Send + Sync>;
type Token = oauth2::StandardTokenResponse<dyn ExtraTokenFields, dyn TokenType>;

pub trait Provider<'a> {
    fn pkce(&self) -> bool;

    fn set_pkce(&self, enabled: bool);

    fn display_name(&self) -> &'a str;

    fn set_display_name(&self, display_name: &str);

    fn client_id(&self) -> &'a str;

    fn set_client_id(&self, client_id: &str);

    fn scopes(&self) -> &'a [&'a str];

    fn set_scopes(&self, scopes: &[&'a str]);

    fn client_secret(&self) -> &'a str;

    fn set_client_secret(&self, client_secret: &'a str);

    fn redirect_url(&self) -> &'a str;

    fn set_redirect_url(&self, redirect_url: &'a str);

    fn auth_url(&self) -> &'a str;

    fn set_auth_url(&self, auth_url: &'a str);

    fn token_url(&self) -> &'a str;

    fn set_token_url(&self, token_url: &'a str);

    fn user_info_url(&self) -> &'a str;

    fn set_user_info_url(&self, user_info: &'a str);

    fn extras(&self) -> HashMap<&'a str, &dyn Any>;

    fn set_extras(&self, extra: HashMap<&'a str, &dyn Any>);

    fn build_auth_url(
        &self,
        state: &str,
        options: &[oauth2::AuthorizationCode],
    ) -> &str;

    fn fetch_raw_user_info(&self, token: Token) -> Result<Vec<u8>, MyErr>;

    fn fetch_token(
        &self,
        code: &str,
        options: &[oauth2::AuthorizationCode],
    ) -> Result<Token, MyErr>;

    fn fetch_auth_user(&self, token: Token) -> Result<AuthUser, MyErr>;

    fn client(&self) -> (); //todo update later
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUser {
    #[serde(rename = "expiry")]
    pub expiry: DateTime<Utc>,

    #[serde(rename = "rawUser")]
    pub raw_user: HashMap<String, serde_json::Value>,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "avatarURL")]
    pub avatar_url: String,

    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,

    #[serde(rename = "avatarUrl")]
    pub avatar_url_deprecated: String, // Deprecated field
}

impl AuthUser {
    fn serialize(&self) -> Vec<u8> {
        serde_json::to_string(self).unwrap().into_bytes()
    }
}
