//! This module contains the logic for fzf configuration

use fzf_wrapped::{Border, Layout, Color};
use serde::Deserialize;

const DEFAULT_BORDER_LABEL: &str = "";

/// The default prompt fzf will show when opening a project
const DEFAULT_OPEN_PROMPT: &str = "Open: ";
/// The default prompt fzf will show when deleting a project
const DEFAULT_DELETE_PROMPT: &str = "Delete: ";

const DEFAULT_POINTER: &str = ">";

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

    /// The prompt fzf should display when opening a project
    ///
    /// Default: `Open: `
    open_prompt: Option<String>,

    /// The prompt fzf should display when opening a project
    ///
    /// Default: `Open: `
    delete_prompt: Option<String>,

    /// The pointer to the current item in fzf
    ///
    /// Default: `>`
    pointer: Option<String>,

    /// The colours fzf should use, same as 
    theme: Option<String>
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

    /// The default prompt fzf will show when opening a project
    pub fn open_prompt(&self) -> String {
        self.open_prompt
            .clone()
            .unwrap_or(DEFAULT_OPEN_PROMPT.to_string())
    }

    /// The default prompt fzf will show when deleting a project
    pub fn delete_prompt(&self) -> String {
        self.delete_prompt
            .clone()
            .unwrap_or(DEFAULT_DELETE_PROMPT.to_string())
    }

    /// The pointer to the current item in fzf
    ///
    /// Default: `>`
    pub fn pointer(&self) -> String {
        self.pointer.clone().unwrap_or(DEFAULT_POINTER.to_string())
    }

    /// The theme to use fzf with
    pub fn theme(&self) -> Color {
        match self.theme.clone() {
            Some(color) => Color::from(color),
            None => Color::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{
        fzf::{DEFAULT_BORDER_LABEL, DEFAULT_DELETE_PROMPT, DEFAULT_OPEN_PROMPT, DEFAULT_POINTER},
        WorkflowsConfig,
    };
    use fzf_wrapped::{Border, Layout, Color};

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

    #[test]
    fn open_prompt_works() {
        let toml = "\
[fzf]
open_prompt = 'Launch: '";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf().open_prompt, Some("Launch: ".to_string()))
    }

    #[test]
    fn default_open_prompt_works() {
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf().open_prompt.clone(), None);

        assert_eq!(config.fzf().open_prompt(), DEFAULT_OPEN_PROMPT.to_string())
    }

    #[test]
    fn delete_prompt_works() {
        let toml = "\
[fzf]
delete_prompt = 'Remove: '";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf().delete_prompt, Some("Remove: ".to_string()))
    }

    #[test]
    fn default_delete_prompt_works() {
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf().delete_prompt.clone(), None);

        assert_eq!(
            config.fzf().delete_prompt(),
            DEFAULT_DELETE_PROMPT.to_string()
        )
    }

    #[test]
    fn pointer_works() {
        let toml = "\
[fzf]
pointer = '->'";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf().pointer, Some("->".to_string()))
    }

    #[test]
    fn default_pointer_works() {
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.fzf().pointer.clone(), None);

        assert_eq!(config.fzf().pointer(), DEFAULT_POINTER.to_string())
    }

    #[test]
    fn theme_works() {
        let toml = "\
                    [fzf]\n\
                    theme = 'bw'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().theme(), Color::Bw);
    }

    #[test]
    fn default_theme_works() {
        let toml = "[fzf]";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().theme, None);

        assert_eq!(config.fzf().theme(), Color::default())
    }

    #[test]
    fn invalid_theme_recovers() {
        let toml = "\
                    [fzf]\n\
                    theme = 'invalid-color'";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        assert_eq!(config.fzf().theme, Some("invalid-color".to_string()));

        assert_eq!(config.fzf().theme(), Color::default());
    }

}
