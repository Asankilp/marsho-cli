mod configs;
mod models;

use models::message::BaseMessage;
use config::Config;
use crate::configs::config::{load_config, ConfigType};

fn main() -> anyhow::Result<()> {
    let marsho_configs = load_config(ConfigType::MarshoCfg)?;
    let model_configs = load_config(ConfigType::ModelCfg)?;
    println!("🍡 喵星人配置：{:?}", marsho_configs);
    println!("🍡 模型配置：{:?}", model_configs);
    println!("🍡 {:?}", BaseMessage::user(&String::from("喵喵喵")));
    Ok(())
}
    // println!("🖥️ 服务器端口：{}", settings.server.port);
