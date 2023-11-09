//! This module contains the logic for configuring tmuxinator

use serde::Deserialize;

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
        self.fresh_config.unwrap_or(false)
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
