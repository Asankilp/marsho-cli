use crate::models::message::BaseMessage;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

const LAST_SESSION_FILE: &str = ".last_session";

pub fn write_session(messages: Vec<BaseMessage>, filename: &str) -> Result<()> {
    let sessions_dir = PathBuf::from("sessions");
    if !sessions_dir.exists() {
        fs::create_dir(&sessions_dir)?;
    }

    let file_path = sessions_dir.join(format!("{}.yaml", filename));
    let yaml_str = serde_yaml::to_string(&messages)?;
    Ok(fs::write(file_path, yaml_str)?)
}

pub fn read_session(filename: &str) -> Result<Vec<BaseMessage>> {
    let sessions_dir = PathBuf::from("sessions");
    let file_path = sessions_dir.join(format!("{}.yaml", filename));

    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(file_path)?;
    let value: Vec<BaseMessage> = serde_yaml::from_str(&contents)?;
    Ok(value)
}

pub fn clear_session(filename: &str) -> Result<()> {
    let sessions_dir = PathBuf::from("sessions");
    let file_path = sessions_dir.join(format!("{}.yaml", filename));

    if file_path.exists() {
        fs::remove_file(file_path)?;
    }

    Ok(())
}

pub fn save_last_session(name: &String) -> Result<String> {
    fs::write(LAST_SESSION_FILE, name)?;
    Ok(name.clone())
}

pub fn get_last_session() -> Result<String> {
    match fs::read_to_string(LAST_SESSION_FILE) {
        Ok(name) => Ok(name),
        Err(_) => Ok("default".to_string())
    }
}
