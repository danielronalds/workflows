use std::{
    fs, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::repo::Repo;

const PROJECTS_DIR: &str = "Projects/";

/// Gets the root directory for the tmuxinator config
///
/// # Parameters
///
/// - `project` The repo to get the root directory for 
///
/// # Returns
///
/// A path buf to ~/Projects/<projectname>
pub fn get_project_root(project: &Repo) -> PathBuf {
    dirs::home_dir()
        .expect("Couldn't get home directory")
        .join(PROJECTS_DIR)
        .join(format!("{}/", project.name()))
}

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

/// Gets the projects currently in ~/Projects/
///
/// # Returns
///
/// A vec of strings containing the names of the directories in the project folder
pub fn get_local_projects() -> Vec<Repo> {
    let home = dirs::home_dir().expect("Couldn't load home directory!");
    let entries = fs::read_dir(home.join(PROJECTS_DIR)).unwrap();

    let local_repos: Vec<Repo> = entries
        .filter_map(|file| {
            let path = file.ok()?.path();
            if !path.is_dir() {
                return None;
            }
            path.file_name()?.to_str().map(|x| Repo::new(x, true))
        })
        .collect();
    return local_repos;
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
pub fn get_users_repos(local_projects: &[Repo]) -> Vec<Repo> {
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
