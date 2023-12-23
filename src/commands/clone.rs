use crate::config::general::GeneralConfig;
use crate::{intergrations, repo::Repo};

/// Attempts to clone the git repo at the given url into the user's project folder
pub fn git_clone(url: Option<String>, config: GeneralConfig) -> Option<Repo> {
    println!("Cloning project...");

    intergrations::git::clone_repo(&url.clone()?, &config);

    // Parsing the url
    let project_name = url?.split('/').last()?.replace(".git", "");

    Some(Repo::new(project_name, true, config.projects_dir()))
}
