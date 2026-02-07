---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 02"
description: "A 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates"
parent: "Rust"
nav_order: 34
date:               2026-01-29 15:00:00
last_modified_date: 2026-02-03 08:00:00
---


# Learning Modular Monolith Architecture with Rust
{: .no_toc }

A 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates
{: .lead }



<!--
<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>
 -->




















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 02
{: .no_toc }

All the [examples](https://github.com/40tude/modular_monolith_tuto) are on GitHub


#### The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate per component
* [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 05 - Anyhow & ThisError
* [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 06 - Add new adapters + Conclusion
* [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Bonus






<div align="center">
<img src="./assets/img02.webp" alt="" width="450" loading="lazy"/><br/>
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

We want to create a `tests/` folder to host the integration tests and the domain tests (since `greet()` is public). At the end, the project folder will look like this:


```text
step_02/
│   Cargo.toml
├───src
│       domain.rs           # Business rules (isolated)
│       error.rs            # Error and Result type alias
│       lib.rs              # Library re-exports
│       main.rs             # Entry point + console I/O
└───tests
        domain_test.rs      # Domain unit tests
        integration_test.rs # Integration tests

```







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Setup

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in the `step_01/` folder

```powershell
cd ..
# make a copy the folder step_01 and name it step_02
Copy-Item ./step_01 ./step_02 -Recurse
cd step_02
code .
```















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Actions

<!-- ###################################################################### -->
### Cargo.toml


```toml
[package]
name = "step_02"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "step_02"
path = "src/main.rs
```





<!-- ###################################################################### -->
### lib.rs



```rust
pub mod domain;
pub mod error;

// DO NOT re-export greet() for convenience
// I want to write domain::greet() in main.rs
// pub use domain::greet;
```

**Points of attention:**
* I no longer re-export `greet()` from the `domain` module. I want to have to write `domain::greet()`. This will help me to **read the code** in 6 months. In `main.rs` I write `use step_02::domain;` then I call `domain::greet(name)`.












<!-- ###################################################################### -->
### domain_test.rs



Create a `tests/` folder with `domain_test.rs` and `integration_test.rs`. Move the tests from `domain.rs` to `domain_test.rs`.

```rust
use step_02::domain::greet;

const MAX_LENGTH: usize = 25;
const TRAILER: &str = "...";

#[test]
fn empty_name_returns_error() {
    let result = greet("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.to_string(), "Name cannot be empty");
}

// the others tests

```
**Points of attention:**
* We are testing `domain`. So at the top of the file there is `use step_02::domain::greet;` and we call `greet()` in the rest of the code.
* This is Ok for me because no confusion is possible.








<!-- ###################################################################### -->
### integration_test.rs



Write the `integration_test.rs` file

```rust
use step_02::domain;

const MAX_LENGTH: usize = 25;
const TRAILER: &str = "...";

#[test]
fn greet_integration() {
    let result = domain::greet("World");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World.");
}

#[test]
fn roberto_integration() {
    let result = domain::greet("Roberto");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Ciao Roberto!");
}

#[test]
fn empty_name_integration() {
    let result = domain::greet("");
    assert!(result.is_err());
}

#[test]
fn long_name_integration() {
    let result = domain::greet("VeryLongNameThatWillBeTruncated");
    assert!(result.is_ok());

    let greeting = result.unwrap();
    assert_eq!(greeting.len(), MAX_LENGTH);
    assert!(greeting.ends_with(TRAILER));
}
```


**Points of attention:**
* At this point `domain_test.rs` and `integration_test.rs` look very similar. This is because our project have only one component (`domain.rs`).
* Later, at the top of the `integration_test.rs` we will have multiple `use step_NN::component;` lines.
* At the top of the file there is a `use step_02::domain;` and in the code we call `domain::greet())`. This is Ok for me because some confusion may happen when more modules are `use`.

















<!-- ###################################################################### -->
### Build, run & test


Build, run and test the application. Find below the expected output:

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














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Summary

{: .new-title }
> What have we done so far?
>
* Nothing change from the outside (which is good)
* `domains.rs` is shorter
* The tests are at the right place
* We now have a set of integration tests
* So far, so good...





























<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Next Steps

* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate per component
* [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 05 - Anyhow & ThisError
* [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 06 - Add new adapters + Conclusion
* [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Bonus
