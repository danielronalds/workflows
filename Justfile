@default:
    just --list

# Runs the project with cargo run
dev:
    cargo run

# Formats the project
fmt:
    cargo fmt

test:
    cargo test

# Runs clippy
check:
    cargo clippy

# Installs to system through cargo
install:
    cargo install --path .
