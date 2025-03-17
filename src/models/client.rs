use std::io::{self, Write};

use futures_util::StreamExt;
use reqwest::{header, Client};
use serde_json::Value;
use url::Url;

use crate::schemas::models::Models;

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

    #[tokio::main]
    pub async fn make_chat_stream(
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

        if let Value::Object(ref mut map) = config {
            map.insert(
                "messages".to_string(),
                serde_json::to_value(message).unwrap(),
            );
            map.insert("stream".to_string(), serde_json::to_value(true).unwrap());
        }

        let res = client
            .post(url.join("chat/completions").unwrap())
            .headers(headers)
            .json(&config)
            .send()
            .await?;

        let mut stream = res.bytes_stream();
        let mut responses = Vec::new();
        let mut last_chunk = String::new();

        while let Some(chunk) = stream.next().await {
            let line = String::from_utf8(chunk?.to_vec()).expect("Found invalid UTF-8");

            if !line.starts_with("data: ") || line.contains("[DONE]") {
                continue;
            }

            // println!("{}", line);
            last_chunk = line.clone();

            let sse_json: Value = serde_json::from_str(&line[6..]).expect("解析的不是json？！！");

            let content = sse_json["choices"][0]["delta"]["content"].as_str();
            if let Some(text) = content {
                if io::stdout().flush().is_err() {
                    println!("flush err")
                }
                print!("{}", text);
                responses.push(String::from(text));
            }
        }

        // 合并所有响应片段
        let complete_message = responses.join("");
        println!();

        // 创建或修改最终响应
        let final_response = match last_chunk
            .lines()
            .filter(|line| line.starts_with("data: ") && !line.contains("[DONE]"))
            .last()
            .and_then(|line| serde_json::from_str::<Value>(&line[6..]).ok())
        {
            Some(mut json) => {
                if let Some(obj) = json.as_object_mut() {
                    obj["choices"][0]["message"]["content"] =
                        Value::String(complete_message.clone());
                }
                Some(json)
            }
            None => Some(serde_json::json!({
                "choices": [{
                    "message": {
                        "content": complete_message
                    }
                }]
            })),
        };

        Ok(final_response.unwrap())
    }

    #[tokio::main]
    pub async fn get_models(&self) -> Result<Models, reqwest::Error> {
        let client = Client::new();
        let url = Url::parse(&self.base_url).unwrap().join("models").unwrap();
        println!("{}", url);

        let api_key = &self.api_key;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(api_key).unwrap(),
        );
        let res = client.get(url).headers(headers).send().await?;
        let body: Models = res.json().await?;

        Ok(body)
    }
}
