use std::io;

use colored::*;

use crate::{commands::open::get_local_project, config::WorkflowsConfig};

/// Lists all local projects under their project directory
///
/// # Parameters
///
/// - `project_dir_filter` The passed in project diretory filter
/// - `config` The user's config
pub fn list_projects(
    project_dir_filter: Option<String>,
    config: WorkflowsConfig,
) -> io::Result<()> {
    config
        .general()
        .projects_dirs()
        .iter()
        .filter(|x| {
            project_dir_filter.is_none()
                || x.to_lowercase()
                    .contains(&project_dir_filter.clone().unwrap().to_lowercase())
        })
        .for_each(|project_dir| {
            println!("{}", project_dir.bold());

            get_local_project(project_dir.to_string())
                .iter()
                .map(|x| format!("• {}", x.name()))
                .for_each(|x| println!("{}", x));

            println!();
        });

    Ok(())
}
