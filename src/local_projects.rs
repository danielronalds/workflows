use std::{fs, io};

use crate::repo::Repo;
use crate::PROJECTS_DIR;

/// Deletes a project from ~/Projects/
///
/// # Parameters
///
/// - `project` The project to delete
pub fn delete_local_project(project: &Repo) -> io::Result<()> {
    fs::remove_dir_all(project.get_project_root())?;
    Ok(())
}

/// Gets the projects currently in ~/Projects/
///
/// # Returns
///
/// A vec of strings containing the names of the directories in the project folder
pub fn get_local_projects() -> Vec<Repo> {
    let home = dirs::home_dir().expect("Couldn't load home directory!");
    let entries = match fs::read_dir(home.join(PROJECTS_DIR)) {
        Ok(entries) => entries,
        Err(_) => {
            fs::create_dir(home.join(PROJECTS_DIR)).expect("Failed to create Projects directory");
            fs::read_dir(home.join(PROJECTS_DIR)).expect("Failed to read directroy")
        }
    };

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
