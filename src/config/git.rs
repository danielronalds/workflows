//! This module contains the logic for git configuration

use serde::Deserialize;

const DEFAULT_CHECK_TREE: bool = true;
const DEFAULT_CHECK_PUSH: bool = true;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
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
        self.check_tree.unwrap_or(DEFAULT_CHECK_TREE)
    }

    /// Whether to check the push status of the repo  before deleting
    ///
    /// Peformance cost is noticable, as it involves networks.
    ///
    /// Default: `true`
    pub fn check_push(&self) -> bool {
        self.check_push.unwrap_or(DEFAULT_CHECK_PUSH)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{
        git::{DEFAULT_CHECK_PUSH, DEFAULT_CHECK_TREE},
        WorkflowsConfig,
    };

    #[test]
    fn check_tree_works() {
        let toml = "\
                    [git]\n\
                    check_tree = false";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.git().check_tree, Some(false));
    }

    #[test]
    fn default_check_tree_works() {
        let toml = "[git]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.git.clone().unwrap().check_tree, None);

        assert_eq!(config.git().check_tree(), DEFAULT_CHECK_TREE);
    }

    #[test]
    fn check_push_works() {
        let toml = "\
                    [git]\n\
                    check_push = false";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.git().check_push, Some(false));
    }

    #[test]
    fn default_check_push_works() {
        let toml = "[git]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.git.clone().unwrap().check_push, None);

        assert_eq!(config.git().check_push(), DEFAULT_CHECK_PUSH);
    }
}
