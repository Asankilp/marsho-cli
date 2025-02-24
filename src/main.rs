mod configs;
mod handlers;
mod models;

use crate::configs::config::{load_marsho_config, load_model_config};
use handlers::handler::MarshoHandler;
use models::context::MarshoContext;
fn main() -> anyhow::Result<()> {
    let marsho_configs = load_marsho_config()?;
    let model_configs = load_model_config()?;
    let mut context = MarshoContext::new();
    let mut handler = MarshoHandler::new(marsho_configs, model_configs.clone(), context);
    // println!("🍡 喵星人配置：{:?}", marsho_configs);
    // let marsho = Marsho {
    //     base_url: marsho_configs.base_url,
    //     api_key: marsho_configs.api_key,
    //     model_args: model_configs,
    //     };
    // println!("🍡 {:?}", marsho);
    println!("输入聊天内容：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let chat = handler.handle(input)?;
    println!("{}", chat);
    Ok(())
}
// println!("🖥️ 服务器端口：{}", settings.server.port);
