---
published: true
lang: en-US
layout: default
title: "Learning Modular Monolith Architecture with Rust - 06"
description: "A 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates"
parent: "Rust"
nav_order: 34
date:               2026-01-29 15:00:00
last_modified_date: 2026-02-07 12:00:00
---


# Learning Modular Monolith Architecture with Rust
{: .no_toc }

A 7-project progression from Hello World to a fully decoupled, I/O-agnostic application using traits and crates
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>





















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 06
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
<img src="./assets/img06.webp" alt="" width="450" loading="lazy"/><br/>
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

We want to add an `adapter_file` crates so that the application can read names and write greetings into files.

If the architecture is correct we should have no or very few modification in the existing code and focus our attention only on the code of the `adapter_file`.

At the end of this episode, the folder hierarchy should look like this:

```text
step_06/
│   Cargo.toml
│   input.txt
│   output.txt
├───.cargo
│       config.toml
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
    ├───adapter_file
    │   │   Cargo.toml
    │   ├───src
    │   │       errors.rs
    │   │       input.rs
    │   │       lib.rs
    │   │       output.rs
    │   └───tests
    │           adapter_file_test.rs
    ├───app
    │   │   Cargo.toml
    │   └───src
    │           main.rs
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

**Points of attention:**
* Obviously there is a new folder, see `adapter_file/` whose organization is similar to `adapter_console/`







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Setup
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
    "crates/adapter_file",
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
* We need to take into account `crates/adapter_file`



<!-- ###################################################################### -->
### The application/src/greeting_service.rs file
<!-- {: .no_toc } -->

Is not modified.





<!-- ###################################################################### -->
### The app/src/main.rs file
<!-- {: .no_toc } -->

Let's see how to use the new adapter:

```rust
use adapter_console::{ConsoleInput, ConsoleOutput};
use adapter_file::{FileInput, FileOutput};
use application::GreetingService;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 06 - File Adapter Demo) ===");

    // Dependency injection: Create file-based adapters
    // let output = ConsoleOutput::new();
    // let input = ConsoleInput::new();

    let output = FileOutput::new("output.txt");
    let input = match FileInput::new("input.txt") {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Failed to read input file: {e}");
            return Ok(());
        }
    };

    let service = GreetingService::new();
    service
        .run_greeting_once(&input, &output)
        // .run_greeting_loop(&input, &output)
        .context("Failed to run greeting service")?;

    Ok(())
}
```


**Points of attention:**
* I commented the creation of the console adapters but not the associated `use` statements. It will be easy to mix the adapters (read in a file, write on the console for example).
* When `input` is created, if `input.txt` file does not exists we must handle the error. **I don't like it**. I would prefer to simply write `let input = FileInput::new("input.txt");` to keep the creation of adapters homogenous. Stop grumbling, a solution exits.










<!-- ###################################################################### -->
### The adapter_file crate
<!-- {: .no_toc } -->

Feel free to copy/paste/rename the `adapter_console` folder.

The `Cargo.toml` file does not change.

In `lib.rs` the last 2 lines change and the file looks like that:

```rust
pub mod errors;
pub mod input;
pub mod output;

pub use input::FileInput;
pub use output::FileOutput;
```


The `errors.rs` looks like this:

```rust
use domain::InfraError;
use std::any::Any;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl InfraError for FileError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub(crate) fn into_infra(e: impl Into<FileError>) -> Box<dyn InfraError> {
    Box::new(e.into())
}
```

**Points of attention:**
* `ConsoleError` changes in `FileError`









The `output.rs` file

```rust
use crate::errors::into_infra;
use domain::{GreetingWriter, InfraError};
use std::path::PathBuf;

pub struct FileOutput {
    path: PathBuf,
}

impl FileOutput {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let _ = std::fs::remove_file(&path);
        Self { path }
    }
}

impl GreetingWriter for FileOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        std::fs::write(&self.path, format!("{greeting}\n")).map_err(into_infra)?;
        Ok(())
    }
}
```

**Points of attention:**
* `ConsoleOutput` is replaced by `FileOutput`
* Note that any existing file is deleted when the object is created (see the `::remove_file()` in `.new()`)



The `input.rs` file:

```rust
use crate::errors::FileError;
use domain::{InfraError, NameReader};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileInput {
    name: String,
}

impl FileInput {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, FileError> {
        let path = path.into();
        let content = fs::read_to_string(&path).map_err(FileError::from)?;

        let name = content
            .lines()
            .next()
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(Self { name })
    }
}

impl NameReader for FileInput {
    fn read_name(&self) -> Result<String, Box<dyn InfraError>> {
        Ok(self.name.clone())
    }
}
```

**Points of attention:**
* `ConsoleInput` is replaced by `FileInput`
* In this version the content of the input file is loaded when the adapter is created and `name` is initialized with the content of the first line.
* If the input file does not exist then an error is reported
* When `read_name()` is called we simply return `name`'s value.


**TODO:**

In a next version:
* Calling `FileInput::new("input.txt")` should be similar to calling `ConsoleOutput::new()`
* We should be able to read more than one name in the input file and write more than one greeting in the output file.
* To do so we will need to modify `FileInput` so that it loads the file on the first read, reports error if needed and behaves like an iterator on each reading.
* Internally this requires a `vector<String>` where the names are stored and an `index` which is incremented on each read.
* This means that FileInput object created in `main()` **MUST** be mutable (which is not the case currently, check the signatures).
* **IMPORTANT:** Lesson learn: We should mimic the API of the standard library. For example, if we want `read_name()` to behave like `Iterator::next()` it should have the same signature : `read_name(&mut self) -> Result<String, Box<dyn InfraError>>` and not `fn read_name(&self) -> Result<String, Box<dyn InfraError>>`. Don't trust me and double check the signature of [Iterator::next()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#required-methods) for example.
* Why is that? Simply because in `read_name()`, if I want to increment the `index` I **mutate the object**. If I don't have `&mut self`, things become complicated with `RefCell` etc.
* If you have any doubt about the mutability of the bindings read this [page]({%link docs/06_programmation/rust/004_mutability/mutability_us.md%}).




<!-- ###################################################################### -->
### Build, run & test


Create an `input.txt` file at the root of the project. Here is an example with one empty line in the middle:

```text
Buck
Roberto
```

Build, run and test the application. Find below the expected output:

```text
cargo run
warning: unused imports: `ConsoleInput` and `ConsoleOutput`
 --> crates\app\src\main.rs:3:23
  |
3 | use adapter_console::{ConsoleInput, ConsoleOutput};
  |                       ^^^^^^^^^^^^  ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `app` (bin "step_06") generated 1 warning (run `cargo fix --bin "step_06" -p app` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_06\debug\step_06.exe`
=== Greeting Service (Step 06 - File Adapter Demo) ===

Goodbye!
```


**Points of attention:**
* Do not worry about the warnings. This is simply because we don't use `ConsoleInput` nor `ConsoleOutput`


A new `output.txt` file is created. Here is its content

```text
Hello Buck.
```

**Points of attention:**
* Only one line
* Modify `input.txt` file with 2 lines (Roberto and Buck for example). The new `output.txt` file will have one line again.





We can "play" with `app/src/main.rs` and uncomment/comment the adapters we want to mix. For example, reading from a file and writing in the terminal. For example, with this setup in `main.rs`:

```rust
let output = ConsoleOutput::new();
// let input = ConsoleInput::new();

// let output = FileOutput::new("output.txt");
let input = match FileInput::new("input.txt") {
    Ok(input) => input,
    Err(e) => {
        eprintln!("Failed to read input file: {e}");
        return Ok(());
    }
};
```

I get this output on the screen:

```text
cargo run
warning: unused import: `ConsoleInput`
 --> crates\app\src\main.rs:3:23
  |
3 | use adapter_console::{ConsoleInput, ConsoleOutput};
  |                       ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `FileOutput`
 --> crates\app\src\main.rs:4:31
  |
4 | use adapter_file::{FileInput, FileOutput};
  |                               ^^^^^^^^^^

warning: `app` (bin "step_06") generated 2 warnings (run `cargo fix --bin "step_06" -p app` to apply 2 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `C:/Users/phili/rust_builds/Documents/Programmation/rust/01_xp/046_modular_monolith/step_06\debug\step_06.exe`
=== Greeting Service (Step 06 - File Adapter Demo) ===
Hello Buck.

Goodbye!
```





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Summary

{: .new-title }
> What have we done so far?
>
* Adding a new adapter is easy and we are able to focus mostly on implementing the methods of the trait. This can be done (and tested) by someone else independently.
* Yes, we took the new adapter into account in `main.rs` and in `Cargo.toml` but that's all.
* Tomorrow we can write `adapter_tcp`, `adapter_sql` using the same process.









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

Next you can read [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}).

* [Episode 00]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}): Introduction + Step 00 - First prototype working
* [Episode 01]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_01.md%}): Step 01 - Split the source code in multiple files
* [Episode 02]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_02.md%}): Step 02 - Add a test folder
* [Episode 03]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_03.md%}): Step 03 - Implement Hexagonal Architecture
* [Episode 04]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_04.md%}): Step 04 - One crate per component
* [Episode 05]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_05.md%}): Step 05 - Anyhow & ThisError
* [Episode 06]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_06.md%}): Step 06 - Add new adapters + Conclusion
* [Episode 07]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_07.md%}): Bonus

