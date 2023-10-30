use std::{io, process::Command};

use crate::repo::Repo;

#[allow(dead_code)]
/// Checks if the repo has every commit pushed
pub fn repo_pushed(repo: &Repo) -> io::Result<bool> {
    let repo_dir = repo.get_project_root();

    let output = Command::new("git")
        .current_dir(repo_dir)
        .args(["push", "-n"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stderr).contains("up-to-date"))
}

#[allow(dead_code)]
/// Checks if the repo has a clean working tree
pub fn repo_clean_tree(repo: &Repo) -> io::Result<bool> {
    let repo_dir = repo.get_project_root();

    let output = Command::new("git")
        .current_dir(repo_dir)
        .args(["status"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).contains("nothing to commit, working tree clean"))
}
