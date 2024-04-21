use eyre::{Context, OptionExt, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    RequestBuilder, Response,
};

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
            HeaderValue::from_str(token).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Self { client })
    }

    pub async fn get_hugo(&self) -> Result<String> {
        Ok(do_request(self.client.get(format!("{URL}/hugo")))
            .await?
            .text()
            .await?)
    }

    pub async fn get_class(&self, id: &str) -> Result<dto::Class> {
        Ok(do_request(self.client.get(format!("{URL}/classes/{id}")))
            .await?
            .json()
            .await?)
    }
}

async fn do_request(req: RequestBuilder) -> Result<Response> {
    dbg!(&req);
    let res = req.send().await?;
    if let Err(err) = res.error_for_status_ref() {
        let text = res.text().await.unwrap_or_default();
        return Err(err).wrap_err(text);
    }

    Ok(res.error_for_status().wrap_err("failed to get class")?)
}
