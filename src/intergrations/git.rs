use std::{
    io,
    process::{Command, Stdio},
};

use crate::{config::WorkflowsConfig, repo::Repo};

use super::fzf::get_project_dir;

/// Represents the possible outcomes of the repo_pushed() function
pub enum PushedResult {
    Status(bool),
    NoConnection,
}

/// Attempts to clone the given repo at the url passed in
///
/// # Returns
///
/// An IO error if the repo doesn't exist, otherwise the selected project_dir
pub fn clone_repo(url: &str, config: &WorkflowsConfig) -> io::Result<String> {
    let project_dir = get_project_dir(config).expect("Failed to get a directory");

    let clone_dir = dirs::home_dir()
        .expect("Failed to get home dir")
        .join(project_dir.clone());

    let mut command = Command::new("git")
        .current_dir(clone_dir.clone())
        .args(["clone", url])
        .stdout(Stdio::piped())
        .spawn()?;

    command.wait()?;
    Ok(project_dir)
}

/// Checks if the repo has every commit pushed
///
/// Always returns false if the user is not connected to the internet
pub fn repo_pushed(repo: &Repo) -> io::Result<PushedResult> {
    let repo_dir = repo.get_project_root().expect("Failed to get the projects root");

    let output = Command::new("git")
        .current_dir(repo_dir)
        .args(["push", "-n"])
        .output()?;

    // If there is no connection, return NoConnection
    if String::from_utf8_lossy(&output.stderr).contains("fatal: Could not read from remote repo") {
        return Ok(PushedResult::NoConnection);
    }

    Ok(PushedResult::Status(
        String::from_utf8_lossy(&output.stderr).contains("up-to-date"),
    ))
}

/// Checks if the repo has a clean working tree
pub fn repo_clean_tree(repo: &Repo) -> io::Result<bool> {
    let repo_dir = repo.get_project_root().expect("Failed to get the project root");

    let output = Command::new("git")
        .current_dir(repo_dir)
        .args(["status"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).contains("nothing to commit, working tree clean"))
}
