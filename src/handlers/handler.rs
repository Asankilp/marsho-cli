use reqwest::{Client, header};
use crate::models::message::Marsho;
pub async fn get(marsho: &Marsho) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = marsho.base_url;
    let res = client.get(url)
        .header(header::USER_AGENT, "reqwest")
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}

