use std::{fs, io};

use crate::config::WorkflowsConfig;
use crate::intergrations;
use crate::intergrations::fzf::get_project_dir;
use crate::repo::Repo;

/// Runs fzf with the user's projects, opening the one they select in a tmuxinator session
///
/// # Parameters
///
/// - `config` The user's config
pub fn open_project(config: WorkflowsConfig) -> io::Result<()> {
    let selected_project = intergrations::fzf::run_fzf(&config.fzf().open_prompt(), true, true, &config);

    if let Some(mut selected_project) = selected_project {
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

            match get_project_dir(&config) {
                Some(project_dir) => {
                    intergrations::gh::clone_repo(&selected_project, project_dir.clone())?;
                    selected_project.set_project_dir(Some(project_dir));
                }
                None => return Ok(()),
            }
        }

        intergrations::tmuxinator::run_tmuxinator(&selected_project, config.tmuxinator())?;
    }

    Ok(())
}

/// Opens a local project with the given name
///
/// # Parameters
///
/// - `project_name` The name of the project to open
/// - `config` The users config
pub fn open_specific_project(project_name: String, config: WorkflowsConfig) -> io::Result<()> {
    let project_name = project_name.trim();

    let local_projects = get_local_projects(config.general().projects_dirs());

    let matching_project = local_projects.iter().find(|x| x.name() == project_name);

    match matching_project {
        Some(repo) => intergrations::tmuxinator::run_tmuxinator(repo, config.tmuxinator())?,
        None => println!("Project not found in local projects folder!"),
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
pub fn get_local_projects(project_dirs: Vec<String>) -> Vec<Repo> {
    let home = dirs::home_dir().expect("Couldn't load home directory!");

    let mut local_repos = vec![];

    for project_dir in project_dirs {
        let entries = match fs::read_dir(home.join(&project_dir)) {
            Ok(entries) => entries,
            Err(_) => {
                fs::create_dir(home.join(&project_dir))
                    .expect("Failed to create Projects directory");
                fs::read_dir(home.join(&project_dir)).expect("Failed to read directroy")
            }
        };

        let mut dirs_projects: Vec<Repo> = entries
            .filter_map(|file| {
                let path = file.ok()?.path();
                if !path.is_dir() {
                    return None;
                }
                path.file_name()?
                    .to_str()
                    .map(|x| Repo::new(x, true, Some(&project_dir)))
            })
            .collect();

        local_repos.append(&mut dirs_projects);
    }

    local_repos
}
