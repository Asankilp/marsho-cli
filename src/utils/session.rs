use crate::models::message::BaseMessage;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

const LAST_SESSION_FILE: &str = ".last_session";
const SESSIONS_DIR: &str = "sessions";

pub fn get_all_session() -> Result<Vec<String>> {
    let sessions_dir = Path::new(SESSIONS_DIR);
    if !sessions_dir.exists() {
        fs::create_dir(sessions_dir)?;
    }
    let entries = fs::read_dir(sessions_dir)?;
    let mut ret = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_stem().unwrap().to_str().unwrap();
        ret.push(filename.to_string());
    }
    Ok(ret)
}

pub fn write_session(messages: Vec<BaseMessage>, filename: &str) -> Result<()> {
    let sessions_dir = PathBuf::from(SESSIONS_DIR);
    if !sessions_dir.exists() {
        fs::create_dir(&sessions_dir)?;
    }

    let file_path = sessions_dir.join(format!("{}.yaml", filename));
    let yaml_str = serde_yaml::to_string(&messages)?;
    Ok(fs::write(file_path, yaml_str)?)
}

pub fn read_session(filename: &str) -> Result<Vec<BaseMessage>> {
    let sessions_dir = PathBuf::from(SESSIONS_DIR);
    let file_path = sessions_dir.join(format!("{}.yaml", filename));

    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(file_path)?;
    let value: Vec<BaseMessage> = serde_yaml::from_str(&contents)?;
    Ok(value)
}

pub fn clear_session(filename: &str) -> Result<()> {
    let sessions_dir = PathBuf::from(SESSIONS_DIR);
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
        Err(_) => Ok("default".to_string()),
    }
}
