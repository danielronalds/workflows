use std::io;

use colored::*;

use crate::{commands::open::get_local_project, config::WorkflowsConfig};

/// Lists all local projects under their project directory
///
/// # Parameters
///
/// - `config` The user's config
pub fn list_projects(config: WorkflowsConfig) -> io::Result<()> {
    for project_dir in config.general().projects_dirs() {
        println!("{}", project_dir.bold());

        get_local_project(project_dir)
            .iter()
            .map(|x| format!("• {}", x.name()))
            .for_each(|x| println!("{}", x));

        println!();
    }

    Ok(())
}
