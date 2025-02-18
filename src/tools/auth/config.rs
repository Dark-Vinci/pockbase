pub struct Oauth2Config<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub scopes: &'a [&'a str],
    pub redirect_url: &'a str,
    pub auth_url: &'a str,
    pub token_url: &'a str,
}
