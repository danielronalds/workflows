use std::path::PathBuf;

#[derive(Clone)]
/// This struct represents a project
///
/// Terminology used is `Repo` as in theory it's a git repo
pub struct Repo {
    /// The name of the repo
    name: String,
    /// Whether the project is local or not
    local: bool,
    /// The path to the projects directory
    project_dir: String,
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
    /// - `name`        The name of the repo
    /// - `local`       Whether the projects exists in defined local repo
    /// - `project_dir` The path to the directory containing the projects
    ///
    /// # Returns
    ///
    /// A Repo struct
    pub fn new<T: Into<String>>(name: T, local: bool, project_dir: T) -> Self {
        let name = name.into();
        let project_dir = project_dir.into();
        Self {
            name,
            local,
            project_dir,
        }
    }

    /// The name of the repo
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the name of the project, with an indicator if it's not local
    pub fn list_name(&self) -> String {
        format!(
            "{}{}",
            match self.local {
                true => "",
                false => "  ",
            },
            self.name()
        )
    }

    /// Whether the project already exists in ~/Projects
    pub fn local(&self) -> bool {
        self.local
    }

    /// Gets the root directory for the tmuxinator config
    ///
    /// # Returns
    ///
    /// A path buf to ~/<project_dir>/<projectname>
    pub fn get_project_root(&self) -> PathBuf {
        dirs::home_dir()
            .expect("Couldn't get home directory")
            .join(self.project_dir.as_str())
            .join(format!("{}/", self.name))
    }
}

impl From<Repo> for String {
    fn from(value: Repo) -> Self {
        value.list_name()
    }
}
