---
published: true
lang: en-US
layout: default
title: "From Folders to Crates: A Practical Guide to Modern Rust Module Trees (No mod.rs, No Tears)"
parent: "Rust"
#math: mathjax
date               : 2025-08-22 17:45:00
last_modified_date : 2025-08-22 17:45:00
---

# From Folders to Crates: A Practical Guide to Modern Rust Module Trees (No mod.rs, No Tears)

## TL;DR
* For beginners
* Starting with a hierarchy of directories and files already in place
* A process to make sure the compiler and the linker can build the lib/app
* Explain the `use ...` statements that satisfy the compiler and make your code easier to read 
* As proposed since 2018+, `mod.rs` files are not used
* Process
    * If you have a sub-directory named `my_dir/`
    * Beside the sub-directory `my_dir/`, create a file named `my_dir.rs`
    * Inside `my_dir.rs` list the module to be added to the module tree (`pub mod zoubida;`)
        * `zoubida` is the name of a module contained in `my_dir/zoubida.rs`
        * `my_dir/zoubida.rs` is either : 
            * module with some code 
            * a *hub* file with a list of modules to be added
    * You start the process with `lib.rs`


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

<!-- ### The context
Imagine...
* Imagine an application that use sensors to make measurements. This is why there is a `sensors/` directory.
* All sensors are input sensors (they are not actuators) and this is why there is an `input/` directory.
* Some of the sensors are temperature sensors. Later the application may use pressure, weight... sensors. Do you see the `temp/` directory?
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
* Use the *modern* way (no `mod.rs` files) for building a modules tree so that... 
* ... we can take temperature measurements in `src/main.rs` and in `examples/ex_01.rs`

### What I keep in mind
* There are a `src/lib.rs` and a `src/main.rs` files in the project directory. This means that the project becomes a library crate with a main binary.
* First the compiler will build the library then it will build the binary (using the content of the library).
* Keep in mind that the build system (compiler, linker and their friends) don't care about files. It only cares about the modules tree.

#### 1. Building the lib
* First I organize the directories the way I want 
* Then I name the files the way I want
* In other words, I don't want the build process to be restrictive — I want the flexibility to configure it according to my project's structure.
* This also means that the `sensors/`, `sensors/input/`, `sensors/input/temp/`... directories are in place **before** the steps below.

Now, in order to build the lib the build system needs to create the module tree so that the compiler and the linker can find, load, compile and link the different modules together. One way of doing (at least one way that I understand, which works and which seems to be scalable) is to use *hub* files. Indeed, [David Wheeler](https://www.40tude.fr/docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.html#indirection) once said : “All problems in computer science can be solved by another level of indirection.” But don't worry, this is not rocket science, let's see how it works. 

* To build the library crate the build system needs to load modules which are, somewhere in the `src/` directory but it does'nt know yet where. 
* So far, the file `lib.rs` (the crate root) contains one line : `pub mod sensors;`. I say "so far" because this may change in the future if some actuators are added to the project (you get the point).
* The line declares that part of the code is in a module named `sensors`. 
* With the line `pub mod sensors;`, the build system will look into the file `src/sensors.rs`
* So far, the file `sensors.rs` contains only one line : `pub mod input;`
* Then, beside the `lib.rs` file I create a *hub* file named `sensors.rs` because there is a `sensors/` sub-directory. 
* Then we follow the white rabbit...

<div align="center">
<img src="./assets/img_02.webp" alt="" width="225" loading="lazy"/>
</div>

* In the `sensors/` directory, I create a *hub* file named `input.rs` because there is an `input/` sub-directory
* So far, `input.rs` contains one line : `pub mod temp;`
* In the `input/` directory, I create a *hub* file named `temps.rs` next to the `temp/` directory. 
* So far, `temp.rs` have 3 lines
```rust
pub mod temp_sensor;  // The trait lives here
pub mod temp_sensor1; // Concrete sensor #1 (folder-backed)
pub mod temp_sensor2; // Concrete sensor #2 (folder-backed)
```
In the `temp/` directory, the file `temp_sensor.rs` is already there but I create 2 *hub* files named `temp_sensor1.rs`  and `temp_sensor2.rs` because there are 2 sub-directories : `temp_sensor1/` and `temp_sensor2/`. 







Just to make sure... The 3 lines above list the module where the trait is defined (in the file `temp_sensor.rs`) and the 2 modules indicating where the trait is implemented. When the compiler will read the content of `temp_sensor1.rs`, look in the `temp_sensor1/` directory it will find the file with the definition of `TempSensor01` and the implementation of the trait `TempSensor` for `TempSensor01` (same thing for `TempSensor02`). 

At the very end of the path, the `temp_sensor1/` directory contains `my_sensor1.rs` while `temp_sensor2/` contains `your_sensor2.rs`.















If you open `my_sensor1.rs` file, it should be noticed that since we are building the library crate, we can bring the `TempSensor` trait into scope using this line :

```rust
use crate::sensors::input::temp::temp_sensor::TempSensor;
```  
We don't write  `use crate_name::...` (where `crate_name` would be the name of the crate, the one defined in `Cargo.toml`). Instead when can write `use crate::...` where `crate` refers to the crate under construction (the lib).  

At the end of this process, the module tree is built, the compiler and the linker are happy, they can do their business and the library is saved on the disk. Now that the lib is ready it is time to build the executable using the content of `main.rs`.



At the end, the project directory looks like that : 


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
├───src
│   │   lib.rs
│   │   main.rs
│   │   sensors.rs
│   │
│   └───sensors
│       │   input.rs
│       │
│       └───input
│           │   temp.rs
│           │
│           └───temp
│               │   temp_sensor.rs
│               │   temp_sensor1.rs
│               │   temp_sensor2.rs
│               │
│               ├───temp_sensor1
│               │       my_sensor1.rs
│               │
│               └───temp_sensor2
│                       your_sensor2.rs
│
└───target
```



 -->




### The context

Imagine... 

* Imagine you’re building an application that uses sensors to collect measurements. That’s why there’s a `sensors/` directory.
* All sensors are *input* sensors (not actuators), so you add an `input/` directory.
* Some of the sensors measure temperature. Later, you may add others (pressure, weight, etc.). That explains the `temp/` directory.
* So far, you have two temperature sensors:
  * `TempSensor01` defined in `my_sensor1.rs` inside a `temp_sensor1/` directory
  * `TempSensor02` defined in `your_sensor2.rs` inside a `temp_sensor2/` directory
* To separate usage from implementation, you define a trait `TempSensor` and implement it for both sensors. Thanks to this, from the application’s perspective, using one sensor or the other looks the same:

  ```rust
  let my_temp = my_sensor.get_temp();
  ```

  where `my_sensor` could be either sensor.
* Notice that the sensor file name (for example `my_sensor1.rs`) does **not** match the directory name (`temp_sensor1/`). This is intentional: I want to keep the file hierarchy independent from the module tree and truly understand the differences between both.
* The project also includes both `lib.rs` and `main.rs`, making it a library crate with a binary crate.

Here’s the initial structure:

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

We want to:

* Use the *modern* style of Rust module organization (no more `mod.rs` files).
* Build a proper module tree so we can call temperature sensors from both `src/main.rs` and `examples/ex_01.rs`.


### What to keep in mind

* Because the project has both `lib.rs` and `main.rs`, Cargo treats it as a library crate plus a binary.
* The compiler first builds the library, then the binary (using the library’s contents).
* The build system doesn’t really care about files or directories — it only cares about the **module tree**.


### 1. Building the library

My approach is:

1. Organize the directories however I like.
2. Name the files however I like.
3. Then, use *hub files* to connect everything into a clean module tree.

This way, the build process doesn’t force me into rigid naming conventions — I have flexibility.

The key point: the `sensors/`, `input/`, and `temp/` directories already exist *before* I add *hub files*.


### Using hub files


The compiler needs a module tree to know how to find and link everything. One simple and scalable way is to use **hub files** — small files that declare which modules exist at a given level.

**Note:**
For what I know, the term *hub files* is absolutely not official. I call these files, *hub files* because they act as connectors in the module tree. As [David Wheeler](https://www.40tude.fr/docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.html#indirection) once said:

> “All problems in computer science can be solved by another level of indirection.”

Don’t worry — this isn’t rocket science. Let’s go step by step.


#### Step 1: Starting from `lib.rs`

Right now, `lib.rs` contains just one line:

```rust
pub mod sensors;
```

This declares a top-level module called `sensors`.

By convention, the compiler will first look for `src/sensors.rs`. Since we want a directory instead (`src/sensors/`), we create a *hub file* named `sensors.rs` next to `lib.rs`.

`sensors.rs` contains:

```rust
pub mod input;
```



#### Step 2: Going deeper

Inside the `sensors/` directory, we add another hub file, `input.rs`, because there is an `input/` subdirectory.

`input.rs` contains:

```rust
pub mod temp;
```

Inside the `input/` directory, we create a hub file named `temp.rs` next to the `temp/` directory.

`temp.rs` contains three lines:

```rust
pub mod temp_sensor;  // The trait lives here
pub mod temp_sensor1; // Concrete sensor #1 (folder-backed)
pub mod temp_sensor2; // Concrete sensor #2 (folder-backed)
```

In the `temp/` directory:

* `temp_sensor.rs` defines the `TempSensor` trait.
* We also create hub files `temp_sensor1.rs` and `temp_sensor2.rs` because there are subdirectories `temp_sensor1/` and `temp_sensor2/`.

When the compiler reads `temp_sensor1.rs`, it looks inside the `temp_sensor1/` directory and finds `my_sensor1.rs`, which defines `TempSensor01` and implements `TempSensor` for it (same logic for `TempSensor02`).

At the leaves of the tree, the directories contain the actual sensor implementations:

```
temp_sensor1/
    my_sensor1.rs
temp_sensor2/
    your_sensor2.rs
```


### Step 3: Using the trait

Inside `my_sensor1.rs`, we can bring the trait into scope like this:

```rust
use crate::sensors::input::temp::temp_sensor::TempSensor;
```

Notice: we don’t write `use crate_name::...` where `crate_name` comes from `Cargo.toml`.
Instead, we use `crate::...` to refer to the current crate under construction (the library).


### Wrapping up

At this point:

* The module tree is fully built.
* The compiler and linker can happily do their work.
* The library is compiled and stored on disk.
* Finally, Cargo builds the binary crate (`main.rs`) using that library.

The final directory structure looks like this:

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
├───src
│   │   lib.rs
│   │   main.rs
│   │   sensors.rs
│   │
│   └───sensors
│       │   input.rs
│       │
│       └───input
│           │   temp.rs
│           │
│           └───temp
│               │   temp_sensor.rs
│               │   temp_sensor1.rs
│               │   temp_sensor2.rs
│               │
│               ├───temp_sensor1
│               │       my_sensor1.rs
│               │
│               └───temp_sensor2
│                       your_sensor2.rs
│
└───target
```

The module tree looks like this :

```
crate (lib.rs)
└── sensors (sensors.rs)
    └── input (input.rs)
        └── temp (temp.rs)
            ├── temp_sensor       (temp_sensor.rs, trait definition)
            ├── temp_sensor1      (temp_sensor1.rs → loads folder temp_sensor1/)
            │   └── my_sensor1    (my_sensor1.rs, implements TempSensor01)
            └── temp_sensor2      (temp_sensor2.rs → loads folder temp_sensor2/)
                └── your_sensor2  (your_sensor2.rs, implements TempSensor02)
```



**Tips'n Tools**
* `cargo install cargo-modules`
* `cargo-modules dependencies --lib`
* Copy the output
* Paste it in [this page](https://dreampuf.github.io/GraphvizOnline/)  


<div align="center">
<img src="./assets/img_03.webp" alt="" width="900" loading="lazy"/>
</div>





#### 2. Building the binary
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



#### 3. Building the example (`examples/ex_01.rs`)
* It should be clear now that that the example is also a *client* of the lib
* All that we already know about `src/main.rs` apply to `examples/ex_01.rs`   
* This includes the line `use crate_name::...`













## What's next?
* `git clone` the project if you wish
* Forget about the code it should work
* Rename everything (files, directories), add directories etc.
* Break everything
* Make it work and make it great again







## Checklist

Welcome on board. Imagine you are in `parent_dir/` and you want to use a subdirectory `child_dir/` as a Rust module. Follow this checklist every time you add a new directory.

### 1. At the parent level

* **File to create**: `parent.rs` (next to `parent_dir/`)
* **Content inside**:

  ```rust
  pub mod child;  // because there is a child_dir/
  ```

  **Rule:** the file name must match the directory name (`child.rs` for `child_dir/`).


### 2. Inside the child directory

* You are now inside `child_dir/`.

* **File to create**: `child.rs` (next to the subdirectory itself).

* **Content inside**:

  * If you want to expose submodules, repeat the pattern:

    ```rust
    pub mod grandchild;  // because there is a grandchild_dir/
    pub mod something;   // because there is a something.rs file
    ```

* **Rule of thumb**:

  * Each *subdirectory* gets a **hub file** at the same level (`child.rs`).
  * Each *standalone file* just works: `something.rs` becomes the `something` module automatically (no hub needed).


### 3. At the leaf level (no more subdirectories)

* Just put your `.rs` files directly in the folder.
* Example: in `grandchild_dir/`, a file `foo.rs` creates the module `grandchild::foo`.
* No hub file is needed unless you want to organize deeper.


### 4. Bringing things into scope

* From anywhere in the crate:

  ```rust
  use crate::parent::child::grandchild::foo;
  ```

* If you’re inside `grandchild/` already, you can shorten paths with `super::` or `self::` if needed.


### 5. General rules

* **One subdirectory → one hub file**
* **Hub file name = subdirectory name + `.rs`**
* **Hub file content = `pub mod ...;` for each file or subdir you want to expose**
* Don’t mirror the file system blindly — choose what you want public.


**Exercice**

You are given the hierarchy below:

```
src/
├── lib.rs
└── animals/
    ├── animals.rs
    ├── cat/
    │   ├── cat.rs
    │   └── persian.rs
    └── dog.rs
```

Questions:
1. For each directory/file, explain what it is , what is it content...
1. Draw the module tree 


* `lib.rs` → `pub mod animals;`

* `animals.rs` →

  ```rust
  pub mod cat;
  pub mod dog;
  ```

* `cat.rs` →

  ```rust
  pub mod persian;
  ```

Module tree:

```
crate
└── animals
    ├── cat
    │   └── persian
    └── dog
```










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
