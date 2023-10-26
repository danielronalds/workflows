mod repo;

mod projects;

fn main() {
    let projects = projects::get_projects();

    let project = rust_fzf::select(
        projects.iter().map(|x| x.name()).collect(),
        vec!["--layout=reverse".to_owned()],
    );

    println!("{}", project);
}
