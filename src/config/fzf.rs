//! This module contains the logic for fzf configuration

use serde::Deserialize;

const DEFAULT_REVERSE_LAYOUT: bool = true;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct FzfConfig {
    /// Whether fzf should have the reverse layout
    ///
    /// Default: `true`
    reverse_layout: Option<bool>, // TODO: Allows user's to choose from any of the three fzf
                                  // layouts by passing in a string
}

impl FzfConfig {
    /// Whether fzf should have the reverse layout
    ///
    /// Default: `true`
    pub fn reverse_layout(&self) -> bool {
        self.reverse_layout.unwrap_or(DEFAULT_REVERSE_LAYOUT)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{WorkflowsConfig, fzf::DEFAULT_REVERSE_LAYOUT};

    #[test]
    fn reverse_layout_dir_works() {
        let toml = "\
[fzf]
reverse_layout = true";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf().reverse_layout, Some(true))
    }

    #[test]
    fn default_reverse_layout_dir_works() {
        // The toml contains [fzf] so that the field is some on the WorkflowsConfig
        // allowing the testing of the reverse_layout field to be `None`
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf.clone().unwrap().reverse_layout, None);

        assert_eq!(
            config.fzf().reverse_layout(),
            DEFAULT_REVERSE_LAYOUT
        )
    }
}
