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
- Library-Crates can be declared in `Cargo.toml` in the `[dependencies]`-section (+ semantic version number)
- Rust's Crate-registry is located at crate.io
- `Cargo.lock` pins Crate version-numbers, unless the newest patches are explicitly fetched using `cargo update`. In other words, updating from 3.5.5 to 3.5.7 requires `cargo update`. Otherwise, `cargo.lock` will preserve the installed package with version `3.5.5`.
- Rust uses `match` expressions to express conditions simlar to Python's lambda-expressions. It consists of "arms", and each arm consists of a "pattern". If the arm's pattern is matched, the corresponding command is executed. A match-expression is finished if one of the arm patterns evaluated to "true".
- Rust allows "shadowing" existing variables, e.g. overriding an existing variable "var_a" of type string with a new variable "var_a" with type int.
- Rust knows multiple signed and unsigned integer types, such as i32, u32, i64 and u64. By default, an integer is 32-bit and signed (`i32`).
- Types can be expressed using `:`, e.g. `let x: i32 = 42`
- `loop { }` is an endless condition-less loop, which can be exited using `break`
- Match can also be used as inline expression, for example: 
    ```rust
    let x = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // handle error here
        }
    };
    ```