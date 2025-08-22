---
published: true
lang: en-US
layout: default
title: "From Folders to Crates: A Practical Guide to Modern Rust Module Trees (No mod.rs, No Tears"
parent: "Rust"
#math: mathjax
date               : 2025-08-22 17:45:00
last_modified_date : 2025-08-22 17:45:00
---

# From Folders to Crates: A Practical Guide to Modern Rust Module Trees (No mod.rs, No Tears

## TL;DR
* For beginners
* Starting from an existing directory hierarchy
* Follow a simple process to make sure the compiler and the linker can build
* How to write the `use ...` statements that satisfies the compiler and makes your code easier to read is also explained
* As proposed since 2018+, in the end, there are no more `mod.rs` files everywhere.


<div align="center">
<img src="./assets/img_00.webp" alt="" width="450" loading="lazy"/>
</div>


## Introduction
* August 2025
* [p 139 of the Rust Prog Language book](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html#alternate-file-paths) they say that using `mod.rs` everywhere is old style, confusing, blablabla...
* What?!!! Old style... No way, not for me. I'm a fashion victim and I want to go to Milan, Paris etc.
    * Surprisingly I just `git clone` [Microsoft Edit](https://github.com/microsoft/edit) and it seems they use `mod.rs` everywhere...

<div align="center">
<img src="./assets/img_01.webp" alt="" width="225" loading="lazy"/>
</div>

* More seriously, this side project is the first step of larger project. Here, once and for all, I want to understand how to build my module tree the right way (at least one way that work and that I can understand) 
* So, below, don't start complaining. The project hierarchy is complicated for good reasons.
* The project is on [GitHub](https://github.com/40tude/traits_for_plugins_01)

### The context
Imagine...
* Imagine an application that use sensors to make measurements. This is why there is a `sensors/` directory.
* All sensors are input sensors (they are not actuators) and this is why there is an `input/` directory.
* Some of the sensors are temperature sensors. Later the application may use pressure, weight... sensors. Do you see the `temp/` directory
* So far we have 2 kinds of temperature sensor 
    * `TempSensor01` in a file named `my_sensor1.rs` in a directory `temp_sensor1/`
    * `TempSensor02` in a file named `your_sensor2.rs` in a directory `temp_sensor2/`
* Even if the code is not the point here, in order to insulate the usage from the implementation, I created a trait `TempSensor` which is implemented by both sensors. Doing so, from the application stand point, taking a temperature measurements with one sensor or the other is almost transparent. It looks like : `let my_temp = my_sensor.get_temp();` where `my_sensor` identify one sensor or the other. 
* While visiting the hierarchy of the project, please note that, at the very end, the name of the sensor file (exampli gratia `my_sensor1.rs`) is **not** in a directory having the same name (e.g. `temp_sensor1`). I do so because I want to make sure I differentiate the file hierarchy from the module tree. 
* The project also have a `lib.rs` and a `main.rs` so that we have a library crate and a binary crate
* All of the above explains the over complicated hierarchy of directories I mentioned before (see below)

```
.
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   README.md
│
├───examples
│       ex_01.rs
│
└───src
    │   lib.rs
    │   main.rs
    │
    └───sensors
        └───input
            └───temp
                │   temp_sensor.rs
                │
                ├───temp_sensor1
                │       my_sensor1.rs
                │
                └───temp_sensor2
                        your_sensor2.rs

```

### The objective
* Use the *modern* way (no `mod.rs` files) of building a module tree so that... 
* we can take temperature measurements in `src/main.rs` and in `examples/ex_01.rs`

### What I keep in mind
* There are a `src/lib.rs` and a `src/main.rs` files in the project directory. This means that the project becomes a library crate with a main binary.
* First the compiler will start building the library then it will build the binary (with the help of the content of the library).

#### Building the lib
* First I organize the directories the way I want 
* Then I name the files the way I want
* In other words, I don't want the build process to be restrictive — I want the flexibility to configure it according to my project's structure.
* This also means that the `sensors/`, `sensors/input/`, `sensors/input/temp/`... directories are in place **before** the following steps.

Now, in order to build the lib we need to create the module tree so that the compiler and the linker can find, load, compile and link the different modules together. One way of doing (at least one way that I understand, which works and which seems to be scalable) is to use *hub* files. Don't worry, it is not rocket science, let's see how it works. 

* To build the library I need to load code which is, somewhere in the `sensors/` directory but we don't yet know where. 
* So far, `lib.rs` contains one line : `pub mod sensors;`. I say "so far" because this will change in the future when more sensors and actuators will be added (you get the point)  
* Then, close to `lib.rs` I create a *hub* file named `sensors.rs` 
* So far, `sensors.rs` contains only one line : `pub mod input;`
* Then we follow the white rabbit...

<div align="center">
<img src="./assets/img_02.webp" alt="" width="225" loading="lazy"/>
</div>

* In the `sensors/` directory, I create a *hub* file named `input.rs`
* So far, `input.rs` contains one line : `pub mod temp;`
* In the `input/` directory, I create a *hub* file named `temps.rs` next to the `temp/` directory. 
* So far, `temp.rs` have 3 lines
```rust
pub mod temp_sensor; // The trait lives here
pub mod temp_sensor1; // Concrete sensor #1 (folder-backed)
pub mod temp_sensor2; // Concrete sensor #2 (folder-backed)
```
In the `temp/` directory, the file `temp_sensor.rs` is already there but I create 2 *hub* files named `temp_sensor1.rs`  and `temp_sensor2.rs`. Theses 3 files are next to the `temp_sensor1/` and `temp_sensor2/`directories. 

Just to make sure... The 3 lines above point respectively to the file where the trait is defined (`temp_sensor.rs`) and the 2 modules where the trait is implemented. When the compiler will look in the `temp_sensor1/` directory it will find the file with the definition of `TempSensor01` and the implementation of the trait `TempSensor` for `TempSensor01` (it is the same thing for `TempSensor02`). 

At the very end of the path, the `temp_sensor1/` directory contains `my_sensor1.rs` while `temp_sensor2/` contains `your_sensor2.rs`.

If you open `my_sensor1.rs` file, it should be noticed that since we are building the library crate, we can bring the `TempSensor` trait into scope using this line :

```rust
use crate::sensors::input::temp::temp_sensor::TempSensor;
```  
We don't write  `use crate_name::...` (where `crate_name` would be the name of the crate, the one defined in `Cargo.toml`). Instead when can write `use crate::...` where `crate` refers to the crate under construction (the lib).  

At the end of this process, the module tree is built, the compiler and the linker are happy, they can do their business and the library is saved on the disk. Now that the lib is ready it is time to build the executable using the content of `main.rs`.

#### Building the binary
* It should be clear now that the binary and the lib are 2 different beasts and this is why, in `main.rs`, we **cannot** write `use crate::...`. Instead we write `use crate_name::...` where `crate_name` is defined in `[package] name = “...”` in `Cargo.toml` (in our case `traits_for_plugins`).

Indeed, `main.rs` is a *client* of `lib.rs` and it does not see the internal modules via `crate::...` directly. In `main.rs`, `crate::...`  refers to the binary crate itself, **not** to the library defined in `lib.rs`. 

To make a long story short:
* Since there a `lib.rs` on the side of `main.rs`
* Then, in `main.rs` we have to `use` as if it is an external crate
* We use the crate name (the one defined in `[package] name = “...”` in `Cargo.toml`). 
* And yes, if there is a `lib.rs` and a `main.rs`, both crates, the library crate and the binary crate have, by default, the same name (this can be modified in `Cargo.toml`, see below)

```toml
[[bin]]
name = "MyApp"  # name of the executable (MyApp.exe under Windows)
path = "src/main.rs"

[lib]
name = "MySuperLib"  # name of the lib (MySuperLib.lib under Windows)
path = "src/lib.rs"

```



#### Building the example (`examples/ex_01.rs`)
* It should be clear now that that the example is also a *client* of the lib
* All that we already know about `src/main.rs` apply to `examples/ex_01.rs`   
* This includes the line `use crate_name::...`












---

## What's next?
* `git clone` the project if you wish
* Forget about the code it should work
* Rename everything (files, directory)
* Break everything
* Make it work and make it great again






<!-- 
---
## How to name a trait ? (by Chat GPT)

## Do / Don’t cheatsheet

**Do**

* Name traits with **nouns** (capabilities) or **adjectives** (markers).
* Name methods with **clear verbs** in `snake_case`.
* Use `as_/to_/into_` consistently for borrowing/owning/consuming conversions.
* Use `try_*` for fallible operations returning `Result`.
* Use `*Ext` for extension traits that add convenience APIs.
* Keep traits focused and methods symmetric.

**Don’t**

* Don’t use `-able` suffixes by default (`Readable`, `Iterable`) — unidiomatic.
* Don’t prefix getters with `get_` unless signaling *checked lookup* (`get`, `get_mut`).
* Don’t repeat the trait name inside method names (avoid stutter).
* Don’t pack unrelated behaviors into one “mega-trait”.

---

## Practical naming templates

* Capability: `Read`, `Write`, `Render`, `Encode`, `Decode`, `Validate`, `Persist`
* Marker: `Clone`, `Copy`, `Debug`, `Send`, `Sync`, `Sized`
* Extension: `FooExt`, `IteratorExt`, `HttpExt`
* Methods: `read`, `write`, `render`, `encode`, `decode`, `validate`, `persist`, `open`, `close`, `start`, `stop`
* Conversions: `as_x`, `to_x`, `into_x`
* Fallible: `try_x`
* Checked access: `get`, `get_mut`




---


## Detailed explanations

### 1) Prefer short **nouns** (or noun-like) for capabilities

* Good: `Read`, `Write`, `Display`, `Iterator`, `Borrow`, `AsRef`, `Deref`, `Drop`, `Hash`, `Eq`, `Ord`
* Avoid: `Readable`, `Iterable`, `Hashable` (the `-able` suffix is rare in Rust)

### 2) Use **adjectives** for marker traits (no required methods)

* Good: `Send`, `Sync`, `Sized`, `Unpin`, `Clone`, `Copy`, `Debug`
* These communicate a property/type marker rather than a behavior API.

### 3) Use `*Ext` for **extension traits**

* Example: `IteratorExt`, `HttpClientExt`
* These add convenience methods to an existing type/trait without forcing blanket impls in user code.

### 4) Use `*Mut` when the trait clearly implies mutability compared to a base trait

* Example: `Borrow` vs `BorrowMut`.

### 5) Avoid prefixing the trait name in its methods

* If the trait is `Read`, the method should be `read`, not `read_read` or `read_from_reader`.

### 6) Keep the **scope tight**

* A trait should express a single responsibility (coherent “capability”). If you feel you need many unrelated methods, split the trait.

---

## How to name trait methods

### 1) Methods are **verbs** in `snake_case`

* `read`, `write`, `flush`, `render`, `validate`, `persist`, `fetch_next`

### 2) Use the standard conversion triad carefully

* `as_*` — cheap borrow/view (no allocation), e.g., `as_str`
* `to_*` — cheap if possible, otherwise **clones/allocates** to produce an owned value, e.g., `to_string`
* `into_*` — **consumes** `self` to produce a new owned value, e.g., `into_string`

### 3) Use `try_*` for fallible actions returning `Result<_, E>`

* e.g., `try_reserve`, `try_parse`

### 4) Getter conventions

* Simple accessors: **no `get_` prefix** — `len()`, `capacity()`, `is_empty()`
* Use `get` when it signals *checked lookup* (often returning `Option`): `get(index)`, `get_mut(index)`

### 5) Mutability reveals intent

* Non-mutable view: `as_*`
* Mutable view: `as_*_mut` or `get_mut`
* Consuming: `into_*`

### 6) Pairs and symmetry help

* If you have `start()` then usually also `stop()`/`shutdown()`
* If you have `open()` then usually `close()`
* Keep names parallel across methods.

### 7) Avoid stutter

* If the trait is `Render`, method should be `render()`, not `render_render()` or `do_render()`.

---

## Associated items inside traits

* Associated types: short, generic, and conventional: `type Item`, `type Error`
* Type parameters: `T`, `U`, `E`, `K`, `V` (domain-specific names are OK if clearer)
* Associated consts: `UPPER_SNAKE_CASE`

---

## Small examples

### Capability trait (noun) + verb methods

```rust
pub trait Render {
    /// Render into the provided target. Returns the number of bytes written.
    fn render(&self, target: &mut Vec<u8>) -> Result<usize, RenderError>;

    /// Render directly to stdout as a convenience.
    fn render_stdout(&self) -> Result<(), RenderError>;
}
```

### Marker trait (adjective) with no methods

```rust
/// Types that are safe to persist as-is without redaction.
/// Marker trait: no required methods.
pub trait PersistSafe {}
```

### Extension trait pattern

```rust
/// Extra convenience methods on `Iterator`.
pub trait IteratorExt: Iterator {
    /// Collect into a `Vec` and assert non-empty.
    fn collect_non_empty(self) -> Result<Vec<Self::Item>, &'static str>
    where
        Self: Sized,
    {
        // NOTE: Uses standard semantics: consumes `self`.
        let v: Vec<_> = self.collect();
        if v.is_empty() { Err("empty") } else { Ok(v) }
    }
}

impl<I: Iterator> IteratorExt for I {}
```

### Conversions (as\_/to\_/into\_)

```rust
pub trait AsBytes {
    /// Borrowing view, no allocation.
    fn as_bytes(&self) -> &[u8];
}

pub trait ToBytes {
    /// May allocate/clone to produce owned bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait IntoBytes {
    /// Consumes `self` to produce owned bytes.
    fn into_bytes(self) -> Vec<u8>;
}
```
 -->
