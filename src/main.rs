mod configs;
mod handlers;
mod models;

use std::io::{self, Write};
use crate::configs::config::{load_marsho_config, load_model_config};
use handlers::handler::MarshoHandler;
use models::{context::MarshoContext, message::BaseMessage};
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
        let mut handler = MarshoHandler::new(marsho_configs.clone(), model_configs.clone(), context.clone());    
        print!(">>> "); io::stdout().flush()?;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(e) => {
                eprintln!("读取输入错误: {}", e);
                break;
            },
            Ok(0) => break,
            Ok(_) => {
                let command = input.trim();
                match command {
                    "/reset" => {
                        context.reset();
                        println!("上下文已重置")
                    }
                    _ => {
                        let chat = handler.handle(input.clone())?;
                        println!("{}", chat);
                        let user_message = BaseMessage::user(input.to_string());
                        let assistant_message = BaseMessage::assistant(chat);
                        context.add(user_message);
                        context.add(assistant_message)
                    }
                }
            }
        }
    }
    Ok(())
}
// println!("🖥️ 服务器端口：{}", settings.server.port);
