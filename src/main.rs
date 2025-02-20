mod configs;
mod models;
mod handlers;

use crate::handlers::handler::make_chat;
use crate::configs::config::{load_model_config, load_marsho_config};
fn main() -> anyhow::Result<()> {
    let marsho_configs = load_marsho_config()?;
    let model_configs = load_model_config()?;
    // println!("🍡 喵星人配置：{:?}", marsho_configs);
    // println!("🍡 模型配置：{:?}", model_configs);
    // let marsho = Marsho {
    //     base_url: marsho_configs.base_url,
    //     api_key: marsho_configs.api_key,
    //     model_args: model_configs,
    //     };
    // println!("🍡 {:?}", marsho);
    println!("输入聊天内容：");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let chat = make_chat(&marsho_configs, &model_configs, &input)?;
    println!("{}", chat);
    Ok(())
}
    // println!("🖥️ 服务器端口：{}", settings.server.port);
