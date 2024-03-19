use crate::set_secret;
use reqwest::blocking::*;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Deserialize)]
struct TokenResponse {
    pub value: Option<String>,
}

pub struct OidcClient;

impl OidcClient {
    fn get_call(url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::blocking::get(url)?;
        let token_response: TokenResponse = response.json()?;
        token_response.value.ok_or("empty".into())
    }

    fn get_id_token_url() -> Result<String, Box<dyn Error>> {
        let runtime_url = env::var("ACTIONS_ID_TOKEN_REQUEST_URL")?;
        if runtime_url.is_empty() {
            return Err("empty".into());
        }
        Ok(runtime_url)
    }

    pub fn get_id_token(audience: Option<&str>) -> Result<String, Box<dyn Error>> {
        let mut id_token_url = OidcClient::get_id_token_url()?;
        if let Some(audience) = audience {
            id_token_url.push_str(&format!("&audience={audience}"));
        }
        let id_token = OidcClient::get_call(&id_token_url)?;
        set_secret(&id_token);
        Ok(id_token)
    }
}
