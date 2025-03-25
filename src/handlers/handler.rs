use crate::{
    configs::config::MarshoConfig,
    models::{client::OpenAIClient, context::MarshoContext, message::BaseMessage},
};
use serde_json::Value;

pub struct MarshoHandler {
    config: MarshoConfig,
    model_config: Value,
    client: OpenAIClient,
}

impl MarshoHandler {
    pub fn new(config: MarshoConfig, model_config: Value) -> Self {
        let client = OpenAIClient::new(config.base_url.clone(), config.api_key.clone());
        Self {
            config,
            model_config,
            client,
        }
    }

    pub fn handle(
        &mut self,
        input: String,
        context: MarshoContext,
        stream: bool,
    ) -> Result<Value, reqwest::Error> {
        let mut message = vec![BaseMessage::system(self.config.system_prompt.to_string())];
        message.extend(context.get().iter().cloned());
        message.extend(vec![BaseMessage::user(input.to_string())]);
        if stream {
            let chat = self
                .client
                .make_chat_stream(&mut self.model_config, message)?;
            Ok(chat)
        } else {
            let chat = self.client.make_chat(&mut self.model_config, message)?;
            Ok(chat)
        }
        // let response = chat["choices"][0]["message"]["content"].as_str().unwrap();
    }

    pub fn models(&mut self) {
        let resp = self.client.get_models().unwrap();
        for model_name in resp.data {
            println!("{}", model_name.id);
        }
    }
}
