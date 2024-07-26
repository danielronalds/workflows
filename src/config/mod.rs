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

pub mod templates;
use templates::WorkspaceTemplate;

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
        return WorkflowsConfig::try_from(home_config_file).ok();
    }

    // If the config is not located in ~/.workflows.toml, then it might be in
    // ~/.config/workflows/config.toml
    let config_dir_file = dirs::config_dir()?.join("workflows/").join("config.toml");

    WorkflowsConfig::try_from(config_dir_file).ok()
}

/// This struct represents the user's configuration
#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct WorkflowsConfig {
    general: Option<GeneralConfig>,
    template: Option<Vec<WorkspaceTemplate>>,
    github: Option<GithubConfig>,
    git: Option<GitConfig>,
    tmuxinator: Option<TmuxinatorConfig>,
    fzf: Option<FzfConfig>,
}

impl WorkflowsConfig {
    /// Returns the [`GeneralConfig`] preferences in the config
    pub fn general(&self) -> GeneralConfig {
        self.general.clone().unwrap_or_default()
    }

    /// Returns the [`TemplatesConfig`] preferences in the config
    pub fn templates(&self) -> Vec<WorkspaceTemplate> {
        self.template.clone().unwrap_or_default()
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

impl TryFrom<PathBuf> for WorkflowsConfig {
    type Error = &'static str;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let toml_string = match fs::read_to_string(value) {
            Ok(toml) => toml,
            Err(_) => return Err("Couldn't read string"),
        };

        match toml::from_str(&toml_string) {
            Ok(config) => Ok(config),
            Err(_) => Err("Couldn't parse config from toml"),
        }
    }
}
