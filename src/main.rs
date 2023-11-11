use std::{env, io};

mod repo;

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

    if args.contains(&"--delete".to_string()) || args.contains(&"-d".to_string()) {
        return commands::delete_project(config);
    }

    commands::open_project(config)
}
