//! This module contains the logic for github configuration

use serde::Deserialize;

const DEFAULT_ENABLED: bool = true;
const DEFAULT_CONFIRM_CLONING: bool = true;
const DEFAULT_PROJECT_INDICATOR: &str = " ";

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

    /// The indicator shown next to non-local projects in fzf
    project_indicator: Option<String>,
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

    pub fn project_indicator(&self) -> String {
        self.project_indicator
            .clone()
            .unwrap_or(DEFAULT_PROJECT_INDICATOR.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{
        github::{DEFAULT_CONFIRM_CLONING, DEFAULT_ENABLED, DEFAULT_PROJECT_INDICATOR},
        WorkflowsConfig,
    };

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

    #[test]
    fn project_indicator_works() {
        let toml = "\
                    [github]\n\
                    project_indicator = 'git: '";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.github().project_indicator, Some("git: ".into()));
    }

    #[test]
    fn default_project_indicator_works() {
        let toml = "[github]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.github.clone().unwrap().project_indicator, None);

        assert_eq!(config.github().project_indicator(), DEFAULT_PROJECT_INDICATOR);
    }
}
