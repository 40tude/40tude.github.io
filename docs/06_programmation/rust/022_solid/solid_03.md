---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide - 03"
description: "A gentle introduction to SOLID principles using Rust. The focus is on Interface Segregation Principle."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-18 18:00:00
---


# SOLID Principles in Rust: A Practical Guide
{: .no_toc }

A gentle introduction to SOLID principles using Rust. The focus is on Interface Segregation Principle.
{: .lead }




<!-- <h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2> -->







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 03
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
## Interface Segregation Principle (ISP)

### The Principle

> "No client should be forced to depend on methods it does not use."

In other words: don't create fat traits that do everything. Split them into focused, cohesive traits.














### The Problem: The God Trait

Imagine we're building a document management system:

```rust
pub trait Document {
    // Reading
    fn get_content(&self) -> &str;
    fn get_metadata(&self) -> &Metadata;
    fn search(&self, query: &str) -> Vec<usize>;

    // Writing
    fn set_content(&mut self, content: String);
    fn append(&mut self, text: &str);
    fn insert(&mut self, pos: usize, text: &str);

    // Formatting
    fn to_html(&self) -> String;
    fn to_markdown(&self) -> String;
    fn to_pdf(&self) -> Vec<u8>;

    // Versioning
    fn save_version(&mut self) -> Version;
    fn list_versions(&self) -> Vec<Version>;
    fn restore_version(&mut self, version: &Version);

    // Permissions
    fn can_read(&self, user: &User) -> bool;
    fn can_write(&self, user: &User) -> bool;
    fn share_with(&mut self, user: &User, permission: Permission);

    // Collaboration
    fn add_comment(&mut self, comment: Comment);
    fn list_comments(&self) -> &[Comment];
    fn notify_watchers(&self);
}
```

**What's wrong?** This trait is massive. Problems:

1. **A simple read-only viewer must implement all 20+ methods**: even though it only needs `get_content` and `get_metadata`
2. **A formatter that generates HTML/MD needs to implement versioning and permissions**
3. **Testing is a nightmare**: mock implementations must implement everything
4. **Changes ripple**: adding a new export format forces every implementation to change
5. **Binary bloat**: even if we only use reading, we pay for the whole trait in compile time and binary size


Just to make sure we realize the impact, let's create a read-only viewer. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_01_isp

// =========================
// God Trait Problem
// =========================

// =========================
// Abstractions
// =========================

mod domain {
    #[derive(Debug, Clone)]
    pub struct Metadata {
        pub title: String,
    }

    #[derive(Debug, Clone)]
    pub struct Version {
        pub id: u32,
    }

    #[derive(Debug)]
    pub struct User {
        pub name: String,
    }

    #[derive(Debug)]
    pub struct Comment {
        pub text: String,
    }

    #[derive(Debug)]
    pub enum Permission {
        Read,
        Write,
    }
}

mod document {
    use super::domain::*;

    // The "God Trait"
    pub trait Document {
        // Reading
        fn get_content(&self) -> &str;
        fn get_metadata(&self) -> &Metadata;
        fn search(&self, query: &str) -> Vec<usize>;

        // Writing
        fn set_content(&mut self, content: String);
        fn append(&mut self, text: &str);
        fn insert(&mut self, pos: usize, text: &str);

        // Formatting
        fn to_html(&self) -> String;
        fn to_markdown(&self) -> String;
        fn to_pdf(&self) -> Vec<u8>;

        // Versioning
        fn save_version(&mut self) -> Version;
        fn list_versions(&self) -> Vec<Version>;
        fn restore_version(&mut self, version: &Version);

        // Permissions
        fn can_read(&self, user: &User) -> bool;
        fn can_write(&self, user: &User) -> bool;
        fn share_with(&mut self, user: &User, permission: Permission);

        // Collaboration
        fn add_comment(&mut self, comment: Comment);
        fn list_comments(&self) -> &[Comment];
        fn notify_watchers(&self);
    }
}

// =========================
// Concrete read-only viewer
// =========================

mod viewer {
    use super::document::Document;
    use super::domain::*;

    pub struct ReadOnlyViewer {
        content: String,
        metadata: Metadata,
        comments: Vec<Comment>,
    }

    impl ReadOnlyViewer {
        pub fn new(content: String, title: String) -> Self {
            Self {
                content,
                metadata: Metadata { title },
                comments: Vec::new(),
            }
        }
    }

    // Forced to implement EVERYTHING, even useless methods
    impl Document for ReadOnlyViewer {
        // Reading (the only useful part)
        fn get_content(&self) -> &str {
            &self.content
        }

        fn get_metadata(&self) -> &Metadata {
            &self.metadata
        }

        fn search(&self, query: &str) -> Vec<usize> {
            self.content.match_indices(query).map(|(i, _)| i).collect()
        }

        // Writing (not supported)
        fn set_content(&mut self, _content: String) {
            panic!("Read-only viewer cannot modify content");
        }

        fn append(&mut self, _text: &str) {
            panic!("Read-only viewer cannot append text");
        }

        fn insert(&mut self, _pos: usize, _text: &str) {
            panic!("Read-only viewer cannot insert text");
        }

        // Formatting (not supported)
        fn to_html(&self) -> String {
            panic!("Read-only viewer cannot export to HTML");
        }

        fn to_markdown(&self) -> String {
            panic!("Read-only viewer cannot export to Markdown");
        }

        fn to_pdf(&self) -> Vec<u8> {
            panic!("Read-only viewer cannot export to PDF");
        }

        // Versioning (not supported)
        fn save_version(&mut self) -> Version {
            panic!("Read-only viewer does not support versioning");
        }

        fn list_versions(&self) -> Vec<Version> {
            Vec::new()
        }

        fn restore_version(&mut self, _version: &Version) {
            panic!("Read-only viewer cannot restore versions");
        }

        // Permissions (hardcoded)
        fn can_read(&self, _user: &User) -> bool {
            true
        }

        fn can_write(&self, _user: &User) -> bool {
            false
        }

        fn share_with(&mut self, _user: &User, _permission: Permission) {
            panic!("Read-only viewer cannot share documents");
        }

        // Collaboration (mostly useless)
        fn add_comment(&mut self, comment: Comment) {
            self.comments.push(comment);
        }

        fn list_comments(&self) -> &[Comment] {
            &self.comments
        }

        fn notify_watchers(&self) {
            // No-op for a simple viewer
        }
    }
}

// =========================
// Usage
// =========================

use document::Document;
use viewer::ReadOnlyViewer;

fn main() {
    let viewer = ReadOnlyViewer::new("Hello SOLID world!".to_string(), "ISP Example".to_string());

    println!("Title: {}", viewer.get_metadata().title);
    println!("Content: {}", viewer.get_content());
    println!("Search 'SOLID': {:?}", viewer.search("SOLID"));
}
```

As before, in the code above, the modules would live in separate files but here they are grouped together for Rust Playground convenience.

Expected output:

```powershell
Title: ISP Example
Content: Hello SOLID world!
Search 'SOLID': [6]
```







### The Solution: Role-Based Traits

Split the god trait into focused interfaces:

```rust
// Core reading operations
pub trait Readable {
    fn get_content(&self) -> &str;
    fn get_metadata(&self) -> &Metadata;
}

// Full-text search
pub trait Searchable {
    fn search(&self, query: &str) -> Vec<usize>;
}

// Editing operations
pub trait Writable {
    fn set_content(&mut self, content: String);
    fn append(&mut self, text: &str);
    fn insert(&mut self, pos: usize, text: &str);
}

// Export formats
pub trait HtmlExportable {
    fn to_html(&self) -> String;
}

pub trait MarkdownExportable {
    fn to_markdown(&self) -> String;
}

pub trait PdfExportable {
    fn to_pdf(&self) -> Vec<u8>;
}

// Version control
pub trait Versionable {
    fn save_version(&mut self) -> Version;
    fn list_versions(&self) -> Vec<Version>;
    fn restore_version(&mut self, version: &Version);
}

// Access control
pub trait Permissioned {
    fn can_read(&self, user: &User) -> bool;
    fn can_write(&self, user: &User) -> bool;
    fn share_with(&mut self, user: &User, permission: Permission);
}

// Collaboration features
pub trait Commentable {
    fn add_comment(&mut self, comment: Comment);
    fn list_comments(&self) -> &[Comment];
}

pub trait Watchable {
    fn notify_watchers(&self);
}
```

Now each component depends only on what it needs:

```rust
// A simple viewer only needs this
fn display_document(doc: &impl Readable) {
    println!("{}", doc.get_content());
}

// An HTML exporter needs two traits
fn export_to_html(doc: &(impl Readable + HtmlExportable)) -> String {
    let html = doc.to_html();
    // Add metadata
    format!(
        "<meta>{}</meta>\n{}",
        doc.get_metadata().title,
        html
    )
}

// A full editor needs more
fn edit_document(doc: &mut (impl Readable + Writable + Versionable)) {
    let backup = doc.save_version();
    doc.append("\n\nNew paragraph");
    // If something fails, we can restore
}

// Types can implement just what they support
pub struct TextDocument {
    content: String,
    metadata: Metadata,
}

impl Readable for TextDocument {
    fn get_content(&self) -> &str { &self.content }
    fn get_metadata(&self) -> &Metadata { &self.metadata }
}

impl Writable for TextDocument {
    fn set_content(&mut self, content: String) {
        self.content = content;
    }
    fn append(&mut self, text: &str) {
        self.content.push_str(text);
    }
    fn insert(&mut self, pos: usize, text: &str) {
        self.content.insert_str(pos, text);
    }
}

impl MarkdownExportable for TextDocument {
    fn to_markdown(&self) -> String {
        self.content.clone() // Already markdown
    }
}

// A read-only archive document doesn't need Writable
pub struct ArchiveDocument {
    content: String,
    metadata: Metadata,
}

impl Readable for ArchiveDocument {
    fn get_content(&self) -> &str { &self.content }
    fn get_metadata(&self) -> &Metadata { &self.metadata }
}

// No Writable implementation - the type system prevents misuse!
```



The impact is important. Let's create a viewer. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_02_isp

// =========================
// God Trait Fix
// =========================

// =========================
// Abstractions
// =========================

mod domain {
    #[derive(Debug, Clone)]
    pub struct Metadata {
        pub title: String,
    }
}

mod traits {
    use super::domain::Metadata;

    // Core reading operations
    pub trait Readable {
        fn get_content(&self) -> &str;
        fn get_metadata(&self) -> &Metadata;
    }

    // Full-text search
    pub trait Searchable {
        fn search(&self, query: &str) -> Vec<usize>;
    }
}

// =========================
// Concrete read-only viewer
// =========================

mod viewer {
    use super::domain::Metadata;
    use super::traits::{Readable, Searchable};

    // A simple read-only viewer
    pub struct ReadOnlyViewer {
        content: String,
        metadata: Metadata,
    }

    impl ReadOnlyViewer {
        pub fn new(content: String, title: String) -> Self {
            Self {
                content,
                metadata: Metadata { title },
            }
        }
    }

    impl Readable for ReadOnlyViewer {
        fn get_content(&self) -> &str {
            &self.content
        }

        fn get_metadata(&self) -> &Metadata {
            &self.metadata
        }
    }

    impl Searchable for ReadOnlyViewer {
        fn search(&self, query: &str) -> Vec<usize> {
            self.content.match_indices(query).map(|(i, _)| i).collect()
        }
    }
}

// =========================
// Usage
// =========================

use traits::{Readable, Searchable};
use viewer::ReadOnlyViewer;

fn main() {
    let viewer = ReadOnlyViewer::new("Hello SOLID world!".to_string(), "ISP Example".to_string());

    println!("Title: {}", viewer.get_metadata().title);
    println!("Content: {}", viewer.get_content());
    println!("Search 'SOLID': {:?}", viewer.search("SOLID"));
}
```

Expected output:
```powershell
Title: ISP Example
Content: Hello SOLID world!
Search 'SOLID': [6]
```

This shows:
* No unnecessary methods
* No panic!()
* The type clearly expresses its capabilities
* The compiler prevents any writing


Now let's create an archiver. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_03_isp

// =========================
// God Trait Fix
// =========================

// =========================
// Abstractions
// =========================

mod domain {
    #[derive(Debug, Clone)]
    pub struct Metadata {
        pub title: String,
    }
}

mod traits {
    use super::domain::Metadata;

    // Core reading operations
    pub trait Readable {
        fn get_content(&self) -> &str;
        fn get_metadata(&self) -> &Metadata;
    }
}

// =========================
// Concrete read-only archiver
// =========================

mod archive {
    use super::domain::Metadata;
    use super::traits::Readable;

    // A read-only archive document
    pub struct ArchiveDocument {
        content: String,
        metadata: Metadata,
    }

    impl ArchiveDocument {
        pub fn new(content: String, title: String) -> Self {
            Self {
                content,
                metadata: Metadata { title },
            }
        }
    }

    impl Readable for ArchiveDocument {
        fn get_content(&self) -> &str {
            &self.content
        }

        fn get_metadata(&self) -> &Metadata {
            &self.metadata
        }
    }
}

// =========================
// Usage
// =========================

use archive::ArchiveDocument;
use traits::Readable;

fn main() {
    let archive = ArchiveDocument::new(
        "This is a historical document.".to_string(),
        "Company Archive 1998".to_string(),
    );

    println!("Title: {}", archive.get_metadata().title);
    println!("Content: {}", archive.get_content());
}
```
Expected output:

```powershell
Title: Company Archive 1998
Content: This is a historical document.
```

This shows:
* The archive cannot be modified
* It does not require searching, exporting, or versioning
* The type is simple, clear, and consistent with its role
















### Real-World Example: Database Connections

First let's start with the bad guy: One size fits all

```rust
pub trait Connection {
    fn execute(&mut self, sql: &str) -> Result<u64>;
    fn query(&mut self, sql: &str) -> Result<ResultSet>;
    fn prepare(&mut self, sql: &str) -> Result<Statement>;
    fn begin_transaction(&mut self) -> Result<Transaction>;
    fn close(self) -> Result<()>;
    fn ping(&self) -> bool;
    fn get_server_version(&self) -> String;
}
```

Now, here is how a good set of traits could be written:

```rust
pub trait Queryable {
    fn query(&mut self, sql: &str) -> Result<ResultSet>;
}

pub trait Executable {
    fn execute(&mut self, sql: &str) -> Result<u64>;
}

pub trait Preparable {
    fn prepare(&mut self, sql: &str) -> Result<Statement>;
}

pub trait Transactional {
    fn begin_transaction(&mut self) -> Result<Transaction>;
}

pub trait ConnectionInfo {
    fn ping(&self) -> bool;
    fn get_server_version(&self) -> String;
}

// Now we can write code that only needs specific capabilities
fn count_users(conn: &mut impl Queryable) -> Result<i64> {
    let result = conn.query("SELECT COUNT(*) FROM users")?;
    // Process result
    Ok(0)
}

// Read-only connections don't need Execute or Transaction
pub struct ReadOnlyConnection {
    // ...
}

impl Queryable for ReadOnlyConnection {
    fn query(&mut self, sql: &str) -> Result<ResultSet> {
        // Implementation
        todo!()
    }
}

impl Preparable for ReadOnlyConnection {
    fn prepare(&mut self, sql: &str) -> Result<Statement> {
        // Implementation
        todo!()
    }
}

// No Execute or Transactional traits - the compiler prevents misuse!
```

I don't show working code here because this is really similar to what we just dit with the last code.










### Combining Traits

When we need multiple capabilities, Rust makes it easy:

1. We can require multiple traits
    ```rust
    fn backup_data(conn: &mut (impl Queryable + Transactional)) -> Result<()> {
        let tx = conn.begin_transaction()?;
        let data = conn.query("SELECT * FROM important_table")?;
        // Save data...
        tx.commit()
    }
    ```

2. Or we can use trait bounds
    ```rust
    fn replicate<C>(source: &mut C, dest: &mut C) -> Result<()>
    where
        C: Queryable + Executable,
    {
        let data = source.query("SELECT * FROM table")?;
        for row in data {
            dest.execute(&format!("INSERT INTO table VALUES ({})", row))?;
        }
        Ok(())
    }
    ```


Let's see in first example how it works when a connection must be queryable + transactional in order to perform a backup. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_04_isp

// =========================
// Combine Traits - Require multiple traits
// =========================

// =========================
// Abstractions
// =========================

mod traits {
    // Allows querying data
    pub trait Queryable {
        fn query(&self, sql: &str) -> Result<Vec<String>, String>;
    }

    // Allows transaction handling
    pub trait Transactional {
        fn begin_transaction(&mut self) -> Result<Transaction, String>;
    }

    pub struct Transaction;

    impl Transaction {
        pub fn commit(self) -> Result<(), String> {
            println!("Transaction committed");
            Ok(())
        }
    }
}

// =========================
// Concrete database
// =========================

mod database {
    use super::traits::{Queryable, Transaction, Transactional};

    // A database connection supporting both querying and transactions
    pub struct DatabaseConnection;

    impl Queryable for DatabaseConnection {
        fn query(&self, sql: &str) -> Result<Vec<String>, String> {
            println!("Running query: {}", sql);
            Ok(vec!["row1".to_string(), "row2".to_string()])
        }
    }

    impl Transactional for DatabaseConnection {
        fn begin_transaction(&mut self) -> Result<Transaction, String> {
            println!("Transaction started");
            Ok(Transaction)
        }
    }
}

// =========================
// Usage
// =========================

use database::DatabaseConnection;
use traits::{Queryable, Transactional};

// Requires multiple traits
fn backup_data(conn: &mut (impl Queryable + Transactional)) -> Result<(), String> {
    let tx = conn.begin_transaction()?;
    let data = conn.query("SELECT * FROM important_table")?;

    for row in data {
        println!("Backing up: {}", row);
    }

    tx.commit()
}

fn main() -> Result<(), String> {
    let mut db = DatabaseConnection;
    backup_data(&mut db)?;
    Ok(())
}
```

Expected output:

```powershell
Transaction started
Running query: SELECT * FROM important_table
Backing up: row1
Backing up: row2
Transaction committed
```


This demonstrates that:
* A function may require multiple capabilities
* The actual type does not matter
* Only behaviors matter




Now, let's see how to combine traits with generic trait bounds. Here we want to replicate data between to systems. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):


```rust
// cargo run -p ex_05_isp

// =========================
// Combine Traits - Use Trait Bounds
// =========================

// =========================
// Abstractions
// =========================
mod traits {
    // Allows reading data
    pub trait Queryable {
        fn query(&self, sql: &str) -> Result<Vec<String>, String>;
    }

    // Allows executing commands
    pub trait Executable {
        fn execute(&mut self, command: &str) -> Result<(), String>;
    }
}

// =========================
// Concrete MemoryStorage
// =========================

mod storage {
    use super::traits::{Executable, Queryable};

    // A simple in-memory storage
    pub struct MemoryStorage {
        pub data: Vec<String>,
    }

    impl Queryable for MemoryStorage {
        fn query(&self, _sql: &str) -> Result<Vec<String>, String> {
            Ok(self.data.clone())
        }
    }

    impl Executable for MemoryStorage {
        fn execute(&mut self, command: &str) -> Result<(), String> {
            println!("Executing: {}", command);
            self.data.push(command.to_string());
            Ok(())
        }
    }
}

// =========================
// Usage
// =========================

use storage::MemoryStorage;
use traits::{Executable, Queryable};

// Uses generic trait bounds
fn replicate<C>(source: &mut C, dest: &mut C) -> Result<(), String>
where
    C: Queryable + Executable,
{
    let data = source.query("SELECT * FROM table")?;

    for row in data {
        let cmd = format!("INSERT INTO table VALUES ({})", row);
        dest.execute(&cmd)?;
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let mut source = MemoryStorage {
        data: vec!["Alice".into(), "Bob".into()],
    };

    let mut dest = MemoryStorage { data: Vec::new() };

    replicate(&mut source, &mut dest)?;

    println!("Destination data: {:?}", dest.data);
    Ok(())
}
```



Expected output:

```powershell
Executing: INSERT INTO table VALUES (Alice)
Executing: INSERT INTO table VALUES (Bob)
Destination data: ["INSERT INTO table VALUES (Alice)", "INSERT INTO table VALUES (Bob)"]
```


What this shows:
* The types are generic
* The constraints are explicit
* The function is reusable and safe





### Rust-Specific Notes

1. **Trait composition is zero-cost**: When we write `impl Readable + Writable`, there's no runtime overhead. It's just compile-time checking.

2. **Blanket implementations**: we can provide default implementations for trait combinations:
   ```rust
   // Any type that's Readable and Writable gets a free copy operation
   impl<T: Readable + Writable> Copyable for T {
       fn copy_to(&self, dest: &mut T) {
           dest.set_content(self.get_content().to_string());
       }
   }
   ```

3. **Sealed traits pattern**: If we want fine-grained control over who can implement traits:
   ```rust
   mod sealed {
       pub trait Sealed {}
   }

   pub trait Readable: sealed::Sealed {
       fn get_content(&self) -> &str;
   }
   ```

4. **Auto traits**: Rust has special marker traits like `Send`, `Sync`, `Copy`. These are automatically implemented when applicable, which is perfect ISP - we get the interface only when it makes sense.






### When to Split Traits

Ask ourself:
- Do all implementors support all methods?
- Could a client need only a subset of the functionality?
- Would testing be easier with smaller interfaces?
- Do different methods serve different use cases/actors?

If yes to any of these, consider splitting the trait.








### When to Apply the Interface Segregation Principle (ISP)?

Context: It is 7:50 AM. The office is empty and the coffee is damn hot. You review the interface implemented yesterday and immediately you feel uncomfortable.

**The question to ask:** *"Am I forced to depend on methods I do not use?"*

* If an interface requires a client to implement or know methods that are irrelevant to its use case, the interface is **too broad**.
* The Interface Segregation Principle is not about creating many interfaces for the sake of it, but about **avoiding forcing clients to depend on things they don’t use**.
* ISP is a thinking tool that helps us say: *"This interface is making me implement things I don’t care about."*








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!--






// =========================
// editor_api module
// =========================
mod editor_api {
    // Public shared state
    pub struct EditorContext {
        pub content: String,
    }

    // Public plugin contract
    pub trait Plugin {
        fn name(&self) -> &str;
        fn execute(&mut self, context: &mut EditorContext);
    }
}

// =========================
// git_plugin module
// =========================
mod git_plugin {
    use crate::editor_api::{EditorContext, Plugin};

    pub struct GitPlugin;

    impl Plugin for GitPlugin {
        fn name(&self) -> &str {
            "Git Integration"
        }

        fn execute(&mut self, context: &mut EditorContext) {
            // Fake git behavior
            context.content.push_str("\n[Git status clean]");
        }
    }
}

// =========================
// spellcheck_plugin module
// =========================
mod spellcheck_plugin {
    use crate::editor_api::{EditorContext, Plugin};

    pub struct SpellCheckPlugin;

    impl Plugin for SpellCheckPlugin {
        fn name(&self) -> &str {
            "Spell Checker"
        }

        fn execute(&mut self, context: &mut EditorContext) {
            // Fake spell check behavior
            context.content.push_str("\n[Spell check OK]");
        }
    }
}


// =========================
// application (composition root)
// =========================
use editor_api::{EditorContext, Plugin};

// Editor lives at the application boundary
struct Editor {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Editor {
    fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    fn run_plugins(&mut self, context: &mut EditorContext) {
        for plugin in &mut self.plugins {
            println!("Running plugin: {}", plugin.name());
            plugin.execute(context);
        }
    }
}

fn main() {
    let mut editor = Editor::new();

    // Plugins are completely independent
    editor.register_plugin(Box::new(git_plugin::GitPlugin));
    editor.register_plugin(Box::new(spellcheck_plugin::SpellCheckPlugin));

    let mut context = EditorContext {
        content: String::from("Hello world"),
    };

    editor.run_plugins(&mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
-->


## Next Step
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/022_solid/solid_00.md%}): Introduction + Single Responsibility Principle
* [Episode 01]({%link docs/06_programmation/rust/022_solid/solid_01.md%}): Open-Closed Principle
* [Episode 02]({%link docs/06_programmation/rust/022_solid/solid_02.md%}): Liskov Substitution Principle
* [Episode 03]({%link docs/06_programmation/rust/022_solid/solid_03.md%}): Interface Segregation Principle
* [Episode 04]({%link docs/06_programmation/rust/022_solid/solid_04.md%}): Dependency Inversion Principle + Conclusion
