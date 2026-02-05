---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 05"
description: "An 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates"
parent: "Rust"
nav_order: 34
date:               2026-01-29 15:00:00
last_modified_date: 2026-02-05 13:00:00
---


# Learning Modular Monolith Architecture with Rust
{: .no_toc }

An 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates
{: .lead }




<!-- <h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2> -->





















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
step_05/
│   Cargo.toml
└───crates
    ├───adapter_console
    │   │   Cargo.toml
    │   ├───src
    │   │       errors.rs
    │   │       input.rs
    │   │       lib.rs
    │   │       output.rs
    │   └───tests
    │           adapter_console_test.rs
    ├───app
    │   │   Cargo.toml
    │   └───src
    │           main.rs
    │           main_downcast.bak
    ├───application
    │   │   Cargo.toml
    │   ├───src
    │   │       errors.rs
    │   │       greeting_service.rs
    │   │       lib.rs
    │   └───tests
    │           application_test.rs
    ├───domain
    │   │   Cargo.toml
    │   ├───src
    │   │       errors.rs
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
# make a copy the folder step_04 and name it step_05
Copy-Item ./step_04 ./step_05 -Recurse
cd step_05
code .
```

* If you have **ANY** doubt about `anyhow` or `thiserror` before you move forward, read this [dedicated page]({%link docs/06_programmation/rust/016_errors/errors_04.md%}).










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





Let's first look at `error.rs`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Name cannot be empty")]
    EmptyName,
}

pub type Result<T> = std::result::Result<T, DomainError>;
```

**Points of attention:**
* Everything here is domain specific
* With the help of `thiserror`, we define a domain error (see `DomainError`) enum. So far it only have one variant (see `EmptyName`).
* The type alias `Result` is updated from `std::result::Result<T, Error>` to `std::result::Result<T, DomainError>`







Now we can update `port.rs`.

```rust
use crate::errors::DomainError;
use std::any::Any;

pub trait InfraError: std::error::Error + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub enum NameReaderError {
    Domain(DomainError),
    Infrastructure(Box<dyn InfraError>),
}

impl std::fmt::Display for NameReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Domain(e) => write!(f, "Domain error: {e}"),
            Self::Infrastructure(e) => write!(f, "Infrastructure error: {e}"),
        }
    }
}

impl std::error::Error for NameReaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain(e) => Some(e),
            Self::Infrastructure(e) => Some(e.as_ref()),
        }
    }
}

impl From<DomainError> for NameReaderError {
    fn from(e: DomainError) -> Self {
        Self::Domain(e)
    }
}

pub trait NameReader {
    fn read_name(&self) -> Result<String, NameReaderError>;
}

pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>>;
}
```


**Points of attention:**

* **Important**. When an adapter implement a trait from the port we need to make sure it can report not only the domain errors but also the errors from the infrastructure (file missing, network error...). However concerning the latter, we don't know and we cannot know them all at compile time (today the output is on a printer, tomorrow "Hello" will be written on screen). This is why a trait for infrastructure errors (see `InfraError`) is defined.
* In fact, `InfraError` is a port, not a domain error. It is a contract that the `domain` imposes on `adapters`: "if you want to report an infra error to me, implement this feature."
* However we need a way to combine both kind of errors (domain and infra) into one and this is where `NameReaderError` enters in action.
* `NameReaderError` is an enum which express the fact that it can be either a domain error or an infrastructure error.
* Right after we found the mandatory implementation to make `NameReaderError` an error "compatible" with the others errors of the type system.
* Finally, the new signature of `read_name` explains that it can report `NameReaderError`.
* `write_greeting` changes from `fn write_greeting(&self, greeting: &str) -> Result<()>` to `fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>>`. This express the fact that it will report infrastructure kind of errors (if any)





In `greeting.rs` only the beginning of the code is updated because now, it can return a domain specific error when the `name` is empty:

```rust
use crate::errors::{DomainError, Result};

pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err(DomainError::EmptyName);
    }
// Rest of the code unmodified
}
```






Finally here is `lib.rs`:

```rust
pub mod errors;
pub mod greeting;
pub mod ports;

// Re-export
pub use greeting::greet;
pub use ports::{GreetingWriter, InfraError, NameReader, NameReaderError};
```

**Points of attention:**
* The reason why `InfraError` and `NameReaderError` are re-exported is to "hide" the name "port" so that in the input module of the adapter_console crate we will write `use domain::{NameReader, NameReaderError}` instead of `use domain::{NameReader, ports::NameReaderError};`





















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
* `thiserror` is added


The `errors.rs` now looks like:

```rust
use domain::InfraError;
use std::any::Any;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("Console output error: {0}")]
    Output(String),

    #[error("Console I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl InfraError for ConsoleError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

**Points of attention:**
* Using `thiserror` we first express what is a `ConsoleError`
* Thanks to the re-export in the domain's lib we write `use domain::InfraError;` and NOT `use domain::ports::InfraError;` and the implementation details (the ports module) remains hidden.
* Second we implement `InfraError` for `ConsoleError`



In the `input.rs` file (`output.rs` respectively) the main change is in the `Result` returned by `read_name()` (`write_greeting()`).

In `output.rs` the changes are minimal, we have:

```rust
use domain::GreetingWriter;
use domain::InfraError;

pub struct ConsoleOutput;

impl ConsoleOutput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl GreetingWriter for ConsoleOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        println!("{greeting}");
        Ok(())
    }
}
```

**Points of attention:**
* Check the `use domain::InfraError;`
* See the `Result<(), Box<dyn InfraError>>`


In `input.rs` we now have:

```rust
use crate::errors::ConsoleError;
use domain::{NameReader, NameReaderError};
use std::io::{self, Write};

pub struct ConsoleInput;

impl ConsoleInput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleInput {
    fn default() -> Self {
        Self::new()
    }
}

impl NameReader for ConsoleInput {
    fn read_name(&self) -> Result<String, NameReaderError> {
        print!("> ");
        io::stdout()
            .flush()
            .map_err(|e| NameReaderError::Infrastructure(Box::new(ConsoleError::from(e))))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| NameReaderError::Infrastructure(Box::new(ConsoleError::from(e))))?;

        let name = input.trim().to_string();

        Ok(name)
    }
}
```

**Points of attention:**
* See the `Result<String, NameReaderError>`
* We had
    ```rust
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {e}"))?;
    ```
* Now we must write
    ```rust
    io::stdout()
        .flush()
        .map_err(|e| NameReaderError::Infrastructure(Box::new(ConsoleError::from(e))))?;
    ```
* This is complex because we must returned a `NameReaderError` which is, after the `flush()` an infrastructure kind of error. However, as requested in `port.rs`, the latter is described as `Infrastructure(Box<dyn InfraError>)` so here we  must `Box::new(ConsoleError::from(e))`.























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




The `error.rs` file has been deleted, only remains `main.rs`:


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

`lib.rs` and `greeting_service.rs` are not impacted.

Let's see how `errors.rs` helps to keep `greeting_service.rs` intact.

```rust
use domain::errors::DomainError;
use domain::{InfraError, NameReaderError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(Box<dyn InfraError>),
}

pub type Result<T> = std::result::Result<T, ApplicationError>;

impl From<Box<dyn InfraError>> for ApplicationError {
    fn from(e: Box<dyn InfraError>) -> Self {
        Self::Infrastructure(e)
    }
}

impl From<NameReaderError> for ApplicationError {
    fn from(e: NameReaderError) -> Self {
        match e {
            NameReaderError::Domain(d) => Self::Domain(d),
            NameReaderError::Infrastructure(i) => Self::Infrastructure(i),
        }
    }
}
```

**Points of attention:**
* Using `thiserror` we explain what is an error at the application level.
* Note how the Result type alias changes from `std::result::Result<T, Error>` to `std::result::Result<T, DomainError>`
* At the end we find the secret elements that help to keep `greeting_service.rs` as it is
    * The `From<Box<dyn InfraError>> for ApplicationError` is used by the `?` operator at the end of the line `output.write_greeting(&greeting)?;` when an infrastructure error need to be converted into an application error.
    * Same reasoning with the `From<NameReaderError> for ApplicationError` and the conversion at the end of the line `let name = input.read_name()?;`

















<!-- ###################################################################### -->
### The integration_tests crate

Only one change in `integration_test.rs` indeed, the line:

```rust
use domain::{GreetingWriter, NameReader, error::Result};
```

becomes:

```rust
use domain::{GreetingWriter, InfraError, NameReader, NameReaderError};
```

Then `read_name` and `write_greeting` have the signature we already explained when we talked about `input` and `output` modules of the `adapter_console` crate.
















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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\step_05.exe`
=== Greeting Service (Step 05 - Modular Monolith & Hexagonal Architecture) ===
Enter a name to greet (or 'quit' to exit):

> Marcel
Hello Marcel.

> q!

Goodbye!
```


```powershell
 cargo test -p domain
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src\lib.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\deps\domain-812d4dc27a84c7ce.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\domain_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\deps\domain_test-11a40ea11f17113d.exe)

running 9 tests
test domain_should_handle_unicode_names ... ok
test domain_should_truncate_long_unicode_names ... ok
test empty_name_returns_error ... ok
test boundary_case_nineteen_chars ... ok
test greeting_length_limit ... ok
test domain_should_not_use_special_greeting_for_similar_names ... ok
test roberto_special_case ... ok
test normal_greeting ... ok
test truncation_for_long_names ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

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
* `anyhow` and `thiserror` are now integrated.
* The need to return specific errors forced us to distinguish between `InfraError` and `DomainError` and to find a way to return either one within a single container (`NameReaderError`).
* It was an opportunity to impose an additional port (`InfraError`) to the adapters if they want to report errors from the infrastructure.
* Implementing the `From` trait for `NameReaderError` or `ApplicationError` helped to keep the `?` in the source code.
* Yes, in `input.rs` the way to write the `NameReaderError::Infrastructure` error is more complex but later, using downcasting at the end user level we will be able to find out from where exactly the error comes from.













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
