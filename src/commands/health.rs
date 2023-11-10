//! This module contains the logic for the health check command

use std::io::{self, stdout, Write};

use colored::Colorize;

/// Checks if the required programs are available on path
pub fn health_check() -> io::Result<()> {
    let dependencies = ["fzf", "gh", "git", "tmux", "tmuxinator"];

    for dependency in dependencies {
        print!("[{}] {}", "~".bright_yellow(), dependency);
        stdout().flush()?;
        let path = which::which(dependency);
        println!(
            "\r[{}]",
            match path {
                Ok(_) => "✓".bright_green().bold(),
                Err(_) => "⨯".bright_red().bold(),
            }
        );
    }

    Ok(())
}
