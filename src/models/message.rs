
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

#[derive(serde::Serialize)]
pub struct BaseMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

impl BaseMessage<'_> {
    pub fn user(content: &str) -> BaseMessage {
        BaseMessage {
            role: "user",
            content: content,
        }
    }

    pub fn system(content: &str) -> BaseMessage {
        BaseMessage {
            role: "system",
            content: content,
        }
    }

    pub fn assistant(content: &str) -> BaseMessage {
        BaseMessage {
            role: "assistant",
            content: content,
        }
    }
}
