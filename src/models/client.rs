use reqwest::{header, Client};
use serde_json::Value;
use url::Url;

use super::message::BaseMessage;
#[derive(serde::Serialize)]

pub struct OpenAIClient {
    base_url: String,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self { base_url, api_key }
    }
    #[tokio::main]
    pub async fn make_chat(
        &self,
        config: &mut Value,
        message: Vec<BaseMessage>,
    ) -> Result<Value, reqwest::Error> {
        let client = Client::new();
        let url = Url::parse(&self.base_url).unwrap();
        let api_key = &self.api_key;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(api_key).unwrap(),
        );

        // 将 message 添加到 config 中
        if let Value::Object(ref mut map) = config {
            map.insert(
                "messages".to_string(),
                serde_json::to_value(message).unwrap(),
            );
        }

        let res = client
            .post(url.join("chat/completions").unwrap())
            .headers(headers)
            .json(&config)
            .send()
            .await?;
        let body = res.text().await?;

        let json: Value = serde_json::from_str(&body).unwrap();

        Ok(json)
    }
}
