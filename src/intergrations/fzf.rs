//! This module contains the code for the fzf intergration
//!
//! Heavily based on the [rust_fzf library](https://crates.io/crates/rust_fzf)

use crate::commands;
use crate::config::WorkflowsConfig;
use crate::intergrations;
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
pub fn run_fzf(prompt: &str, delete_mode: bool, config: &WorkflowsConfig) -> (String, Vec<Repo>) {
    let fzf_config = config.fzf();
    let mut fzf = fzf_wrapped::Fzf::builder()
        .prompt(prompt)
        .pointer(fzf_config.pointer())
        .color(fzf_config.theme())
        .border(fzf_config.border())
        .ansi(true)
        .layout(fzf_config.layout())
        .border_label(fzf_config.border_label())
        .build()
        .unwrap();

    fzf.run().expect("Failed to run fzf");

    // NOTE: Experiment with colours for local projects and git projects

    let local_projects = commands::get_local_projects(config.general().projects_dir());
    fzf.add_items(local_projects.clone())
        .expect("Failed to add local repos");

    let mut git_projects = vec![];
    if config.github().enabled() && !delete_mode {
        git_projects =
            intergrations::gh::get_gh_repos(&local_projects, config.general().projects_dir());
        let _ = fzf.add_items(git_projects.clone()); // Ignoring output, as if the user selects a
                                                     // project before this has loaded, then a
                                                     // BrokenPipe error occurs because fzf has
                                                     // closed... But we don't care about whether
                                                     // this succeeds to not
    }

    let project = fzf.output().expect("Failed to get output");

    let projects: Vec<Repo> = local_projects
        .iter()
        .chain(git_projects.iter())
        .map(|x| x.to_owned())
        .collect();

    (project, projects)
}
