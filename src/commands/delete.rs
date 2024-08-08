//! This module contains the logic for the delete command

use std::fs;
use std::io::{self, stdout, Write};

use colored::Colorize;

use crate::config::WorkflowsConfig;
use crate::intergrations::{self, git::PushedResult};
use crate::repo::Repo;

use super::get_local_projects;

/// Runs fzf with only local projects, and deletes the selected one.
///
/// If the user passes in a project name that is valid, fzf is not launched.
///
/// # Parameters
///
/// - `project` The passed in project from a parameter
/// - `config` The user's config
pub fn delete_project(project: Option<String>, config: WorkflowsConfig) -> io::Result<()> {
    // User has passed in a project with an argument
    if let Some(project) = project {
        let local_projects = get_local_projects(config.general().projects_dirs());

        let local_repo = local_projects.iter().find(|x| x.name() == project);

        return match local_repo {
            None => {
                eprintln!("No project named {} could be found!", project.bold());
                Ok(())
            }
            Some(repo) => delete_local_project(repo, config),
        };
    }

    let project = intergrations::fzf::run_fzf(&config.fzf().delete_prompt(), true, &config);

    if let Some(project) = project {
        delete_local_project(&project, config)?;
    }

    Ok(())
}

/// Deletes a project from ~/Projects/
///
/// # Parameters
///
/// - `repo`   The project to delete
/// - `config` The user's config
fn delete_local_project(repo: &Repo, config: WorkflowsConfig) -> io::Result<()> {
    let binding = repo.get_project_root().expect("Failed to get project root");
    let project_root = binding.to_str().expect("Failed to get str");

    println!("Deleting project located at {}\n", project_root.bold());

    if config.git().check_push() {
        // Checking if the project has been pushed
        print!("[{}] main pushed...", "~".bright_yellow());
        stdout().flush()?;
        // Output the result depending on the status of the function return
        println!(
            "\r{}\n",
            match intergrations::git::repo_pushed(repo)? {
                PushedResult::Status(status) => format!(
                    "[{}] main pushed   ",
                    match status {
                        false => "⨯".bright_red().bold(),
                        true => "✓".bright_green().bold(),
                    }
                ),
                PushedResult::NoConnection => format!(
                    "{}, cannot get push status",
                    "No Connection".bright_red().bold()
                ),
            }
        );
    }

    if config.git().check_tree() {
        // Checking if the project has a clean work tree
        print!("[{}] clean working tree...", "~".bright_yellow().bold());
        stdout().flush()?;
        println!(
            "\r[{}] clean working tree   \n",
            match intergrations::git::repo_clean_tree(repo)? {
                false => "⨯".bright_red().bold(),
                true => "✓".bright_green().bold(),
            }
        );
    }

    if config.git().check_push() || config.git().check_tree() {
        // Only displaying the check message if checks have been made
        println!(
            "{}: These checks are only for the main branch of the repo\n",
            "NOTE".bright_red().bold()
        );
    }

    if !casual::confirm(format!("Delete {}?", repo.name())) {
        return Ok(());
    }
    println!("Deleting tmuxinator config");
    intergrations::tmuxinator::delete_tmuxinator(repo)?;
    println!("Deleting project located at {}", project_root);
    delete_project_dir(repo)?;

    println!("Deleted {}!", repo.name());
    Ok(())
}

/// Deletes a project from ~/Projects/
///
/// # Parameters
///
/// - `project` The project to delete
fn delete_project_dir(project: &Repo) -> io::Result<()> {
    match project.get_project_root() {
        Some(project_root) => fs::remove_dir_all(project_root),
        None => Ok(()),
    }
}
