#[derive(Debug)]
pub struct Marsho {
    pub base_url: String,
    pub api_key: String,
    // contexts: Vec<String>,
}

#[derive(Debug)]
pub struct BaseMessage<'a> {
    role: &'a str,
    content: &'a String,
}

impl BaseMessage<'_> {
    pub fn user(content: &String) -> BaseMessage {
        BaseMessage {
            role: "user",
            content: content,
        }
    }

    pub fn system(content: &String) -> BaseMessage {
        BaseMessage {
            role: "system",
            content: content,
        }
    }

    pub fn assistant(content: &String) -> BaseMessage {
        BaseMessage {
            role: "assistant",
            content: content,
        }
    }
}
