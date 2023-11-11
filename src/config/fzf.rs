//! This module contains the logic for fzf configuration

use fzf_wrapped::Layout;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct FzfConfig {
    /// Whether fzf should have the reverse layout
    ///
    /// Default: `true`
    layout: Option<String>,
}

impl FzfConfig {
    /// Whether fzf should have the reverse layout
    ///
    /// Default: `true`
    pub fn layout(&self) -> Layout {
        match self.layout.clone() {
            Some(layout) => Layout::from(layout),
            None => Layout::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::WorkflowsConfig;
    use fzf_wrapped::Layout;

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
}
