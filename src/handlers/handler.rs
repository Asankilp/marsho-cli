use reqwest::{Client, header};

pub async fn get(base_url: &String, api_key: &String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client.get(url)
        .header(header::USER_AGENT, "reqwest")
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}

