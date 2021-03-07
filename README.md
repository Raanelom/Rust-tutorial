# Rust-tutorial
Using the Rust-book: https://doc.rust-lang.org/book

## Chapter 1: Getting Started
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

## Chapter 2: Programming a Guessing Game
https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

- The chapter-title is pretty much self-descriptive

### Today I Learned:
- Rust automatically brings a small set of functions into scope, referred to as "the prelude": https://doc.rust-lang.org/stable/std/prelude/index.html
- Bringing other libraries into scope can be done by using the reserved keyword `use` (e.g. `use std::io;`)
- Rust's (module) namespace-separator is `::`. It can also be used for calling "associated functions" (a.k.a. static functions). For example `String::new();`.
- Variables are immutable by default <3. Use `let` to create a new variable (e.g. `let x = "hello world";`)
- To allow a variable to be modifyable, use the explicit keyword `mut` (e.g. `let mut x = "hello world";`)
- Passing variables to other functions explicitly requires the reference-symbol `&`. By default, referenced variables are immutable (even if declared as `mut`-variable). To allow the function to edit the referenced variable, explicitly mention `mut` again as follows:
    ```rust
    let mut some_variable = String::new();
    some_function(&mut some_variable);
    ```
- Functions may return `Result`-objects (of type `enum`). A `Result` is either `Ok` or `Err`. If `Err` is returned, it can be caught using the function `expect("alternative error message here");`.
- Print variables using inline `{}`, e.g.:
    ```rust
    println("x = {}, y = {}", x, y);
    ```
