---
published: true
author: 40tude
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 03"
description: "Learn Rust modular monolith: 7-step tutorial from Hello World to I/O-agnostic application with hexagonal architecture, traits and crates. For beginners, tinkerers, hobbyists, amateurs, and early-career developers..."
image: /docs/06_programmation/rust/025_modular_monolith/assets/img12.webp
twitter:
  card: summary_large_image
parent: "Rust"
nav_order: 34
date:               2026-01-29 15:00:00
last_modified_date: 2026-02-03 08:00:00
---


# Learning Modular Monolith Architecture with Rust
{: .no_toc }

Learn Rust modular monolith: 7-step tutorial from Hello World to I/O-agnostic application with hexagonal architecture, traits and crates. For beginners, tinkerers, hobbyists, amateurs, and early-career developers...
{: .lead }



<!--
<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2> -->





















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 03
{: .no_toc }

All the [examples](https://github.com/40tude/modular_monolith_tuto) are on GitHub


#### The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): 🟢 Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): 🟢 Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): 🟢 Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): 🟢 Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): 🔵 Step 04 - One crate per component
* [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): 🔴 Step 05 - Anyhow & ThisError
* [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): 🟢 Step 06 - Add new adapters + Conclusion
* [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}): 🟢 Bonus






<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
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

We want to start implementing an Hexagonal Architecture. The implementation will be finished in [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}) when the `greeting_service.rs` file and its use cases will be available.

If you have **ANY** doubt about Hexagonal Architecture, Ports and Adapters, before you move forward, read this [dedicated page]({%link docs/06_programmation/rust/024_hexagonal/hexagonal_lite.md%}).

Anyway, at the end of Step 03, the folder hierarchy should look like:

```text
step_03/
│   Cargo.toml
├───src
│   │   domain.rs              # Business rules (isolated)
│   │   error.rs               # Error and Result type alias
│   │   lib.rs                 # Library re-exports
│   │   main.rs                # Entry point
│   │   ports.rs               # Traits definition
│   └───adapters               # Implementations
│           console_input.rs
│           console_output.rs
│           mod.rs
└───tests
        adapters_test.rs       # Adapters unit tests
        domain_test.rs         # Domain unit tests
        integration_test.rs    # Integration tests
```










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Setup

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

* Before to read further,  feel free to read this [page]({%link docs/06_programmation/rust/024_hexagonal/hexagonal_lite.md%}).
* Indeed since the previous page exists, I do no plan to explain the concepts of the Hexagonal Architecture once again.

















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Actions





<!-- ###################################################################### -->
### Cargo.toml


```toml
[package]
name = "step_03"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "step_03"
path = "src/main.rs"
```











<!-- ###################################################################### -->
### ports.rs

Create a `ports.rs` file.

```rust
use crate::error::Result;

pub trait NameReader {
    fn read_name(&self) -> Result<String>;
}

pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<()>;
}
```


**Points of attention:**
* The idea is to make sure that our application get inputs only from objects with the `NameReader` trait while it writes the greeting on objects having the `GreetingWriter` trait.







<!-- ###################################################################### -->
### adapters/console_input.rs


Create an `adapters/` folder and add 2 concrete implementations of the previous traits in the `console_input.rs` and `console_output.rs` files.

```rust
// console_output.rs
use crate::error::Result;
use crate::ports;

#[derive(Default)]
pub struct ConsoleOutput;

impl ConsoleOutput {
    pub fn new() -> Self {
        Self
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
* Did you notice that `.new()` returns `Self` with a capital 'S'?





<!-- ###################################################################### -->
### adapters/console_output.rs


```rust
// console_input.rs
use std::io::{self, Write};
use crate::error::Result;
use crate::ports;

#[derive(Default)]
pub struct ConsoleInput;

impl ConsoleInput {
    pub fn new() -> Self {
        Self
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
* It is almost a copy and paste from `step_02/main.rs`.





<!-- ###################################################################### -->
### adapters/mod.rs


Last but not least, add a file `adapters/mod.rs` that “describes” the modules that make up the `adapters` module and re-export `ConsoleInput` and `ConsoleOutput` names.


```rust
// mode.rs
pub mod console_input;
pub mod console_output;

pub use console_input::ConsoleInput;
pub use console_output::ConsoleOutput;
```





<!-- ###################################################################### -->
### lib.rs

At this point, we have the Ports (traits) and the Adapters (implementations). We need to add them to the `lib.rs` file so that the `main.rs` file can use them:

```rust
pub mod adapters;
pub mod domain;
pub mod error;
pub mod ports;
```




<!-- ###################################################################### -->
### main.rs

Finally we can rewrite `main.rs` file as follow:

```rust
// main.rs
use step_03::adapters;
use step_03::domain;
use step_03::ports;
use step_03::error::Result;

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
* Now, we first instantiate an input and an output data types (`adapters`) which implement the traits defined in `ports.rs`
    * See the `let input = adapters::ConsoleInput::new();` for example
* Then we call `run_greeting_loop()` using references to the adapters as arguments
    * See the `&input` for example
* The signature of `run_greeting_loop()` shows that it accepts **ANY** reference to variable having the right trait
    * See the `input: &dyn ports::NameReader`
* Here we make it works with `ConsoleInput` and `ConsoleOutput` but it would work the same way with WebInput and StonesOutput if they have the expected traits.
* In the code above look for `.read_name()` and `.write_greeting()` and realize we don't really know on who they will be applied.




<!-- ###################################################################### -->
### tests/adapters_test.rs

The mock adapters used for the first time in `tests/adapters_test.rs` deserve some attention. Let's think about it for a second. We just introduced the ports `NameReader` and `GreetingWriter` specifically so that the domain doesn't depend on any concrete I/O. But if we want to test the whole flow (read a name → greet → write the greeting) without actually typing in a console... we need fake implementations of those traits. That's what mocks are.

A mock adapter is just a struct that implements a port trait but does something trivial and predictable instead of real I/O. `MockInput` holds a hardcoded name and returns it when `read_name()` is called. `MockOutput` captures whatever greeting is passed to `write_greeting()` into an internal `Vec<String>` so we can inspect it afterwards.

Here is the beginning of the code:

```rust
use step_03::domain;
use step_03::ports::{GreetingWriter, NameReader};
use step_03::error::Result;

/// Mock input adapter that returns a predefined name.
struct MockInput {
    name: String,
}

impl MockInput {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl NameReader for MockInput {
    fn read_name(&self) -> Result<String> {
        Ok(self.name.clone())
    }
}

/// Mock output adapter that captures written greetings.
struct MockOutput {
    written: std::cell::RefCell<Vec<String>>,
}

impl MockOutput {
    fn new() -> Self {
        Self {
            written: std::cell::RefCell::new(Vec::new()),
        }
    }
    fn get_written(&self) -> Vec<String> {
        self.written.borrow().clone()
    }
}

impl GreetingWriter for MockOutput {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        self.written.borrow_mut().push(greeting.to_string());
        Ok(())
    }
}

#[test]
fn mock_input_reader() {
    let input = MockInput::new("Alice");
    let name = input.read_name().unwrap();
    assert_eq!(name, "Alice");
}

// Others test below
```

**Points of attention:**

* The beautiful thing is that from the test's perspective, there's no difference between a mock and a real adapter. We write:

    ```rust
    let input = MockInput::new("Alice");
    let output = MockOutput::new();

    let name = input.read_name().unwrap();
    let greeting = domain::greet(&name).unwrap();
    output.write_greeting(&greeting).unwrap();

    let written = output.get_written();
    assert_eq!(written[0], "Hello Alice.");
    ```

* This is exactly the same sequence of calls we'd have in `main()` with real `ConsoleInput` and `ConsoleOutput`.
* The only difference is that nobody is sitting at a keyboard, and the result ends up in a vector instead of on the screen. That's the whole point of hexagonal architecture paying off: the domain and the wiring are testable in complete isolation, no I/O involved.
* There are not shown in the code fragment above but in `tests/adapters_test.rs` we also define `FailingInput` and `FailingOutput` (mocks that always return an error). These let us verify that our error handling works without needing to unplug a cable or corrupt a file.






















<!-- ###################################################################### -->
### Build, run & test

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

There are more tests.
* Indeed one test of the `greeting_loop_with_mocks()` function have been added to `main.rs` because `run_greeting_loop()` is not public.
* In addition, the file `test/adapters_test.rs` host tests for the adapters using Mock implementations.
* `tests/integration_test.rs` now use mock adapters
* `tests/domain_test.rs` is not modified
* The source code of the test is available on [GitHub on this page](https://github.com/40tude/modular_monolith_tuto/tree/main/step_03).








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Summary

{: .new-title }
> What have we done so far?
>
* `domain.rs` was **NOT** impacted. We "just" re-arrange the machinery around it
* ports = traits
* adapters = implementations
* Hexagonal Architecture is not rocket science but it really helps to name things and to describe the roles.













<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Next Steps

Next you can read [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}) but I would recommend to take a break because there is a kind of quantic jump between Episode 03 and Episode 04. Sleep well, see you tomorrow.

* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate per component
* [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 05 - Anyhow & ThisError
* [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 06 - Add new adapters + Conclusion
* [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}): Bonus
