//! This module contains the logic for github configuration

use serde::Deserialize;

const DEFAULT_ENABLED: bool = true;
const DEFAULT_CONFIRM_CLONING: bool = true;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct GithubConfig {
    /// Whether github projects should be loaded
    ///
    /// Default: `true`
    enabled: Option<bool>,

    /// Whether to ask before cloning a github repo
    ///
    /// Default: `true`
    confirm_cloning: Option<bool>,
}

impl GithubConfig {
    /// Whether github projects should be loaded
    ///
    /// Default: `true`
    pub fn enabled(&self) -> bool {
        self.enabled.unwrap_or(DEFAULT_ENABLED)
    }

    /// Whether to ask before cloning a github repo
    ///
    /// Default: `true`
    pub fn confirm_cloning(&self) -> bool {
        self.confirm_cloning.unwrap_or(DEFAULT_CONFIRM_CLONING)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{WorkflowsConfig, github::{DEFAULT_ENABLED, DEFAULT_CONFIRM_CLONING}};

    #[test]
    fn enabled_works() {
        let toml = "\
                    [github]\n\
                    enabled = false";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.github().enabled, Some(false));
    }

    #[test]
    fn default_enabled_works() {
        let toml = "[github]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.github.clone().unwrap().enabled, None);

        assert_eq!(config.github().enabled(), DEFAULT_ENABLED);
    }

    #[test]
    fn confirm_cloning_works() {
        let toml = "\
                    [github]\n\
                    confirm_cloning = false";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.github().confirm_cloning, Some(false));
    }

    #[test]
    fn default_confirm_cloning_works() {
        let toml = "[github]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.github.clone().unwrap().confirm_cloning, None);

        assert_eq!(config.github().confirm_cloning(), DEFAULT_CONFIRM_CLONING);
    }
}
