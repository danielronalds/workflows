use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use colored::Colorize;

use crate::{
    config::{templates::WorkspaceTemplate, WorkflowsConfig},
    intergrations::fzf::{get_template, get_project_dir},
};

/// Creates a new project in the selected project directory
///
/// # Parameters
///
/// - `project_name` The name of the project to create
/// - `config` The users config
///
/// # Returns
///
/// A tuple in the format (project_name, projects_dir), otherwise `None` if the user didn't create
/// a project
pub fn new_project(
    project_name: Option<String>,
    config: WorkflowsConfig,
) -> io::Result<Option<(String, String)>> {
    if let Some(project_name) = project_name {
        let projects_dir = match get_project_dir(&config) {
            Some(projects_dir) => projects_dir,
            None => return Ok(None)
        };

        let project_dir = dirs::home_dir()
            .expect("Failed to get home directory")
            .join(projects_dir.clone())
            .join(&project_name);

        fs::create_dir_all(&project_dir)?;

        let template = get_template(config);
        run_template(template, &project_name, project_dir)?;

        return Ok(Some((project_name, projects_dir)));
    }

    Ok(None)
}

/// Runs the commands associated with a template
///
/// # Parameters
///
/// - `template` The templae to execute
/// - `project_name` The name of the project being created
/// - `project_dir` The directory the project is in, e.g. `~/Projects/workflows`
///
/// # Returns
///
/// An IO result
fn run_template(
    template: Option<WorkspaceTemplate>,
    project_name: &str,
    project_dir: PathBuf,
) -> io::Result<()> {
    if template.is_none() {
        return Ok(());
    }
    let template = template.expect("checked");

    for command in template.commands() {
        println!("{}", command.bold());

        let output = Command::new("sh")
            .arg("-c")
            .current_dir(&project_dir)
            .env("WORKFLOWS_PROJECT_NAME", project_name)
            .arg(command)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;

        println!();
    }

    Ok(())
}
