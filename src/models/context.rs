use super::message::BaseMessage;

pub struct MarshoContext<'a> {
    messages: Vec<BaseMessage<'a>>,
}

impl<'a> MarshoContext<'a> {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add(&mut self, message: BaseMessage<'a>) {
        self.messages.push(message);
    }

    pub fn get(&self) -> &Vec<BaseMessage> {
        &self.messages
    }

    pub fn reset(&mut self) {
        self.messages.clear();
    }
}
