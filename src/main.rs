mod repo;

mod projects;

fn main() {
    let projects = projects::get_projects();

    for repo in projects {
        println!("{}", repo.name());
    }
}
