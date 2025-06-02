---
layout: default
title: "Rust on Windows 11: My Complete Setup Guide"
parent: "Rust"
#math: mathjax
date               : 2025-06-02 09:00:00
last_modified_date : 2025-06-32 09:00:00
---

# Rust on Windows 11: My Complete Setup Guide

<div align="center">
<img src="./assets/rust_analyzer.webp" alt="" width="450" loading="lazy"/>
</div>

## Install Rust
* From this [page](https://www.rust-lang.org/tools/install)
* Download then run ``rustup-init.exe`` (64b)
<!-- * From this [page](https://rustup.rs/) -->
* Open a terminal and run this command `rustc --version`

<div align="center">
<img src="./assets/rustc_version.webp" alt="" width="450" loading="lazy"/>
</div>


## Install VSCode
* Form this [page](https://code.visualstudio.com/download), download and install VSCode



## Install rust-analyzer extension for VSCode
* From VSCode (extensions pane)  
* Or from this [page](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) 

<div align="center">
<img src="./assets/rust-analyzer.webp" alt="" width="900" loading="lazy"/>
</div>


## Setup Linting
* From VSCode
* `CTRL + ,` to open the Settings
* Type in `rust-analyzer check`
* Raplace ``check`` by `clippy` 

<div align="center">
<img src="./assets/linting.webp" alt="" width="900" loading="lazy"/>
</div>


## Setup Inlay Hints
* In VSCode
* `CTRL + ,`
* Enter `inlay`
* Select `offUnlessPressed`
* Then press `CTRL + ALT` in the editor when you want to check the types of the variables

<div align="center">
<img src="./assets/inlay_hints.webp" alt="" width="450" loading="lazy"/>
</div>


## Debug
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
    * see below the red dot on the left
    * You can either click or strike ``F9``
* In the editor click on the "Debug" (see below in the red rectangle)


* Understand that the output happen in the ``DEBUG CONSOLE`` not in the `TERMINAL`. See below

<div align="center">
<img src="./assets/debug_03.webp" alt="" width="450" loading="lazy"/>
</div>

* You can then inspect variables, go step by step...

<div align="center">
<img src="./assets/debug_04.webp" alt="" width="900" loading="lazy"/>
</div>

## This may help
* Read this [page](https://code.visualstudio.com/docs/languages/rust)
* Enter `rustup doc` in a terminal
