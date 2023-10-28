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
    github: Option<GithubConfig>,
    tmuxinator: Option<TmuxinatorConfig>,
}

impl WorkflowsConfig {
    /// Reads the passed in file and attempts to parse a [`WorkflowsConfig`] using toml
    fn from(config_file: PathBuf) -> Option<Self> {
        let toml_string = fs::read_to_string(config_file).ok()?;

        toml::from_str(&toml_string).ok()
    }

    /// Returns the [`GhConfig`] settings in the config
    pub fn github(&self) -> GithubConfig {
        self.github.clone().unwrap_or_default()
    }

    /// Returns the [`TmuxinatorConfig`] preferences in the config
    pub fn tmuxinator_config(&self) -> TmuxinatorConfig {
        self.tmuxinator.clone().unwrap_or_default()
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct GithubConfig {
    /// Whether to ask before cloning a github repo
    ///
    /// Default: `true`
    confirm_before_cloning: Option<bool>,

}

impl GithubConfig {
    /// Whether to ask before cloning a github repo
    ///
    /// Default: `true`
    pub fn confirm_before_cloning(&self) -> bool {
        self.confirm_before_cloning.unwrap_or(true)
    }
}

#[derive(Deserialize, Default, Clone)]
/// The Tmuxinator config options
pub struct TmuxinatorConfig {
    /// Whether a new tmuxinator config should be generated every boot
    ///
    /// Default: `false`
    fresh_config: Option<bool>,

    /// The command to run on opening the tmuxinator session
    ///
    /// Default: `"editor"`
    window_name: Option<String>,

    /// The name of the tmuxinator spawned window
    ///
    /// Default: `"nvim ."`
    on_open: Option<String>,
}

impl TmuxinatorConfig {
    /// The name of the tmuxinator spawned window
    ///
    /// Default: `false`
    pub fn fresh_config(&self) -> bool {
        self.fresh_config.clone().unwrap_or(false)
    }

    /// The command to run on opening the tmuxinator session
    ///
    /// Default: `"editor"`
    pub fn on_open(&self) -> String {
        self.on_open.clone().unwrap_or("nvim .".to_string())
    }

    /// The name of the tmuxinator spawned window
    ///
    /// Default: `"nvim ."`
    pub fn window_name(&self) -> String {
        self.window_name.clone().unwrap_or("editor".to_string())
    }
}
