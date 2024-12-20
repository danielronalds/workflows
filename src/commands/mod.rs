//! This module contains all the commands the program contains

// TODO: Add --generate-config flag for creating a default configuration
// TODO: Add rename command

mod help;
pub use help::show_help_dialog;

mod health;
pub use health::health_check;

mod delete;
pub use delete::delete_project;

mod open;
pub use open::get_local_projects;
pub use open::open_project;
pub use open::open_specific_project;

mod new;
pub use new::new_project;

mod clone;
pub use clone::git_clone;

mod borrow;
pub use borrow::borrow_project;

mod list;
pub use list::list_projects;
