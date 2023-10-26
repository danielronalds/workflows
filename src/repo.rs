#[derive(Clone)]
/// This struct represents a github repo
pub struct Repo {
    /// The name of the repo
    name: String,
    /// Whether the project is local or not
    local: bool
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
}
