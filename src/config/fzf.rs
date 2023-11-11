//! This module contains the logic for fzf configuration

use fzf_wrapped::{Border, Layout};
use serde::Deserialize;

const DEFAULT_BORDER_LABEL: &str = "";

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct FzfConfig {
    /// What layout fzf should use
    ///
    /// Default: `default`
    layout: Option<String>,

    /// What border fzf should use
    ///
    /// Default: `none`
    border: Option<String>,

    /// What label should be shown in the border, requires border to not be none
    ///
    /// Default: `""`
    border_label: Option<String>,
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

    /// What label should be shown in the border, requires border to not be none
    ///
    /// Default: `""`
    pub fn border_label(&self) -> String {
        self.border_label
            .clone()
            .unwrap_or(DEFAULT_BORDER_LABEL.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{fzf::DEFAULT_BORDER_LABEL, WorkflowsConfig};
    use fzf_wrapped::{Border, Layout};

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

    #[test]
    fn border_label_works() {
        let toml = "\
                    [fzf]\n\
                    border_label = 'Workflows'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().border_label, Some("Workflows".to_string()));
    }

    #[test]
    fn default_border_label_works() {
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().border_label, None);

        assert_eq!(
            config.fzf().border_label(),
            DEFAULT_BORDER_LABEL.to_string()
        );
    }
}
