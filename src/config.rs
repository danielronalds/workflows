use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

pub mod github;
use github::GithubConfig;

pub mod tmuxinator;
use tmuxinator::TmuxinatorConfig;

pub mod fzf;
use fzf::FzfConfig;

/// Attempt to read the config file located at either of the following two paths:
/// - ~/.config/workflows/config.toml
/// - ~/.workflows.toml
///
/// # Returns
///
/// The user's config or if any errors occurs, the default options
pub fn get_config() -> Option<WorkflowsConfig> {
    let config_dir = dirs::config_dir()?.join("workflows/");
    let config_dir_file = config_dir.join("config.toml");

    if config_dir_file.is_file() {
        return WorkflowsConfig::from(config_dir_file);
    }

    // If the config is not located in ~/.config/workflows/config.toml, then it might be in
    // ~/.workflows.toml
    let home_config_file = dirs::home_dir()?.join(".workflows.toml");
    WorkflowsConfig::from(home_config_file)
}

/// This struct represents the user's configuration
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct WorkflowsConfig {
    github: Option<GithubConfig>,
    tmuxinator: Option<TmuxinatorConfig>,
    fzf: Option<FzfConfig>,
}

impl WorkflowsConfig {
    /// Reads the passed in file and attempts to parse a [`WorkflowsConfig`] using toml
    fn from(config_file: PathBuf) -> Option<Self> {
        let toml_string = fs::read_to_string(config_file).ok()?;

        toml::from_str(&toml_string).ok()
    }

    /// Returns the [`FzfConfig`] preferences in the config
    pub fn fzf(&self) -> FzfConfig {
        self.fzf.clone().unwrap_or_default()
    }

    /// Returns the [`GithubConfig`] settings in the config
    pub fn github(&self) -> GithubConfig {
        self.github.clone().unwrap_or_default()
    }

    /// Returns the [`TmuxinatorConfig`] preferences in the config
    pub fn tmuxinator_config(&self) -> TmuxinatorConfig {
        self.tmuxinator.clone().unwrap_or_default()
    }
}
