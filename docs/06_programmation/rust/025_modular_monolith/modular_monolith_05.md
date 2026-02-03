---
published: false
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 03"
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
### This is Episode 04
{: .no_toc }

All the [examples](https://github.com/40tude/modular_monolith_tuto) are on GitHub


#### The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate par component






<div align="center">
<img src="./assets/img04.webp" alt="" width="450" loading="lazy"/><br/>
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

We want to have one crate per component. At the end the folder hierarchy should look like:

```text
step_05
│   Cargo.toml
└───crates
    ├───adapter_console                  # depends on domain
    │   │   Cargo.toml
    │   ├───src
    │   │       error.rs                 # errors are crate specific
    │   │       input.rs
    │   │       lib.rs
    │   │       output.rs
    │   └───tests
    │           adapter_console_test.rs  # tests are crate specific
    ├───app                              # depends application + adapter_console
    │   │   Cargo.toml
    │   └───src
    │           error.rs                 # errors are crate specific
    │           main.rs
    ├───application                      # depends on domain
    │   │   Cargo.toml
    │   ├───src
    │   │       error.rs                 # errors are crate specific
    │   │       greeting_service.rs
    │   │       lib.rs
    │   └───tests
    │           application_test.rs      # tests are crate specific
    ├───domain                           # doesn't depend on anyone
    │   │   Cargo.toml
    │   ├───src
    │   │       error.rs                 # errors are crate specific
    │   │       greeting.rs
    │   │       lib.rs
    │   │       ports.rs
    │   └───tests
    │           domain_test.rs           # tests are crate specific
    └───integration_tests                # dedicated crate for integration testing
        │   Cargo.toml                   # depends on domain + application + adapter_console
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











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Actions

Update `Cargo.toml`

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
```

**Points of attention:**
* The root `Cargo.toml` defines a workspace with multiple independent crates
* Each component can be developped and tested independently
* The `app` crate has a `[[bin]]` section in its `Cargo.toml`, enabling `cargo run` (in addition to `cargo run -p app`)
* Integration tests have their own crate for separation of concerns and are run with `cargo test -p integration_tests`


**Points of attention:**
* Up to now it was Ok to share `Error` and `Result<T>` between the module.
* This is no longer thecase. Indeed each crate will developp its own error code/message
* This is why in most of the crates there is an `error.rs` file
* Now these `error.rs` files all look the same but tomorow, one crate may start using `thiserror` while the others keep their error schema.









<!-- ###################################################################### -->
### The app crate
{: .no_toc }

Here is `Cargo.toml`:

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
```

The `src/amin.rs` is now very short. Indeed the `run_greeting_loop()` function call in now a method that belongs to a `GreetingService` structure.

```rust
use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

mod error;
use error::Result;

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 05 - Modular Monolith & Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Dependency injection: Create adapters
    let input = ConsoleInput::new();
    let output = ConsoleOutput::new();

    // Create application service and run
    let service = GreetingService::new();
    service.run_greeting_loop(&input, &output)?;

    Ok(())
}
```

**Points of attention:**
* Above, note the `mod error;` and the `use error::Result;`



Find below the error.rs file. At this point, no matter the crate, the error.rs file are all similar:

```rust
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
```




















<!-- ###################################################################### -->
### The integration_tests crate
{: .no_toc }

Here is `Cargo.toml`:
```toml
[package]
[package]
name = "integration_tests"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
application = { path = "../application" }
adapter_console = { path = "../adapter_console" }
```


This crate is really a place holder for the tests. Indeed it does contains code which is executed at runtime. Just tests. This is why `src/lib.rs` is empty.

```rust
use application::GreetingService;
use domain::{GreetingWriter, NameReader, error::Result};

struct MockNameReader {
    names: Vec<String>,
    index: std::cell::Cell<usize>,
}

impl MockNameReader {
    fn new(names: Vec<&str>) -> Self {
        Self {
            names: names.into_iter().map(String::from).collect(),
            index: std::cell::Cell::new(0),
        }
    }
}

impl NameReader for MockNameReader {
    fn read_name(&self) -> Result<String> {
        let idx = self.index.get();
        if idx < self.names.len() {
            self.index.set(idx + 1);
            Ok(self.names[idx].clone())
        } else {
            Ok("quit".to_owned())
        }
    }
}

struct MockGreetingWriter {
    greetings: std::cell::RefCell<Vec<String>>,
}

impl MockGreetingWriter {
    fn new() -> Self {
        Self {
            greetings: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn greetings(&self) -> Vec<String> {
        self.greetings.borrow().clone()
    }
}

impl GreetingWriter for MockGreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        self.greetings.borrow_mut().push(greeting.to_owned());
        Ok(())
    }
}

#[test]
fn domain_greet_function() {
    // Arrange
    let reader = MockNameReader::new(vec!["Alice", "Bob", "quit"]);
    // let reader = MockNameReader::new(vec!["Alice", "Bob"]);
    let writer = MockGreetingWriter::new();
    let service = GreetingService::new();

    // Act
    let result = service.run_greeting_loop(&reader, &writer);

    // Assert
    assert!(result.is_ok());
    let greetings = writer.greetings();
    assert_eq!(greetings.len(), 2);
    assert!(greetings[0].contains("Alice"));
    assert!(greetings[1].contains("Bob"));
}
// Other tests follow here
```

**Points of attention:**
* Note that `error::Result` is provided by `domain`. Indeed in the implementations of the mock reader (mock writer), the function `read_name()` (`write_greeting`) return a `domain::Result<()>`. If tomorow we change the kind of error in the domain crate, with `thiserror` for example, then the `integration_tests` crate will **NOT** be impacted.

**Points of attention:**
* Why above, in `MockNameReader`, `index` is `std::cell::Cell<usize>`?
* `NameReader` trait declares `fn read_name(&self)`, a shared, immutable reference.
* Inside `read_name`, we need to increment `index` to return the next name on each call. But `&self` forbids mutating struct fields directly. If `index` is a plain `usize`, the compiler rejects `self.index += 1` because you don't have `&mut self`. Trust me I tried.
* `Cell<usize>` provides interior mutability. This allows us to modify a value behind a `&self` reference in a safe way (for Copy types like `usize`).
* The same logic applies to `greetings: RefCell<Vec<String>>` in the writer. Indeed a `Vec<String>` does not have the Copy trait, so it needs `RefCell` instead of `Cell`.

**Points of attention:**
* The test must run without any adatper. This is why we create reader and writer mockup
* Since the loop run until it read `quit` (or `exit`) the reader own a vector of words.
* In the implementation we fasten our seat belt and if we reach the end of the vector then we simulate the reading of the word `quit`
* Again, because of the loop, the mock writer have a vector of greetings which will be anlazed in the tests
* Here only on test is shown. As in main.rs we create a `reader`, a `writer`, a `GreetingService` and invoke the `.run_greeting_loop()` method
* Thanks to the mockup **we don't have to wait the availability of real adapters to test the behavior of the overall application**.









<!-- ###################################################################### -->
### The application crate
{: .no_toc }

Here is `Cargo.toml`:

```toml
[package]
[package]
name = "application"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
```

`application/src/error.rs` is the same while `application/src/lib.rs` is minimal:

```rust
pub mod error;
pub mod greeting_service;
pub use greeting_service::GreetingService;
```




The  code of `application/src/greeting_service.rs` is almost a copy paste from the `run_greeting_loop()` of `step_04`.

```rust
use crate::error::Result;
use domain;

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
            let name = input.read_name()?;

            if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
                println!("\nGoodbye!");
                break;
            }

            if name.is_empty() {
                continue;
            }

            match domain::greet(&name) {
                Ok(greeting) => {
                    output.write_greeting(&greeting)?;
                }
                Err(e) => {
                    eprintln!("Error: {}\n", e);
                }
            }
            println!(); // Extra newline for readability
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
* Make sur to cathc the line `use crate::error::Result;`
* Did you notice the `&self` as a first parameter of `.run_greeting_loop()`?








<!-- ###################################################################### -->
### The domain crate
{: .no_toc }

Here is `Cargo.toml`:
```toml
[package]
name = "domain"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
```

Here is ``domain/src/lib.rs`:

```rust
pub mod error;
pub mod greeting;
pub mod ports;

pub use greeting::greet;
pub use ports::{GreetingWriter, NameReader};
```



`error.rs` is the same. The `domain/src/ports.rs` file is back:

```rust
// ports.rs
use crate::error::Result;

pub trait NameReader {
    fn read_name(&self) -> Result<String>;
}

pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<()>;
}
```


`domain/src/greeting.rs` remains unchanged. Yes, the name of the file is changed but `greet()` signature is the same.

```rust
// greeting.rs
use crate::error::Result;

pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string().into());
    }

    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    const MAX_LENGTH: usize = 25;
    const GREETING_PREFIX: &str = "Hello ";
    const GREETING_SUFFIX: &str = ".";
    const TRAILER: &str = "...";

    let available_for_name = MAX_LENGTH - GREETING_PREFIX.len() - GREETING_SUFFIX.len();

    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    let truncate_length = MAX_LENGTH - GREETING_PREFIX.len() - TRAILER.len();
    let truncated_name = &name[..truncate_length.min(name.len())];

    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}
```

**Points of attention:**
* Make sure to understand `Result` come from the `error` module which is crate dependant.























<!-- ###################################################################### -->
### The adapter_console crate
{: .no_toc }

Here is `Cargo.toml`:

```toml
[package]
name = "adapter_console"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
```


```rust
// lib.rs
pub mod error;
pub mod input;
pub mod output;

pub use input::ConsoleInput;
pub use output::ConsoleOutput;
```



The previous `adapters/` folder is renamed `adapter_console` and the crate contains both, intput and output console adpaters modules in the same crate. Below `input.rs` and `output.rs` contain respectively a large part of `console_input.rs` dans `console.output.rs` from the previous project `step_04`.

```rust
// output.rs
use crate::error::Result;
use domain::GreetingWriter;

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
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        println!("{greeting}");
        Ok(())
    }
}
```

```rust
// input.rs
use std::io::{self, Write};

use domain::NameReader;
use crate::error::Result;

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
    fn read_name(&self) -> Result<String> {
        print!("> ");
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {e}"))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read from stdin: {e}"))?;

        let name = input.trim().to_string();

        Ok(name)
    }
}
```







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


Find below 2 examples of expected outputs:

```powershell
cargo test -p integration_tests
   Compiling application v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_05\crates\application)
   Compiling integration_tests v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_05\crates\integration_tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.86s
     Running unittests src\lib.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\deps\integration_tests-d9b20696f8963066.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\integration_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\deps\integration_test-92fdd84cf9e6dd41.exe)

running 5 tests
test complete_flow_long_name ... ok
test service_with_mocks ... ok
test domain_greet_function ... ok
test complete_flow_normal_greeting ... ok
test empty_name_error_handling ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests integration_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```



```powershell
cargo run
   Compiling app v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_05\crates\app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_05\debug\step_05.exe`
=== Greeting Service (Step 05 - Modular Monolith & Hexagonal Architecture) ===
Enter a name to greet (or 'quit' to exit):

> James HOLDEN
Hello James HOLDEN.

> quit

Goodbye!
```





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Summary
{: .new-title }
> What have we done so far?
>
* Every component is in its own crate
* Each crate has its own Result and Error
* Each crate has its set of tests
* Development and testing can be done independantly, per crate, in parallel, at different speed, by different teams...
* ...











<div align="center">
<img src="./assets/img06.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 06 - Start using `anyhow` and `thiserror` crates


### Objective
{: .no_toc }

We want to include `anyhow` and `thiserror` crates.

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

Update `Cargo.toml`

```toml
[workspace]
members = [
    "crates/shared",
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



#### **The shared crate**
{: .no_toc }

```rust
[package]
name = "shared"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
thiserror.workspace = true
```

In `crates/shared/src/lib.rs`, comment out the `Error` type alias and modify the code as below.

```rust
// pub type Error = Box<dyn std::error::Error>;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Custom error: {0}")]
    Custom(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}
```


#### **The app crate**
{: .no_toc }

```toml
[package]
name = "app"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "step_06"
path = "src/main.rs"

[dependencies]
shared = { path = "../shared" }
application = { path = "../application" }
adapter_console = { path = "../adapter_console" }
anyhow.workspace = true

```

```rust
use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

// use shared::Result;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 06 - Modular Monolith & Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    let input = ConsoleInput::new();
    let output = ConsoleOutput::new();

    // Create application service and run
    let service = GreetingService::new();
    service
        .run_greeting_loop(&input, &output)
        .context("In the loop")?;

    Ok(())
}
```


#### **The domain crate**
{: .no_toc }

Simplify the `Err()` line in `crates/domain/src/greetings.rs` from:
```rust
pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string().into());
    }
    // Rest of the code unmodified
}
```

to:

```rust
pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err("Name cannot be empty".into());
    }
    // Rest of the code unmodified
}
```

Modify the test `empty_name_returns_error()` as shown below:

```rust
#[test]
fn empty_name_returns_error() {
    let result = greet("");
    assert!(result.is_err());
    // assert_eq!(result.unwrap_err(), "Name cannot be empty");
    let err = result.unwrap_err();
    // assert_eq!(err.to_string(), "Name cannot be empty");
    assert_eq!(err.to_string(), "Custom error: Name cannot be empty");
}
```
**Points of attention:**
* Now, on error the message is "Custom error: ...".





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
   Compiling shared v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_06\crates\shared)
   Compiling domain v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_06\crates\domain)
   Compiling adapter_console v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_06\crates\adapter_console)
   Compiling application v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_06\crates\application)
   Compiling integration_tests v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_06\crates\integration_tests)
   Compiling app v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_06\crates\app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.20s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_06\debug\step_06.exe`
=== Greeting Service (Step 06 - Modular Monolith & Hexagonal Architecture) ===
Enter a name to greet (or 'quit' to exit):

> Marcel
Hello Marcel.

> quit

Goodbye!

```


```powershell
cargo test -p domain
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src\lib.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_06\debug\deps\domain-41d9db1c7e499853.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\domain_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_06\debug\deps\domain_test-ef809e7bfc49c7bf.exe)

running 9 tests
test boundary_case_nineteen_chars ... ok
test domain_should_not_use_special_greeting_for_similar_names ... ok
test domain_should_truncate_long_unicode_names ... ok
test greeting_length_limit ... ok
test empty_name_returns_error ... ok
test domain_should_handle_unicode_names ... ok
test normal_greeting ... ok
test truncation_for_long_names ... ok
test roberto_special_case ... ok

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
* the inmpact of the transition is minimal thanks to the way `Result` and `Error` were initially definde


<div align="center">
<img src="./assets/img07.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>














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

Update `Cargo.toml`


```toml
[workspace]
members = [
    "crates/domain",
    "crates/application",
    "crates/adapter_console",
    "crates/adapter_file",
    "crates/app",
]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"

# Shared dependencies across all crates
[workspace.dependencies]
thiserror = "2.0"
anyhow = "1.0"
```



**Points of attention:**
* ...


Build, run and test the application. Find below the expected output:







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Summary
{: .new-title }
> What have we done so far?
>
* Blablabla
    * **Blablabla:** ...
    * **Blablabla:** ...
* ...








<div align="center">
<img src="./assets/img08.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

<!--
* Avoid overthinking. Move forward, write code. Then study how to make your code easier to change. Otherwise, the risk is to "conceptualize the concept" and never actually add features to your application.
-->














<div align="center">
<img src="./assets/img09.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography
* link_00


<!--
Clean Architecture
Pragmatic programmer
Lien sur flashcards? https://rust-deck-befcc06ba7fa.herokuapp.com/practice
-->

<!-- <div align="center">
<img src="./assets/img99.webp" alt="" width="900" loading="lazy"/><br/>
<span></span>
</div> -->





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Next Steps
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate par component

