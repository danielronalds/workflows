use std::{
    env,
    io::{self},
};

use fzf_intergration::run_fzf;

use crate::{repo::Repo, tmuxinator_intergration::run_tmuxinator};

mod repo;

mod projects;

mod fzf_intergration;

mod tmuxinator_intergration;

mod git_intergration;

const TERMINAL: &str = "kitty";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let delete_mode = args.contains(&String::from("--delete"));

    let (project, projects) = run_fzf(match delete_mode {
        true => "Delete: ",
        false => "Open: ",
    }, delete_mode);

    let selected_projects: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();

    if let Some(selected_project) = selected_projects.get(0) {
        if delete_mode {
            let clean_tree = git_intergration::repo_clean_tree(&selected_project)?;
            let pushed = git_intergration::repo_pushed(&selected_project)?;
            println!("Clean tree: {}\nPushed: {}", clean_tree, pushed);
            return Ok(());
        }

        if !selected_project.local() {
            if !casual::confirm("Project is not local, clone it to ~/Projects/?") {
                return Ok(());
            }
            projects::clone_repo(selected_project)?;
        }

        run_tmuxinator(TERMINAL, selected_project)?;
    }

    Ok(())
}
