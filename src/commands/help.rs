/// Displays the help dialog to stdout
pub fn show_help_dialog() {
    let help_dialog = format!("\
workflows v{}

A cli tool for creating a seemless workflow with remote and local git repos.

To open a project, run workflows with no arguments.

Commands
--new       Creates a new project
--open      Opens a local project
--clone     Clones the git repo from the given URL and opens it using workflows
--delete    Deletes the given project from the local machine
--health    Checks that workflows can access the required programs
--help      Show this dialog
",
env!("CARGO_PKG_VERSION"));

    println!("{}", help_dialog);
}
