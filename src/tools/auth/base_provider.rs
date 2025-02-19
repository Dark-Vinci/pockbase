use {
    crate::tools::auth::auth::MyErr,
    oauth2::{
        basic::{BasicClient, BasicErrorResponse, BasicTokenResponse},
        reqwest, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
        CodeTokenRequest, RedirectUrl, TokenResponse, TokenUrl,
    },
    std::{any::Any, collections::HashMap},
};

#[derive(Clone, Debug)]
pub struct BaseProvider<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    client_display_name: &'a str,
    display_name: &'a str,
    redirect_url: &'a str,
    auth_url: &'a str,
    token_url: &'a str,
    user_info_url: &'a str,
    scopes: &'a [&'a str],
    pkce: bool,
    extra: HashMap<&'a str, Box<dyn Any>>,
    provider_url: &'static str,
}

impl<'a> BaseProvider<'a> {
    pub fn pkce(&self) -> bool {
        self.pkce
    }

    pub fn set_pkce(&mut self, enable: bool) {
        self.pkce = enable;
    }

    pub fn display_name(&self) -> &str {
        self.display_name
    }

    pub fn set_display_name(&mut self, name: &str) {
        self.display_name = name;
    }

    pub fn scopes(&self) -> &[&'a str] {
        self.scopes
    }

    pub fn set_scopes(&mut self, scopes: &[&str]) {
        self.scopes = scopes;
    }

    pub fn client_id(&self) -> &str {
        self.client_id
    }

    pub fn set_client_id(&mut self, client_id: &'a str) {
        self.client_id = client_id;
    }

    pub fn client_secret(&self) -> &str {
        self.client_secret
    }

    pub fn set_client_secret(&mut self, secret: &'a str) {
        self.client_secret = secret;
    }

    pub fn redirect_url(&self) -> &str {
        self.redirect_url
    }

    pub fn set_redirect_url(&mut self, redirect_url: &'a str) {
        self.redirect_url = redirect_url;
    }

    pub fn token_url(&self) -> &str {
        self.token_url
    }

    pub fn set_token_url(&mut self, token_url: &'a str) {
        self.token_url = token_url;
    }

    pub fn auth_url(&self) -> &str {
        self.auth_url
    }

    pub fn set_auth_url(&mut self, auth_url: &'a str) {
        self.auth_url = auth_url;
    }

    pub fn user_info_url(&self) -> &str {
        self.user_info_url
    }

    pub fn set_user_info_url(&mut self, user_info_url: &'a str) {
        self.auth_url = user_info_url;
    }

    pub fn extra(&self) -> HashMap<&'a str, Box<dyn Any>> {
        self.extra.clone()
    }

    pub fn set_extra(&mut self, extra: HashMap<&'a str, Box<dyn Any>>) {
        self.extra = extra;
    }

    pub fn client(
        &self,
        code: &str,
    ) -> CodeTokenRequest<BasicErrorResponse, BasicTokenResponse> {
        let code = AuthorizationCode::new(code.into());

        let res = BasicClient::new(ClientId::new(self.client_id.into()))
            .set_client_secret(ClientSecret::new(self.client_secret.into()))
            .set_auth_uri(AuthUrl::new(self.auth_url.into()).unwrap()) // handle unwrap
            .set_token_uri(TokenUrl::new(self.token_url.into()).unwrap()) // handle unwrap
            .set_redirect_uri(
                RedirectUrl::new(self.redirect_url.into()).unwrap(),
            )
            .exchange_code(code);

        res
    }

    pub fn fetch_user_raw_data(
        &self,
        token: BasicTokenResponse,
    ) -> Result<bytes::Bytes, MyErr> {
        self.send_raw_user_info_request(token)
    }

    pub async fn fetch_token(
        &self,
        token: &str,
    ) -> Result<BasicTokenResponse, MyErr> {
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let response = self.client(token).request_async(&http_client).await?;

        Ok(response)
    }

    async fn send_raw_user_info_request(
        &self,
        token: BasicTokenResponse,
    ) -> Result<bytes::Bytes, MyErr> {
        let client = reqwest::Client::new();

        let res = client
            .get(self.provider_url)
            .bearer_auth(token.access_token().secret())
            .send()
            .await?
            .bytes()
            .await?;

        Ok(res)
    }
}
