use std::fs;
use std::process::Command;
use std::process::Output;

mod repo;
use repo::Repo;

fn main() {
    let git_repos = extract_repos(get_repos_raw());

    let projects = get_current_projects();

    let local_projects: Vec<Repo> = git_repos
        .iter()
        .filter(|x| projects.contains(&x.name()))
        .map(|x| x.to_owned())
        .collect();

    for local_project in local_projects {
        println!("{}", local_project.name());
    }
}

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

/// Exectures the gh repo list command and returns the result as an option
fn get_repos_raw() -> Option<Output> {
    Command::new("gh")
        .arg("repo")
        .arg("list")
        .arg("--limit")
        .arg("1000")
        .output()
        .ok()
}

/// Gets the list of repos from the "gh repo list" command output
fn extract_repos(output: Option<Output>) -> Vec<Repo> {
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
