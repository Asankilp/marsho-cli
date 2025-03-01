mod commands;
mod configs;
mod handlers;
mod models;

use crate::commands::{handle_reset, Command};
use crate::configs::config::{load_marsho_config, load_model_config};
use handlers::handler::MarshoHandler;
use models::{context::MarshoContext, message::BaseMessage};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut context = MarshoContext::new();
    let marsho_configs = load_marsho_config()?;
    let model_configs = load_model_config()?;
    println!("Marsho-CLI!");
    println!("使用 /reset 命令重置上下文");

    // println!("🍡 喵星人配置：{:?}", marsho_configs);
    // let marsho = Marsho {
    //     base_url: marsho_configs.base_url,
    //     api_key: marsho_configs.api_key,
    //     model_args: model_configs,
    //     };
    // println!("🍡 {:?}", marsho);
    loop {
        let mut handler = MarshoHandler::new(
            marsho_configs.clone(),
            model_configs.clone(),
            context.clone(),
        );
        print!(">>> ");
        io::stdout().flush()?;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(e) => {
                eprintln!("读取输入错误: {}", e);
                break;
            }
            Ok(0) => break,
            Ok(_) => match Command::from_input(&input) {
                Command::Reset => handle_reset(&mut context),
                Command::Exit => break,
                Command::Chat(input) => {
                    let chat = handler.handle(input.clone())?;
                    let reply = chat["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap()
                        .to_string();
                    println!("{}", reply);
                    let user_message = BaseMessage::user(input);
                    let assistant_message = BaseMessage::assistant(reply);
                    context.add(user_message);
                    context.add(assistant_message);
                }
            },
        }
    }
    Ok(())
}
// println!("🖥️ 服务器端口：{}", settings.server.port);
