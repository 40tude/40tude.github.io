---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 04"
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




<!-- <h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2> -->





















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
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate per component
* [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 05 - Anyhow & ThisError
* [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 06 - Add new adapters + Conclusion
* [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}): Bonus






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

We want to have one crate per component and, among others, an application crate in charge of the use cases. At the end of Step 04 folder hierarchy should look like this:

```text
step_04
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
* You should have a terminal open and you should be in the `step_03/` folder

```powershell
cd ..
# make a copy the folder step_03 and name it step_04
Copy-Item ./step_03 ./step_04 -Recurse
cd step_04
code .
```











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Actions

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
```

**Points of attention:**
* The root `Cargo.toml` defines a workspace with multiple independent crates
* Each component can be developed and tested independently
* As we will see, the `app` crate has a `[[bin]]` section in its `Cargo.toml`, enabling `cargo run` (in addition to `cargo run -p app`)
* Integration tests have their own crate for separation of concerns and we can run them with `cargo test -p integration_tests`


**Points of attention:**
* Up to now it was Ok to share `Error` and `Result<T>` between the modules.
* This is no longer the case. Indeed each crate must be independent, and we should not be surprised if some of our crates develop their own error code/message.
* This is why in most of the crates there is an `error.rs` file
* As for now, all these `error.rs` files look the same but tomorrow, one crate may start using `thiserror` while the others may keep their error schema.









<!-- ###################################################################### -->
### The app crate
<!-- {: .no_toc } -->

Here is `Cargo.toml`:

```toml
[package]
name = "app"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "step_04"
path = "src/main.rs"

[dependencies]
application = { path = "../application" }
adapter_console = { path = "../adapter_console" }
```



The `src/main.rs` is now very short. Indeed the `run_greeting_loop()` function call in now a method that belongs to a `GreetingService` structure (hosted in the application crate).




```rust
use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

mod error;
use error::Result;

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 04 - Modular Monolith & Hexagonal Architecture) ===");
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



Find below the `error.rs` file. At this point, no matter the crate, **ALL** the `error.rs` files look the same:

```rust
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
```




















<!-- ###################################################################### -->
### The integration_tests crate
<!-- {: .no_toc } -->

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


This crate is really a place holder for the tests. Indeed it does not contain any code which is executed at runtime. Just tests. This is why `src/lib.rs` is empty.

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
* Note that `error::Result` is provided by `domain` crate. Indeed in the implementations of the mock reader (respectively the mock writer), the function `read_name()` (`write_greeting`) returns a `domain::Result<()>`. If tomorrow we change the kind of error in the `domain` crate, with `thiserror` for example, then the `integration_tests` crate will **NOT** be impacted which a good thing.

**Points of attention:**
* Why above, in `MockNameReader`, `index` is `std::cell::Cell<usize>`?
* `NameReader` trait declares `fn read_name(&self)`, a shared, immutable reference.
* Inside `read_name`, we need to increment `index` to return the next name on each call. But `&self` forbids mutating struct fields directly. If `index` is a plain `usize`, the compiler rejects `self.index += 1` because you don't have `&mut self`. Trust me, I tried.
* `Cell<usize>` provides interior mutability. This allows us to modify a value behind a `&self` reference in a safe way (for Copy types like `usize`).
* The same logic applies to `greetings: RefCell<Vec<String>>` in the writer. Indeed a `Vec<String>` does not have the Copy trait, so it needs `RefCell` instead of `Cell`.

**Points of attention:**
* The fact that `NameReader` trait declares `fn read_name(&self)`, a shared, immutable reference is **an error of design**
* We will talk about it again in [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}) and fix the problem in [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}).

**Points of attention:**
* The tests must run without any adapter. This is why we create reader and writer mockups.
* Since the loop run until it read "quit" (or "exit" or "q!") the reader own a vector of words.
* In the implementation we fasten our seat belt and if we reach the end of the vector then we simulate the reading of the word "quit".
* Again, because of the loop, the mock writer have a vector of greetings which will be analyzed later in the tests.
* Here, only one test is shown. As in `main.rs` we create a `reader`, a `writer`, a `GreetingService` and invoke the `.run_greeting_loop()` method
* Thanks to the mockups **we don't have to wait the availability of real adapters to test the behavior of the overall application**.









<!-- ###################################################################### -->
### The application crate
<!-- {: .no_toc } -->

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




The  code of `application/src/greeting_service.rs` is almost a copy paste from the `run_greeting_loop()` we ahd in `main.rs` in `step_03`.

```rust
use crate::error::Result;

#[derive(Default)]
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

            if name.eq_ignore_ascii_case("quit")
            || name.eq_ignore_ascii_case("exit")
            || name.eq_ignore_ascii_case("q!")
        {
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
```

**Points of attention:**
* Make sure to understand the line `use crate::error::Result;`
* Did you notice the `&self` as a first parameter of `.run_greeting_loop()`?








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
```

Here is `domain/src/lib.rs`:

```rust
pub mod error;
pub mod greeting;
pub mod ports;

pub use greeting::greet;
pub use ports::{GreetingWriter, NameReader};
```



`error.rs`, `domain/src/ports.rs` do not change. `domain/src/greeting.rs` remains unchanged. Yes, regarding the latter, the name of the file has changed but the signature of the `greet()` function is the same.



















<!-- ###################################################################### -->
### The adapter_console crate
<!-- {: .no_toc } -->

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

Here is `adapter_console/src/lib.rs`:

```rust
// lib.rs
pub mod error;
pub mod input;
pub mod output;

pub use input::ConsoleInput;
pub use output::ConsoleOutput;
```



The previous `adapters/` folder is renamed `adapter_console` and the crate contains both, input and output console adapters modules in the same crate. Below `input.rs` and `output.rs` contain respectively a large part of `console_input.rs` dans `console.output.rs` from the previous project `step_03`.


```rust
// output.rs
use crate::error::Result;
use domain::GreetingWriter;

#[derive(Default)]
pub struct ConsoleOutput;

impl ConsoleOutput {
    pub fn new() -> Self {
        Self
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

#[derive(Default)]
pub struct ConsoleInput;

impl ConsoleInput {
    pub fn new() -> Self {
        Self
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
   Compiling domain v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_04\crates\domain)
   Compiling adapter_console v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_04\crates\adapter_console)
   Compiling application v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_04\crates\application)
   Compiling integration_tests v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_04\crates\integration_tests)
   Compiling app v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_04\crates\app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.04s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_04\debug\step_04.exe`
=== Greeting Service (Step 04 - Modular Monolith & Hexagonal Architecture) ===
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
* Every component is now in its own crate
* Each crate has its own `Result` and `Error`
* Each crate has its own set of tests
* Development and testing can be done independently, per crate, in parallel, at different speed, by different teams...


At this point, the organization looks like:

<div align="center">
<img src="./assets/img11.webp" alt="" width="900" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

* Our core business, our domain, is in yellow. It depends on no one. It just do its job.
* The application crate run one use case (loop). It feeds the domain with some input data and get some output data in returns.
* 2 ports are imposed by the domain.
* They explain how the domain is willing to read/write data
* As an adapter if you do not conform to the ports you cannot pass/receive data to/from the domain. You're useless.
* The adapter on the left pass the data from the outside world to the input port
* The second adapter pass the inner data to the outside world through the output port
* We can have more than 6 ports (here we only have 2)
* We can have more than one adapter per port.


If you read the `Cargo.toml` files of each crate and if you pay attention to the dependencies section you should draw a graph similar to the one below:

<div align="center">
<img src="./assets/img10.webp" alt="" width="900" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

* `app` depends on `application` and `adapters`
* `application` depends on `domain`
* `adapters` depends on `domain`
* `domain` depends on nobody











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Next Steps

Next you can read [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}).

* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate per component
* [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 05 - Anyhow & ThisError
* [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 06 - Add new adapters + Conclusion
* [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}): Bonus

