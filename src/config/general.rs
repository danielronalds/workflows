use serde::Deserialize;

/// The default location for projects to be stored, ~/Projects/
const DEFAULT_PROJECTS_DIR: &str = "Projects/";

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct GeneralConfig {
    /// Where projects should be stored, relative path from the user's home
    projects_dir: Option<String>,
}

impl GeneralConfig {
    /// Where projects should be stored, relative path from the user's home
    pub fn projects_dir(&self) -> String {
        self.projects_dir
            .clone()
            .unwrap_or(DEFAULT_PROJECTS_DIR.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{general::DEFAULT_PROJECTS_DIR, WorkflowsConfig};

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
}
