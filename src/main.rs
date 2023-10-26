use std::fs;
use std::process::Command;

mod repo;
use repo::Repo;

fn main() {
    let git_repos = get_users_repos();

    let projects = get_current_projects();

    let local_projects: Vec<Repo> = git_repos
        .iter()
        .filter_map(|repo| {
            if projects.contains(&repo.name()) {
                return None;
            }
            Some(repo.to_owned())
        })
        .collect();

    for local_project in local_projects {
        println!("{}", local_project.name());
    }
}

/// Gets the projects currently in ~/Projects/
///
/// # Returns
///
/// A vec of strings containing the names of the directories in the project folder
fn get_current_projects() -> Vec<String> {
    let entries = fs::read_dir("/home/danielr/Projects/").unwrap();
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
                name.map(|name| Repo::new(name, repo_string))
            })
            .filter(|repo| !repo.name().is_empty())
            .collect();

        return repos;
    }

    vec![]
}
