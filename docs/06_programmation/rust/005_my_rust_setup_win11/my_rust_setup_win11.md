---
layout: default
title: "Rust on Windows 11, My Complete Setup Guide"
parent: "Rust"
#math: mathjax
date               : 2025-06-02 09:00:00
last_modified_date : 2025-06-32 09:00:00
---

# Rust on Windows 11, My Complete Setup Guide

<div align="center">
<img src="./assets/rust_analyzer.webp" alt="" width="225" loading="lazy"/>
</div>

## Install Rust
* From this [page](https://www.rust-lang.org/tools/install)
* Download then run ``rustup-init.exe`` (64b)
<!-- * From this [page](https://rustup.rs/) -->
* Open a terminal and run this command `rustc --version` just to make sure everything is OK.

<div align="center">
<img src="./assets/rustc_version.webp" alt="" width="450" loading="lazy"/>
</div>


## Install VSCode
* From this [page](https://code.visualstudio.com/download), download and install VSCode



## Install rust-analyzer extension for VSCode
* Open VSCode then from 
    * The extensions pane, on the left  
    * Or from this [page](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) 
* Install ``rust-analyzer``
<div align="center">
<img src="./assets/rust-analyzer.webp" alt="" width="900" loading="lazy"/>
</div>






## Setup Linting
* From VSCode
* `CTRL + ,` to open the ``Settings``
* Type in `rust-analyzer check`
* Replace ``check`` by `clippy` in the ``Rust-analyzer > Check : Command`` (see below) 

<div align="center">
<img src="./assets/linting.webp" alt="" width="900" loading="lazy"/>
</div>






## Setup Inlay Hints

By default, I find the type information a little too invasive, unnecessarily lengthening the lines of code and blurring readability. They're very useful, but I'd like to display them on demand.

* In VSCode
* `CTRL + ,` to open the ``Settings``
* Enter `inlay`
* Select `offUnlessPressed`
* Then press `CTRL + ALT` in the editor when you want to check the types of the variables

<div align="center">
<img src="./assets/inlay_hints.webp" alt="" width="450" loading="lazy"/>
</div>

### Not pressing CTRL+ALT
<div align="center">
<img src="./assets/types_no.webp" alt="" width="450" loading="lazy"/>
</div>

### Pressing CTRL+ALT

Do you see the `i32` and `u8` in gray ?

<div align="center">
<img src="./assets/types_yes.webp" alt="" width="450" loading="lazy"/>
</div>









## Setup line width
* When saving file the formatter may reformat long lines
* I create a ``Rustfmt.toml`` file at the root of the project
* So far my ``Rustfmt.toml`` has only only one line

```toml
max_width = 200
```

* [Read this page](https://rust-lang.github.io/rustfmt/?version=v1.8.0&search=)












## Debugging code 1/2
* Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/)
    * This is my preferred option because [I code in C++.]({%link docs/06_programmation/cpp/index.md%}). It comes with everything : compiler, linker, debugger...
    * Others may install [CodeLLDB VSCode extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb). It only provides a debugger. May be enough if you plan to only use Rust.

<div align="center">
<img src="./assets/visual_studio_ build_tools.webp" alt="" width="900" loading="lazy"/>
</div>

<!-- https://visualstudio.microsoft.com/visual-cpp-build-tools/ -->

<!-- * Optional. You can install [this extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) if you plan to use C++  -->

* Open a terminal
* Create a project (`cargo new rust_test4web`)

<div align="center">
<img src="./assets/debug_01.webp" alt="" width="900" loading="lazy"/>
</div>

* `cd .\rust_test4web\`
* `code .`
* Copy the lines below for example in the file `src\main.rs`

```rust
fn main() {
    let x = 5;
    println!("{}", x);

    let y = 6u8;
    println!("{}", y);

    let mut zoubida = 18;
    println!("{}", zoubida);

    zoubida = 19;
    println!("{}", zoubida);
}

```
* Set a breakpoint on line 8 
    * See below the red dot on the left
    * You can either click or strike ``F9`` when the cursor is on the line of interest
* In the editor click on the "Debug" (see below in the red rectangle)

<div align="center">
<img src="./assets/debug_02.webp" alt="" width="450" loading="lazy"/>
</div>

* A `target/debug` directory is created 
* While looking for the outputs, keep in mind they occurs in the ``DEBUG CONSOLE`` **not** in the `TERMINAL`. See below

<div align="center">
<img src="./assets/debug_03.webp" alt="" width="450" loading="lazy"/>
</div>

* Once the code stops on the line, you can then inspect variables, go step by step...

<div align="center">
<img src="./assets/debug_04.webp" alt="" width="900" loading="lazy"/>
</div>























## Debugging code 2/2
If you want to debug code when you press F5 and have more options (like passing arguments for example)
* Create a `.vscode` folder at the root of the project
* Create a `tasks.json` file in that directory
* Copy and paste the lines below

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo-build-debug",
            "type": "cargo",
            "command": "build",
            "args": [],
            "problemMatcher": [
                "$rustc"
            ],
            "group": {
                "kind": "build",
                "isDefault": true
            }
        },
        {
            "label": "cargo-build-release",
            "type": "cargo",
            "command": "build",
            "args": [
                "--release"
            ],
            "problemMatcher": [
                "$rustc"
            ]
        }
    ]
}

```
* Save the ``.json`` file (`CTRL+S`)
* Create a `launch.json` file in the `.vscode` folder
* Copy and paste the lines below

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "cppvsdbg",
            "request": "launch",
            "name": "Debug",
            "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
            "args": [],
            "cwd": "${workspaceFolder}",
            "environment": [
                {
                    "name": "RUST_BACKTRACE",
                    "value": "short"
                }
            ],
            "preLaunchTask": "cargo-build-debug"
        },
        {
            "type": "cppvsdbg",
            "request": "launch",
            "name": "Release",
            "program": "${workspaceFolder}/target/release/${workspaceFolderBasename}.exe",
            "args": [],
            "cwd": "${workspaceFolder}",
            "environment": [
                {
                    "name": "RUST_BACKTRACE",
                    "value": "short"
                }
            ],
            "preLaunchTask": "cargo-build-release"
        }
    ]
}
```

Here is how it should look like 

<div align="center">
<img src="./assets/debug_05.webp" alt="" width="900" loading="lazy"/>
</div>


* On the left hand side, click on the `Run & Debug` icon (the bug and the triangle icon, `CTRL+SHIFT+D` otherwise)
    * Make sure the current config in the list box is set on ``Debug``
    * See below

<div align="center">
<img src="./assets/debug_07.webp" alt="" width="450" loading="lazy"/>
</div>



* Switch back to the code
* Set a breakpoint somewhere
* Press `F5`
    * A `target/debug` directory is created 
    * The debugger stops on the breakpoint 

<div align="center">
<img src="./assets/debug_06.webp" alt="" width="900" loading="lazy"/>
</div>




## 3 ways to run the Debug version of your code without the debugger

Make sure the current configuration is still `Debug`

### Option 1 :
* If you press ``CTRL+F5`` this run the Debug version of the code but without debugging it
* So, the debugger does not stop on the breakpoint.

### Option 2 :
* In the editor
* Click `Run` (instead of `Debug`)

<div align="center">
<img src="./assets/run.webp" alt="" width="450" loading="lazy"/>
</div>

### Option 3 :
* In VSCode
* Open a terminal ``CTRL+ù`` (azerty keyboard)
* Enter `cargo run`

<div align="center">
<img src="./assets/cargo_run.webp" alt="" width="450" loading="lazy"/>
</div>


















## Create and Run a Release Version 1/2

* On the left hand side, click on the `Run & Debug` icon (`CTRL+SHIFT+D`)
    * Make sure the current config is set on ``Release``

<div align="center">
<img src="./assets/release.webp" alt="" width="450" loading="lazy"/>
</div>

* You can either press F5 or CTRL+F5
* A `target/release` directory is created 



## Create and Run a Release Version 2/2
* In VSCode
* Open a terminal ``CTRL+ù`` (azerty keyboard)
* Enter `cargo run --release`






## How to build only (either, Debug or Release version)

### Option 1 :  
* Click on ``Terminal/Run Task...`` option
* Select ``cargo-build-debug`` or ``cargo-build-release``

<div align="center">
<img src="./assets/tasks.webp" alt="" width="900" loading="lazy"/>
</div>

### Option 2 :  
* Open a terminal in VSCode
* Either type `cargo build` or `cargo build --release`

























## Optional

### Color Syntax for `.toml` files
* In VSCode install "Even Better TOML" extension

<div align="center">
<img src="./assets/even_better_toml.webp" alt="" width="450" loading="lazy"/>
</div>



### Get hints help & support while editing cargo.toml files
* Mostly helps when editing versions of the crates to be included in the project
* In VSCode install "dependi" extension (crates extension is now deprecated)

<div align="center">
<img src="./assets/dependi.webp" alt="" width="450" loading="lazy"/>
</div>

















## This may help
* Read this [page](https://code.visualstudio.com/docs/languages/rust)
* Enter `rustup doc` in a terminal
