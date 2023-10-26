use std::io;

use crate::{repo::Repo, tmuxinator_intergration::does_tmuxinator_project_exist};

mod repo;

mod projects;

mod tmuxinator_intergration;

fn main() -> io::Result<()> {
    let projects = projects::get_projects();

    let project = rust_fzf::select(
        projects.iter().map(|x| x.name()).collect(),
        vec!["--layout=reverse".to_owned()],
    );

    let project: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();
    let project = project.get(0).unwrap();

    if !project.local() {
        println!("Project is not local, cloning it to project folder\n");
        projects::clone_repo(&project)?;
    }

    println!(
        "{}, config? {}",
        project.name(),
        does_tmuxinator_project_exist(&project)
    );

    Ok(())
}
