use std::{
    env,
    io::{self, stdout, Write},
};

use colored::Colorize;

mod repo;
use config::WorkflowsConfig;
use repo::Repo;

use crate::intergrations::git::PushedResult;

mod local_projects;

mod intergrations;

mod config;

/// TODO: Make this path configurable via the user's config
pub const PROJECTS_DIR: &str = "Projects/";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--health".to_string()) {
        return health_check();
    }

    if args.contains(&"--version".to_string()) {
        println!("workflows v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config = config::get_config().unwrap_or_default();

    let delete_mode = args.contains(&"--delete".to_string()) || args.contains(&"-d".to_string());

    let (project, projects) = intergrations::fzf::run_fzf(
        match delete_mode {
            true => "Delete: ",
            false => "Open: ",
        },
        delete_mode,
        &config,
    );

    let selected_projects: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();

    if let Some(selected_project) = selected_projects.get(0) {
        if delete_mode {
            return delete_project(selected_project, config);
        }

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
            intergrations::gh::clone_repo(selected_project)?;
        }

        intergrations::tmuxinator::run_tmuxinator(selected_project, config.tmuxinator())?;
    }

    Ok(())
}

/// Checks if the required programs are available on path
fn health_check() -> io::Result<()> {
    let dependencies = ["fzf", "gh", "git", "tmux", "tmuxinator"];

    for dependency in dependencies {
        print!("[{}] {}", "~".bright_yellow(), dependency);
        stdout().flush()?;
        let path = which::which(dependency);
        println!(
            "\r[{}]",
            match path {
                Ok(_) => "✓".bright_green().bold(),
                Err(_) => "⨯".bright_red().bold(),
            }
        );
    }

    Ok(())
}

/// Deletes a project from ~/Projects/
///
/// # Parameters
///
/// - `repo`   The project to delete
/// - `config` The user's config
fn delete_project(repo: &Repo, config: WorkflowsConfig) -> io::Result<()> {
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
    println!("Deleting project from ~/Projects/");
    local_projects::delete_local_project(repo)?;

    println!("Deleted {}!", repo.name());
    Ok(())
}
