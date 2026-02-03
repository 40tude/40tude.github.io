---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 05"
description: "An 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates"
parent: "Rust"
nav_order: 34
date:               2026-01-29 15:00:00
last_modified_date: 2026-02-03 08:00:00
---


# Learning Modular Monolith Architecture with Rust
{: .no_toc }

An 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>





















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 05
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






<div align="center">
<img src="./assets/img05.webp" alt="" width="450" loading="lazy"/><br/>
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

We want to include `anyhow` and `thiserror` crates. At the end of this episode, the folder hierarchy should look like this:


```text
step_05
│   Cargo.toml
└───crates
    ├───adapter_console
    │   │   Cargo.toml
    │   ├───src
    │   │       input.rs
    │   │       lib.rs
    │   │       output.rs
    │   └───tests
    │           adapter_console_test.rs
    ├───app
    │   │   Cargo.toml
    │   └───src
    │           main.rs
    ├───application
    │   │   Cargo.toml
    │   ├───src
    │   │       error.rs
    │   │       greeting_service.rs
    │   │       lib.rs
    │   └───tests
    │           application_test.rs
    ├───domain
    │   │   Cargo.toml
    │   ├───src
    │   │       error.rs
    │   │       greeting.rs
    │   │       lib.rs
    │   │       ports.rs
    │   └───tests
    │           domain_test.rs
    └───integration_tests
        │   Cargo.toml
        ├───src
        │       lib.rs
        └───tests
                integration_test.rs
```






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Setup

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in the `step_04/` folder

```powershell
cd ..
# make a copy the folder step_05 and name it step_06
Copy-Item ./step_04 ./step_05 -Recurse
cd step_05
code .
```











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Actions
{: .no_toc }

<!-- ###################################################################### -->
### Cargo.toml


```toml
[workspace]
members = [
    "crates/domain",
    "crates/application",
    "crates/adapter_console",
    "crates/app",
    "crates/integration_tests",
]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"

[workspace.dependencies]
thiserror = "2.0"
anyhow = "1.0"
```



**Points of attention:**
* Obviously, `thiserror` and `anyhow` are listed





<!-- ###################################################################### -->
### The domain crate
<!-- {: .no_toc } -->


Here is `Cargo.toml`:

```toml
[package]
name = "domain"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
thiserror.workspace = true
```

**Points of attention:**
* `thiserror` is added







Let's first update `port.rs`.

```rust
pub type PortError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, PortError>;

pub trait NameReader {
    fn read_name(&self) -> Result<String>;
}

pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<()>;
}
```

**Points of attention:**
* The signatures of both traits are unchanged.
* **Pay attention**. Indeed in the traits we need to indicate that `read_name` and `write_greeting` may return errors due to the port, **NOT** to the business logic. Think about the case where you will send a name via RS-232, UDP, carrier pigeon... This is why in the port module we type alias `Result<T>` as `Result<T, PortError>` and we type alias `PortError` as a `Box<dyn std::error::Error>`.
    * Why do we need `Box <dyn>`? Because there are multiple possible source or error that we don't know yet at compile time: I/O, Network...
    * Why do we need `Send + Sync`? For thread safety. Not useful here but we never know.
* The point to keep in mind is:
    * The business logic may return an error if the parameter is empty
    * `Ports` are part of the domain. They may report specific I/O error.



This is what is shown in `lib.rs` where `Error`, `Result` and `PortError` are exported:

```rust
pub mod error;
pub mod greeting;
pub mod ports;

pub use error::{Error, Result};
pub use greeting::greet;
pub use ports::{GreetingWriter, NameReader, PortError};
```




Now we can look at `error.rs`

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Name cannot be empty")]
    EmptyName,
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Points of attention:**
* The type alias `Result` remains unchanged
* The definition of `Error` is totally different but this does not have any impact on `Result` type alias



Only the beginning of `greeting.rs` is updated because now it returns a specific error when the `name` is empty:

```rust
use crate::error::{Error, Result};

pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err(Error::EmptyName);
    }
// Rest of the code unmodified
}
```










































<!-- ###################################################################### -->
### The app crate
<!-- {: .no_toc } -->

```toml
[package]
name = "app"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "step_05"
path = "src/main.rs"

[dependencies]
application = { path = "../application" }
adapter_console = { path = "../adapter_console" }
anyhow.workspace = true
```

**Points of attention:**
* `anyhow` is in the dependencies


The `error.rs` file is deleted, only remains `main.rs`:


```rust
use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 05 - Modular Monolith & Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Dependency injection: Create adapters
    let input = ConsoleInput::new();
    let output = ConsoleOutput::new();

    // Create application service and run
    let service = GreetingService::new();
    service
        .run_greeting_loop(&input, &output)
        .context("Failed to run interactive loop")?;

    Ok(())
}
```

**Points of attention:**
* Check the `.context()` that `anyhow` provides.



















<!-- ###################################################################### -->
### The application crate
<!-- {: .no_toc } -->

```toml
[package]
name = "application"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
thiserror.workspace = true
```

**Points of attention:**
* `thiserror` is added



As before (see the `error.rs` file of the `domain` crate), in the error.rs file only Error is updated:

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Domain(#[from] domain::Error),

    #[error(transparent)]
    Adapter(Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = std::result::Result<T, Error>;
```


```rust
use crate::error::{Error, Result};

pub struct GreetingService;

impl GreetingService {
    pub fn new() -> Self {
        Self
    }

    pub fn run_greeting_loop(
        &self,
        input: &dyn domain::NameReader,
        output: &dyn domain::GreetingWriter,
    ) -> Result<()> {
        loop {
            let name = input.read_name().map_err(Error::Adapter)?;

            if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
                println!("\nGoodbye!");
                break;
            }

            if name.is_empty() {
                continue;
            }

            let greeting = domain::greet(&name)?;
            output.write_greeting(&greeting).map_err(Error::Adapter)?;

            println!();
        }

        Ok(())
    }
}

impl Default for GreetingService {
    fn default() -> Self {
        Self::new()
    }
}
```

**Points of attention:**
* Have you seen the `.map_err(Error::Adapter)`
* This is the line `let greeting = domain::greet(&name)?;` which force us to have `Domain(#[from] domain::Error)` in the `error.rs` file
















<!-- ###################################################################### -->
### The adapter_console crate
<!-- {: .no_toc } -->

```toml
[package]
name = "adapter_console"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
thiserror.workspace = true
```

**Points of attention:**

thiserror is added



```rust
// errors;rs
/// Errors produced by console I/O adapters.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred during console interaction.
    #[error("console I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// A domain rule was violated.
    #[error(transparent)]
    Domain(#[from] domain::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
```


**Points of attention:**

* `Result` remains unchanged
* `Error` is now a custom type
* See the `#[error(transparent)]`



























<!-- ###################################################################### -->
### Build, run & test



Build, run and test the application. Try this:

```powershell
cargo test -p adapter_console
cargo test -p adapter_console --test adapter_console_test
cargo test -p adapter_console --test adapter_console_test console # any test containing "console"

cargo test -p application
cargo test -p domain --test domain_test
cargo test -p integration_tests

cargo run -p app
cargo run
```


Build, run and test the application. Find below the expected output:


```powershell
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\step_05.exe`
=== Greeting Service (Step 05 - Modular Monolith & Hexagonal Architecture) ===
Enter a name to greet (or 'quit' to exit):

> Marcel
Hello Marcel.

> exit

Goodbye!
```


```powershell
cargo test -p domain
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src\lib.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\deps\domain-812d4dc27a84c7ce.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\domain_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\deps\domain_test-11a40ea11f17113d.exe)

running 9 tests
test domain_should_handle_unicode_names ... ok
test domain_should_truncate_long_unicode_names ... ok
test normal_greeting ... ok
test domain_should_not_use_special_greeting_for_similar_names ... ok
test boundary_case_nineteen_chars ... ok
test empty_name_returns_error ... ok
test greeting_length_limit ... ok
test roberto_special_case ... ok
test truncation_for_long_names ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests domain

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
* `anyhow` and `thiserror` are now integrated
* the impact of the transition is minimal thanks to the way `Result` and `Error` were initially defined













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
