//! This module contains the logic for configuring tmuxinator

use serde::Deserialize;

const DEFAULT_FRESH_CONFIG: bool = false;
const DEFAULT_WINDOW_NAME: &str = "editor";
pub const DEFAULT_START_COMMAND: &str = "nvim .";

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
/// The Tmuxinator config options
pub struct TmuxinatorConfig {
    // Docs in the getter methods
    enabled: Option<bool>,
    fresh_config: Option<bool>,
    window_names: Option<Vec<String>>,
    start_commands: Option<Vec<String>>,
}

impl TmuxinatorConfig {
    /// Whether Tmuxinator support should be enabled. If false, then tmux is called in the launched
    /// directory
    ///
    /// Default: `true`
    pub fn enabled(&self) -> bool {
        self.enabled.unwrap_or(true)
    }

    /// Whether a new tmuxinator config should be generated every boot
    ///
    /// Default: `false`
    pub fn fresh_config(&self) -> bool {
        self.fresh_config.unwrap_or(DEFAULT_FRESH_CONFIG)
    }

    /// The command to run on opening the tmuxinator session
    ///
    /// Default: `"editor"`
    pub fn start_commands(&self) -> Vec<String> {
        self.start_commands
            .clone()
            .unwrap_or(vec![DEFAULT_START_COMMAND.to_string()])
    }

    /// The name of the tmuxinator spawned window
    ///
    /// Default: `"nvim ."`
    pub fn window_names(&self) -> Vec<String> {
        self.window_names
            .clone()
            .unwrap_or(vec![DEFAULT_WINDOW_NAME.to_string()])
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{
        tmuxinator::{DEFAULT_FRESH_CONFIG, DEFAULT_START_COMMAND, DEFAULT_WINDOW_NAME},
        WorkflowsConfig,
    };

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
                    window_names = ['testing', 'second_window']";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(
            config.tmuxinator().window_names,
            Some(vec!["testing".to_string(), "second_window".to_string()])
        );
    }

    #[test]
    fn default_window_name_works() {
        let toml = "[tmuxinator]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator.clone().unwrap().window_names, None);

        assert_eq!(
            config.tmuxinator().window_names(),
            vec![DEFAULT_WINDOW_NAME]
        );
    }

    #[test]
    fn on_open_works() {
        let toml = "\
                    [tmuxinator]\n\
                    start_commands = ['nvim .', 'yazi']";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(
            config.tmuxinator().start_commands,
            Some(vec!["nvim .".to_string(), "yazi".to_string()])
        );
    }

    #[test]
    fn default_on_open_works() {
        let toml = "[tmuxinator]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.tmuxinator.clone().unwrap().start_commands, None);

        assert_eq!(
            config.tmuxinator().start_commands(),
            vec![DEFAULT_START_COMMAND]
        );
    }
}
