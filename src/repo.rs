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
    /// - `name` The name of the repo
    /// - `url`  The URL of the repo, in the format "{user}/{name}"
    ///
    /// # Returns
    ///
    /// A Repo struct
    pub fn new<T: Into<String>>(name: T) -> Self {
        let name = name.into();
        Self { name, local: false }
    }

    /// The name of the repo
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn local(&self) -> bool {
        self.local
    }

    pub fn set_local(&mut self, value: bool) {
        self.local = value
    }
}
