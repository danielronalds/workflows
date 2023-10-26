use std::{
    fs,
    process::{Command, Stdio}, io,
};

use crate::repo::Repo;

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
    let tmuxinator_configs = dirs::home_dir()
        .expect("Unable to get home dir")
        .join(".config/")
        .join("tmuxinator");

    let config_filename = format!("{}.yml", project.name());

    let configs = fs::read_dir(tmuxinator_configs)
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
        return Ok(());
    }

    Command::new(terminal)
        .args(["tmuxinator","start", &project.name()])
        .spawn()?;

    Ok(())
}
