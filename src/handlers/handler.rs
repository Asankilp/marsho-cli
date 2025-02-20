use reqwest::{Client, header};
use serde_json::Value;
use crate::{configs::config::{MarshoConfig, ModelConfig}, models::message::BaseMessage};
use url::Url;

#[derive(serde::Serialize)]
struct RequestJson<'a> {
    model: String,
    messages: Vec<BaseMessage<'a>>,
}

#[tokio::main]
pub async fn make_chat(marsho: &MarshoConfig, arg: &ModelConfig, message: &String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = Url::parse(&marsho.base_url).unwrap();
    let api_key = &marsho.api_key;
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(api_key).unwrap());
    let request = RequestJson {
        model: arg.model.clone(),
        messages: vec![
            BaseMessage::system(&marsho.system_prompt),
            BaseMessage::user(&message),
            ],
    };
    
    let res = client.post(url.join("chat/completions").unwrap())
        .headers(headers)
        .json(&request)
        .send()
        .await?;
    let body = res.text().await?;
    
    // 解析JSON并提取content
    let json: Value = serde_json::from_str(&body).unwrap();
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("无法获取响应内容")
        .to_string();
        
    Ok(content)
}

