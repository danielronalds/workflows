//! This module contains the code for the fzf intergration
//!
//! Heavily based on the [rust_fzf library](https://crates.io/crates/rust_fzf)

use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};

use crate::config::WorkflowsConfig;
use crate::intergrations;
use crate::local_projects;
use crate::repo::Repo;

/// Run fzf to select a project. If in delete mode, only local projects will be displayed
///
/// # Parameters
///
/// - `prompt`      The prompt to display in the fzf menu
/// - `delete_mode` Whether the selected project will be deleted or not
/// - `config`      The users config
///
/// # Returns
///
/// A tuple with the first element being the name of the project selected, and the vec of Repos
/// being the merged list of local and github repos
pub fn run_fzf(
    prompt: &str,
    delete_mode: bool,
    config: &WorkflowsConfig,
) -> (String, Vec<Repo>) {
    let local_projects = local_projects::get_local_projects();

    let mut fzf_args = vec![];
    fzf_args.push(format!("--prompt={}", prompt));
    if config.fzf().reverse_layout() {
        fzf_args.push("--layout=reverse".to_string());
    }
    fzf_args.push("--color=16".into());

    let (child, mut child_in) = run_fzf_with_local(&local_projects, fzf_args);
    let mut git_projects = vec![];

    if config.github().enabled() && !delete_mode {
        git_projects = intergrations::gh::get_gh_repos(&local_projects);
        let mut fzf_in = String::new();
        for selection in &git_projects {
            fzf_in.push_str(&selection.name());
            fzf_in.push('\n');
        }
        let _ = child_in.write_all(fzf_in.as_bytes());
    }

    let output = child
        .wait_with_output()
        .expect("Failed to read fzf command stdout");

    let project = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let projects: Vec<Repo> = local_projects
        .iter()
        .chain(git_projects.iter())
        .map(|x| x.to_owned())
        .collect();

    (project, projects)
}

/// Runs fzf with the local projects
///
/// # Parameters
/// - `local_projects` The list of repos to run fzf with initially
/// - `args`           The arguments to run fzf with
///
/// # Returns
///
/// The fzf proccess and its stdin for adding more projects
fn run_fzf_with_local<T: ToString>(local_projects: &[Repo], args: Vec<T>) -> (Child, ChildStdin) {
    let mut child = Command::new("fzf")
        .args(args.iter().map(|x| x.to_string()))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let mut fzf_in = String::new();
    for selection in local_projects {
        fzf_in.push_str(&selection.name());
        fzf_in.push('\n');
    }
    stdin
        .write_all(fzf_in.as_bytes())
        .expect("Failed to write fzf_input to fzf command stdin");

    (child, stdin)
}
