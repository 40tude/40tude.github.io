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
<img src="./assets/img00.webp" alt="" width="900" loading="lazy"/><br/>
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

```powershell
mkdir modular_monolith
cd modular_monolith
git init
cargo new step_00
cd step_00
code
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

In the project, create an `examples/` folder. In the folder write `ex00.rs` which uses a function `greet()` to format the message `Hello XYZ` when "XYZ" is used as an argument.

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


There is an exception in our business. If the argument is "Roberto", the application writes "Ciao Roberto!"

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
Implement both cases and the error management


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



Add one test

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


Add a loop in `main()` and more tests. Make sure `?` operator can be used in `main()`:

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

Here are the tests
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
        // Note: Unicode characters may have different byte lengths
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


Polish the `main()` function so that CTRL+C can be avoided

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

Read again this page about [errors]({%link docs/06_programmation/rust/016_errors/errors_06.md%}) and modify the code in consequence.

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

The POC is done! Review the code, add comments, run the tests...

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
// The rest of the code in unchanged
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
* Test are written





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 01

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
## Step 02

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
## Step 03

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
