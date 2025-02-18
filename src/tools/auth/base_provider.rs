use {
    oauth2::{
        basic::{
            BasicClient, BasicErrorResponse, BasicRevocationErrorResponse,
            BasicTokenIntrospectionResponse, BasicTokenResponse,
        },
        reqwest, AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet,
        RedirectUrl, StandardRevocableToken, TokenUrl,
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
    extra: HashMap<&'a str, &'a dyn Any>,
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

    pub fn extra(&self) -> HashMap<&str, &dyn Any> {
        self.extra.clone()
    }

    pub fn set_extra(&mut self, extra: HashMap<&str, &dyn Any>) {
        self.extra = extra;
    }

    pub fn oauth2_config(
        &self,
    ) -> oauth2::Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    > {
        BasicClient::new(ClientId::new(self.client_id.into()))
            .set_client_secret(ClientSecret::new(self.client_secret.into()))
            .set_auth_uri(AuthUrl::new(self.auth_url.into()).unwrap()) // handle unwrap
            .set_token_uri(TokenUrl::new(self.token_url.into()).unwrap()) // handle unwrap
            .set_redirect_uri(
                RedirectUrl::new(self.redirect_url.into()).unwrap(),
            )
    }

    pub fn client(&self, token: &str) -> () {
        let a = oauth2::AuthorizationCode::new();
        let req = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
    }
}
