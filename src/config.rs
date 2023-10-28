use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

/// Attempt to read the config file located at ~/.config/workflows/config.toml
///
/// Wraps the get_config_option() function so that it always returns a config.
///
/// # Returns
///
/// The user's config or if any errors occurs, the default options
pub fn get_config() -> WorkflowsConfig {
    get_config_option().unwrap_or_default()
}

/// Attempt to read the config file located at ~/.config/workflows/config.toml
fn get_config_option() -> Option<WorkflowsConfig> {
    let config_dir = dirs::config_dir()?.join("workflows/");
    let config_file = config_dir.join("config.toml");

    WorkflowsConfig::from(config_file)
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WorkflowsConfig {
    tmuxinator: Option<Tmuxinator>,
}

impl WorkflowsConfig {
    /// Reads the passed in file and attempts to parse a [`WorkflowsConfig`] using toml
    fn from(config_file: PathBuf) -> Option<Self> {
        let toml_string = fs::read_to_string(config_file).ok()?;

        toml::from_str(&toml_string).ok()
    }

    /// Returns the [`Tmuxinator`] preferences in the config
    pub fn tmuxinator(&self) -> Tmuxinator {
        self.tmuxinator.clone().unwrap_or_default()
    }
}

#[derive(Deserialize, Default, Clone)]
/// The Tmuxinator config options
pub struct Tmuxinator {
    /// The command to run on opening the tmuxinator session
    window_name: Option<String>,
    /// The name of the tmuxinator spawned window
    on_open: Option<String>,
}

impl Tmuxinator {
    /// The command to run on opening the tmuxinator session
    pub fn on_open(&self) -> String {
        self.on_open.clone().unwrap_or("nvim .".to_string())
    }

    /// The name of the tmuxinator spawned window
    pub fn window_name(&self) -> String {
        self.window_name.clone().unwrap_or("editor".to_string())
    }
}
