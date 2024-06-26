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
        headers.insert("Authorization", HeaderValue::from_str(token).unwrap());
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Ok(Self { client })
    }

    pub async fn get_hugo(&self) -> Result<String> {
        do_request(self.client.get(format!("{URL}/hugo")))
            .await?
            .text()
            .await
            .wrap_err("failed to get hugo")
    }

    pub async fn get_class(&self, id: &str) -> Result<dto::Class> {
        do_request_body(self.client.get(format!("{URL}/classes/{id}")))
            .await
            .wrap_err("failed to get class")
    }

    pub async fn post_class(&self, class: &dto::Class) -> Result<dto::Class> {
        do_request_body(self.client.post(format!("{URL}/classes")).json(class))
            .await
            .wrap_err("creating class")
    }
}

async fn do_request_body<T: serde::de::DeserializeOwned>(req: RequestBuilder) -> Result<T> {
    Ok(do_request(req).await?.json().await?)
}

async fn do_request(req: RequestBuilder) -> Result<Response> {
    let res = req.send().await?;
    if let Err(err) = res.error_for_status_ref() {
        let text = res.text().await.unwrap_or_default();
        return Err(err).wrap_err(text);
    }

    res.error_for_status().wrap_err("failed make request")
}
