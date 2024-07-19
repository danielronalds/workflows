use serde::Deserialize;

/// The default location for projects to be stored, ~/Projects/
const DEFAULT_PROJECTS_DIR: &str = "Projects/";
const DEFAULT_OPEN_NEW_PROJECTS: bool = true;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct GeneralConfig {
    /// Where projects should be stored, relative path from the user's home
    projects_dir: Option<String>,

    /// Whether projects should be opened after they're created
    open_new_projects: Option<bool>,
}

impl GeneralConfig {
    /// Where projects should be stored, relative path from the user's home
    pub fn projects_dir(&self) -> String {
        self.projects_dir
            .clone()
            .unwrap_or(DEFAULT_PROJECTS_DIR.to_string())
    }

    /// Whether projects should be opened after they're created
    pub fn open_new_projects(&self) -> bool {
        self.open_new_projects.unwrap_or(DEFAULT_OPEN_NEW_PROJECTS)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{general::{DEFAULT_PROJECTS_DIR, DEFAULT_OPEN_NEW_PROJECTS}, WorkflowsConfig};

    #[test]
    fn projects_dir_works() {
        let toml = "\
[general]
projects_dir = 'Testing'";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.general().projects_dir, Some("Testing".to_string()))
    }

    #[test]
    fn default_projects_dir_works() {
        let toml = "[general]";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.general.clone().unwrap().projects_dir, None);

        assert_eq!(
            config.general().projects_dir(),
            DEFAULT_PROJECTS_DIR.to_string()
        )
    }

    #[test]
    fn open_new_projects_works() {
        let toml = "\
[general]
open_new_projects = false";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.general().open_new_projects, Some(false))
    }

    #[test]
    fn default_open_new_projects_works() {
        let toml = "[general]";

        let config: WorkflowsConfig = toml::from_str(toml).expect("Failed to unwrap toml");

        assert_eq!(config.general.clone().unwrap().open_new_projects, None);

        assert_eq!(
            config.general().open_new_projects(),
            DEFAULT_OPEN_NEW_PROJECTS
        )
    }
}
