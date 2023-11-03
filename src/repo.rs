use std::path::PathBuf;

use crate::PROJECTS_DIR;

#[derive(Clone)]
/// This struct represents a project
///
/// Terminology used is `Repo` as in theory it's a git repo
pub struct Repo {
    /// The name of the repo
    name: String,
    /// Whether the project is local or not
    local: bool,
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name()
    }
}

impl Repo {
    /// Creates a repo struct
    ///
    /// # Arguments
    ///
    /// - `name`   The name of the repo
    /// - `local`  Whether the projects exists in ~/Projects/
    ///
    /// # Returns
    ///
    /// A Repo struct
    pub fn new<T: Into<String>>(name: T, local: bool) -> Self {
        let name = name.into();
        Self { name, local }
    }

    /// The name of the repo
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Whether the project already exists in ~/Projects
    pub fn local(&self) -> bool {
        self.local
    }

    /// Gets the root directory for the tmuxinator config
    ///
    /// # Returns
    ///
    /// A path buf to ~/Projects/<projectname>
    pub fn get_project_root(&self) -> PathBuf {
        dirs::home_dir()
            .expect("Couldn't get home directory")
            .join(PROJECTS_DIR)
            .join(format!("{}/", self.name))
    }
}

impl Into<String> for Repo {
    fn into(self) -> String {
        self.name()
    }
}
