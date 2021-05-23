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


## Chapter 3: Common Programming Concepts
https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html

Reseved keywords: https://doc.rust-lang.org/stable/book/appendix-01-keywords.html


### Today I Learned:
- Besides (mutable) variables, there's also the concept of "constants" (`const`). These require type-annotation.
- Variables can't be declared in global scope. Constants can.
- Integers range from 8-bit (`[u/i]8`) to 128-bit (`[u/i]128`)
- `isize` and `usize` adapt to the restrictions of the OS (in most cases 32-bit vs 64-bit)
- Possible notations:
    | Type | Value |
    | ---- | ----- |
    | Decimal | `98_222` |
    | Hex | `0xff` |
    | Octal | `0o77` |
    | Binary | `0b1111_0000` |
    | Byte (u8 only) | `b'A'` |
- Rust panics in debug mode if an overflow occurs. In Release-mode, the value wraps around (thus for 8-bit integers, 256 => 0, 257 => 1, etc.)
- There are two type of reserved keywords for floating-points: `f32` and `f64` (default)
- Boolean type can be used with keyword `bool`
- Rust's `char` type is a four-byte Unicode Scalar Value
- A "Scalar type" represents a single value (such as int, string, bool and char), whereas "Compound types" groups multilpe values into one type.
- Rust has two Compound types: "array" `[]` and "tuple" (`tup`). Example:
    ```rust
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    let test_array = [1, 2, 3];
    let two = test_array[1];
    ```
- Array-types and length can be declared inline:
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
- Repeating the same number in an array:
    ```rust
    let a = [3; 5];
    // a = [3, 3, 3, 3, 3]
    ```
- Using array-indices that are out of bounds will produce an error at compile-time. E.g.:
    ```rust
    let a: [i32; 3] = [1, 2, 3];
    println!("{}", a[9]); // This will not compile
    ```
- `snake case` is the conventional style for declaring functions.
- It doesn't matter in which order functions are declared in a source-file.
- Function-parameters require static typing
- Rust distinguishes between "Statements" and "Expressions". Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value.
- Rule of thumb: statements end with a semicolon, whereas expressions do not
- Use of the `return` keyword is optional. A function with return type works as follows:
    ```rust
    fn five() -> i32 {
        5 // Return value is implicit
    }
    ```
- Comments: `//`. There are no block comments in Rust.
- `if` and `else`-expressions have the following shape:
    ```rust
    if some_condition {
        // handle some_condition
    } else if some_other_condition {
        // handle some_other_condition
    } else {
        // handle other cases
    }
- Inline `if`/`else` expressions are also possible:
    ```rust
    // Note: arms should return the same data-type
    let some_str = if some_no > 5 { "high" } else { "low" };
    ```
- Besides `loop`, Rust has `while` and `for` keywords to create repeating code blocks
- `loop`-statements are able to return a value using the `break`-keyword + return-value.
- `for`-loops can be compared with C#'s foreach:
    ```rust
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    ```
- A `Range` can be used in combination with a `for`-loop:
    ```rust
    for number in (1..4) { } // Looping over numbers 1, 2, 3
    ```

## Chapter 4: Understanding Ownership
https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html

### Today I Learned:
- Each value in Rust has a variable that's its "owner". The relation between values and owners is strictly 1-to-1. If the owner goes out of scope, the value will be dropped.
- Besides string literals (stored on the Stack) there is the `String`-type, which is stored on the heap.
- `String`s are mutated via the `push_str`-method:
    ```rust
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    ```
- The difference between these two string-types is that string-literals are known at compile-time, whereas `String` is only known at run-time.
- Copying the contents of one variable into another, will invalidate the initial variable. For example:
    ```rust
    let s1 = String::from("some string");
    let s2 = s1; // Invalidate s1, use s2 from now on
    // This doesn't hold for integer-variables, for which the size is known at compile-time and are therefore stored on the Stack
    ```
- Use `clone()` to explicityly copy heap-data to another variable.
- Variables that store data on the heap and that are passed into a function are invalid after the function ends (by default) because it is freed at the end of the scope (= end of function). Example:
    ```rust
    fn main() {
        let s = String::from("hello");  // s comes into scope

        some_function(s); // use s in some_function() and hand over the ownership 
        // s is no longer valid here
    }
    ```
- Variables can be passed to a function without the function taking ownership of the variable by using a reference-indicator `&`. This is called "borrowing":
    ```rust
    fn main() {
        some_var = String::from("some_value");
        some_function(&some_var);
        println!("reuse some_var: {}", some_var);
    }

    fn some_function(some_var: &String) {
        // "borrow" value some_var here
        ...
    }
    ```
- References are immutable by default. They can be made mutable by using the `&mut`-keyword. Only one mutable reference per scope is allowed:
    ```rust
    let mut some_var = String::from("some_value");
    let some_var_ref = &mut some_var;

    some_var_ref.push_str("s");
    ```
- It is possible to pass a part of a string as a reference, also referred to as "string slices":
    ```rust
    let s = String::from("some__value");
    let hello = &s[0..5];
    let world = &s[6..11];
    ```
- String literals are of type `&str` (e.g. `let s = "some value" <- s is of type &str`); `&str` is an immutable reference.
- Slices also generalise to other types, such as lists.


## Chapter 5: Using Structs to Structure Related Data
https://doc.rust-lang.org/stable/book/ch05-00-structs.html

### Today I Learned:
- A `struct` defines the structure of an object. It can hold together multiple related values.
- It's possible to access fields using the dot notation (e.g. user.email)
- Fields can only be updated if the entire struct is marked mutable
- We can use "field init shorthand" to provide values to fields, e.g.:
    ```rust
    // Default assignment notation
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }
    // Field init shorthand notation
    fn build_user(email: String, username: String) -> User {
        User {
            email, // Note that we don't explicitly link the value here
            username, // Here neither
            active: true,
            sign_in_count: 1,
        }
    }
- This only works if the parameter-name matches the field name
- To copy over some fields from one object to another, use the `..` notation:
    ```rust
    let user2 = User {
        email: String::from("some@email.com"), // Set a unique e-mailadress
        ..user1 // Copy the remaining fields from user 1
    }
- If you're in doubt whether to pick a Tuple or Struct, choose the Tuple Struct: a named Tuple: 
    ```rust
    struct Color(i32, i32, i32);
    ```
- To print entire structs, one could use pretty-print-options using `{:?}` or `{:#?} instead of `{}` and by annotating structs with `#[derive(Debug)]`.
- There's a difference between methods and functions; methods are defined within the context of a struct, whereas functions are not. The first parameter of a method is always `self`. Methods are defined within an `impl`-block
- `self` can also be borrowed or referenced, just like any other parameter
- `impl`-blocks can also hold "associated functions", i.e. a function without the "self"-parameter. Calling associated functions can be done using the `::`-syntax.
- It's possible to have multiple `impl`-blocks


## Chapter 6: Enums and Pattern Matching
https://doc.rust-lang.org/stable/book/ch06-00-enums.html

- Enums can be used for enumerating several options, e.g.:
    ```rust
    enum IpAddrKind {
        V4,
        V6
    }
    ```
- It's also possible to pass parameters (any amount/kind) to enums:
    ```rust
    enum IpAddr {
        V4(String),
        V6(String)
    }
    ```
- Just like structs, use `impl` to add methods to an enum
- Rust doesn't have null values! :party:
- 
TODO: continue @ https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html (at the end of the section)