use std::io::{self};

use fzf_intergration::run_fzf;

use crate::{repo::Repo, tmuxinator_intergration::run_tmuxinator};

mod repo;

mod projects;

mod fzf_intergration;

mod tmuxinator_intergration;

const TERMINAL: &str = "kitty";

fn main() -> io::Result<()> {
    let (project, projects) = run_fzf();

    let selected_projects: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();

    if let Some(selected_project) = selected_projects.get(0) {
        if !selected_project.local() {
            if !casual::confirm("Project is not local, clone it to ~/Projects/?") {
                return Ok(());
            }
            projects::clone_repo(&selected_project)?;
        }

        run_tmuxinator(TERMINAL, selected_project)?;
    }

    Ok(())
}
