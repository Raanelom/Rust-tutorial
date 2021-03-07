# Rust-tutorial
Using the Rust-book: https://doc.rust-lang.org/book

## Chapter 1 Getting Started
https://doc.rust-lang.org/book/ch01-00-getting-started.html

- Writing the first lines of code (hello_world)
- Using Cargo, the Rust-package manager

### Today I Learned
- `rustup` is a CLI for managing Rust versions and tools.
- Rust is ahead-of-time compiled
- Source code can be compiled using the (rustup native) `rustc`-command
- Formatting code is easy using the (rustup native) command `rustfmt`
- Function-like calls using `!`-signs as suffix indicate the use of a "macro" instead of a "function". For instance `println!("hello world")`.
- `cargo` is Rust's Swiss army knife => among others it is a package (`crate`) manager
- `cargo new` automatically creates a git-repository
- Cargo-configuration files follow the [TOML](https://toml.io/en/)-standard
- Convenient commands: `cargo build | run | check`