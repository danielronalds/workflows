use std::{io, process::{Command, Stdio}};

use crate::{config::general::GeneralConfig, repo::Repo};

/// Represents the possible outcomes of the repo_pushed() function
pub enum PushedResult {
    Status(bool),
    NoConnection,
}

/// Attempts to clone the given repo at the url passed in
///
/// # Returns
///
/// An IO error if the repo doesn't exist, otherwise `Ok(())`
pub fn clone_repo(url: &str, config: &GeneralConfig) -> io::Result<()> {
    let clone_dir = dirs::home_dir()
                .expect("Failed to get home dir")
                .join(config.projects_dir());

    let mut command = Command::new("git")
        .current_dir(clone_dir)
        .args(["clone", url])
        .stdout(Stdio::piped())
        .spawn()?;

    command.wait()?;
    Ok(())
}

/// Checks if the repo has every commit pushed
///
/// Always returns false if the user is not connected to the internet
pub fn repo_pushed(repo: &Repo) -> io::Result<PushedResult> {
    let repo_dir = repo.get_project_root();

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
    let repo_dir = repo.get_project_root();

    let output = Command::new("git")
        .current_dir(repo_dir)
        .args(["status"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).contains("nothing to commit, working tree clean"))
}
