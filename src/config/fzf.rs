//! This module contains the logic for fzf configuration

use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct FzfConfig {
    /// Whether fzf should have the reverse layout
    ///
    /// Default: `true`
    reverse_layout: Option<bool>,
}

impl FzfConfig {
    /// Whether fzf should have the reverse layout
    ///
    /// Default: `true`
    pub fn reverse_layout(&self) -> bool {
        self.reverse_layout.unwrap_or(true)
    }
}
