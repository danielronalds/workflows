use std::io;

use crate::{repo::Repo, tmuxinator_intergration::run_tmuxinator};

mod repo;

mod projects;

mod tmuxinator_intergration;

const TERMINAL: &str = "kitty";

fn main() -> io::Result<()> {
    let projects = projects::get_projects();

    let project = rust_fzf::select(
        projects.iter().map(|x| x.name()).collect(),
        vec!["--layout=reverse".to_owned()],
    );

    let selected_projects: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();

    if let Some(selected_project) = selected_projects.get(0) {
        if !selected_project.local() {
            println!("Project is not local, cloning it to project folder\n");
            projects::clone_repo(&selected_project)?;
        }

        run_tmuxinator(TERMINAL, selected_project)?;
    }

    Ok(())
}
