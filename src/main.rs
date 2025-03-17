mod commands;
mod configs;
mod handlers;
mod models;
mod schemas;
mod utils;

use crate::commands::{handle_reset, Command};
use crate::configs::config::{load_marsho_config, load_model_config};
use colored::*;
use handlers::handler::MarshoHandler;
use models::{context::MarshoContext, message::BaseMessage};
use std::io::{self, Write};
use utils::session::{self, get_all_session, get_last_session, save_last_session};

const ASCII_BANNER: &str = r#"
  __  __                        _                        ____   _       ___ 
 |  \/  |   __ _   _ __   ___  | |__     ___            / ___| | |     |_ _|
 | |\/| |  / _` | | '__| / __| | '_ \   / _ \   _____  | |     | |      | | 
 | |  | | | (_| | | |    \__ \ | | | | | (_) | |_____| | |___  | |___   | | 
 |_|  |_|  \__,_| |_|    |___/ |_| |_|  \___/           \____| |_____| |___|
                                                                            
"#;

fn main() -> anyhow::Result<()> {
    let mut context = MarshoContext::new();
    let marsho_configs = load_marsho_config()?;
    let model_configs = load_model_config()?;
    let mut session_name = get_last_session()?;

    println!("{}", ASCII_BANNER.bright_magenta());
    println!("使用 /reset 命令重置上下文");
    println!("使用 /session <会话名称> 命令切换会话");
    println!("使用 /models 命令查看可用模型");
    println!("使用 /config 命令查看配置");
    println!("使用 /exit 命令退出");
    println!("---------------------------------- ");
    // println!("Base URL: {}", marsho_configs.base_url.bright_blue().bold());
    println!(
        "模型名称: {}",
        model_configs["model"]
            .as_str()
            .unwrap()
            .bright_blue()
            .bold()
    );
    if marsho_configs.api_key.is_empty() {
        println!("{}", "WARN: API Key 未设置".bright_yellow().bold());
    }
    let mut handler = MarshoHandler::new(
        marsho_configs.clone(),
        model_configs.clone(),
        context.clone(),
    );

    loop {
        print!("[{}] >>> ", session_name.bright_green().bold());
        io::stdout().flush()?;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(e) => {
                eprintln!("读取输入错误: {}", e);
                break;
            }
            Ok(0) => break,
            Ok(_) => match Command::from_input(&input) {
                Command::Models => handler.models(),
                Command::Reset => handle_reset(&mut context, &session_name),
                Command::Config(input) => {
                    if input.is_empty() {
                        println!("当前配置：{:?}", marsho_configs)
                    } else {
                        let item:Vec<&str> = input.split_whitespace().collect();
                        if item.len() == 3 {
                            println!("option:{} args{} {}", item[0], item[1],item[2])
                            //TODO: update config
                        } else {
                            println!("{}", input)
                        }
                    }
                }
                Command::Exit => break,
                Command::Chat(input) => {
                    let loaded_session = session::read_session(&session_name)?;
                    context.set(loaded_session);

                    // println!("{:?}", context);
                    let chat = handler.handle(input.clone(), marsho_configs.stream)?;
                    let reply = chat["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap()
                        .to_string();
                    if !marsho_configs.stream {
                        println!("{}", reply);
                    }
                    let user_message = BaseMessage::user(input);
                    let assistant_message = BaseMessage::assistant(reply);
                    context.add(user_message);
                    context.add(assistant_message);
                    session::write_session(context.get().to_vec(), &session_name)?;
                }
                Command::Session(input) => {
                    if input.is_empty() {
                        println!("请使用格式：/session <会话名称>");
                        let session_names=get_all_session()?;
                        for name in session_names {
                            println!("{}", name);
                        }
                    } else if input.contains(char::is_whitespace) {
                        println!("会话名称不能包含空格");
                    } else {
                        session_name = save_last_session(&input)?;
                        context.reset();
                        println!("会话已切换到：{}", session_name.bright_green().bold());
                    }
                }
            },
        }
    }
    Ok(())
}
// println!("🖥️ 服务器端口：{}", settings.server.port);
