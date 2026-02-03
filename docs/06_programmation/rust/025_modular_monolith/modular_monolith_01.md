---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 01"
description: "An 8-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates"
parent: "Rust"
nav_order: 34
date:               2026-01-29 15:00:00
last_modified_date: 2026-02-03 08:00:00
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
### This is Episode 01
{: .no_toc }

All the [examples](https://github.com/40tude/modular_monolith_tuto) are GitHub


#### The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder






<div align="center">
<img src="./assets/img01.webp" alt="" width="450" loading="lazy"/><br/>
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

## Objective

We want to split the last version of the POC among multiple files. At the end the project will look like this:

```text
step_01/
│   Cargo.toml
└───src
        domain.rs
        error.rs
        lib.rs
        main.rs
```








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Setup

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in the `step_00/` folder.

```powershell
cd ..
# make a copy the folder step_00 and name it step_01
Copy-Item ./step_00 ./step_01 -Recurse
cd step_01
code .
```







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Actions
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

**Points of attention:**
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

**Points of attention:**
* See how `greet ()` is **re-exported**. This allows the functions from the `domain` module (such as `greet()`) to be used directly in the `main` crate, without having to write `domain::greet`. This may simplifies importing for crate users. They can `use crate::greet;` instead of `use crate::domain::greet;`. It is therefore a question of ease of use vs clarity for the code consumers. I'm not always a big fan of it and I will explain why later.
* `Error` and `Result` are part of the lib because they are used by `domain.rs` and `main.rs`



The remaining of the code is the `main.rs` file.

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
**Points of attention:**
* How `Result` and `greet` are shortcutted with the `use` statements.
* Make sure to understand why here, we write `use step_01::Result;` while in `domain.rs` we wrote `use crate::Result;`. If needed, you can read again this [page]({%link docs/06_programmation/rust/013_no_more_mod_rs/no_more_mod_rs.md%}).

Build, run and test the application. Find below the expected output:

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

**Points of attention:**

* We can now test the domain module in isolation
    ```powershell
    cargo test domain
    ```
* We could develop independently as long as the signature of `greet()` remains stable.


{: .new-title }
> Summary
>
* Nothing change from the outside
* We have a more modular project however















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Next Step
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
