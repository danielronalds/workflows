use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub struct GeneralConfig {
    /// Where projects should be stored, relative path from the user's home
    projects_dir: Option<String>,
}

/// The default location for projects to be stored, ~/Projects/
const DEFAULT_PROJECTS_DIR: &str = "Projects/";

impl GeneralConfig {
    /// Where projects should be stored, relative path from the user's home
    pub fn projects_dir(&self) -> String {
        self.projects_dir
            .clone()
            .unwrap_or(DEFAULT_PROJECTS_DIR.to_string())
    }
}
