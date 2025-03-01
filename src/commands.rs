use crate::{models::context::MarshoContext, utils::session};

pub enum Command {
    Reset,
    Exit,
    Session(String),
    Chat(String),
}

impl Command {
    pub fn from_input(input: &str) -> Command {
        let input = input.trim();
        if let Some(session_cmd) = input.strip_prefix("/session") {
            let session_name = session_cmd.trim();
            Command::Session(session_name.to_string())
        } else {
            match input {
                "/reset" => Command::Reset,
                "/exit" => Command::Exit,
                chat => Command::Chat(chat.to_string()),
            }
        }
    }
}

pub fn handle_reset(context: &mut MarshoContext, session_name: &str) {
    context.reset();
    session::clear_session(session_name).unwrap();
    println!("上下文已重置");
}

// pub fn handle_session(session_name: &str) {
//     if session_name.is_empty() {
//         println!("请使用格式：/session <会话名称>");
//     } else if session_name.contains(char::is_whitespace) {
//         println!("会话名称不能包含空格");
//     } else {
//         println!("会话已切换到：{}", session_name.bright_green().bold());
//     }
// }
