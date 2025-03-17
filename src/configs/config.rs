use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

// pub enum ConfigType {
//     MarshoCfg,
//     ModelCfg,
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarshoConfig {
    pub base_url: String,
    pub api_key: String,
    pub stream: bool,
    pub system_prompt: String,
}

impl Default for MarshoConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.deepseek.com".to_string(),
            api_key: "".to_string(),
            stream: true,
            system_prompt: String::from("你是一只可爱的猫娘，你的名字叫Marsho"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelConfig {
    pub model: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model: "deepseek-reasoner".to_string(),
        }
    }
}

pub fn load_model_config() -> anyhow::Result<Value> {
    let config_path = "model_config.yaml";
    let config_str = serde_yaml::to_string(&ModelConfig::default())?;
    let path = Path::new(config_path);

    if !path.exists() {
        std::fs::write(config_path, config_str)?;
    }

    let json_str = std::fs::read_to_string(config_path)?;
    serde_yaml::from_str(&json_str).map_err(Into::into)
}

pub fn load_marsho_config() -> anyhow::Result<MarshoConfig> {
    let config_path = "config.yaml";
    let path = Path::new(config_path);

    if !path.exists() {
        let config_str = serde_yaml::to_string(&MarshoConfig::default())?;
        std::fs::write(config_path, config_str)?;
    }

    let settings = config::Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()?;

    settings.try_deserialize().map_err(Into::into)
}
