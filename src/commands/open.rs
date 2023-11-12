use std::{fs, io};

use crate::config::WorkflowsConfig;
use crate::intergrations;
use crate::repo::Repo;

/// Runs fzf with the user's projects, opening the one they select in a tmuxinator session
///
/// # Parameters
///
/// - `config` The user's config
pub fn open_project(config: WorkflowsConfig) -> io::Result<()> {
    let (project, projects) = intergrations::fzf::run_fzf("Open: ", false, &config);

    let selected_projects: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();

    if let Some(selected_project) = selected_projects.get(0) {
        if !selected_project.local() {
            if config.github().confirm_cloning()
                && !casual::prompt("Project is not local, clone it to ~/Projects/?")
                    .suffix(" [Y/n] ")
                    .default("y".to_string())
                    .matches(|s| matches!(&*s.trim().to_lowercase(), "n" | "no" | "y" | "yes"))
                    .map(|s| matches!(&*s.to_lowercase(), "y" | "yes"))
            {
                return Ok(());
            }
            intergrations::gh::clone_repo(selected_project, config.general().projects_dir())?;
        }

        intergrations::tmuxinator::run_tmuxinator(selected_project, config.tmuxinator())?;
    }

    Ok(())
}

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
