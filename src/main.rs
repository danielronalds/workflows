use std::{env, io};

mod repo;

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

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        commands::show_help_dialog();
        return Ok(());
    }

    let config = config::get_config().unwrap_or_default();

    if args.contains(&"--delete".to_string()) || args.contains(&"-d".to_string()) {
        return commands::delete_project(config);
    }

    if args.contains(&"--new".to_string()) || args.contains(&"-n".to_string()) {
        let project = commands::new_project(args.get(2).cloned(), config.general())?;
        if let Some(project) = project {
            let project = repo::Repo::new(project, true, config.general().projects_dir());
            return intergrations::tmuxinator::run_tmuxinator(&project, config.tmuxinator());
        }
    }

    if args.contains(&"--clone".to_string()) || args.contains(&"-c".to_string()) {
        let repo = commands::git_clone(args.get(2).cloned(), config.general());
        if let Some(repo) = repo {
            return intergrations::tmuxinator::run_tmuxinator(&repo, config.tmuxinator());
        }
    }

    commands::open_project(config)
}
