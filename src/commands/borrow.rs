use std::io;

use colored::Colorize;

use crate::{
    config::WorkflowsConfig,
    intergrations::{self, fzf::get_project_dir},
};

use super::delete;

/// Clones a user-selected project from github, and prompts to delete it after the session ends
///
/// # Parameters
///
/// - `config` The user's config
pub fn borrow_project(config: WorkflowsConfig) -> io::Result<()> {
    if !config.github().enabled() {
        println!(
            "{} borrowing can only be done when github intergration is enabled",
            "ERROR".bright_red()
        );
        return Ok(());
    }

    let selected_project =
        intergrations::fzf::run_fzf(&config.fzf().open_prompt(), false, true, &config);

    if let Some(mut selected_project) = selected_project {
        match get_project_dir(&config) {
            Some(project_dir) => {
                intergrations::gh::clone_repo(&selected_project, project_dir.clone())?;
                selected_project.set_project_dir(Some(project_dir));
            }
            None => return Ok(()),
        }

        intergrations::tmuxinator::run_tmuxinator(&selected_project, config.tmuxinator())?;

        println!();
        delete::delete_local_project(&selected_project, true, config)?;
    }

    Ok(())
}
