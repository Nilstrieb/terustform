use eyre::{Context, OptionExt, Result};
use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Clone)]
pub struct CorsClient {
    client: reqwest::Client,
}

const URL: &str = "https://api.cors-school.nilstrieb.dev/api";

impl CorsClient {
    pub async fn new(email: String, password: String) -> Result<Self> {
        let client = reqwest::Client::new();
        let login = dto::UserLogin { email, password };
        let token = client
            .post(format!("{URL}/login"))
            .json(&login)
            .send()
            .await
            .wrap_err("failed to send login request")?;
        let token = token.error_for_status().wrap_err("failed to login")?;
        let token = token
            .headers()
            .get("Token")
            .ok_or_eyre("does not have Token header in login response")?
            .to_str()
            .wrap_err("Token is invalid utf8")?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token,)).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Self { client })
    }

    pub async fn get_hugo(&self) -> Result<String> {
        Ok(self
            .client
            .get(format!("{URL}/hugo"))
            .send()
            .await?
            .text()
            .await?)
    }
}
