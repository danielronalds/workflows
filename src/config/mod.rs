use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

pub mod general;
use general::GeneralConfig;

pub mod github;
use github::GithubConfig;

pub mod tmuxinator;
use tmuxinator::TmuxinatorConfig;

pub mod fzf;
use fzf::FzfConfig;

pub mod git;
use git::GitConfig;

/// Attempt to read the config file located at either of the following two paths:
///
/// - ~/.workflows.toml
/// - ~/.config/workflows/config.toml
///
/// # Returns
///
/// `Some(WorkflowsConfig)` If one of the files was able to be read, otherwise `None`
pub fn get_config() -> Option<WorkflowsConfig> {
    let home_config_file = dirs::home_dir()?.join(".workflows.toml");
    if home_config_file.is_file() {
        return WorkflowsConfig::from(home_config_file);
    }

    // If the config is not located in ~/.workflows.toml, then it might be in
    // ~/.config/workflows/config.toml
    let config_dir_file = dirs::config_dir()?.join("workflows/").join("config.toml");

    WorkflowsConfig::from(config_dir_file)
}

/// This struct represents the user's configuration
#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct WorkflowsConfig {
    general: Option<GeneralConfig>,
    github: Option<GithubConfig>,
    git: Option<GitConfig>,
    tmuxinator: Option<TmuxinatorConfig>,
    fzf: Option<FzfConfig>,
}

impl WorkflowsConfig {
    /// Reads the passed in file and attempts to parse a [`WorkflowsConfig`] using toml
    // FIX: implement the try_from trait instead
    fn from(config_file: PathBuf) -> Option<Self> {
        let toml_string = fs::read_to_string(config_file).ok()?;

        toml::from_str(&toml_string).ok()
    }

    /// Returns the [`GeneralConfig`] preferences in the config
    pub fn general(&self) -> GeneralConfig {
        self.general.clone().unwrap_or_default()
    }

    /// Returns the [`FzfConfig`] preferences in the config
    pub fn fzf(&self) -> FzfConfig {
        self.fzf.clone().unwrap_or_default()
    }

    /// Returns the [`GitConfig`] preferences in the config
    pub fn git(&self) -> GitConfig {
        self.git.clone().unwrap_or_default()
    }

    /// Returns the [`GithubConfig`] settings in the config
    pub fn github(&self) -> GithubConfig {
        self.github.clone().unwrap_or_default()
    }

    /// Returns the [`TmuxinatorConfig`] preferences in the config
    pub fn tmuxinator(&self) -> TmuxinatorConfig {
        self.tmuxinator.clone().unwrap_or_default()
    }
}
