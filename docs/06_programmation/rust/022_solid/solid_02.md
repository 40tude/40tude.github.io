---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide - 02"
description: "A gentle introduction to SOLID principles using Rust. Here we focus is on Liskov Substitution Principle."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-15 11:00:00
---


# SOLID Principles in Rust: A Practical Guide
{: .no_toc }

A gentle introduction to SOLID principles using Rust. Here we focus is on Liskov Substitution Principle.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>










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

Classic example from OOP - the Rectangle/Square problem:

```rust
pub trait Shape {
    fn set_width(&mut self, width: f64);
    fn set_height(&mut self, height: f64);
    fn area(&self) -> f64;
}

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

// This function expects Shape behavior
fn process_shape(shape: &mut dyn Shape) {
    shape.set_width(5.0);
    shape.set_height(10.0);
    let area = shape.area();

    // We expect: width=5, height=10, area=50
    // With Rectangle: CORRECT (5 * 10 = 50)
    // With Square: WRONG! (10 * 10 = 100)
    // The last set_height overwrote the width!

    println!("Expected area: 50, Got: {}", area);
}
```

**The violation**: `Square` doesn't truly substitute for `Shape`. The caller expects setting width and height independently, but `Square` violates that expectation.














### The Solution: Better Abstractions

Don't force types into hierarchies they don't belong in. Model what they actually are:

```rust
// Immutable shapes with clear contracts
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

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

// Now this works correctly for ANY Shape
fn print_shape_info(shape: &dyn Shape) {
    println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
}
```

No mutation, no violated expectations. Each shape upholds the `Shape` contract.
















### Real-World Example: Storage Backends

Let's say we're building a key-value store with multiple backends:

```rust
use std::collections::HashMap;

pub trait Storage {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: String, value: String);
    fn delete(&mut self, key: &str) -> bool;
}

// In-memory backend
pub struct MemoryStorage {
    data: HashMap<String, String>,
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

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        // What if key contains "../"? Path traversal vulnerability!
        // What if key is too long? Filename limit exceeded!
        std::fs::read_to_string(format!("{}/{}", self.base_path, key)).ok()
    }

    fn set(&mut self, key: String, value: String) {
        // Fails silently if disk is full - violates caller expectations!
        std::fs::write(format!("{}/{}", self.base_path, key), value).ok();
    }

    fn delete(&mut self, key: &str) -> bool {
        // Returns true even if file didn't exist - lies to caller!
        std::fs::remove_file(format!("{}/{}", self.base_path, key)).is_ok()
    }
}
```

The `FileStorage` violates LSP in multiple ways:
- Key constraints differ from other implementations (no "../", length limits)
- Error handling is inconsistent (silent failures)
- Return values don't match semantics (delete returns true for non-existent files)















### The Fix: Make Contracts Explicit

```rust
use std::path::{Path, PathBuf};

pub enum StorageError {
    InvalidKey,
    IoError(std::io::Error),
    ConnectionError,
}

pub trait Storage {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    fn set(&mut self, key: String, value: String) -> Result<(), StorageError>;
    fn delete(&mut self, key: &str) -> Result<bool, StorageError>; // true if existed
}

// Now FileStorage can properly handle errors
impl Storage for FileStorage {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        self.validate_key(key)?;

        let path = self.key_to_path(key);
        match std::fs::read_to_string(&path) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(StorageError::IoError(e)),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), StorageError> {
        self.validate_key(&key)?;

        let path = self.key_to_path(&key);
        std::fs::write(&path, value)
            .map_err(|e| StorageError::IoError(e))
    }

    fn delete(&mut self, key: &str) -> Result<bool, StorageError> {
        self.validate_key(key)?;

        let path = self.key_to_path(key);
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(StorageError::IoError(e)),
        }
    }
}

impl FileStorage {
    fn validate_key(&self, key: &str) -> Result<(), StorageError> {
        if key.contains("..") || key.contains('/') || key.contains('\\') {
            return Err(StorageError::InvalidKey);
        }
        if key.len() > 255 {
            return Err(StorageError::InvalidKey);
        }
        Ok(())
    }

    fn key_to_path(&self, key: &str) -> PathBuf {
        Path::new(&self.base_path).join(key)
    }
}
```

Now all implementations have the same contract:
- Errors are explicit and handled consistently
- Return values have clear semantics
- Callers can substitute any `Storage` without surprises











### Rust-Specific Notes

1. **Type system enforces LSP**: Unlike dynamic languages, Rust's type system catches many LSP violations at compile time. If our trait method signature is `fn foo(&self) -> i32`, we can't accidentally return a `String`.

2. **Use Result for fallible operations**: Don't silently fail or panic. Make errors part of the contract via `Result<T, E>`.

3. **Trait bounds make contracts explicit**:
   ```rust
   pub trait Storage: Send + Sync {
       // Now callers know implementations are thread-safe
   }
   ```

4. **Don't overuse inheritance thinking**: Coming from OOP, we might force types into "is-a" relationships. In Rust, prefer composition and focused traits.











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
