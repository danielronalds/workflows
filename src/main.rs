use std::{env, io};

mod repo;
use repo::Repo;

mod local_projects;

mod intergrations;

mod config;

mod commands;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--health".to_string()) {
        return commands::health_check();
    }

    if args.contains(&"--version".to_string()) {
        println!("workflows v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let config = config::get_config().unwrap_or_default();

    // TODO: Separate the delete and opening logic into their own modules within `commands`
    let delete_mode = args.contains(&"--delete".to_string()) || args.contains(&"-d".to_string());

    let (project, projects) = intergrations::fzf::run_fzf(
        match delete_mode {
            true => "Delete: ",
            false => "Open: ",
        },
        delete_mode,
        &config,
    );

    let selected_projects: Vec<Repo> = projects
        .iter()
        .filter(|x| x.name() == project)
        .map(|x| x.to_owned())
        .collect();

    if let Some(selected_project) = selected_projects.get(0) {
        if delete_mode {
            return commands::delete_project(selected_project, config);
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
            intergrations::gh::clone_repo(selected_project, config.general().projects_dir())?;
        }

        intergrations::tmuxinator::run_tmuxinator(selected_project, config.tmuxinator())?;
    }

    Ok(())
}
