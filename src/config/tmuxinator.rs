//! This module contains the logic for configuring tmuxinator

use serde::Deserialize;

const DEFAULT_FRESH_CONFIG: bool = false;
const DEFAULT_ON_OPEN: &str = "nvim .";
const DEFAULT_WINDOW_NAME: &str = "editor";

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
/// The Tmuxinator config options
pub struct TmuxinatorConfig {
    /// Whether a new tmuxinator config should be generated every boot
    ///
    /// Default: `false`
    fresh_config: Option<bool>,

    // TODO: Allows user's to configure multiple windows via the configuration

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
        self.fresh_config.unwrap_or(DEFAULT_FRESH_CONFIG)
    }

    /// The command to run on opening the tmuxinator session
    ///
    /// Default: `"editor"`
    pub fn on_open(&self) -> String {
        self.on_open.clone().unwrap_or(DEFAULT_ON_OPEN.to_string())
    }

    /// The name of the tmuxinator spawned window
    ///
    /// Default: `"nvim ."`
    pub fn window_name(&self) -> String {
        self.window_name.clone().unwrap_or(DEFAULT_WINDOW_NAME.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{WorkflowsConfig, tmuxinator::{DEFAULT_FRESH_CONFIG, DEFAULT_WINDOW_NAME, DEFAULT_ON_OPEN}};

    #[test]
    fn fresh_config_works() {
        let toml = "\
                    [tmuxinator]\n\
                    fresh_config = false";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator().fresh_config, Some(false));
    }

    #[test]
    fn default_fresh_config_works() {
        let toml = "[tmuxinator]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator.clone().unwrap().fresh_config, None);

        assert_eq!(config.tmuxinator().fresh_config(), DEFAULT_FRESH_CONFIG);
    }

    #[test]
    fn window_name_works() {
        let toml = "\
                    [tmuxinator]\n\
                    window_name = 'testing'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator().window_name, Some("testing".to_string()));
    }

    #[test]
    fn default_window_name_works() {
        let toml = "[tmuxinator]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator.clone().unwrap().window_name, None);

        assert_eq!(config.tmuxinator().window_name(), DEFAULT_WINDOW_NAME);
    }

    #[test]
    fn on_open_works() {
        let toml = "\
                    [tmuxinator]\n\
                    on_open = 'testing'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator().on_open, Some("testing".to_string()));
    }

    #[test]
    fn default_on_open_works() {
        let toml = "[tmuxinator]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator.clone().unwrap().on_open, None);

        assert_eq!(config.tmuxinator().on_open(), DEFAULT_ON_OPEN);
    }
}
