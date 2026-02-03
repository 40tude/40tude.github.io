---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 02"
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
### This is Episode 02
{: .no_toc }

All the [examples](https://github.com/40tude/modular_monolith_tuto) are GitHub


#### The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder

<!--

* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Start Implementing Hexagonal Architecture
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 0X -
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 0X -
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 0X -
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}): Step 0X -

-->









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
## Step 02: Add a test folder

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
* You should have a terminal open and you should be in the `step_01/` folder

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

Optional. I no longer re-export `greet()` from `domain`. I want to have to write `domain::greet()`. This will help me to **read the code** in 6 months. In `main.rs` I write `use step_02::domain;` then I call `domain::greet(name)`.

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
**Points of attention:**
* We are testing `domain`. So at the top of the file there is `use step_02::domain::greet;` and we call `greet()` in the rest of the code. No confusion is possible.



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
**Points of attention:**
* At this point `domain_test.rs` and `integration_test.rs` look very similar. This is because our project have one component (`domain.rs`). Later, at the top of the `integration_test.rs` we will have multiple `use step_NN::component;` lines.
* At the top of the file there is `use step_02::domain;` and we call `domain::greet())` in the rest of the code. Later, this will help us to **read the code**.



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






{: .new-title }
> Summary
>
* Nothing change from the outside (which is good)
* `domains.rs` is shorter
* The tests are at the right place
* We now have a set of integration tests
* So far, so good...













<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 03 - Start Implementing Haxagonal Architecture

### Objective
{: .no_toc }

We want to start implementing an Hexagonal Architecture. At the end the folder hierarchy should look like:

```text
step_03
│   Cargo.toml
├───src/
│   │   domain.rs   # Business rules (isolated)
│   │   lib.rs      # Library re-exports
│   │   main.rs     # Entry point
│   │   ports.rs    # Traits definition
│   └───adapters/
│           console_input.rs  # Implementations
│           console_output.rs
│           mod.rs
└───tests/
        adapters_test.rs      # Adapters unit tests
        domain_test.rs        # Domain unit tests
        integration_test.rs   # Integration tests
```

### Setup
{: .no_toc }

* Save your work
* Quit VSCode
* You should have a terminal open and you should be in the `step_02/` folder

```powershell
cd ..
# make a copy the folder step_02 and name it step_03
Copy-Item ./step_02 ./step_03 -Recurse
cd step_03
code .
```

* If you have **ANY** doubt about Hexagonal Architecture, Ports and Adapters, read this [dedicated page]({%link docs/06_programmation/rust/024_hexagonal/hexagonal_lite.md%}).

### Actions
{: .no_toc }

Update `Cargo.toml`

```toml
[package]
name = "step_03"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "step_03"
path = "src/main.rs"
```


Create a `ports.rs` file.

```rust
use crate::Result;

pub trait NameReader {
    fn read_name(&self) -> Result<String>;
}

pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<()>;
}
```


**Points of attention:**
* The idea is to make sure that our application get the names only from objects with the `NameReader` trait while it writes the greeting on objects having the `GreetingWriter` trait.





Create an `adapters/` folder and add 2 concrete implementations of the previous traits in the `console_input.rs` and `console_output.rs` files.

```rust
// console_output.rs
use crate::Result;
use crate::ports;

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

impl ports::GreetingWriter for ConsoleOutput {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        println!("{}", greeting);
        Ok(())
    }
}
```

**Points of attention:**
* No surprise, since this is an implementation of the `GreetingWriter` and since it is named `ConsoleOutput` it... Yes, it writes on the console.
* Did you notice the `use crate::Result;` and `use crate::ports;` at the top of the file?
* Did you notice that `.new()` and `.default()` returns `Self`?



```rust
// console_input.rs
use std::io::{self, Write};

use crate::Result;
use crate::ports;

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

impl ports::NameReader for ConsoleInput {
    fn read_name(&self) -> Result<String> {
        print!("> ");
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {}", e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read from stdin: {}", e))?;

        let name = input.trim().to_string();

        Ok(name)
    }
}
```

**Points of attention:**
* No surprise here too. It is almost a copy and paste from `step_02/main.rs`.


Last but not least, add a file `adapters/mod.rs` that “describes” the modules that make up the `adapters` module and re-export `ConsoleInput` and `ConsoleOutput` names.


```rust
// mode.rs
pub mod console_input;
pub mod console_output;

pub use console_input::ConsoleInput;
pub use console_output::ConsoleOutput;
```



At this point, we have the Ports (traits) and the Adapters (implementations). We need to add them to the `lib.rs` file so that the `main.rs` file can use them:

```rust
// lib.rs
pub mod adapters;
pub mod domain;
pub mod ports;

// Type aliases for error handling
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
```





Finally we can rewrite `main.rs` file as follow:

```rust
// main.rs
use step_03::adapters;
use step_03::domain;
use step_03::ports;

use step_03::Result;

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 03 - Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Dependency injection: Create adapters
    let input = adapters::ConsoleInput::new();
    let output = adapters::ConsoleOutput::new();

    run_greeting_loop(&input, &output)?;

    Ok(())
}

fn run_greeting_loop(
    input: &dyn ports::NameReader,
    output: &dyn ports::GreetingWriter,
) -> Result<()> {
    loop {
        // Read name from input adapter
        let name = input.read_name()?;

        if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        if name.is_empty() {
            continue;
        }

        // Call domain logic (pure business rules)
        match domain::greet(&name) {
            Ok(greeting) => {
                // Write greeting to output adapter
                output.write_greeting(&greeting)?;
            }
            Err(e) => {
                eprintln!("Error: {}\n", e);
            }
        }
        println!();
    }
    Ok(())
}
```

**Points of attention:**
* Up to now, in `main()` we were calling `domain::greet()` directly.
* Now, we first instanciate an input and an output data types (`adapters`) which implement the traits defined in `ports.rs` (see the `let input = adapters::ConsoleInput::new();` for example)
* Then we call `run_greeting_loop()` using references to adapters (see the `&input` for example) as arguments
* The signature of `run_greeting_loop()` shows that the it accepts **ANY** reference to variable having the right trait (see the `input: &dyn ports::NameReader`).
* Here we make it works with `ConsoleInput` and `ConsoleOutput` but it would work the same way with WebInput and StonesOutput if they had the expected traits.
* In the code above look for `.read_name()` and `.write_greeting()` and realize we don't really know on who they will be applied.






Build, run and test the application. Find below the expected output:

```powershell
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_03\debug\step_03.exe`
=== Greeting Service (Step 03 - Hexagonal Architecture) ===
Enter a name to greet (or 'quit' to exit):

> Roberto
Ciao Roberto!

> exit

Goodbye!
```



```powershell
cargo test
   Compiling step_03 v0.1.0 (C:\Users\phili\OneDrive\Documents\Programmation\rust\01_xp\046_modular_monolith\step_03)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.82s
     Running unittests src\lib.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_03\debug\deps\step_03-6353ffb82854041d.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\main.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_03\debug\deps\step_03-e5e90dd1ed39887f.exe)

running 1 test
test tests::greeting_loop_with_mocks ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\adapters_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_03\debug\deps\adapters_test-46d627c56f70ee9b.exe)

running 9 tests
test error_propagation ... ok
test failing_input ... ok
test greeting_flow_with_roberto ... ok
test multiple_greetings ... ok
test failing_output ... ok
test mock_input_reader ... ok
test greeting_flow_with_long_name ... ok
test greeting_flow_with_mocks ... ok
test mock_output_writer ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests\domain_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_03\debug\deps\domain_test-73a3c5e5e6aeee7f.exe)

running 9 tests
test boundary_case_nineteen_chars ... ok
test domain_should_handle_unicode_names ... ok
test domain_should_not_use_special_greeting_for_similar_names ... ok
test greeting_length_limit ... ok
test normal_greeting ... ok
test truncation_for_long_names ... ok
test empty_name_returns_error ... ok
test domain_should_truncate_long_unicode_names ... ok
test roberto_special_case ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests\integration_test.rs (C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_03\debug\deps\integration_test-16290d5b02f48fa2.exe)

running 6 tests
test empty_name_integration ... ok
test long_name_integration ... ok
test multiple_greetings_integration ... ok
test end_to_end_with_dependency_injection ... ok
test greet_integration ... ok
test roberto_integration ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests step_03

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**Points of attention:**
* There are more tests.
    * Indeed one test of the `greeting_loop_with_mocks()` function have been added to `main.rs` because `run_greeting_loop()` is not public.
    * In addition the file `test/adapters_test.rs` host tests for the adapters using Mock implementations.
    * `tests/integration_test.rs` now use mock adapters
    * `tests/domain_test.rs` is not modified



{: .new-title }
> Summary
>
* `domain.rs` was **NOT** impacted. We "just" re-arrange the machinery around it
* ports = traits
* adapters = implementations
*







<div align="center">
<img src="./assets/img04.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 04 - Join the ports within the domain


### Objective
{: .no_toc }

We want integrates the traits defintinion available in `ports.rs` into the file `domain.rs`.

### Setup
{: .no_toc }

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


### Actions
{: .no_toc }

Update `Cargo.toml`

```toml
[package]
name = "step_04"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "step_04"
path = "src/main.rs"
```


Copy the content of `ports.rs` into `domain.rs`, then delete `ports.rs` :

```rust
// domain.rs
use crate::Result;

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

pub trait NameReader {
    fn read_name(&self) -> Result<String>;
}

pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<()>;
}
```

In `lib.rs` remove the line `pub mod ports;`

```rust
// lib.rs
pub mod adapters;
pub mod domain;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
```


In `main.rs` remove the line `use step_03::ports;`, replace `step_03` by `step_04` and update the signature of run_greeting_loop() so that it use `&dyn domain::NameReader` in place of `&dyn port::NameReader`.


```rust
// main.rs
use step_04::adapters;
use step_04::domain;

use step_04::Result;

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 04 - Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    let input = adapters::ConsoleInput::new();
    let output = adapters::ConsoleOutput::new();

    run_greeting_loop(&input, &output)?;

    Ok(())
}

fn run_greeting_loop(
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
        println!();
    }
    Ok(())
}
```

**Points of attention:**
* Including the ports into the `domain.rs` file help to express our intent. We want to make sure that the communication with the domain happens via ports.
* **Read the code** and the signature of `run_greeting_loop()`. We had `fn run_greeting_loop(input: &dyn ports::NameReader...` now we write `fn run_greeting_loop(input: &dyn domain::NameReader...`



Build, run and test the application. Find below the expected output:

```powershell
=== Greeting Service (Step 04 - Hexagonal Architecture) ===
Enter a name to greet (or 'quit' to exit):

> sd sdf sd fs df sd f sd fs df sd f sdf s df
Hello sd sdf sd fs df ...

> quit

Goodbye!
```

The tests output do not change.












{: .new-title }
> Summary
>
* Nothing change form the outside
* Ports are now included in the domain. This help to express the intent. The domain comunicate with the rest of the via trough the port.
* Adpaters which want to be used by the domain must implement those trait
* The domain does not depend on the adapters. The adapter depends on the domain. This is DIP (dependency inversion principle)








<div align="center">
<img src="./assets/img05.webp" alt="" width="450" loading="lazy"/><br/>
<span></span>
</div>














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 05 - One crate par component


### Objective
{: .no_toc }

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









#### **The app crate**
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




















#### **The integration_tests crate**
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









#### **The application crate**
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








#### **The domain crate**
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























#### **The adapter_console crate**
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





{: .new-title }
> Summary
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




















{: .new-title }
> Summary
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








{: .new-title }
> Summary
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
## Next Step
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files

<!--

* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Start Implementing Hexagonal Architecture
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 0X -
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 0X -
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 0X -
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}): Step 0X -

-->

