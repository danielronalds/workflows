use std::{io, process::Command};

use crate::repo::Repo;

/// Represents the possible outcomes of the repo_pushed() function
pub enum PushedResult {
    Status(bool),
    NoConnection
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

    Ok(PushedResult::Status(String::from_utf8_lossy(&output.stderr).contains("up-to-date")))
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
