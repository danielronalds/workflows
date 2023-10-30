use std::{
    env,
    io::{self},
};

use local_projects::delete_local_project;

use repo::Repo;

use intergrations::tmuxinator::{run_tmuxinator, delete_tmuxinator};

use intergrations::fzf::run_fzf;

mod repo;

mod local_projects;

mod intergrations;

mod config;

pub const PROJECTS_DIR: &str = "Projects/";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config = config::get_config().unwrap_or_default();

    let delete_mode = args.contains(&String::from("--delete"));

    let (project, projects) = run_fzf(
        match delete_mode {
            true => "Delete: ",
            false => "Open: ",
        },
        delete_mode,
        config.fzf(),
        config.github()
    );

    let selected_projects: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();

    if let Some(selected_project) = selected_projects.get(0) {
        if delete_mode {
            if !casual::confirm("Are you sure everything is commited and pushed?") {
                return Ok(());
            }
            println!("Deleting tmuxinator config");
            delete_tmuxinator(selected_project)?;
            println!("Deleting project from ~/Projects/");
            delete_local_project(selected_project)?;

            println!("Deleted {}!", selected_project.name());
            return Ok(());
        }

        if !selected_project.local() {
            if config.github().confirm_cloning()
                && !casual::prompt("Project is not local, clone it to ~/Projects/?")
                    .suffix(" [Y/n] ")
                    .default("y".to_string())
                    .matches(|s| matches!(&*s.trim().to_lowercase(), "n" | "no" | "y" | "yes"))
                    .map(|s| matches!(&*s.to_lowercase(), "y" | "yes"))
            {
                return Ok(());
            }
            intergrations::gh::clone_repo(selected_project)?;
        }

        run_tmuxinator(selected_project, config.tmuxinator_config())?;
    }

    Ok(())
}
