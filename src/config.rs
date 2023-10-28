use std::{fs, error::Error, path::PathBuf};

use serde::Deserialize;

pub fn get_config() -> WorkflowsConfig {
    get_config_option().unwrap_or(WorkflowsConfig::default())
}

fn get_config_option() -> Option<WorkflowsConfig> {
    let config_dir = dirs::config_dir()?.join("workflows/");
    let config_file = config_dir.join("config.toml");

    Some(WorkflowsConfig::from(config_file))
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WorkflowsConfig {
    preferences: Option<Preferences>,
}

impl WorkflowsConfig {
    fn from(config_file: PathBuf) -> Self {
        Self::new(config_file).unwrap_or(Self::default())
    }

    fn new(config_file: PathBuf) -> Result<Self, Box<dyn Error>> {
        let toml_string = fs::read_to_string(config_file)?;

        let config: Self = toml::from_str(&toml_string)?;

        Ok(config)
    }

    pub fn preferences(&self) -> Preferences {
        self.preferences.clone().unwrap_or(Preferences::default())
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct Preferences {
    window_name: Option<String>,
    on_open: Option<String>,
}

impl Preferences {
    pub fn on_open(&self) -> String {
        self.on_open.clone().unwrap_or("nvim .".to_string())
    }

    pub fn window_name(&self) -> String {
        self.window_name.clone().unwrap_or("editor".to_string())
    }
}
