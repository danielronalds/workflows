/// Displays the help dialog to stdout
pub fn show_help_dialog() {
    let help_dialog = format!(
        "\
workflows v{}

A cli tool for creating a seemless workflow with remote and local git repos.

To open a project, run workflows with no arguments.

Commands
  --new       -n   Creates a new project
  --open      -o   Opens a local project
  --clone     -c   Clones the git repo from the given URL and opens it using workflows
  --delete    -d   Deletes the given project from the local machine
  --health         Checks that workflows can access the required programs
  --help      -h   Show this dialog
",
        env!("CARGO_PKG_VERSION")
    );

    println!("{}", help_dialog);
}
