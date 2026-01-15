---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide - 03"
description: "A gentle introduction to SOLID principles using Rust. Focus is on interface Segregation Principle."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-15 11:00:00
---


# SOLID Principles in Rust: A Practical Guide
{: .no_toc }

A gentle introduction to SOLID principles using Rust. Focus is on Interface Segregation Principle.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>







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

1. **A simple read-only viewer must implement all 20+ methods** (even though it only needs `get_content` and `get_metadata`)
2. **A formatter that generates HTML/MD needs to implement versioning and permissions**
3. **Testing is a nightmare** - mock implementations must implement everything
4. **Changes ripple** - adding a new export format forces every implementation to change
5. **Binary bloat** - even if we only use reading, we pay for the whole trait in compile time and binary size












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










### Real-World Example: Database Connections

```rust
// BAD: One size fits all
pub trait Connection {
    fn execute(&mut self, sql: &str) -> Result<u64>;
    fn query(&mut self, sql: &str) -> Result<ResultSet>;
    fn prepare(&mut self, sql: &str) -> Result<Statement>;
    fn begin_transaction(&mut self) -> Result<Transaction>;
    fn close(self) -> Result<()>;
    fn ping(&self) -> bool;
    fn get_server_version(&self) -> String;
}

// GOOD: Focused traits
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











### Combining Traits

When we need multiple capabilities, Rust makes it easy:

```rust
// Require multiple traits
fn backup_data(conn: &mut (impl Queryable + Transactional)) -> Result<()> {
    let tx = conn.begin_transaction()?;
    let data = conn.query("SELECT * FROM important_table")?;
    // Save data...
    tx.commit()
}

// Or use trait bounds
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

Context: It is 7:50 AM. The office is empty and the coffee damn hot. You review the interface implemented yesterday and immediately you feel uncomfortable.

**The question to ask:** *"Am I forced to depend on methods I do not use?"*

* If an interface requires a client to implement or know methods that are irrelevant to its use case, the interface is **too broad**.
* The Interface Segregation Principle favors **small, role-focused interfaces** over large, generic ones.
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
