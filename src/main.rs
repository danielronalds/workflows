use crate::repo::Repo;

mod repo;

mod projects;

mod tmuxinator_intergration;

fn main() {
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

    println!(
        "{}, config? {}",
        project.name(),
        tmuxinator_intergration::does_tmuxinator_project_exist(&project)
    );
}
