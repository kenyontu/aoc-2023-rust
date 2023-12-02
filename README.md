# Advent of Code solutions in Rust

Requires [cargo-watch](https://github.com/watchexec/cargo-watch) and [just](https://github.com/casey/just), install them using `cargo` with:

```bash
cargo install cargo-watch
cargo install just
```

Each day is a library crate in the workspace, you execute the tests of a specific day and part by running:

```bash
just test <day> <part>

# Example - Executing tests for the second part of the first day:
just test 1 2
```
