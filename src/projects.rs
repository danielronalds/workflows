use std::{fs, process::Command};

use crate::repo::Repo;

const PROJECTS_DIR: &str = "Projects/";

pub fn get_projects() -> Vec<Repo> {
    let local_projects: Vec<Repo> = get_local_projects()
        .iter()
        .map(|x| {
            let mut repo = Repo::new(x);
            repo.set_local(true);
            repo
        })
        .collect();

    let github_projects = get_users_repos();

    local_projects
        .iter()
        .chain(
            github_projects
                .iter()
                .filter(|repo| !local_projects.contains(repo)),
        )
        .map(|repo| repo.to_owned())
        .collect()
}

/// Gets the projects currently in ~/Projects/
///
/// # Returns
///
/// A vec of strings containing the names of the directories in the project folder
fn get_local_projects() -> Vec<String> {
    let home = dirs::home_dir().expect("Couldn't load home directory!");
    let entries = fs::read_dir(home.join(PROJECTS_DIR)).unwrap();

    let directories: Vec<String> = entries
        .filter_map(|file| {
            let path = file.ok()?.path();
            if !path.is_dir() {
                return None;
            }
            path.file_name()?.to_str().map(|x| x.to_owned())
        })
        .collect();
    return directories;
}

/// Gets the list of repos from the "gh repo list" command output
///
/// # Returns
///
/// A vec of repo structs
fn get_users_repos() -> Vec<Repo> {
    let output = Command::new("gh")
        .arg("repo")
        .arg("list")
        .arg("--limit")
        .arg("1000")
        .output()
        .ok();

    if let Some(output) = output {
        let repo_strings: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .filter(|x| x.contains('/'))
            .map(|x| x.to_owned())
            .collect();

        let repos: Vec<Repo> = repo_strings
            .iter()
            .filter_map(|repo_string| {
                let name = repo_string.split('/').nth(1);
                name.map(|name| Repo::new(name))
            })
            .filter(|repo| !repo.name().is_empty())
            .collect();

        return repos;
    }

    vec![]
}