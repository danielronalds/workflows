//! This module contains all the commands the program contains

// TODO: Add --new command for creating a new workspace
// TODO: Add --generate-config flag for creating a default configuration

mod health;
pub use health::health_check;

mod delete;
pub use delete::delete_project;

mod open;
pub use open::open_project;
