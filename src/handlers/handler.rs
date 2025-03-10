use crate::{
    configs::config::MarshoConfig,
    models::{client::OpenAIClient, context::MarshoContext, message::BaseMessage},
};
use serde_json::Value;

pub struct MarshoHandler {
    config: MarshoConfig,
    model_config: Value,
    context: MarshoContext,
    client: OpenAIClient,
}

impl MarshoHandler {
    pub fn new(config: MarshoConfig, model_config: Value, context: MarshoContext) -> Self {
        let client = OpenAIClient::new(config.base_url.clone(), config.api_key.clone());
        Self {
            config,
            model_config,
            context,
            client,
        }
    }

    pub fn handle(&mut self, input: String, stream: bool) -> Result<Value, reqwest::Error> {
        let mut message = vec![BaseMessage::system(self.config.system_prompt.to_string())];
        message.extend(self.context.get().iter().cloned());
        message.extend(vec![BaseMessage::user(input.to_string())]);
        if stream == true {
            let chat = self.client.make_chat_stream(&mut self.model_config, message)?;
            Ok(chat)

        } else {
            let chat = self.client.make_chat(&mut self.model_config, message)?;
            Ok(chat)
    }
        // let response = chat["choices"][0]["message"]["content"].as_str().unwrap();
    }
}
