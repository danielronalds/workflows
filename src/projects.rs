use std::{fs, io, path::PathBuf};

use crate::repo::Repo;
use crate::PROJECTS_DIR;

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

/// Deletes a project from ~/Projects/
///
/// # Parameters
///
/// - `project` The project to delete
pub fn delete_local_project(project: &Repo) -> io::Result<()> {
    fs::remove_dir_all(get_project_root(project))?;
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
    local_repos
}
