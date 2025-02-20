use serde::{Deserialize, Serialize};
use std::path::Path;

pub enum ConfigType {
    MarshoCfg,
    ModelCfg,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MarshoConfig {
    base_url: String,
    api_key: String,
    stream: bool,
}

impl Default for MarshoConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.deepseek.com".to_string(),
            api_key: "".to_string(),
            stream: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelConfig {
    model: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model: "deepseek-reasoner".to_string(),
        }
    }
}

pub fn load_config(r#type: ConfigType) -> anyhow::Result<()> {
    let config_path: &str;
    let config_str: String;
    match r#type {
        ConfigType::MarshoCfg => {
            config_path = "config.yaml";
            config_str = serde_yaml::to_string(&MarshoConfig::default())?;
        }
        ConfigType::ModelCfg => {
            config_path = "model_config.yaml";
            config_str = serde_yaml::to_string(&ModelConfig::default())?;
        }
    }
    println!("{}", config_path);
    let path = Path::new(config_path);

    if !path.exists() {
        // let default_config = MarshoConfig::default();
        // let config_str = serde_yaml::to_string(&default_config)?;
        std::fs::write(config_path, config_str)?;
    }

    let settings = config::Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()?;
    
    settings.try_deserialize().map_err(Into::into)
}
