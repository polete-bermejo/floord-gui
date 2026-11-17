use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Configuration {
    pub organizer: bool,
    pub inbox: Option<PathBuf>,
    pub delay: u64,

    pub cleanup: bool,
    pub paths: Vec<PathBuf>,
    pub span: u64,
    pub grace: u64,
    pub allow_deletion: bool,

    pub daemon: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            organizer: false,
            inbox: None,
            delay: 5,
            cleanup: false,
            paths: vec![],
            span: 30,
            grace: 7,
            allow_deletion: false,
            daemon: true,
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow!("Could not determine config directory"))?;

    Ok(config_dir.join("floord/config.toml"))
}

pub fn generate_config() -> Result<()> {
    let config_path = get_config_path()?;
    let parent = config_path
        .parent()
        .ok_or_else(|| anyhow!("Could not get parent directory"))?;
    if !config_path.exists() {
        create_dir_all(parent)?;
        let content = toml::to_string_pretty(&Configuration::default())?;
        fs::write(config_path, content)?;
    }
    Ok(())
}

pub fn load_config() -> Result<Configuration> {
    let config_path = get_config_path()?;
    let content = fs::read_to_string(config_path)?;
    let config = toml::from_str(&content);
    Ok(config?)
}

#[warn(unused)]
pub fn save_config(config: &Configuration) -> Result<()> {
    let config_path = get_config_path()?;
    let content = toml::to_string_pretty(&config)?;
    Ok(fs::write(config_path, content)?)
}
