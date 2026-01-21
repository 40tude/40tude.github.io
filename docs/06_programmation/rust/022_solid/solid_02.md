---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide - 02"
description: "A gentle introduction to SOLID principles using Rust. The focus is on Liskov Substitution Principle."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-18 09:00:00
---


# SOLID Principles in Rust: A Practical Guide
{: .no_toc }

A gentle introduction to SOLID principles using Rust. The focus is on Liskov Substitution Principle.
{: .lead }




<!-- <h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2> -->










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 02
{: .no_toc }

#### The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/022_solid/solid_00.md%}): Introduction + Single Responsibility Principle
* [Episode 01]({%link docs/06_programmation/rust/022_solid/solid_01.md%}): Open-Closed Principle
* [Episode 02]({%link docs/06_programmation/rust/022_solid/solid_02.md%}): Liskov Substitution Principle
* [Episode 03]({%link docs/06_programmation/rust/022_solid/solid_03.md%}): Interface Segregation Principle
* [Episode 04]({%link docs/06_programmation/rust/022_solid/solid_04.md%}): Dependency Inversion Principle + Conclusion

<div align="center">
<img src="./assets/img00.webp" alt="" width="450" loading="lazy"/><br/>
<span>1986</span>
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
## Liskov Substitution Principle (LSP)








### The Principle

> "Functions that use references to base classes must be able to use objects of derived classes without knowing it."

In Rust terms:

> "Code that depends on a trait must work correctly with any implementation of that trait."

LSP is about **keeping promises**. If our trait says "this method returns the sum of two numbers", then every implementation better return the sum - not the difference, not a random number, not a side effect.









### The Problem: Surprising Substitutions

Historic and classic example from OOP - the Rectangle/Square problem. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_01_lsp

// =========================
// Rectangle/Square problem
// =========================

// =========================
// Abstractions
// =========================

pub trait Shape {
    fn set_width(&mut self, width: f64);
    fn set_height(&mut self, height: f64);
    fn area(&self) -> f64;
}

// =========================
// Concrete shapes
// =========================

// Rectangle
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Square
// A square is a rectangle, right? Mathematically yes. In software? Trouble.
pub struct Square {
    side: f64,
}

impl Shape for Square {
    fn set_width(&mut self, width: f64) {
        self.side = width; // Setting width changes the square's side
    }

    fn set_height(&mut self, height: f64) {
        self.side = height; // Setting height ALSO changes the square's side
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// =========================
// Usage
// =========================

// This function expects Shape behavior
fn main() {
    let mut my_square = Square { side: 20.0 };
    let area = my_square.area();
    println!("Expected area: 400, Got: {}", area);

    my_square.set_width(10.0);
    my_square.set_height(13.0);
    let area = my_square.area();

    // We expect: width=10, height=13, area=130
    // With Rectangle: CORRECT (10 * 13 = 130)
    // With Square: WRONG! (13 * 13 = 169)
    // The last set_height overwrote the width
    println!("Expected area: 130, Got: {}", area);
}
```

Expected output:
```powershell
Expected area: 400, Got: 400
Expected area: 130, Got: 169
```

**The violation**: `Square` doesn't truly substitute for `Shape`. The caller expects setting width and height independently, but `Square` violates that expectation.














### The Solution: Better Abstractions

We should not force types into hierarchies they don't belong to. We should model what they actually are. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_02_lsp

// =========================
// Rectangle/Square solution
// =========================

// =========================
// Abstractions
// =========================

// Immutable shapes with clear contracts
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// =========================
// Concrete shapes
// =========================

// Rectangle
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// Square
pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

// =========================
// Usage
// =========================

fn main() {
    let my_square = Square { side: 20.0 };
    println!(
        "Area: {}, Perimeter: {}",
        my_square.area(),
        my_square.perimeter()
    );

    let my_rect = Rectangle {
        width: 6.0,
        height: 7.0,
    };
    println!(
        "Area: {}, Perimeter: {}",
        my_rect.area(),
        my_rect.perimeter()
    );
}
```

Expected output:

```powershell
Area: 400, Perimeter: 80
Area: 42, Perimeter: 26
```

No mutation, no violated expectations. Each shape upholds the `Shape` contract.
















### Real-World Example: Storage Backends

Let's say we're building a key-value store with multiple backends. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_lsp_03

// =========================
// Storage Backends - Problem
// =========================

use std::collections::HashMap;

// =========================
// Abstractions
// =========================

pub trait Storage {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn delete(&mut self, key: &str) -> bool;
}

// Simple Redis mock so the example compiles
pub struct RedisClient;

impl RedisClient {
    fn get(&self, _key: &str) -> Result<String, ()> {
        Err(())
    }
    fn set(&self, _key: &str, _value: &str) -> Result<(), ()> {
        Ok(())
    }
    fn del(&self, _key: &str) -> Result<(), ()> {
        Ok(())
    }
}

// =========================
// Concrete storages
// =========================

// In-memory backend
pub struct MemoryStorage {
    data: HashMap<String, String>,
}

impl MemoryStorage {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Storage for MemoryStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}

// Redis backend
pub struct RedisStorage {
    client: RedisClient,
}

impl RedisStorage {
    fn new() -> Self {
        Self {
            client: RedisClient,
        }
    }
}

impl Storage for RedisStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.client.get(key).ok()
    }

    fn set(&mut self, key: String, value: String) {
        self.client.set(&key, &value).ok();
    }

    fn delete(&mut self, key: &str) -> bool {
        self.client.del(key).is_ok()
    }
}

// BAD: File storage that violates LSP
pub struct FileStorage {
    base_path: String,
}

impl FileStorage {
    fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }
}

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        // Path traversal, filename length, permissions, etc.
        std::fs::read_to_string(format!("{}/{}", self.base_path, key)).ok()
    }

    fn set(&mut self, key: String, value: String) {
        // Fails silently if disk is full
        std::fs::write(format!("{}/{}", self.base_path, key), value).ok();
    }

    fn delete(&mut self, key: &str) -> bool {
        // Lies if file never existed
        std::fs::remove_file(format!("{}/{}", self.base_path, key)).is_ok()
    }
}

// =========================
// Usage
// =========================

// Generic function using the Storage trait
fn demo(storage: &mut dyn Storage) {
    storage.set("key".into(), "value".into());
    println!("Value = {:?}", storage.get("key"));
    println!("Deleted = {}", storage.delete("key"));
}

fn main() {
    let mut mem = MemoryStorage::new();
    let mut redis = RedisStorage::new();
    let mut file = FileStorage::new(".");

    demo(&mut mem);
    demo(&mut redis);
    demo(&mut file); // LSP violations hidden behind the trait
}
```
Expected output:

```powershell
Value = Some("value")
Deleted = true
Value = None
Deleted = true
Value = Some("value")
Deleted = true
```




`FileStorage` complies with the Storage interface, but violates its implicit contracts:

- `.get()`: Vulnerable to path traversal
- `.set()`: Fails silently
- `.delete()`: Lies about the result

The client code (`demo()`) works with all implementations, but its assumptions are false with FileStorage.











### The Fix: Make Contracts Explicit

You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_lsp_04

// =========================
// Storage Backends - Fix
// =========================

use std::collections::HashMap;
use std::path::{Path, PathBuf};

// =========================
// Abstractions
// =========================

pub trait Storage {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn delete(&mut self, key: &str) -> bool;
}

// Simple Redis mock so the example compiles
pub struct RedisClient;

impl RedisClient {
    fn get(&self, _key: &str) -> Result<String, ()> {
        Err(())
    }
    fn set(&self, _key: &str, _value: &str) -> Result<(), ()> {
        Ok(())
    }
    fn del(&self, _key: &str) -> Result<(), ()> {
        Ok(())
    }
}

// =========================
// Concrete storages
// =========================

// In-memory backend
pub struct MemoryStorage {
    data: HashMap<String, String>,
}

impl MemoryStorage {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Storage for MemoryStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}

// Redis backend
pub struct RedisStorage {
    client: RedisClient,
}

impl RedisStorage {
    fn new() -> Self {
        Self {
            client: RedisClient,
        }
    }
}

impl Storage for RedisStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.client.get(key).ok()
    }

    fn set(&mut self, key: String, value: String) {
        self.client.set(&key, &value).ok();
    }

    fn delete(&mut self, key: &str) -> bool {
        self.client.del(key).is_ok()
    }
}

// =========================
// FIXED: LSP-compliant FileStorage
// =========================

pub struct FileStorage {
    base_path: String,
}

impl FileStorage {
    fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    // Prevent path traversal and invalid filenames
    fn validate_key(&self, key: &str) -> bool {
        !key.contains("..") && !key.contains('/') && !key.contains('\\') && key.len() <= 255
    }

    fn key_to_path(&self, key: &str) -> PathBuf {
        Path::new(&self.base_path).join(key)
    }
}

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        // Invalid keys behave like "not found"
        if !self.validate_key(key) {
            return None;
        }

        let path = self.key_to_path(key);
        // IO errors are mapped to None, just like missing keys
        std::fs::read_to_string(path).ok()
    }

    fn set(&mut self, key: String, value: String) {
        if !self.validate_key(&key) {
            return;
        }

        let path = self.key_to_path(&key);

        // Ensure failures are no longer silent
        if let Err(e) = std::fs::write(path, value) {
            eprintln!("FileStorage set failed: {}", e);
        }
    }

    fn delete(&mut self, key: &str) -> bool {
        if !self.validate_key(key) {
            return false;
        }

        let path = self.key_to_path(key);

        match std::fs::remove_file(path) {
            Ok(()) => true, // File really existed and was deleted
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(e) => {
                eprintln!("FileStorage delete failed: {}", e);
                false
            }
        }
    }
}

// =========================
// Usage
// =========================

fn demo(storage: &mut dyn Storage) {
    storage.set("key".into(), "value".into());
    println!("Value = {:?}", storage.get("key"));
    println!("Deleted = {}", storage.delete("key"));
}

fn main() {
    let mut mem = MemoryStorage::new();
    let mut redis = RedisStorage::new();
    let mut file = FileStorage::new(".");

    demo(&mut mem);
    demo(&mut redis);
    demo(&mut file);
}
```

Expected output:

```powershell
Value = Some("value")
Deleted = true
Value = None
Deleted = true
Value = Some("value")
Deleted = true
```



Let's make sure we are in sync. Above, we are running the same `demo()` function on three different implementations:

| Storage      | get("key")    | delete("key") | Why                              |
| ------------ | ------------- | ------------- | -------------------------------- |
| Memory       | Some("value") | true          | The key was stored in RAM        |
| Redis (mock) | None          | true          | `get()` always fails in the mock |
| File         | Some("value") | true          | File was written and deleted     |

Where:

* **RedisStorage** returns `None` because the mock client always fails on `get()`.
* That is **not** an LSP violation, it’s just a dummy implementation.
* The important part is that **no implementation lies or behaves inconsistently anymore**.


The Liskov Substitution Principle says: *"Any implementation of an interface must be usable **without breaking the expectations** of the code that depends on that interface."* In our case, the **implicit contract** of `Storage` is:

* `get()`
    * returns `Some(value)` if the key exists
    * returns `None` if it doesn’t
* `set()`
    * tries to store the value
* `delete()`
    * returns `true` **only if something was actually deleted**

The **old FileStorage** violated this:

| Method     | Problem                                        |
| ---------- | ---------------------------------------------- |
| `get()`    | Path traversal, invalid keys, OS errors        |
| `set()`    | Failed silently                                |
| `delete()` | Returned `true` even if the file never existed |

That means **client code could not trust the behavior**.


Now let's read again the comments of the fixed FileStorage because they highlight the **important part**,

```rust
impl FileStorage {
    fn new(base_path: &str) -> Self {
        Self { base_path: base_path.into() }
    }

    // Prevent path traversal and invalid filenames
    fn validate_key(&self, key: &str) -> bool {
        !key.contains("..")
            && !key.contains('/')
            && !key.contains('\\')
            && key.len() <= 255
    }

    fn key_to_path(&self, key: &str) -> PathBuf {
        Path::new(&self.base_path).join(key)
    }
}

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        // Invalid keys behave like "not found"
        if !self.validate_key(key) {
            return None;
        }

        let path = self.key_to_path(key);
        // IO errors are mapped to None, just like missing keys
        std::fs::read_to_string(path).ok()
    }

    fn set(&mut self, key: String, value: String) {
        if !self.validate_key(&key) {
            return;
        }

        let path = self.key_to_path(&key);

        // Ensure failures are no longer silent
        if let Err(e) = std::fs::write(path, value) {
            eprintln!("FileStorage set failed: {}", e);
        }
    }

    fn delete(&mut self, key: &str) -> bool {
        if !self.validate_key(key) {
            return false;
        }

        let path = self.key_to_path(key);

        match std::fs::remove_file(path) {
            Ok(()) => true, // File really existed and was deleted
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(e) => {
                eprintln!("FileStorage delete failed: {}", e);
                false
            }
        }
    }
}
```

This explain why this is now LSP-compliant. The **interface did not change**, but the **behavior now matches the contract**:

| Method     | Now guarantees                               |
| ---------- | -------------------------------------------- |
| `get()`    | Returns `None` for invalid or missing keys   |
| `set()`    | Tries to store, logs errors                  |
| `delete()` | Returns `true` only if something was deleted |

This means:

* The client code can rely on the return values
* No hidden side effects
* No misleading results
* No extra constraints on the caller

So FileStorage can now replace MemoryStorage or RedisStorage without breaking anything. That is exactly what LSP requires.

**Summary:**
* The original FileStorage violated LSP because it lied about its behavior and introduced hidden constraints.
* The fixed version respects the same interface but enforces the same observable behavior as the other implementations.


**Note:**

Our abstraction is still very weak because:

* `set()` cannot report failure
* `get()` hides IO errors
* `delete()` hides permission issues

So this is LSP-compliant, not robust but OK for a short demo code.





### Rust-Specific Notes

1. **Type system enforces LSP**: Rust's type system catches many LSP violations at compile time. If our trait method signature is `fn foo(&self) -> i32`, we can't accidentally return a `String`.

2. **Use Result for fallible operations**: We should not silently fail or panic. Instead let's make errors part of the contract via `Result<T, E>`.

3. **Trait bounds make contracts explicit**:
   ```rust
   pub trait Storage: Send + Sync {
       // Now callers know implementations are thread-safe
   }
   ```
4. **Don't overuse inheritance thinking**: Coming from OOP, we might force types into **"is-a"** relationships. In Rust, we should prefer **composition** (**has-a**) and focused traits.











### Rules of Thumb for LSP

1. **Preconditions cannot be strengthened**: If the trait accepts any string, implementations can't suddenly require non-empty strings
2. **Postconditions cannot be weakened**: If the trait promises to return a value, implementations can't return `None` in cases where the trait wouldn't
3. **Invariants must be preserved**: If the trait maintains some property, all implementations must maintain it
4. **No new exceptions**: In Rust, this means the error type in `Result<T, E>` should cover all failure modes












### When to Apply the Liskov Substitution Principle (LSP)?

Context: It is 8:20 AM. You replaced an implementation with another one. Tests start failing.

**The question to ask:** *"Can I replace this type with one of its subtypes without surprising the caller?"*

* If using a subtype forces the caller to add special cases, defensive checks, or different logic, LSP is likely violated.
* The Liskov Substitution Principle is not about inheritance syntax, but about **behavioral compatibility**.
* LSP is a thinking tool that helps us say: *"If I have to know the concrete type, then substitution is broken."*










## Next Step
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/022_solid/solid_00.md%}): Introduction + Single Responsibility Principle
* [Episode 01]({%link docs/06_programmation/rust/022_solid/solid_01.md%}): Open-Closed Principle
* [Episode 02]({%link docs/06_programmation/rust/022_solid/solid_02.md%}): Liskov Substitution Principle
* [Episode 03]({%link docs/06_programmation/rust/022_solid/solid_03.md%}): Interface Segregation Principle
* [Episode 04]({%link docs/06_programmation/rust/022_solid/solid_04.md%}): Dependency Inversion Principle + Conclusion
