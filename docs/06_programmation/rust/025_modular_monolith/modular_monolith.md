---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust"
description: "An 8-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates"
parent: "Rust"
nav_order: 34
date:               2026-01-29 15:00:00
last_modified_date: 2026-01-29 16:30:00
---


# Learning Modular Monolith Architecture with Rust
{: .no_toc }

An 8-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* For beginners

All the [examples](https://github.com/40tude/modular_monolith_tuto) are GitHub


<div align="center">
<img src="./assets/img00.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Introduction




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 00

### Objective
{: .no_toc }

We want a working prototype (POC).

### Setup
{: .no_toc }
```powershell
mkdir modular_monolith
cd modular_monolith
git init
cargo new step_00
cd step_00
code .
```

### Actions
{: .no_toc }

```powershell
# open an integrated terminal
cargo run
```

Expected output:

```powershell
cargo run
   Compiling step_00 v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_00)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_00\debug\step_00.exe`
Hello, world!
```

In the project, create an `examples/` folder. In the folder write a `ex00.rs` code which uses a function `greet()` to format the message `Hello XYZ` when "XYZ" is used as an argument.

```rust
fn main() {
    let greeting = greet("Bob");
    println!("{}", greeting);
}

fn greet(name: &str) -> String {
    format!("Hello {}.", name)
}
```

Run the application with `cargo run --example ex00`. Expected output:
```powershell
Hello Bob.
```


There is an exception in our business. If the argument is "Roberto", the application writes "Ciao Roberto!". Copy `ex00.rs` into `ex01.rs` and modify the code to take this requirement into account:

```rust
fn main() {
    let greeting = greet("Roberto");
    println!("{}", greeting);
}

fn greet(name: &str) -> String {
    // Special case for Roberto
    if name == "Roberto" {
        return "Ciao Roberto!".to_string();
    }

    format!("Hello {}.", name)
}
```

Run the application:

```powershell
cargo run -q --example ex01
Ciao Roberto!
```



There are 2 other specific cases in our business.
1. If the length of the parameter is 0, nothing is displayed and an error is returned
2. The output cannot exceed 25 chars. If the parameter is too long, the output is truncated and ends with "...".

Copy `ex01.rs` into `ex02.rs`, implement both cases and the error management


```rust
fn main() {
    match greet("Alice") {
        Ok(greeting) => println!("{}\n", greeting),
        Err(e) => eprintln!("Error: {}\n", e),
    }
}

fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    // Calculate greeting length
    let greeting_prefix = "Hello ";
    let greeting_suffix = ".";
    const MAX_LENGTH: usize = 25;
    let available_for_name = MAX_LENGTH - greeting_prefix.len() - greeting_suffix.len();

    // If name fits within limit
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate with ellipsis
    const TRAILER: &str = "...";
    let truncate_length = MAX_LENGTH - greeting_prefix.len() - TRAILER.len();
    let truncated_name = &name[..truncate_length.min(name.len())];
    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}
```

Run the application and make some experiments:

```powershell
cargo run -q --example ex02
Hello Alice.
```



Copy `ex02.rs` into `ex03.rs` and add one test:

```rust
fn main() {
    match greet("Roberto") {
        Ok(greeting) => println!("{}\n", greeting),
        Err(e) => eprintln!("Error: {}\n", e),
    }
}

// Generates a greeting according to business rules
fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    // Calculate greeting length
    let greeting_prefix = "Hello ";
    let greeting_suffix = ".";
    const MAX_LENGTH: usize = 25;
    let available_for_name = MAX_LENGTH - greeting_prefix.len() - greeting_suffix.len();

    // If name fits within limit
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate with ellipsis
    const TRAILER: &str = "...";
    let truncate_length = MAX_LENGTH - greeting_prefix.len() - TRAILER.len();
    let truncated_name = &name[..truncate_length.min(name.len())];
    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_returns_error() {
        let result = greet("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Name cannot be empty");
    }
}
```
Run the application and make some experiments:

```powershell
cargo test --example ex03
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests examples\ex03.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_00\debug\examples\ex03-259cabc647968b82.exe)

running 1 test
test tests::empty_name_returns_error ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


Copy `ex03.rs` into `ex04.rs`, add a loop in `main()` and more tests. Make sure `?` operator can be used in `main()`:

```rust
use std::io;
fn main() -> Result<(), String> {
    loop {
        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let name = input.trim();

        // Exit condition
        if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        // Skip empty input
        if name.is_empty() {
            continue;
        }

        match greet(name) {
            Ok(greeting) => println!("{}\n", greeting),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }

    Ok(())
}

/// Generates a greeting according to business rules
fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    // Calculate greeting length
    let greeting_prefix = "Hello ";
    let greeting_suffix = ".";
    const MAX_LENGTH: usize = 25;
    let available_for_name = MAX_LENGTH - greeting_prefix.len() - greeting_suffix.len();

    // If name fits within limit
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate with ellipsis
    const TRAILER: &str = "...";
    let truncate_length = MAX_LENGTH - greeting_prefix.len() - TRAILER.len();
    let truncated_name = &name[..truncate_length.min(name.len())];
    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}
```

Expected output (exit with CTRL+C):

```powershell
cargo run --example ex04
   Compiling step_00 v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_00)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_00\debug\examples\ex04.exe`
sdf
Hello sdf.
```

Here are the tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    const MAX_LENGTH: usize = 25;
    const TRAILER: &str = "...";

    #[test]
    fn empty_name_returns_error() {
        let result = greet("");
        assert!(result.is_err());
        // assert_eq!(result.unwrap_err(), "Name cannot be empty");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Name cannot be empty");
    }

    #[test]
    fn normal_greeting() {
        let result = greet("Alice");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello Alice.");
    }

    #[test]
    fn roberto_special_case() {
        let result = greet("Roberto");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Ciao Roberto!");
    }

    #[test]
    fn domain_should_not_use_special_greeting_for_similar_names() {
        // Case sensitive - "roberto" should get normal greeting
        let result = greet("roberto");
        assert_eq!(result.unwrap(), "Hello roberto.");

        // Different name
        let result = greet("Robert");
        assert_eq!(result.unwrap(), "Hello Robert.");
    }

    #[test]
    fn greeting_length_limit() {
        // "Hello " (6) + "." (1) = 7, so max name is 18 chars for MAX_LENGTH total
        let result = greet("ExactlyEighteenChr");
        assert!(result.is_ok());

        let greeting = result.unwrap();
        assert_eq!(greeting, "Hello ExactlyEighteenChr.");
        assert_eq!(greeting.len(), MAX_LENGTH);
    }

    #[test]
    fn truncation_for_long_names() {
        let long_name = "ThisIsAVeryLongNameThatExceedsTheLimit";
        let result = greet(long_name);
        assert!(result.is_ok());

        let greeting = result.unwrap();
        assert!(greeting.starts_with("Hello "));
        assert!(greeting.ends_with(TRAILER));
        assert_eq!(greeting.len(), MAX_LENGTH);
    }

    #[test]
    fn boundary_case_nineteen_chars() {
        // 19 chars should trigger truncation (6 + 19 + 1 = 26, exceeds MAX_LENGTH)
        let name = "NineteenCharactersX";
        let result = greet(name);
        assert!(result.is_ok());

        let greeting = result.unwrap();
        assert!(greeting.ends_with(TRAILER));
        assert_eq!(greeting.len(), MAX_LENGTH);
    }

    #[test]
    fn domain_should_handle_unicode_names() {
        let result = greet("José");
        assert_eq!(result.unwrap(), "Hello José.");

        let result = greet("François");
        assert_eq!(result.unwrap(), "Hello François.");
    }

    #[test]
    fn domain_should_truncate_long_unicode_names() {
        // **Pay attention to:** Unicode characters may have different byte lengths
        let long_unicode_name = "Müller-Öffentlicher-Straßenbahn-Überführung";
        let result = greet(long_unicode_name);

        assert!(result.is_ok());
        let greeting = result.unwrap();
        assert_eq!(greeting.len(), MAX_LENGTH);
        assert!(greeting.ends_with(TRAILER));
    }
}
```

Expected output:

```powershell
 cargo test --example ex04
   Compiling step_00 v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_00)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running unittests examples\ex04.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_00\debug\examples\ex04-b27bcdfa5005af06.exe)

running 9 tests
test tests::boundary_case_nineteen_chars ... ok
test tests::domain_should_handle_unicode_names ... ok
test tests::domain_should_not_use_special_greeting_for_similar_names ... ok
test tests::empty_name_returns_error ... ok
test tests::roberto_special_case ... ok
test tests::truncation_for_long_names ... ok
test tests::domain_should_truncate_long_unicode_names ... ok
test tests::normal_greeting ... ok
test tests::greeting_length_limit ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


Copy `ex04.rs` into `ex05.rs` improve the `main()` function so that CTRL+C can be avoided:

```rust
use std::io::{self, Write};
fn main() -> Result<(), String> {
    println!("=== Greeting Service (Step 00) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    loop {
        // Prompt for input
        print!("> ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let name = input.trim();

        // Exit condition
        if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        // Skip empty input
        if name.is_empty() {
            continue;
        }

        match greet(name) {
            Ok(greeting) => println!("{}\n", greeting),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }

    Ok(())
}
// The rest of the code in unchanged
```

Expected output:

```powershell
cargo run --example ex05
=== Greeting Service (Step 00) ===
Enter a name to greet (or 'quit' to exit):

> ert
Hello ert.

> quit

Goodbye!
```

On this excellent [Web site](https://www.40tude.fr/docs/06_programmation/rust/), read again this page about [errors]({%link docs/06_programmation/rust/016_errors/errors_06.md%}). Then copy `ex05.rs` into `ex06.rs` and modify the code in consequence:

```rust
use std::io::{self, Write};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 00) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    loop {
        // Prompt for input
        print!("> ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let name = input.trim();

        // Exit condition
        if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        // Skip empty input
        if name.is_empty() {
            continue;
        }

        // Call domain logic
        match greet(name) {
            Ok(greeting) => println!("{}\n", greeting),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }

    Ok(())
}

// Generates a greeting according to business rules
fn greet(name: &str) -> Result<String> {
 ...
}

// The rest of the code in unchanged

```

The POC is done! Copy `ex06.rs` into `ex07.rs`, review the code once again, add comments, run the tests and take a break.

```rust
//! Greeting service example demonstrating error handling,
//! business rules, and basic I/O interaction.
//!
//! Run with:
//! - cargo run --example ex06
//! - cargo test --example ex06

use std::io::{self, Write};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Application entry point.
///
/// Runs an interactive loop that asks the user for a name,
/// applies greeting rules, and handles errors gracefully.
fn main() -> Result<()> {
    println!("=== Greeting Service (Step 00) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    loop {
        // Prompt for input
        print!("> ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let name = input.trim();

        // Exit condition
        if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        // Skip empty input
        if name.is_empty() {
            continue;
        }

        // Call domain logic
        match greet(name) {
            Ok(greeting) => println!("{}\n", greeting),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }

    Ok(())
}

/// Generates a greeting according to business rules.
///
/// Rules:
/// - Default: "Hello {name}." with a maximum of 25 characters total
/// - Special case: "Roberto" returns "Ciao Roberto!"
/// - If the name is too long, it is truncated and suffixed with "..."
///
/// # Errors
/// Returns an error if the name is empty.
fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string().into());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    const MAX_LENGTH: usize = 25;
    const GREETING_PREFIX: &str = "Hello ";
    const GREETING_SUFFIX: &str = ".";
    const TRAILER: &str = "...";

    let available_for_name = MAX_LENGTH - GREETING_PREFIX.len() - GREETING_SUFFIX.len();

    // Name fits within the allowed length
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate and add ellipsis
    let truncate_length = MAX_LENGTH - GREETING_PREFIX.len() - TRAILER.len();

    let truncated_name = &name[..truncate_length.min(name.len())];
    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}
// The rest of the code is  unchanged
```

Expected output:

```powershell
cargo run --example ex06
   Compiling step_00 v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_00)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_00\debug\examples\ex06.exe`
=== Greeting Service (Step 00) ===
Enter a name to greet (or 'quit' to exit):

> Zorro
Hello Zorro.

> exit

Goodbye!
```


{: .new-title }
> Summary
>
* We have a working proof of concept
* Business rule (say "Hello") is applied
* Exceptions are managed ("Roberto", empty parameter)
* Errors are returned
* Tests are written




























<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 01

### Objective
{: .no_toc }

We want to split the last version of our POC into a `main.rs`, a `lib.rs` and a `domain.rs` files.

### Setup
{: .no_toc }
* Save your work
* Quit VSCode
* You should have a terminal open and you should be in `step_00/` folder.

```powershell
cd ..
# make a copy the folder step_00 and name it step_01
Copy-Item ./step_00 ./step_01 -Recurse
cd step_01
code .
```

### Actions
{: .no_toc }

* Move `examples/ex07.rs` into `src/main.rs`
* Delete the `examples/` folder

Update `Cargo.toml`

```toml
[package]
name = "step_01"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "step_01"
path = "src/main.rs"
```


Extract from `main.rs` the `greet()` function and the tests and copy them in a new file `domain.rs`.


```rust
use crate::Result;

/// Business domain for greeting logic
/// Generates a greeting according to business rules.
/// Rules:
/// - Default: "Hello {name}." with a maximum of 25 characters total
/// - Special case: "Roberto" returns "Ciao Roberto!"
/// - If the name is too long, it is truncated and suffixed with "..."
///
/// # Errors
/// Returns an error if the name is empty.
pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string().into());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    const MAX_LENGTH: usize = 25;
    const GREETING_PREFIX: &str = "Hello ";
    const GREETING_SUFFIX: &str = ".";
    const TRAILER: &str = "...";

    let available_for_name = MAX_LENGTH - GREETING_PREFIX.len() - GREETING_SUFFIX.len();

    // Name fits within the allowed length
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate and add ellipsis
    let truncate_length = MAX_LENGTH - GREETING_PREFIX.len() - TRAILER.len();

    let truncated_name = &name[..truncate_length.min(name.len())];
    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}

#[cfg(test)]
mod tests {
    // The tests are here
}
```

**Pay attention to:**
* `use crate::Result;` statement at the top of `domain.rs`.
* `greet()` is now public.
* Since `greet()` is public we could have the tests outside of this file to make sure they behave like any other consumer.

Create a `lib.rs`

```rust
pub mod domain;

pub use domain::greet;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
```

**Pay attention to:**
* How `greet ()` is **re-exported**. This allows the functions from the `domain` module (such as `greet()`) to be used directly in the `main` crate, without having to write `domain::greet`. This may simplifies importing for crate users. They can `use crate::greet;` instead of `use crate::domain::greet;`. It is therefore a question of ease of use vs clarity for the code consumers. I'm not always a big fan of it and I will explain why later.
* `Error` and `Result` are part of the lib because they are used by `domain.rs` and `main.rs`



The remaining of the code is the `main.rs`.

```rust
use std::io::{self, Write};

use step_01::Result;
use step_01::greet;

// Application entry point.
//
// Runs an interactive loop that asks the user for a name,
// applies greeting rules, and handles errors gracefully.
fn main() -> Result<()> {
    println!("=== Greeting Service (Step 01) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    loop {
        // Prompt for input
        print!("> ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        let name = input.trim();

        // Exit condition
        if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        // Skip empty input
        if name.is_empty() {
            continue;
        }

        // Call domain logic
        match greet(name) {
            Ok(greeting) => println!("{}\n", greeting),
            Err(e) => eprintln!("Error: {}\n", e),
        }
    }

    Ok(())
}
```
**Pay attention to:**
* How `Result` and `greet` are shortcutted with the `use` statements.

Run and test the example.

```powershell
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_01\debug\step_01.exe`
=== Greeting Service (Step 01) ===
Enter a name to greet (or 'quit' to exit):

> quit

Goodbye!

```

```powershell
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src\lib.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_01\debug\deps\step_01-83edb83cf52bf88d.exe)

running 9 tests
test domain::tests::domain_should_handle_unicode_names ... ok
test domain::tests::domain_should_not_use_special_greeting_for_similar_names ... ok
test domain::tests::test_truncation_for_long_names ... ok
test domain::tests::test_normal_greeting ... ok
test domain::tests::domain_should_truncate_long_unicode_names ... ok
test domain::tests::test_boundary_case_nineteen_chars ... ok
test domain::tests::test_greeting_length_limit ... ok
test domain::tests::test_empty_name_returns_error ... ok
test domain::tests::test_roberto_special_case ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_01\debug\deps\step_01-a5fcc988c1d0de71.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests step_01

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```




{: .new-title }
> Summary
>
* Nothing change from the outside
* We have a more modular project however
















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 02

### Objective
{: .no_toc }

We want to create a `tests/` folder to host the integration tests and the domain tests (since `greet()` is public). See :

```text
step_02/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point + console I/O
│   ├── domain.rs        # Business rules (isolated)
│   └── lib.rs           # Library re-exports
└── tests/
    ├── integration_test.rs  # Integration tests
    └── domain_test.rs       # Domain unit tests
```


### Setup
{: .no_toc }

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in `step_01/` folder

```powershell
cd ..
# make a copy the folder step_01 and name it step_02
Copy-Item ./step_01 ./step_02 -Recurse
cd step_02
code .
```


### Actions
{: .no_toc }

Update `Cargo.toml`

```toml
[package]
name = "step_02"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "step_02"
path = "src/main.rs
```

Optional. I no longer re-export `greet()` from `domain`. I want to have to write `domain::greet()`. This will help me to read the code in 6 months. In `main.rs` I write `use step_02::domain;` then I call `domain::greet(name)`.

```rust
/// Step 02: Extracted Domain
///
/// This step demonstrates the separation of business logic from infrastructure.
/// The domain module contains pure business rules that are independent of I/O.
pub mod domain;

// DO NOT re-export greet() for convenience
// I want to write domain::greet() in main.rs
// pub use domain::greet;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
```




Create a `tests/` folder with `domain_test.rs` and `integration_test.rs`. Move the tests from `domain.rs` to `domain_test.rs`.

```rust
/// Unit tests specifically for domain logic
/// These tests verify business rules in isolation
use step_02::domain::greet;

const MAX_LENGTH: usize = 25;
// const GREETING_PREFIX: &str = "Hello ";
// const GREETING_SUFFIX: &str = ".";
const TRAILER: &str = "...";

#[test]
fn empty_name_returns_error() {
    let result = greet("");
    assert!(result.is_err());
    // assert_eq!(result.unwrap_err(), "Name cannot be empty");
    let err = result.unwrap_err();
    assert_eq!(err.to_string(), "Name cannot be empty");
}

// the others tests

```
Note
* We are testing `domain`. So at the top of the file there is `use step_02::domain::greet;` and we call `greet()` in the rest of the code.



Write the `integration_test.rs` file

```rust
use step_02::domain;

const MAX_LENGTH: usize = 25;
// const GREETING_PREFIX: &str = "Hello ";
// const GREETING_SUFFIX: &str = ".";
const TRAILER: &str = "...";

#[test]
fn greet_integration() {
    // Test normal greeting
    let result = domain::greet("World");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World.");
}

#[test]
fn roberto_integration() {
    // Test special case
    let result = domain::greet("Roberto");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Ciao Roberto!");
}

#[test]
fn empty_name_integration() {
    // Test error case
    let result = domain::greet("");
    assert!(result.is_err());
}

#[test]
fn long_name_integration() {
    // Test truncation
    let result = domain::greet("VeryLongNameThatWillBeTruncated");
    assert!(result.is_ok());

    let greeting = result.unwrap();
    assert_eq!(greeting.len(), MAX_LENGTH);
    assert!(greeting.ends_with(TRAILER));
}
```
Note
* At this point `domain_test.rs` and `integration_test.rs` look very similar. This is because we only have one component (`domain.rs`)
* At the top of the file there is `use step_02::domain;` and we call `domain::greet())` in the rest of the code.



Run and test

```powershell
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_02\debug\step_02.exe`
=== Greeting Service (Step 01) ===
Enter a name to greet (or 'quit' to exit):

> Buck
Hello Buck.

> Quit

Goodbye!

```





```powershell
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src\lib.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_02\debug\deps\step_02-21333d23e061cf65.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_02\debug\deps\step_02-573b6b6ff085e89c.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\domain_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_02\debug\deps\domain_test-3722a907fedce176.exe)

running 9 tests
test boundary_case_nineteen_chars ... ok
test domain_should_not_use_special_greeting_for_similar_names ... ok
test domain_should_handle_unicode_names ... ok
test empty_name_returns_error ... ok
test domain_should_truncate_long_unicode_names ... ok
test normal_greeting ... ok
test roberto_special_case ... ok
test truncation_for_long_names ... ok
test greeting_length_limit ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\integration_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_02\debug\deps\integration_test-c2e846bff70f532b.exe)

running 4 tests
test empty_name_integration ... ok
test long_name_integration ... ok
test roberto_integration ... ok
test greet_integration ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests step_02

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```






{: .new-title }
> Summary
>
* Nothing change from the outside (which is good)
* `domains.rs` is shorter
* The tests are at the right place
* We now have integration tests
* So far, so good...
















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 03

### Objective
{: .no_toc }

We want ...

### Setup
{: .no_toc }

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in `step_02/` folder

```powershell
cd ..
# make a copy the folder step_02 and name it step_03
Copy-Item ./step_02 ./step_03 -Recurse
cd step_03
code .
```


### Actions
{: .no_toc }





{: .new-title }
> Summary
>
* Blablabla
    * **Blablabla:** ...
    * **Blablabla:** ...
* ...


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 04


### Objective
{: .no_toc }

We want ...

### Setup
{: .no_toc }

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in `step_03/` folder

```powershell
cd ..
# make a copy the folder step_03 and name it step_04
Copy-Item ./step_03 ./step_04 -Recurse
cd step_04
code .
```


### Actions
{: .no_toc }






{: .new-title }
> Summary
>
* Blablabla
    * **Blablabla:** ...
    * **Blablabla:** ...
* ...

<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 05


### Objective
{: .no_toc }

We want ...

### Setup
{: .no_toc }

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in the `step_04/` folder

```powershell
cd ..
# make a copy the folder step_04 and name it step_05
Copy-Item ./step_04 ./step_05 -Recurse
cd step_05
code .
```


### Actions
{: .no_toc }






{: .new-title }
> Summary
>
* Blablabla
    * **Blablabla:** ...
    * **Blablabla:** ...
* ...














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 06
{: .no_toc }


### Objective
{: .no_toc }

We want ...

### Setup
{: .no_toc }

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in the `step_05/` folder

```powershell
cd ..
# make a copy the folder step_05 and name it step_06
Copy-Item ./step_05 ./step_06 -Recurse
cd step_06
code .
```


### Actions
{: .no_toc }






{: .new-title }
> Summary
>
* Blablabla
    * **Blablabla:** ...
    * **Blablabla:** ...
* ...


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 07


### Objective
{: .no_toc }

We want ...

### Setup
{: .no_toc }

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in the `step_06/` folder

```powershell
cd ..
# make a copy the folder step_06 and name it step_07
Copy-Item ./step_06 ./step_07 -Recurse
cd step_07
code .
```


### Actions
{: .no_toc }






{: .new-title }
> Summary
>
* Blablabla
    * **Blablabla:** ...
    * **Blablabla:** ...
* ...










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography
* link_00


<!-- <div align="center">
<img src="./assets/img99.webp" alt="" width="900" loading="lazy"/><br/>
<span></span>
</div> -->
