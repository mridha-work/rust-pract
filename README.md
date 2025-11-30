# rust-pract

This repository contains Rust programming exercises and practice projects to learn and improve Rust development skills.

## Project Structure

Each project exercise is organized as a separate binary target:

```
src/
├── palindrome/
└── ...
```

## Running the Code

To run the first project `[[bin]]` defined in `Cargo.toml`, use the following command:

```bash
cargo run
```

To run a specific project exercise, use the following command:

```bash
cargo run --bin {project-name}
```

### Examples:

```bash
# Run longest palindrome project
cargo run --bin palindrome
```
