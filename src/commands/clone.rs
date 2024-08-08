use crate::config::WorkflowsConfig;
use crate::{intergrations, repo::Repo};

/// Attempts to clone the git repo at the given url into the user's project folder
pub fn git_clone(url: Option<String>, config: &WorkflowsConfig) -> Option<Repo> {
    let project_dir = intergrations::git::clone_repo(&url.clone()?, config).ok()?;

    // Parsing the url
    let project_name = url?.split('/').last()?.replace(".git", "");

    Some(Repo::new(
        project_name,
        true,
        Some(project_dir.to_string_lossy().to_string()),
    ))
}
