//! This module contains the logic for github configuration

use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct GitConfig {
    /// Whether to check for a clean work tree in the repo before deleting
    check_tree: Option<bool>,

    /// Whether to check the push status of the repo  before deleting
    check_push: Option<bool>,
}

impl GitConfig {
    /// Whether to check for a clean work tree in the repo before deleting
    ///
    /// Peformance cost is neglagible
    ///
    /// Default: `true`
    pub fn check_tree(&self) -> bool {
        self.check_tree.unwrap_or(true)
    }

    /// Whether to check the push status of the repo  before deleting
    ///
    /// Peformance cost is noticable, as it involves networks.
    ///
    /// Default: `true`
    pub fn check_push(&self) -> bool {
        self.check_push.unwrap_or(true)
    }
}
