use std::{fs, io, path::PathBuf, process::Command};

use crate::{projects, repo::Repo};

const EDITOR: &str = "nvim";

/// The path to the tmuxinator config directory
///
/// # Returns
///
/// A PathBuf leading to ~/.config/tmuxinator/
fn tmuxinator_config_dir() -> PathBuf {
    let tmuxinator_configs = dirs::home_dir()
        .expect("Unable to get home dir")
        .join(".config/")
        .join("tmuxinator");

    tmuxinator_configs
}

/// Checks if the project already has a tmuxinator project
///
/// # Parameters
///
/// - `project` The project to check for
///
/// # Returns
///
/// `true` if <projectname>.yml exists in ~/.config/tmuxinator/
fn tmuxinator_project_exist(project: &Repo) -> bool {
    let config_filename = format!("{}.yml", project.name());

    let configs = fs::read_dir(tmuxinator_config_dir())
        .expect("Failed to read directory")
        .filter_map(|file| {
            let filename = file.ok()?.file_name();
            if filename.to_str()? == &config_filename {
                return Some(filename);
            }
            None
        });

    configs.count() == 1
}

/// Creates a tmuxinator config for a project
///
/// # Parameters
///
/// - `project` The project to create the config for
pub fn create_tmuxinator_config(project: &Repo, editor: &str) -> io::Result<()> {
    let config_filename = format!("{}.yml", project.name());
    let config_path = tmuxinator_config_dir();
    let project_root = projects::get_project_root(project);
    let contents = format!(
        "\
# {}

name: {}
root: {}

windows:
  - editor: {} .",
        config_path
            .to_str()
            .expect("Failed to cast pathbuf to string"),
        project.name(),
        project_root
            .to_str()
            .expect("Failed to cast pathbuf to string"),
        editor
    );

    fs::write(config_path.join(config_filename), contents.trim())?;

    Ok(())
}

/// Attempts to run the selected project with tmuxinator
///
/// Fails if there is not a tmuxinator config for it to use
///
/// TODO: Create config if it doesn't exist
///
/// # Parameters
///
/// - `terminal` The terminal emulator to use
/// - `project`  The project to run
pub fn run_tmuxinator(terminal: &str, project: &Repo) -> io::Result<()> {
    if !tmuxinator_project_exist(project) {
        create_tmuxinator_config(project, EDITOR)?;
    }

    Command::new(terminal)
        .args(["tmuxinator", "start", &project.name()])
        .spawn()?;

    Ok(())
}
