use std::{fs, io, path::PathBuf, process::Command};

use crate::config::tmuxinator::{TmuxinatorConfig, DEFAULT_START_COMMAND};
use crate::repo::Repo;

/// The path to the tmuxinator config directory
///
/// # Returns
///
/// A [`PathBuf`] leading to ~/.config/tmuxinator/
fn tmuxinator_config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Unable to get home dir")
        .join(".config/")
        .join("tmuxinator")
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

    let config_dir = tmuxinator_config_dir();

    if !config_dir.exists() {
        return false;
    }

    let configs = fs::read_dir(config_dir)
        .expect("Failed to read directory")
        .filter_map(|file| {
            let filename = file.ok()?.file_name();
            if filename.to_str()? == config_filename {
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
/// - `project`     The project to create the config for
/// - `config`      The user's config
pub fn create_tmuxinator_config(project: &Repo, config: TmuxinatorConfig) -> io::Result<()> {
    let config_filename = format!("{}.yml", project.name());

    let config_dir = tmuxinator_config_dir();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let contents = get_config_contents(project, config);

    fs::write(config_dir.join(config_filename), contents.trim())?;

    Ok(())
}

/// Generates a tmuxinator config's contents for the given repo according to the user's preferences
///
/// # Parameters
///
/// - `project`     The project to create the config for
/// - `config`      The user's config
fn get_config_contents(project: &Repo, config: TmuxinatorConfig) -> String {
    let mut content = format!(
        "\
# {}

name: {}
root: {}

windows:",
        tmuxinator_config_dir()
            .to_str()
            .expect("Failed to cast pathbuf to string"),
        project.name(),
        project
            .get_project_root().expect("Failed to get the projects root")
            .to_str()
            .expect("Failed to cast pathbuf to string"),
    );
    for i in 0..config.window_names().len() {
        content.push_str(
            format!(
                "\n - {}: {}",
                config
                    .window_names()
                    .get(i)
                    .expect("Safe to unwrap here due to the constraint of the for loop"),
                config
                    .start_commands()
                    .get(i)
                    .unwrap_or(&DEFAULT_START_COMMAND.to_string())
            )
            .as_str(),
        );
    }

    content
}

/// Deletes a tmuxinator config for a project
///
/// # Parameters
///
/// - `project` The project to delete
pub fn delete_tmuxinator(project: &Repo) -> io::Result<()> {
    if !tmuxinator_project_exist(project) {
        return Ok(());
    }

    let config_filename = format!("{}.yml", project.name());

    fs::remove_file(tmuxinator_config_dir().join(config_filename))?;

    Ok(())
}

/// Attempts to run the selected project with tmuxinator
///
/// Fails if there is not a tmuxinator config for it to use
///
/// # Parameters
///
/// - `project`  The project to run
/// - `config`   The tmuxinator config of the program
pub fn run_tmuxinator(project: &Repo, config: TmuxinatorConfig) -> io::Result<()> {
    // fresh_config() call going first as it's faster than checking if the project exists already
    if config.fresh_config() || !tmuxinator_project_exist(project) {
        create_tmuxinator_config(project, config)?;
    }

    let command = format!("tmuxinator start {}", &project.name());

    let _ = Command::new("sh").args(["-c", &command]).spawn()?.wait();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{get_config_contents, tmuxinator_config_dir};
    use crate::config::{tmuxinator::DEFAULT_START_COMMAND, WorkflowsConfig};
    use crate::repo::Repo;

    #[test]
    fn tmuxinator_config_works_with_multiple_windows() {
        let toml = "\
                    [tmuxinator]\n\
                    window_names = ['editor', 'files']\n\
                    start_commands = ['nvim .', 'yazi']";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        let project = Repo::new("test-repo", true, Some("Projects/test-repo"));

        let generated_config = get_config_contents(&project, config.tmuxinator());

        assert_eq!(
            generated_config,
            format!(
                "\
# {}

name: {}
root: {}

windows:
 - editor: nvim .
 - files: yazi",
                tmuxinator_config_dir()
                    .to_str()
                    .expect("Failed to cast pathbuf to string"),
                project.name(),
                project
                    .get_project_root().unwrap()
                    .to_str()
                    .expect("Failed to cast pathbuf to string"),
            )
        );
    }

    #[test]
    fn tmuxinator_config_works_with_not_enough_start_commands() {
        let toml = "\
                    [tmuxinator]\n\
                    window_names = ['editor', 'files']\n\
                    start_commands = ['nvim .']";

        let config: WorkflowsConfig = toml::from_str(toml).unwrap();

        let project = Repo::new("test-repo", true, Some("Projects/test-repo"));

        let generated_config = get_config_contents(&project, config.tmuxinator());

        assert_eq!(
            generated_config,
            format!(
                "\
# {}

name: {}
root: {}

windows:
 - editor: nvim .
 - files: {}",
                tmuxinator_config_dir()
                    .to_str()
                    .expect("Failed to cast pathbuf to string"),
                project.name(),
                project
                    .get_project_root().unwrap()
                    .to_str()
                    .expect("Failed to cast pathbuf to string"),
                DEFAULT_START_COMMAND
            )
        );
    }
}
