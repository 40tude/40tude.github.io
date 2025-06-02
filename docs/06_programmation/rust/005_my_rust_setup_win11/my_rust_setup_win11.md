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
* Do you see the `i32` and `u8` in gray ?

<div align="center">
<img src="./assets/types_yes.webp" alt="" width="450" loading="lazy"/>
</div>









## Setup line width
* When saving file the formatter may reformat long lines
* I create a ``Rustfmt.toml`` file at the root of the project
* So far mine has only only one line

```toml
max_width = 200
```

* [Read this page](https://rust-lang.github.io/rustfmt/?version=v1.8.0&search=)












## Debug - Green Slope
* Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/)

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
* Copy this code for example in `src\main.rs`

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
    * You can either click or strike ``F9``
* In the editor click on the "Debug" (see below in the red rectangle)

<div align="center">
<img src="./assets/debug_02.webp" alt="" width="450" loading="lazy"/>
</div>


* Understand that the output happen in the ``DEBUG CONSOLE`` not in the `TERMINAL`. See below

<div align="center">
<img src="./assets/debug_03.webp" alt="" width="450" loading="lazy"/>
</div>

* You can then inspect variables, go step by step...

<div align="center">
<img src="./assets/debug_04.webp" alt="" width="900" loading="lazy"/>
</div>







## Debug - Blue Slope
If you want to debug code when you press F5.
* Create a `.vscode` folder at the root of the project
* Create a `launch.json` file in that directory
* Copy and paste the code below

```json
{
    "configurations": [
        {
            "type": "cppvsdbg",
            "request": "launch",
            "name": "run trash_me",
            "cwd": "${workspaceFolder}",
            "program": "${workspaceFolder}\\target\\debug\\NAME_OF_APPLICATION.exe",
            "environment": [
                {
                    "name": "RUST_BACKTRACE",
                    "value": "short"
                },
                {
                    "name": "RUSTC_TOOLCHAIN",
                    "value": "C:\\Users\\phili\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc"
                }
            ],
            "args": [],
            "sourceFileMap": {},
            "osx": {
                "MIMode": "lldb"
            }
        }
    ]
}
```
Here is how it should look like 

<div align="center">
<img src="./assets/debug_05.webp" alt="" width="900" loading="lazy"/>
</div>


* Replace `NAME_OF_APPLICATION` with the name of your application
* Save the ``.json`` file (`CTRL+S`)
* Switch to the editor
* Set a breakpoint somewhere
* Press `F5`

<div align="center">
<img src="./assets/debug_06.webp" alt="" width="900" loading="lazy"/>
</div>














## Run
* In the editor
* If you click Run (instead of Debug)
* Outputs happen in a terminal

<div align="center">
<img src="./assets/run.webp" alt="" width="900" loading="lazy"/>
</div>

* Alternatively you can
    * Open a terminal ``CTRL+ù`` (azerty keyboard)
    * Enter `cargo run`

<div align="center">
<img src="./assets/cargo_run.webp" alt="" width="900" loading="lazy"/>
</div>



## This may help
* Read this [page](https://code.visualstudio.com/docs/languages/rust)
* Enter `rustup doc` in a terminal
