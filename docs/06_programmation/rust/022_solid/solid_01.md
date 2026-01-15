---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide - 01"
description: "A gentle introduction to SOLID principles using Rust."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-15 11:00:00
---


# SOLID Principles in Rust: A Practical Guide
{: .no_toc }

A gentle introduction to SOLID principles using Rust.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 01
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
## Open-Closed Principle (OCP)



### The Principle

> "Software entities should be open for extension but closed for modification."

In other words: when requirements change, we should be able to add new behavior **without changing existing code**.

This sounds like black magic, but it's actually straightforward: **depend on abstractions (traits), not concretions**. When we need new behavior, implement a new type that satisfies the trait.










### The Problem: Modification Hell

Let's build a report generator that can output different formats:

```rust
// cargo run -p ex_01_ocp

// This enum defines all supported report formats
// Adding a new format requires modifying this enum
pub enum ReportFormat {
    Text,
    Html,
    Pdf,
}

// This struct represents the report data
pub struct Report {
    title: String,
    data: Vec<String>,
}

impl Report {
    // This method violates the Open-Closed Principle
    // Each new format requires modifying this method
    pub fn generate(&self, format: ReportFormat) -> String {
        match format {
            ReportFormat::Text => {
                let mut output = format!("=== {} ===\n", self.title);
                for item in &self.data {
                    output.push_str(&format!("- {}\n", item));
                }
                output
            }
            ReportFormat::Html => {
                let mut output = format!("<h1>{}</h1>\n<ul>\n", self.title);
                for item in &self.data {
                    output.push_str(&format!("  <li>{}</li>\n", item));
                }
                output.push_str("</ul>");
                output
            }
            ReportFormat::Pdf => {
                // Fake PDF generation
                format!("PDF: {} [binary data]", self.title)
            }
        }
    }
}

fn main() {
    // Sample report data
    let report = Report {
        title: String::from("Monthly Sales"),
        data: vec![
            String::from("Product A: 120 units"),
            String::from("Product B: 98 units"),
            String::from("Product C: 143 units"),
        ],
    };

    // Generate reports in different formats
    let text_report = report.generate(ReportFormat::Text);
    let html_report = report.generate(ReportFormat::Html);
    let pdf_report = report.generate(ReportFormat::Pdf);

    println!("--- TEXT REPORT ---\n{}", text_report);
    println!("--- HTML REPORT ---\n{}", html_report);
    println!("--- PDF REPORT ---\n{}", pdf_report);
}
```

Expected output

```powershell
--- TEXT REPORT ---
=== Monthly Sales ===
- Product A: 120 units
- Product B: 98 units
- Product C: 143 units

--- HTML REPORT ---
<h1>Monthly Sales</h1>
<ul>
  <li>Product A: 120 units</li>
  <li>Product B: 98 units</li>
  <li>Product C: 143 units</li>
</ul>
--- PDF REPORT ---
PDF: Monthly Sales [binary data]
```

**What happens when we need XML output?** We have to:
1. Add `Xml` to the enum
2. Modify the `generate()` method
3. Recompile everything
4. Risk breaking existing formats
5. Every developer working on reports has merge conflicts

**This violates Open-Closed Principle** because adding a new format requires modifying existing code.













### The Solution: Trait-Based Extension

```rust
// cargo run -p ex_02_ocp

// =========================
// Abstractions
// =========================

// The report doesn't know about specific formats
pub struct Report {
    title: String,
    data: Vec<String>,
}

// However, the report has a generate() method which calls the .format() method of the formatter
// The call will be resolve at runtime (via a vtable)
impl Report {
    pub fn generate(&self, formatter: &dyn ReportFormatter) -> String {
        formatter.format(&self.title, &self.data)
    }
}

// If a type wants to have the ReportFormatter trait it must implement the format method
pub trait ReportFormatter {
    fn format(&self, title: &str, data: &[String]) -> String;
}

// =========================
// Concrete formatters
// =========================

// Plain text output (same behavior as before)
pub struct TextFormatter;

impl ReportFormatter for TextFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = format!("=== {} ===\n", title);
        for item in data {
            output.push_str(&format!("- {}\n", item));
        }
        output
    }
}

// HTML output (same structure as initial example)
pub struct HtmlFormatter;

impl ReportFormatter for HtmlFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = format!("<h1>{}</h1>\n<ul>\n", title);
        for item in data {
            output.push_str(&format!("  <li>{}</li>\n", item));
        }
        output.push_str("</ul>");
        output
    }
}

// Fake PDF output (same spirit as before)
pub struct PdfFormatter;

impl ReportFormatter for PdfFormatter {
    fn format(&self, title: &str, _data: &[String]) -> String {
        format!("PDF: {} [binary data]", title)
    }
}

// New XML output - extension without modification
pub struct XmlFormatter;

impl ReportFormatter for XmlFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = String::from("<report>\n");
        output.push_str(&format!("  <title>{}</title>\n", title));
        output.push_str("  <items>\n");

        for item in data {
            output.push_str(&format!("    <item>{}</item>\n", item));
        }

        output.push_str("  </items>\n</report>");
        output
    }
}

// =========================
// Usage
// =========================

fn main() {
    let report = Report {
        title: "Monthly Sales".to_string(),
        data: vec![
            "Product A: 120 units".to_string(),
            "Product B: 98 units".to_string(),
            "Product C: 143 units".to_string(),
        ],
    };

    println!("--- TEXT ---\n{}", report.generate(&TextFormatter));
    println!("--- HTML ---\n{}", report.generate(&HtmlFormatter));
    println!("--- PDF ---\n{}", report.generate(&PdfFormatter));
    println!("--- XML ---\n{}", report.generate(&XmlFormatter));
}
```
Expected output

```powershell
--- TEXT ---
=== Monthly Sales ===
- Product A: 120 units
- Product B: 98 units
- Product C: 143 units

--- HTML ---
<h1>Monthly Sales</h1>
<ul>
  <li>Product A: 120 units</li>
  <li>Product B: 98 units</li>
  <li>Product C: 143 units</li>
</ul>
--- PDF ---
PDF: Monthly Sales [binary data]
--- XML ---
<report>
  <title>Monthly Sales</title>
  <items>
    <item>Product A: 120 units</item>
    <item>Product B: 98 units</item>
    <item>Product C: 143 units</item>
  </items>
</report>
```

Now adding XML (or JSON, or Markdown, or whatever) requires **zero changes** to `Report` or existing formatters. we just add a new type.
















### Taking It Further: Static Dispatch


If we want to avoid dynamic dispatch overhead we could use a generic version of the `generate()` method.

```rust
impl Report {
    pub fn generate<F: ReportFormatter>(&self, formatter: &F) -> String {
        formatter.format(&self.title, &self.data)
    }
}
```

With the generic version, the compiler monomorphizes `generate()`. The compiler generates a specialized version for each `formatter` type **at compile time**. This removes the vtable indirection at the cost of potentially larger binaries.


```rust
// cargo run -p ex_03_ocp

// =========================
// Abstractions
// =========================

// The report doesn't know about specific formats
pub struct Report {
    title: String,
    data: Vec<String>,
}

// However, the report has a generate method which calls the .format() method of the formatter
// The call will be resolve at compile time
impl Report {
    // Generic version using static dispatch
    pub fn generate<F: ReportFormatter>(&self, formatter: &F) -> String {
        formatter.format(&self.title, &self.data)
    }
}

// If a type wants to have the ReportFormatter trait it must implement the format method
pub trait ReportFormatter {
    fn format(&self, title: &str, data: &[String]) -> String;
}

// =========================
// Concrete formatters
// =========================

// Plain text output
pub struct TextFormatter;

impl ReportFormatter for TextFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = format!("=== {} ===\n", title);
        for item in data {
            output.push_str(&format!("- {}\n", item));
        }
        output
    }
}

// HTML output
pub struct HtmlFormatter;

impl ReportFormatter for HtmlFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = format!("<h1>{}</h1>\n<ul>\n", title);
        for item in data {
            output.push_str(&format!("  <li>{}</li>\n", item));
        }
        output.push_str("</ul>");
        output
    }
}

// Fake PDF output
pub struct PdfFormatter;

impl ReportFormatter for PdfFormatter {
    fn format(&self, title: &str, _data: &[String]) -> String {
        format!("PDF: {} [binary data]", title)
    }
}

// XML output
pub struct XmlFormatter;

impl ReportFormatter for XmlFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = String::from("<report>\n");
        output.push_str(&format!("  <title>{}</title>\n", title));
        output.push_str("  <items>\n");

        for item in data {
            output.push_str(&format!("    <item>{}</item>\n", item));
        }

        output.push_str("  </items>\n</report>");
        output
    }
}

// =========================
// Usage
// =========================

fn main() {
    let report = Report {
        title: "Monthly Sales".to_string(),
        data: vec![
            "Product A: 120 units".to_string(),
            "Product B: 98 units".to_string(),
            "Product C: 143 units".to_string(),
        ],
    };

    println!("--- TEXT ---\n{}", report.generate(&TextFormatter));
    println!("--- HTML ---\n{}", report.generate(&HtmlFormatter));
    println!("--- PDF ---\n{}", report.generate(&PdfFormatter));
    println!("--- XML ---\n{}", report.generate(&XmlFormatter));
}
```

Expected output
```powershell
--- TEXT ---
=== Monthly Sales ===
- Product A: 120 units
- Product B: 98 units
- Product C: 143 units

--- HTML ---
<h1>Monthly Sales</h1>
<ul>
  <li>Product A: 120 units</li>
  <li>Product B: 98 units</li>
  <li>Product C: 143 units</li>
</ul>
--- PDF ---
PDF: Monthly Sales [binary data]
--- XML ---
<report>
  <title>Monthly Sales</title>
  <items>
    <item>Product A: 120 units</item>
    <item>Product B: 98 units</item>
    <item>Product C: 143 units</item>
  </items>
</report>
```


Here too, OCP is respected: `XmlFormatter` is added without modifying Report.

We have two valid variants:
* `&dyn ReportFormatter` → maximum flexibility (runtime)
* `F: ReportFormatter` → maximum performance (compile-time)

The choice of dispatch becomes an implementation detail, not an architectural change.























### Real-World Example: Plugin System


If we do NOT have plugins we could write something similar to the previous traits based solution. The code is open for extension, we can add "plugins" (SpellCheck, Git...) but as before the number of tools is fixed, everything must be known at compile time. In the previous code, in the `main()` function we had a set `report.generate()` function calls. Here, in the `editor.run()` we have a set of of Check this out:

```rust
// cargo run -p ex_04_ocp

// =========================
// Static dispatch example
// =========================

// =========================
// Abstractions
// =========================

// Editor with static dispatch
pub struct Editor;

impl Editor {
    pub fn run<T1: Tool, T2: Tool>(
        &self,
        tool1: &T1,
        tool2: &T2,
        context: &mut EditorContent,
    ) {
        tool1.apply(context);
        tool2.apply(context);
    }
}

// Here the content of the Editor is just a String
pub struct EditorContent {
    pub content: String,
}

// If a type wants to have the Tool trait it must implement the apply method
pub trait Tool {
    fn apply(&self, context: &mut EditorContent);
}

// =========================
// Concrete tools
// =========================

// Spell checker
pub struct SpellCheck;

impl Tool for SpellCheck {
    fn apply(&self, context: &mut EditorContent) {
        context.content.push_str("\n[Spell check OK]");
    }
}

// Git integration
pub struct Git;

impl Tool for Git {
    fn apply(&self, context: &mut EditorContent) {
        context.content.push_str("\n[Git status clean]");
    }
}


// =========================
// Usage
// =========================

fn main() {
    let editor = Editor;
    let spellcheck = SpellCheck;
    let git = Git;

    let mut context = EditorContent {
        content: String::from("Hello world"),
    };

    editor.run(&spellcheck, &git, &mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
```

Expected output:

```powershell
--- FINAL CONTENT ---
Hello world
[Spell check OK]
[Git status clean]

```

Open-Closed Principle shines in plugin architectures. Imagine a text editor with plugins.


In the code below, the editor remains **closed** (we don't modify it) but **open**. We can extend it with plugins.


```rust
// cargo run -p ex_05_ocp

// =========================
// Dynamic dispatch example
// =========================

// =========================
// Abstractions
// =========================

// A TxtProcessor is a vector of processing to be applied on text
// It knows nothing about the processing nor the text
pub struct TxtProcessor {
    processings: Vec<Box<dyn Processing>>,
}

impl TxtProcessor {
    pub fn new() -> Self {
        Self { processings: Vec::new() }
    }

    pub fn register_tool(&mut self, processing: Box<dyn Processing>) {
        self.processings.push(processing);
    }

    pub fn run(&mut self, content: &mut EditorContent) {
        for processing in &mut self.processings {
            println!("Running processing: {}", processing.name());
            processing.apply(content); // Apply the processing to the shared content
        }
    }
}

// Here the content of the TxtProcessor is just a String
pub struct EditorContent {
    pub content: String,
}

// If a type wants to have the Processing trait it must implement the 2 methods below
pub trait Processing {
    fn name(&self) -> &str;
    fn apply(&mut self, context: &mut EditorContent);
}


// =========================
// Concrete tools
// =========================

// Lowercase processing
pub struct LowerCase;

impl Processing for LowerCase {
    fn name(&self) -> &str {
        "LowerCase"
    }

    fn apply(&mut self, context: &mut EditorContent) {
        context.content = context.content.to_lowercase();
        context.content.push_str("\n[LowerCase OK]");
    }
}

// SpellChecker processing
pub struct SpellChecker;

impl Processing for SpellChecker {
    fn name(&self) -> &str {
        "SpellChecker"
    }

    fn apply(&mut self, context: &mut EditorContent) {
        // Fake spell checker
        context.content.push_str("\n[SpellChecker OK]");
    }
}

// =========================
// Usage
// =========================

fn main() {
    let mut editor = TxtProcessor::new();

    editor.register_tool(Box::new(LowerCase));
    editor.register_tool(Box::new(SpellChecker));

    let mut ed_context = EditorContent {
        content: String::from("HELLO WORLD"),
    };

    editor.run(&mut ed_context);

    println!("--- FINAL CONTENT ---");
    println!("{}", ed_context.content);
}
```
Expected output:

```powershell
Running processing: LowerCase
Running processing: SpellChecker
--- FINAL CONTENT ---
hello world
[LowerCase OK]
[SpellChecker OK]
```

This example base on dynamic dispatch works and shows that:
* Editor is closed to modification.
* New behaviors are added via Tool implementations.
* The control flow is invariant.
* The behavior is extensible at runtime via dyn Tool.
* Each tool can independently transform the editor state without the editor knowing anything about the concrete behavior.



Now let's try this static

```rust
// cargo run -p ex_06_ocp
// ! DOES NOT COMPILE

// =========================
// Static dispatch example
// =========================

// Shared editor state
pub struct EditorContext {
    pub content: String,
}

// Compile-time behavior abstraction
pub trait Tool {
    fn name(&self) -> &str;
    fn apply(&mut self, context: &mut EditorContext);
}

// Editor using static dispatch
pub struct Editor<T: Tool> {
    tools: Vec<T>,
}

impl<T: Tool> Editor<T> {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
        }
    }

    pub fn register_tool(&mut self, tool: T) {
        self.tools.push(tool);
    }

    pub fn run(&mut self, context: &mut EditorContext) {
        for tool in &mut self.tools {
            println!("Running tool: {}", tool.name());
            tool.apply(context); // Direct call, no vtable
        }
    }
}

// Spell check tool
pub struct SpellCheck;

impl Tool for SpellCheck {
    fn name(&self) -> &str {
        "Spell Checker"
    }

    fn apply(&mut self, context: &mut EditorContext) {
        // Normalize text to simulate spell checking
        context.content = context.content.to_lowercase();
        context.content.push_str("\n[Spell check OK]");
    }
}

// Git integration tool
pub struct Git;

impl Tool for Git {
    fn name(&self) -> &str {
        "Git Integration"
    }

    fn apply(&mut self, context: &mut EditorContext) {
        context.content.push_str("\n[Git status clean]");
    }
}

fn main() {
    let mut editor = Editor::new();

    // Tools must be of the same concrete type T
    editor.register_tool(SpellCheck);
    editor.register_tool(Git);

    let mut context = EditorContext {
        content: String::from("HELLO WORLD"),
    };

    editor.run(&mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
```
It does'nt work. Expected output

```text
Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
  --> src/main.rs:73:26
   |
72 |     editor.register_tool(SpellCheck);
   |     ------               ---------- this argument has type `SpellCheck`...
   |     |
   |     ... which causes `editor` to have type `Editor<SpellCheck>`
73 |     editor.register_tool(Git);
   |            ------------- ^^^ expected `SpellCheck`, found `Git`
   |            |
   |            arguments to this method are incorrect
   |
note: method defined here
  --> src/main.rs:28:12
   |
28 |     pub fn register_tool(&mut self, tool: T) {
   |            ^^^^^^^^^^^^^            -------

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin "playground") due to 1 previous error

```


Let's try to understand what happens here and first let's understand why formatters do not have this problem. In the report/formatter example we have:

```rust
report.generate(&TextFormatter);
report.generate(&HtmlFormatter);
```

Even in the "static dispatch" version:

```rust
pub fn generate<F: ReportFormatter>(&self, formatter: &F) -> String
```

Everything works because:

* The `Report` **does not store** the formatter
* The formatter is **passed as a parameter**
* Each call to `generate()` is **independent**

So the compiler can safely do this:

* Monomorphize `generate::<TextFormatter>`
* Monomorphize `generate::<HtmlFormatter>`
* Monomorphize `generate::<PdfFormatter>`

There is **no need for a single common concrete type**, because nothing is being collected or stored. This is why **static dispatch works perfectly** in that scenario.


Ok... What changes with the editor + plugins example? In the editor case, this line changes everything:

```rust
tools: Vec<T>
```

A `Vec<T>` means:

> "This collection contains elements of **one and only one concrete type**."

When we write:

```rust
editor.register_tool(SpellCheck);
```

The the compiler **infers**:

```rust
editor: Editor<SpellCheck>
```

From that moment on:
* `T = SpellCheck`
* `Vec<T>` becomes `Vec<SpellCheck>`
* `register_tool()` expects a `SpellCheck`, not *any* `Tool`

So when we later write:

```rust
editor.register_tool(Git);
```

We are effectively asking Rust to put a `Git` into a `Vec<SpellCheck>`, which is impossible.


OK... Now let's make sure we understand why the dynamic dispatch works here

With this version:

```rust
Vec<Box<dyn Tool>>
```

We are no longer storing concrete types, but **trait objects**.

That means:
* The editor does not know *which* concrete type it is calling
* Only the **behavioral contract (`Tool`)** matters
* The actual method implementation is resolved **at runtime**


The editor doesn’t even know which trait are stored and that’s exactly what dynamic dispatch is designed for.


Summary
* In the formatter example, both dynamic and static dispatch work because the formatter is passed as a parameter and never stored. Each call to `generate()` can be monomorphized independently.
* In the editor example, tools are registered and stored in a collection. With static dispatch, this requires a single concrete type, which makes heterogeneous plugins impossible.
* Dynamic dispatch solves this by erasing the concrete type behind a trait object, allowing runtime extensibility at the cost of indirection.
* In Rust, choosing between static and dynamic dispatch is not about OCP correctness, but about whether behavior composition happens at compile time or at runtime.

Ok so there is no way... Really?


```rust
// cargo run -p ex_07_ocp

// =========================
// Static dispatch example
// =========================

// Shared editor state
pub struct EditorContext {
    pub content: String,
}

// Compile-time behavior abstraction
pub trait Tool {
    fn name(&self) -> &str;
    fn apply(&mut self, context: &mut EditorContext);
}

// Apply a tool or a chain of tools
pub trait ToolChain {
    fn apply(&mut self, context: &mut EditorContext);
}

// Implementation for a unique tool
// A single Tool is also a valid ToolChain
// If we don't have this implementation there is no way to implement the recusrsive
impl<T: Tool> ToolChain for T {
    fn apply(&mut self, context: &mut EditorContext) {
        // self.apply(context); // CANNOT work: .apply() calls .apply()
        Tool::apply(self, context);
    }
}

// Recusrsive implementation for a tuple
// A tuple (Head, Tail) is a ToolChain if:
//      Head is a Tool
//      Tail is already a ToolChain
impl<Head, Tail> ToolChain for (Head, Tail)
where
    Head: Tool,
    Tail: ToolChain,
{
    fn apply(&mut self, context: &mut EditorContext) {
        self.0.apply(context);
        self.1.apply(context);
    }
}

// Editor using static dispatch
pub struct Editor<T> {
    tools: T,
}

impl<T: ToolChain> Editor<T> {
    pub fn new(tools: T) -> Self {
        Self { tools }
    }

    pub fn run(&mut self, context: &mut EditorContext) {
        self.tools.apply(context);
    }
}

// Spell check tool
pub struct SpellCheck;

impl Tool for SpellCheck {
    fn name(&self) -> &str {
        "Spell Checker"
    }

    fn apply(&mut self, context: &mut EditorContext) {
        // Normalize text to simulate spell checking
        context.content = context.content.to_lowercase();
        context.content.push_str("\n[Spell check OK]");
    }
}

// Git integration tool
pub struct Git;

impl Tool for Git {
    fn name(&self) -> &str {
        "Git Integration"
    }

    fn apply(&mut self, context: &mut EditorContext) {
        context.content.push_str("\n[Git status clean]");
    }
}

fn main() {
    let mut editor = Editor::new((
        SpellCheck,
        Git,
    ));

    // At this point the chain is complete:
    // Git implements Tool
    // therefore Git implements ToolChain
    // therefore (SpellCheck, Git) implements ToolChain
    // therefore Editor<(SpellCheck, Git)> is valid

    let mut context = EditorContext {
        content: String::from("HELLO WORLD"),
    };

    editor.run(&mut context);

    println!("--- FINAL CONTENT ---");
    println!("{}", context.content);
}
```

```text
--- FINAL CONTENT ---
hello world
[Spell check OK]
[Git status clean]
```

It is important to note that in the static version, tools form a compile-time pipeline where all steps are always executed in order; selecting or skipping tools at runtime requires dynamic dispatch. This also explains the order of the output in the terminal.

In the case of plugins, I believe dynamic dispatch is the right choice.
























### Rust-Specific Notes

1. **Trait objects vs generics**:
   - Use `&dyn Trait` when we need runtime polymorphism (heterogeneous collections)
   - Use `<T: Trait>` when we can do compile-time polymorphism (better performance)

2. **Enums can help**: Rust's enums with pattern matching can be appropriate when:
   - The set of variants is truly closed (won't change)
   - We want exhaustiveness checking
   - Example: `Result<T, E>` is an enum because success/failure is closed

3. **Sealed traits**: If we want a trait that's extensible within our crate but not outside, use the sealed trait pattern:
   ```rust
   mod sealed {
       pub trait Sealed {}
   }

   pub trait ReportFormatter: sealed::Sealed {
       fn format(&self, title: &str, data: &[String]) -> String;
   }

   impl sealed::Sealed for TextFormatter {}
   impl ReportFormatter for TextFormatter { /* ... */ }
   ```


















### When to Apply the Open-Closed Principle (OCP)?

Context: It is 8:10 AM. Coffee is still hot. A new feature request just arrived.

**The question to ask:** *"If I need to add new behavior, am I forced to modify existing, working code?"*

* If adding a feature requires changing code that already works and is already in production, the design may not be closed for modification.
* The Open-Closed Principle is not about never changing code, but about **protecting stable code from frequent changes**.
* OCP is a thinking tool that helps us say: *"I should be able to extend this behavior without reopening code that was already validated."*























