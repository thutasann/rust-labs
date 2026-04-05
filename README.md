# Rust Labs

## Workspace

```.toml
[workspace]
members = [
    "rs-basics/hello_rust",
    // add more members here
]
```

## Create inside the workspace

```bash
cargo new rs-basics/hello_rust
cargo new rs-basics/ownership
cargo new projects/cli_app
```

## Run from root

```bash
cargo run -p hello_rust
cargo run -p ownership
cargo run -p cli_app
```

## Table of Content

| Topic     | Source                 | About       |
| --------- | ---------------------- | ----------- |
| Rs Basics | [Source](./rs-basics/) | Rust Basics |
