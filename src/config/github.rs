//! This module contains the logic for github configuration

use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
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
        self.enabled.unwrap_or(true)
    }

    /// Whether to ask before cloning a github repo
    ///
    /// Default: `true`
    pub fn confirm_cloning(&self) -> bool {
        self.confirm_cloning.unwrap_or(true)
    }
}
