use std::process::Command;
use std::process::Output;

fn main() {
    let output = get_repos_raw();

    for repo in extract_repos(output) {
        println!("NAME: {}, \nURL: {}\n", repo.name, repo.url);
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

struct Repo {
    name: String,
    url: String,
}

impl Repo {
    pub fn new<T: Into<String>>(name: T, url: T) -> Self {
        let name = name.into();
        let url = format!("https://www.github.com/{}.git", url.into());
        Self { name, url }
    }
}

/// Gets the list of repos from the "gh repo list" command output
fn extract_repos(output: Option<Output>) -> Vec<Repo> {
    if let Some(output) = output {
        let repo_strings: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .filter(|x| x.contains('/'))
            .map(|x| x.to_owned())
            .collect();

        let repos: Vec<Repo> = repo_strings
            .iter()
            .filter_map(|repo_string| {
                let name = repo_string.split('/').nth(1);
                name.map(|name| Repo::new(name, repo_string))
            })
            .filter(|repo| !repo.name.is_empty())
            .collect();

        return repos;
    }
    vec![]
}
