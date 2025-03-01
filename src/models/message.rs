#[derive(serde::Serialize, serde::Deserialize, Clone)]
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
