//! This module contains the logic for fzf configuration

use fzf_wrapped::{Layout, Border};
use serde::Deserialize;

// TODO: Add option for configuring a border label

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct FzfConfig {
    /// What layout fzf should use
    ///
    /// Default: `default`
    layout: Option<String>,

    /// What border fzf should use
    ///
    /// Default: `none`
    border: Option<String>
}

impl FzfConfig {
    /// What layout fzf should use
    ///
    /// Default: `default`
    pub fn layout(&self) -> Layout {
        match self.layout.clone() {
            Some(layout) => Layout::from(layout),
            None => Layout::default(),
        }
    }

    /// What border fzf should use
    ///
    /// Default: `none`
    pub fn border(&self) -> Border {
        match self.border.clone() {
            Some(border) => Border::from(border),
            None => Border::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::WorkflowsConfig;
    use fzf_wrapped::{Layout, Border};

    #[test]
    fn layout_works() {
        let toml = "\
                    [fzf]\n\
                    layout = 'reverse'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().layout(), Layout::Reverse);
    }

    #[test]
    fn default_layout_works() {
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().layout, None);

        assert_eq!(config.fzf().layout(), Layout::default())
    }

    #[test]
    fn border_works() {
        let toml = "\
                    [fzf]\n\
                    border = 'rounded'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().border(), Border::Rounded);
    }

    #[test]
    fn default_border_works() {
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().border, None);

        assert_eq!(config.fzf().border(), Border::default())
    }

    #[test]
    fn invalid_border_recovers() {
        let toml = "\
                    [fzf]\n\
                    border = 'invalid-border'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().border, Some("invalid-border".to_string()));

        assert_eq!(config.fzf().border(), Border::default());
    }
}
