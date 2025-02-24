// #[derive(Debug)]
// pub struct Marsho {
//     pub base_url: String,
//     pub api_key: String,
//     pub model_args: ModelConfig,
//     // contexts: Vec<String>,
// }

// pub struct MessageSegment<'a> {
//     message: Vec<BaseMessage<'a>>,
// }

#[derive(serde::Serialize, Clone)]
pub struct BaseMessage {
    pub role: String,
    pub content: String,
}

impl BaseMessage {
    pub fn user(content: String) -> BaseMessage {
        BaseMessage {
            role: "user".to_string(),
            content: content,
        }
    }

    pub fn system(content: String) -> BaseMessage {
        BaseMessage {
            role: "system".to_string(),
            content: content,
        }
    }

    pub fn assistant(content: String) -> BaseMessage {
        BaseMessage {
            role: "assistant".to_string(),
            content: content,
        }
    }
}
