use std::process::Command;

fn main() {
    let output = Command::new("gh").arg("repo").arg("list").output();

    if let Ok(output) = output {
        let output_string = String::from_utf8_lossy(&output.stdout);
        output_string
            .split_whitespace()
            .filter(|x| x.contains('/'))
            .for_each(|repo_name| println!("{}", repo_name));
    }
}
