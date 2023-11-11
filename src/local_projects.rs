use std::{fs, io};

use crate::repo::Repo;

/// Gets the projects currently in ~/Projects/
///
/// # Parameters
///
/// - `project_dir` The directory containing local projects
///
/// # Returns
///
/// A vec of strings containing the names of the directories in the project folder
pub fn get_local_projects(project_dir: String) -> Vec<Repo> {
    let home = dirs::home_dir().expect("Couldn't load home directory!");
    let entries = match fs::read_dir(home.join(&project_dir)) {
        Ok(entries) => entries,
        Err(_) => {
            fs::create_dir(home.join(&project_dir)).expect("Failed to create Projects directory");
            fs::read_dir(home.join(&project_dir)).expect("Failed to read directroy")
        }
    };

    let local_repos: Vec<Repo> = entries
        .filter_map(|file| {
            let path = file.ok()?.path();
            if !path.is_dir() {
                return None;
            }
            path.file_name()?
                .to_str()
                .map(|x| Repo::new(x, true, &project_dir))
        })
        .collect();
    local_repos
}
