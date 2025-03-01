use crate::models::context::MarshoContext;

pub enum Command {
    Reset,
    Exit,
    Chat(String),
}

impl Command {
    pub fn from_input(input: &str) -> Command {
        match input.trim() {
            "/reset" => Command::Reset,
            "/exit" => Command::Exit,
            chat => Command::Chat(chat.to_string()),
        }
    }
}

pub fn handle_reset(context: &mut MarshoContext) {
    context.reset();
    println!("上下文已重置");
}
