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

# Runs test, fmt --check, and clippy. Ideal before commiting
check:
    cargo test
    cargo fmt --check
    cargo clippy

# Installs to system through cargo
install:
    cargo install --path .
