use reqwest::{header, Client};
use serde_json::Value;
use url::Url;
use futures_util::StreamExt;

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
            let text = chunk?
                .to_vec()
                .try_into()
                .map(String::from_utf8)
                .ok()
                .and_then(Result::ok)
                .unwrap_or_default();

            last_chunk = text.clone();
            
            for line in text.lines() {
                // print!("{}", line);
                if !line.starts_with("data: ") || line.contains("[DONE]") {
                    continue;
                }

                let content = serde_json::from_str::<Value>(&line[6..])
                    .ok()
                    .and_then(|json| json.get("choices").cloned())
                    .and_then(|choices| choices.get(0).cloned())
                    .and_then(|choice| choice.get("delta").cloned())
                    .and_then(|delta| delta.get("content").cloned())
                    .and_then(|content| content.as_str().map(String::from));

                if let Some(text) = content {
                    print!("{}", text);
                    responses.push(text);
                }
            }
        }

        // 合并所有响应片段
        let complete_message = responses.join("");
        print!("\n");

        // 创建或修改最终响应
        let final_response = match last_chunk
            .lines()
            .filter(|line| line.starts_with("data: ") && !line.contains("[DONE]"))
            .last()
            .and_then(|line| serde_json::from_str::<Value>(&line[6..]).ok())
        {
            Some(mut json) => {
                if let Some(obj) = json.as_object_mut() {
                    obj["choices"][0]["message"]["content"] = Value::String(complete_message.clone());
                }
                Some(json)
            }
            None => Some(serde_json::json!({
                "choices": [{
                    "message": {
                        "content": complete_message
                    }
                }]
            }))
        };

        Ok(final_response.unwrap())
    }
}
