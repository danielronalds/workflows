//! This module contains all logic for interacting with the `tmux` command line program

use std::{io, process::Command};

use crate::repo::Repo;

/// Gets a list of active tmux sessions
///
/// # Returns
///
/// A list of the active tmux session names
fn get_active_tmux_session() -> io::Result<Vec<String>> {
    let ls_output = Command::new("sh").args(["-c", "tmux ls"]).output()?;

    let stdout = String::from_utf8(ls_output.stdout).expect("This should always convert");

    let active_sessions: Vec<String> = stdout
        .lines()
        .filter(|x| !x.is_empty())
        .filter_map(|x| {
            let session_name = x.split(':').next()?.to_string();
            Some(session_name)
        })
        .collect();

    Ok(active_sessions)
}

/// Opens a project in a tmux session
///
/// # Parameters
///
/// - `project` The project to open
pub fn run_tmux(project: &Repo) -> io::Result<()> {
    let active_sessions = get_active_tmux_session().expect("Failed to get active_sessions");

    if active_sessions.contains(&project.name()) {
        return attach_tmux_session(project.name());
    }

    create_tmux_session(project)
}

/// Attaches to an existing tmux session
///
/// # Parameters
///
/// - `session_name` The session to attach to
fn attach_tmux_session(session_name: String) -> io::Result<()> {
    let command = format!("tmux a -t {}", session_name);

    let _ = Command::new("sh").args(["-c", &command]).spawn()?.wait();

    Ok(())
}

/// Creates a tmux session for the given project
///
/// # Parameters
///
/// - `project` The project to create a session for
fn create_tmux_session(project: &Repo) -> io::Result<()> {
    let command = format!(
        "tmux new -s {} -c {}",
        project.name(),
        project
            .get_project_root()
            .expect("Failed to get project's root")
            .to_str()
            .expect("Failed to convert Pathbuf to string")
    );

    let _ = Command::new("sh").args(["-c", &command]).spawn()?.wait();

    Ok(())
}
