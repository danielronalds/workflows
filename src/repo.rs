#[derive(Clone)]
/// This struct represents a github repo
pub struct Repo {
    /// The name of the repo
    name: String,
    /// The url to clone the repo with
    url: String,
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
    pub fn new<T: Into<String>>(name: T, url: T) -> Self {
        let name = name.into();
        let url = format!("https://github.com/{}.git", url.into());
        Self { name, url }
    }

    /// The name of the repo
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// The url to clone the repo with
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests if the parsed URL is the same as the github clone url
    fn parsed_url_same_as_github_url() {
        let url = "danielronalds/workflow";
        let repo = Repo::new("workflow", url);
        assert_eq!(repo.url(), "https://github.com/danielronalds/workflow.git") // URL grabbed from github
    }
}
