//! This module contains all the commands the program contains

// TODO: Add --new command for creating a new workspace
// TODO: Add --generate-config flag for creating a default configuration
// TODO: Add --git command for cloning a git repo

mod health;
pub use health::health_check;

mod delete;
pub use delete::delete_project;

mod open;
pub use open::get_local_projects;
pub use open::open_project;
