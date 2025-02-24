use crate::{
    configs::config::MarshoConfig,
    models::{client::OpenAIClient, context::MarshoContext, message::BaseMessage},
};
use serde_json::Value;

pub struct MarshoHandler<'a> {
    config: MarshoConfig,
    model_config: Value,
    context: MarshoContext<'a>,
    client: OpenAIClient,
}

impl<'a> MarshoHandler<'a> {
    pub fn new(config: MarshoConfig, model_config: Value, context: MarshoContext<'a>) -> Self {
        let client = OpenAIClient::new(config.base_url.clone(), config.api_key.clone());
        Self {
            config,
            model_config,
            context,
            client,
        }
    }

    pub fn handle(&mut self, input: String) -> Result<String, reqwest::Error> {
        let message = vec![
            BaseMessage::system(&self.config.system_prompt),
            BaseMessage::user(&input),
        ];
        let chat = self.client.make_chat(&mut self.model_config, message)?;
        let response = chat["choices"][0]["message"]["content"].as_str().unwrap();
        Ok(response.to_string())
    }
}
