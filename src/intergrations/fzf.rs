//! This module contains the code for the fzf intergration
//!
//! Heavily based on the [rust_fzf library](https://crates.io/crates/rust_fzf)

use fzf_wrapped::Fzf;

use crate::commands;
use crate::config::fzf::FzfConfig;
use crate::config::templates::WorkspaceTemplate;
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
pub fn run_fzf(prompt: &str, delete_mode: bool, config: &WorkflowsConfig) -> Option<Repo> {
    let mut fzf = get_fzf_instance(prompt, config.fzf());

    fzf.run().expect("Failed to run fzf");

    let local_projects = commands::get_local_projects(config.general().projects_dir());
    fzf.add_items(local_projects.clone())
        .expect("Failed to add local repos");

    let mut git_projects = vec![];
    if config.github().enabled() && !delete_mode {
        git_projects =
            intergrations::gh::get_gh_repos(&local_projects, config.general().projects_dir());
        let _ = fzf.add_items(
            git_projects
                .iter()
                .map(|x| x.list_name(&config.github().project_indicator())),
        );
    }

    let projects: Vec<Repo> = local_projects
        .iter()
        .chain(git_projects.iter())
        .map(|x| x.to_owned())
        .collect();

    let project_name = fzf.output().expect("Failed to get output");

    if project_name.is_empty() {
        return None;
    }

    // Searching first without taking away the indicater prepend. Finds the project if it's local
    let filtered_project = projects
        .iter()
        .filter(|x| x.name() == project_name)
        .map(|x| x.to_owned())
        .next();

    let project = match filtered_project {
        Some(local_project) => local_project,
        None => {
            // If the project is not found with the previous search, it's a remote project
            let trimmed_name = &project_name[config.github().project_indicator().len()..];
            projects
                .iter()
                .find(|x| x.name() == trimmed_name)
                .expect("No repo exists")
                .to_owned()
        }
    };

    Some(project)
}

/// Prompts the user to select a template
///
/// # Parameters
///
/// - `config` The user's config
///
/// # Returns
///
/// `None` if the user doesn't have any templates or selects blank
pub fn get_template(config: WorkflowsConfig) -> Option<WorkspaceTemplate> {
    let templates = config.templates();
    let fzf_config = config.fzf();

    if templates.is_empty() {
        return None;
    }

    let mut template_names: Vec<String> = templates.iter().map(|x| x.name().to_string()).collect();
    template_names.push(fzf_config.no_template_option());

    let fzf = get_fzf_instance(fzf_config.template_prompt(), config.fzf());

    let selected_template = fzf_wrapped::run_with_output(fzf, template_names)?;

    match selected_template == fzf_config.no_template_option() {
        true => None,
        false => templates
            .iter()
            .find(|x| x.name() == selected_template)
            .cloned(),
    }
}

/// Gets the users Fzf instance, as defined by their config
///
/// # Parameters
///
/// - `prompt` The prompt for fzf to have
/// - `config` The user's definied fzf config
///
/// # Returns
///
/// An [`Fzf`] instance
fn get_fzf_instance(prompt: impl Into<String>, config: FzfConfig) -> Fzf {
    fzf_wrapped::Fzf::builder()
        .prompt(prompt)
        .pointer(config.pointer())
        .color(config.theme())
        .border(config.border())
        .ansi(true)
        .layout(config.layout())
        .border_label(config.border_label())
        .build()
        .unwrap()
}
