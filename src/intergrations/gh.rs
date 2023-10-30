//! This module contains the code for intergrating with the `gh` cli tool

use std::io;
use std::process::{Command, Stdio};

use crate::repo::Repo;
use crate::PROJECTS_DIR;

/// Clones a repo using `gh`, streaming its output to stdout.
///
/// **Blocks execution until finished**
///
/// # Parameters
///
/// - `repo` The repo to clone
pub fn clone_repo(repo: &Repo) -> io::Result<()> {
    let clone_dir = dirs::home_dir()
        .expect("couldn't get home dir")
        .join(PROJECTS_DIR);
    let mut command = Command::new("gh")
        .current_dir(clone_dir)
        .args(["repo", "clone", &repo.name()])
        .stdout(Stdio::piped())
        .spawn()?;
    command.wait().expect("Failed to wait on clone");
    Ok(())
}

/// Gets the list of repos from the "gh repo list" command output, filtering out local projects
///
/// # Parameters
///
/// - `local_projects` Repo structs to filter out
///
/// # Returns
///
/// A vec of repo structs
pub fn get_gh_repos(local_projects: &[Repo]) -> Vec<Repo> {
    let output = Command::new("gh")
        .args(["repo", "list", "--limit", "1000"])
        .output()
        .ok();

    if let Some(output) = output {
        if String::from_utf8_lossy(&output.stderr).contains("error connecting") {
            return vec![];
        }

        let repo_strings: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .filter(|x| x.contains('/'))
            .map(|x| x.to_owned())
            .collect();

        let repos: Vec<Repo> = repo_strings
            .iter()
            .filter_map(|repo_string| {
                let name = repo_string.split('/').nth(1);
                name.map(|name| Repo::new(name, false))
            })
            .filter(|repo| !repo.name().is_empty() && !local_projects.contains(repo))
            .collect();

        return repos;
    }

    vec![]
}
