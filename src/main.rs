use std::process::Command;
use std::process::Output;

fn main() {
    let output = get_repos_raw();

    for repo in extract_repos(output) {
        println!("{}", repo);
    }
}

/// Exectures the gh repo list command and returns the result as an option
fn get_repos_raw() -> Option<Output> {
    Command::new("gh")
        .arg("repo")
        .arg("list")
        .arg("--limit")
        .arg("1000")
        .output()
        .ok()
}

/// Gets the list of repos from the "gh repo list" command output
fn extract_repos(output: Option<Output>) -> Vec<String> {
    match output {
        Some(output) => String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .filter(|x| x.contains('/'))
            .map(|x| x.to_owned())
            .collect(),
        None => vec![],
    }
}
