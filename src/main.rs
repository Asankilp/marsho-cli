mod configs;
mod models;

use models::message::BaseMessage;
use crate::configs::config::{load_model_config, load_marsho_config};
use crate::models::message::Marsho;
fn main() -> anyhow::Result<()> {
    let marsho_configs = load_marsho_config()?;
    let model_configs = load_model_config()?;
    println!("🍡 喵星人配置：{:?}", marsho_configs);
    println!("🍡 模型配置：{:?}", model_configs);
    println!("🍡 {:?}", BaseMessage::user(&String::from("喵喵喵")));
    let marsho = Marsho {
        base_url: marsho_configs.base_url,
        api_key: String::from("sk-1234567890"),
        };
    println!("🍡 {:?}", marsho);
    Ok(())
}
    // println!("🖥️ 服务器端口：{}", settings.server.port);
