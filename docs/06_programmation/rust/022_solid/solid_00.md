---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide"
description: "A gentle introduction to SOLID principles using Rust."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-13 18:00:00
---


# SOLID Principles in Rust: A Practical Guide
{: .no_toc }

A gentle introduction to SOLID principles using Rust.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>

* 01/13 : OK up to the end of Single Responsibility Principle (SRP)



<!--
TODO
* Comment lire les messages d'erreur du compilateur en lien avec la doc. Quand rustc dit "expected X, found Y", comment naviguer vers la doc pour comprendre le problème.
-->


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }
* You are not an expert in Rust but you're not a newbie either.
* SOLID
    * **S**ingle Responsibility Principle:
    * **O**pen-Closed Principle:
    * **L**iskov Substitution Principle:
    * **I**nterface Segregation Principle:
    * **D**ependency Inversion Principle:
* SOLID help us at the mid level
    - **Module** = a group of related functions and types (`mod`)
    - **Component** = a crate (lib or binary)
    - **Class** = a `struct` with associated functions and trait implementations
* SOLID principles are not rules to follow, but questions to ask when code starts to feel uncomfortable. See the "When to Apply the ... Principle?" sections.



Play, break, rebuild the [Coffee Shop Order System companion project](https://github.com/40tude/coffee-shop-solid) on GitHub

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
## Introduction: Why Should I Care About SOLID?

I'm, by far, not an expert. However, I still have in mind the project to rewrite in Rust my [Fraud Detection project (Python)](https://github.com/40tude/fraud_detection_2) and I would like to make sure it follows the good practices. In addition, the more I read and the more I use Rust, the more I want to learn how to leverage the type system to define the architecture/design of my applications.

In the following, I put black on white what I understand, take the time to confirm with other sources and illustrate the concepts with some code. I hate code snippets that are incomplete and does not work. However, here they are hard to avoid. This is why, in addition to this article I created what I call the [Coffee Shop Order System companion project](https://github.com/40tude/coffee-shop-solid) which is available on GitHub. It is a demo where I apply what is discussed her. At the time of writing it is working, not yet perfect but working. And you know what? Regarding the code snippets of this article... I did'nt say my last word and I may decide to rewrite them so that we can copy/paste them in our best friend, aka [Rust Playground](https://play.rust-lang.org/). We will see...

Anyway, I'm reading Uncle Bob's [Clean Architecture](https://amzn.eu/d/2khTpqS) book and wondering how these principles, born in the world of Java and C#, apply to Rust. After all, we're not dealing with inheritance hierarchies, we don't have traditional classes, and everything compiles into a single binary. So what gives?

<div align="center">
<img src="./assets/img02.webp" alt="" width="450" loading="lazy"/><br/>
<span>2017 - <a href="https://amzn.eu/d/2khTpqS" target="_blank">Clean Architecture</a></span>
</div>



Here's the thing: **SOLID** isn't about the language features - it's about **organizing our code so it doesn't turn into a tangled mess that makes us want to rage-quit and become a farmer**.

<div align="center">
<img src="./assets/img05.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>2017 - <a href="https://amzn.eu/d/2khTpqS" target="_blank">Clean Architecture</a></span> -->
</div>

The principles are about managing dependencies, separating concerns, and making our code maintainable as it grows.

### The "One Binary" Question
{: .no_toc }

Let's address the elephant in the room right now: the author talks a lot about "components" that can be independently deployed (JARs, DLLs, Gems). In Rust, we typically compile everything into a single binary. Does this make SOLID irrelevant? Absolutely not and here's why:

1. **Independence is logical, not physical**: When we talk about "independent components" in SOLID, we're really talking about modules/crates that:
   - Have clear boundaries (traits)
   - Can be understood in isolation
   - Can be tested independently
   - Can be developed by different teams without stepping on each other's toes
   - Can evolve without breaking dependents

2. **Rust's crate system provides boundaries**: Even though everything links into one binary, our crates are still separate compilation units with explicit interfaces. A well-designed crate can be understood without reading its dependencies' implementation.

3. **The binary is our deployment unit**: In the JVM world, they talk about deploying individual JARs. In Rust, we deploy the whole binary - but the internal organization still matters enormously for maintainability, testability, and team collaboration.

Think of it this way: when the Death Star blows up, it doesn't matter that it was one giant structure - what matters is that the exhaust port was poorly isolated from the reactor core. Dependencies matter, even in a monolith.




### What SOLID Means at the Mid-Level
{: .no_toc }

Uncle Bob is clear: SOLID operates at the **module/class level**, not at the architectural level (that's coming later in the book) and not at the `for loop` level either. In Rust terms:
- **Module** = a group of related functions and types (`mod`)
- **Component** = a crate (library or binary)
- **Class** = a `struct` with associated functions and trait implementations

SOLID tells us how to organize these pieces so that:
- They tolerate change
- They're easy to understand
- They're reusable
- They don't create ripple effects when modified

### Rust's Secret Weapons for SOLID
{: .no_toc }

Rust has features that greatly facilitate the implementation of SOLID principles:

- **Traits**: Perfect abstraction mechanism (interfaces without the baggage)
- **Ownership system**: Forces us to think about responsibilities and boundaries
- **Type system**: Compile-time enforcement of contracts
- **Pattern matching**: Extensibility without modification
- **No null**: Makes interface contracts explicit
- **Error handling**: `Result<T, E>` makes failure cases part of the contract

Alright, enough philosophy. It is dogfight time! Let's dive into each principle with real code.


<figure style="text-align: center;">
  <iframe width="560" height="315" src="https://www.youtube.com/embed/EZfM2VMs_vI?si=FHS-1PFIqBG70Ffs&amp;start=55" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
  <figcaption>I love the opening theme. It gives me goosebumps every time.</figcaption>
</figure>





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## 1. Single Responsibility Principle (SRP)

### The Principle
{: .no_toc }

> "A module should have one, and only one, reason to change."

Or, as Uncle Bob refines it:

> "A module should be responsible to one, and only one, actor."

In my view, above, an “actor” is the person, group, or system that has the authority to make decisions about the module’s features, and to which the module is therefore accountable.

The Single Responsibility Principle is **NOT** "do one thing" (that's for functions). It is about **reasons to change**. If our module changes because the accounting department wants something **AND** because the operations team wants something, we've got two reasons to change - that's a violation.

### The Problem: Accidental Coupling
{: .no_toc }

Let's say we're building a payroll system. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/). Here's what violates Single Responsibility Principle:

```rust
use std::fmt;

/// Dummy database error type
#[derive(Debug)]
pub struct DbError;

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error")
    }
}

/// Dummy database abstraction
pub struct Database;

impl Database {
    /// Execute a fake SQL query
    pub fn execute(&mut self, query: &str, params: &[&dyn fmt::Debug]) -> Result<(), DbError> {
        println!("Executing SQL: {}", query);
        println!("With params: {:?}", params);
        Ok(())
    }
}

pub struct Employee {
    pub id: u32,
    pub name: String,
    pub hours_worked: f64,
    pub rate: f64,
}

impl Employee {
    /// Calculate pay (used by Accounting)
    pub fn calculate_pay(&self) -> f64 {
        let regular_hours = self.hours_worked.min(40.0);
        let overtime_hours = (self.hours_worked - 40.0).max(0.0);
        regular_hours * self.rate + overtime_hours * self.rate * 1.5
    }

    /// Calculate overtime hours for operations report (used by Operations)
    pub fn calculate_overtime_hours(&self) -> f64 {
        (self.hours_worked - 40.0).max(0.0)
    }

    /// Save to database (used by DBAs / Infrastructure)
    pub fn save(&self, db: &mut Database) -> Result<(), DbError> {
        db.execute(
            "INSERT INTO employees VALUES (?, ?, ?, ?)",
            &[&self.id, &self.name, &self.hours_worked, &self.rate],
        )
    }

    /// Generate report (used by HR)
    pub fn generate_report(&self) -> String {
        format!(
            "Employee Report\n\
             Name: {}\n\
             Hours: {}\n\
             Pay: ${:.2}",
            self.name,
            self.hours_worked,
            self.calculate_pay()
        )
    }
}

fn main() {
    let employee = Employee {
        id: 1,
        name: "Alice".to_string(),
        hours_worked: 45.0,
        rate: 20.0,
    };

    // Accounting client
    let pay = employee.calculate_pay();
    println!("Accounting: pay = ${:.2}", pay);

    // Operations client
    let overtime = employee.calculate_overtime_hours();
    println!("Operations: overtime hours = {}", overtime);

    // Infrastructure / DBA client
    let mut db = Database;
    employee.save(&mut db).unwrap();

    // HR client
    let report = employee.generate_report();
    println!("\nHR Report:\n{}", report);
}
```

Expected output:

```powershell
Accounting: pay = $950.00
Operations: overtime hours = 5
Executing SQL: INSERT INTO employees VALUES (?, ?, ?, ?)
With params: [1, "Alice", 45.0, 20.0]

HR Report:
Employee Report
Name: Alice
Hours: 45
Pay: $950.00
```


**What's wrong here?** This `Employee` data type serves **four different actors**:
1. **Accounting** - needs `calculate_pay()`
2. **Operations** - needs `calculate_overtime_hours()`
3. **DBAs** - need `save()`
4. **HR** - needs `generate_report()`

And all theses methods belong to the same object.

Now imagine:
- Accounting wants to change overtime calculation rules
- Operations wants to track overtime differently
- DBAs want to switch from SQL to NoSQL
- HR wants reports in JSON instead of text

Every change affects the same data type (`Employee`). Merge conflicts galore. Changes for one team risk breaking functionality for another team.



### The Solution: Separate the Actors
{: .no_toc }

Below make sure to realize that now the code is dispatched among different files.

{% raw %}
```rust
// src/domain/employee.rs
// Core data - this is just data, no behavior
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub hours_worked: f64,
    pub rate: f64,
}
```



```rust
// src/accounting/payroll.rs
// Accounting's responsibility
pub struct PayrollCalculator;

impl PayrollCalculator {
    pub fn calculate_pay(employee: &Employee) -> f64 {
        let regular_hours = employee.hours_worked.min(40.0);
        let overtime_hours = (employee.hours_worked - 40.0).max(0.0);
        regular_hours * employee.rate + overtime_hours * employee.rate * 1.5
    }
}
```



```rust
// src/operations/overtime.rs
// Operations' responsibility
pub struct OvertimeTracker;

impl OvertimeTracker {
    pub fn calculate_overtime_hours(employee: &Employee) -> f64 {
        (employee.hours_worked - 40.0).max(0.0)
    }
}
```


```rust
// src/infrastructure/repository.rs
// Infrastructure/DBA's responsibility
pub struct EmployeeRepository {
    db: Database,
}

impl EmployeeRepository {
    pub fn save(&mut self, employee: &Employee) -> Result<(), DbError> {
        self.db.execute(
            "INSERT INTO employees VALUES (?, ?, ?, ?)",
            &[&employee.id, &employee.name, &employee.hours_worked, &employee.rate],
        )
    }

    pub fn find_by_id(&self, id: u32) -> Result<Employee, DbError> {
        // Implementation
        todo!()
    }
}
```



```rust
// src/hr/reporting.rs
// HR's responsibility
pub struct EmployeeReporter;

impl EmployeeReporter {
    pub fn generate_text_report(employee: &Employee) -> String {
        format!(
            "Employee Report\n\
             Name: {}\n\
             Hours: {}\n\
             Pay: ${:.2}",
            employee.name,
            employee.hours_worked,
            PayrollCalculator::calculate_pay(employee)
        )
    }

    pub fn generate_json_report(employee: &Employee) -> String {
        format!(
            r#"{{"name": "{}", "hours": {}, "pay": {:.2}}}"#,
            employee.name,
            employee.hours_worked,
            PayrollCalculator::calculate_pay(employee)
        )
    }
}
```
{% endraw %}

Again, I hate code snippets we can't "play" with. This is why you can find below a multi modal, yet monolithic version of the previous code that you can copy and paste in [Rust Playground](https://play.rust-lang.org/). It is not yet perfect but we are moving in the right direction and keep in mind we focus on separating the actors.

<div align="center">
<img src="./assets/img06.webp" alt="" width="900" loading="lazy"/><br/>
<span>A solution to our SRP violation running in the Rust Playground</span>
</div>


{% raw %}
```rust
use std::fmt;

// Domain
// This module should be in: src/domain/employee.rs
// Core data - this is just data, no behavior
mod domain {
    pub struct Employee {
        pub id: u32,
        pub name: String,
        pub hours_worked: f64,
        pub rate: f64,
    }
}

// Infrastructure/DBA's responsibility
mod infrastructure {
    // This module should be in: src/infrastructure/db.rs
    pub mod db {
        use std::fmt;

        /// Dummy database error type
        #[derive(Debug)]
        pub struct DbError;

        impl fmt::Display for DbError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Database error")
            }
        }

        /// Dummy database abstraction
        pub struct Database;

        impl Database {
            /// Execute a fake SQL query
            pub fn execute(
                &self,
                query: &str,
                params: &[&dyn fmt::Debug],
            ) -> Result<(), DbError> {
                println!("Executing SQL: {}", query);
                println!("With params: {:?}", params);
                Ok(())
            }
        }
    }

    // This module should be in: src/infrastructure/repository.rs
    pub mod repository {
        use crate::domain::Employee;
        use super::db::{Database, DbError};

        /// Infrastructure / DBA's responsibility
        pub struct EmployeeRepository {
            pub db: Database,
        }

        impl EmployeeRepository {
            pub fn save(&self, employee: &Employee) -> Result<(), DbError> {
                self.db.execute(
                    "INSERT INTO employees VALUES (?, ?, ?, ?)",
                    &[&employee.id, &employee.name, &employee.hours_worked, &employee.rate],
                )
            }

            pub fn find_by_id(&self, _id: u32) -> Result<Employee, DbError> {
                // Fake implementation for demo purposes
                Ok(Employee {
                    id: 1,
                    name: "Alice".to_string(),
                    hours_worked: 40.0,
                    rate: 20.0,
                })
            }
        }
    }
}

// This module should be in: src/accounting/payroll.rs
// Accounting's responsibility
mod accounting {
    use crate::domain::Employee;

    pub struct PayrollCalculator;

    impl PayrollCalculator {
        pub fn calculate_pay(employee: &Employee) -> f64 {
            let regular_hours = employee.hours_worked.min(40.0);
            let overtime_hours = (employee.hours_worked - 40.0).max(0.0);
            regular_hours * employee.rate + overtime_hours * employee.rate * 1.5
        }
    }
}

// This module should be in: src/operations/overtime.rs
// Operations' responsibility
mod operations {
    use crate::domain::Employee;

    pub struct OvertimeTracker;

    impl OvertimeTracker {
        pub fn calculate_overtime_hours(employee: &Employee) -> f64 {
            (employee.hours_worked - 40.0).max(0.0)
        }
    }
}

// This module should be in: src/hr/reporting.rs
// HR's responsibility
mod hr {
    use crate::domain::Employee;
    use crate::accounting::PayrollCalculator;

    /// HR's responsibility
    pub struct EmployeeReporter;

    impl EmployeeReporter {
        pub fn generate_text_report(employee: &Employee) -> String {
            format!(
                "Employee Report\n\
                 Name: {}\n\
                 Hours: {}\n\
                 Pay: ${:.2}",
                employee.name,
                employee.hours_worked,
                PayrollCalculator::calculate_pay(employee)
            )
        }

        pub fn generate_json_report(employee: &Employee) -> String {
            format!(
                r#"{{"name": "{}", "hours": {}, "pay": {:.2}}}"#,
                employee.name,
                employee.hours_worked,
                PayrollCalculator::calculate_pay(employee)
            )
        }
    }
}

// This should be in: src/main.rs
fn main() {
    use domain::Employee;
    use accounting::PayrollCalculator;
    use operations::OvertimeTracker;
    use infrastructure::repository::EmployeeRepository;
    use infrastructure::db::Database;
    use hr::EmployeeReporter;

    let employee = Employee {
        id: 1,
        name: "Alice".to_string(),
        hours_worked: 45.0,
        rate: 20.0,
    };

    // Accounting client
    let pay = PayrollCalculator::calculate_pay(&employee);
    println!("Accounting: pay = ${:.2}", pay);

    // Operations client
    let overtime = OvertimeTracker::calculate_overtime_hours(&employee);
    println!("Operations: overtime hours = {}", overtime);

    // Infrastructure client
    let repo = EmployeeRepository { db: Database };
    repo.save(&employee).unwrap();

    // HR client
    println!(
        "\nHR Text Report:\n{}",
        EmployeeReporter::generate_text_report(&employee)
    );
    println!(
        "\nHR JSON Report:\n{}",
        EmployeeReporter::generate_json_report(&employee)
    );
}
```
{% endraw %}


Expected output:

```powershell
Accounting: pay = $950.00
Operations: overtime hours = 5
Executing SQL: INSERT INTO employees VALUES (?, ?, ?, ?)
With params: [1, "Alice", 45.0, 20.0]

HR Text Report:
Employee Report
Name: Alice
Hours: 45
Pay: $950.00

HR JSON Report:
{"name": "Alice", "hours": 45, "pay": 950.00}
```


Now:
- If payroll rules change, only `PayrollCalculator` changes
- If we switch databases, only `EmployeeRepository` changes
- If HR wants new report formats, only `EmployeeReporter` changes
- Each actor owns their own code meaning

The teams can run independent testing

```powershell
cargo test --package accounting   # Only accounting tests
cargo test --package hr           # Only HR tests
```

We can have independent deployment (if Rust Workspaces)

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "domain",
    "accounting",
    "operations",
    "infrastructure",
    "hr",
]
```

### Rust-Specific Notes
{: .no_toc }

1. **No methods on data structs**: In Rust, we're not forced to put methods on types. We can have "plain old data" (POD) structs and separate modules/types for behavior. This naturally encourages Single Responsibility Principle.

2. **Files Organization**
In the real life we could have something like:

```
src/
├── domain/
│   └── employee.rs          ← Just data, no behavior
│
├── accounting/              ← Accounting team's module
│   └── payroll.rs           ← calculate_pay
│
├── operations/              ← Operations team's module
│   └── overtime.rs          ← calculate_overtime_hours
│
├── infrastructure/          ← DBA team's module
│   └── repository.rs        ← save/load
│   └── db.rs                ← Database, DbError
│
├── hr/                      ← HR team's module
│   └── reporting.rs         ← generate_report
```

3. **Module organization**:

We can organize our crates like this:

   ```
   src/
     employee.rs         // Core data type
     payroll.rs          // PayrollCalculator
     operations.rs       // OvertimeTracker
     repository.rs       // EmployeeRepository
     reporting.rs        // EmployeeReporter
     db.rs               // Database
   ```

If you hesitate when I talk about, files, crates, modules and module trees... Read this [post]({%link docs/06_programmation/rust/013_no_more_mod_rs/no_more_mod_rs.md%}).

4. **Ownership clarifies responsibility**: When we pass `&Employee` vs `Employee` vs `&mut Employee`, we're being explicit about responsibility. The repository needs mutable access to the DB but not to employees. The calculator needs read-only access to employees.


**Note**:
This is more or less the organization in the [Coffee Shop Order System companion project](https://github.com/40tude/coffee-shop-solid) where we have something similar to:

```
src/
├── domain/          ← Core data (Employee equivalent)
├── services/        ← Business logic teams
│   ├── pricing_calculator.rs   ← Accounting team
│   └── order_service.rs        ← Operations team
├── adapters/        ← Infrastructure teams
│   ├── repository.rs           ← DBA team
│   └── notifier.rs             ← Customer Service team

```

### When to apply the Single Responsibility Principle (SRP)?
{: .no_toc }

Context: It is 8 AM. Coffee in one hand, eyes on the screen, we are reviewing yesterday’s code.

**The question to ask:** *"If this code needs to change, who is requesting the change, and why?"*

* If the answer points to **multiple actors** with different goals (business rules, persistence, presentation, infrastructure), then the code is likely doing too much and should be split.
* The Single Responsibility Principle is less about counting changes and more about identifying **who has the authority** to demand them. If that doesn't work for you, you can count the number of people/teams who can "write the specifications" or the number of people/teams "who will validate and accept the new features". If the number is larger than one, you're in trouble.
* SRP is a thinking tool that helps us say: *"This code makes me nervous because too many people could ask me to change it."*


<div align="center">
<img src="./assets/img01.webp" alt="" width="450" loading="lazy"/><br/>
<span><a href="https://youtu.be/KrLFEHeKNBw?si=cpM1aUZl3V5-6yx_&t=28" target="_blank">1984</a></span>
</div>












<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 2. Open-Closed Principle (OCP)

### The Principle
{: .no_toc }

> "Software entities should be open for extension but closed for modification."

In other words: when requirements change, we should be able to add new behavior **without changing existing code**.

This sounds like black magic, but it's actually straightforward: **depend on abstractions (traits), not concretions**. When we need new behavior, implement a new type that satisfies the trait.

### The Problem: Modification Hell
{: .no_toc }

Let's build a report generator that can output different formats:

```rust
pub enum ReportFormat {
    Text,
    Html,
    Pdf,
}

pub struct Report {
    title: String,
    data: Vec<String>,
}

impl Report {
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
```

**What happens when we need XML output?** We have to:
1. Add `Xml` to the enum
2. Modify the `generate()` method
3. Recompile everything
4. Risk breaking existing formats
5. Every developer working on reports has merge conflicts

**This violates Open-Closed Principle**: adding a new format requires modifying existing code.

### The Solution: Trait-Based Extension
{: .no_toc }

```rust
// Define the abstraction
pub trait ReportFormatter {
    fn format(&self, title: &str, data: &[String]) -> String;
}

// The report doesn't know about specific formats
pub struct Report {
    title: String,
    data: Vec<String>,
}

impl Report {
    pub fn generate(&self, formatter: &dyn ReportFormatter) -> String {
        formatter.format(&self.title, &self.data)
    }
}

// Each format is a separate implementation - CLOSED for modification
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

pub struct PdfFormatter;

impl ReportFormatter for PdfFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        format!("PDF: {} [binary data]", title)
    }
}

// Want XML? Just add a new type - NO MODIFICATION of existing code
pub struct XmlFormatter;

impl ReportFormatter for XmlFormatter {
    fn format(&self, title: &str, data: &[String]) -> String {
        let mut output = format!("<report>\n  <title>{}</title>\n  <items>\n", title);
        for item in data {
            output.push_str(&format!("    <item>{}</item>\n", item));
        }
        output.push_str("  </items>\n</report>");
        output
    }
}

// Usage
fn main() {
    let report = Report {
        title: "Sales Q4".to_string(),
        data: vec!["Item 1".to_string(), "Item 2".to_string()],
    };

    println!("{}", report.generate(&TextFormatter));
    println!("{}", report.generate(&HtmlFormatter));
    println!("{}", report.generate(&XmlFormatter)); // New format, zero changes to Report
}
```

Now adding XML (or JSON, or Markdown, or whatever) requires **zero changes** to `Report` or existing formatters. we just add a new type.

### Taking It Further: Static Dispatch
{: .no_toc }

If we want to avoid dynamic dispatch overhead:

```rust
impl Report {
    pub fn generate<F: ReportFormatter>(&self, formatter: &F) -> String {
        formatter.format(&self.title, &self.data)
    }
}
```

This uses monomorphization - the compiler generates a specialized version for each formatter type at compile time. Zero runtime cost, full extensibility.

### Real-World Example: Plugin System
{: .no_toc }

Open-Closed Principle shines in plugin architectures. Imagine a text editor with plugins:

```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&mut self, context: &mut EditorContext);
}

pub struct Editor {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Editor {
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn run_plugins(&mut self, context: &mut EditorContext) {
        for plugin in &mut self.plugins {
            plugin.execute(context);
        }
    }
}

// Third-party plugin - doesn't require modifying Editor
pub struct SpellCheckPlugin;

impl Plugin for SpellCheckPlugin {
    fn name(&self) -> &str { "Spell Checker" }

    fn execute(&mut self, context: &mut EditorContext) {
        // Check spelling
    }
}

// Another third-party plugin
pub struct GitPlugin;

impl Plugin for GitPlugin {
    fn name(&self) -> &str { "Git Integration" }

    fn execute(&mut self, context: &mut EditorContext) {
        // Git operations
    }
}
```

The editor is **closed** (we don't modify it) but **open** (we can extend it).

### Rust-Specific Notes
{: .no_toc }

1. **Trait objects vs generics**:
   - Use `&dyn Trait` when we need runtime polymorphism (heterogeneous collections)
   - Use `<T: Trait>` when we can do compile-time polymorphism (better performance)

2. **Enums are not always bad**: Rust's enums with pattern matching can be appropriate when:
   - The set of variants is truly closed (won't change)
   - we want exhaustiveness checking
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
{: .no_toc }

Context: It is 8:10 AM. Coffee is still hot. A new feature request just arrived.

**The question to ask:** *"If I need to add new behavior, am I forced to modify existing, working code?"*

* If adding a feature requires changing code that already works and is already in production, the design may not be closed for modification.
* The Open-Closed Principle is not about never changing code, but about **protecting stable code from frequent changes**.
* OCP is a thinking tool that helps us say: *"I should be able to extend this behavior without reopening code that was already validated."*









<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 3. Liskov Substitution Principle (LSP)

### The Principle
{: .no_toc }

> "Functions that use references to base classes must be able to use objects of derived classes without knowing it."

In Rust terms:

> "Code that depends on a trait must work correctly with any implementation of that trait."

LSP is about **keeping promises**. If our trait says "this method returns the sum of two numbers", then every implementation better return the sum - not the difference, not a random number, not a side effect.

### The Problem: Surprising Substitutions
{: .no_toc }

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
{: .no_toc }

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
{: .no_toc }

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
{: .no_toc }

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
{: .no_toc }

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
{: .no_toc }

1. **Preconditions cannot be strengthened**: If the trait accepts any string, implementations can't suddenly require non-empty strings
2. **Postconditions cannot be weakened**: If the trait promises to return a value, implementations can't return `None` in cases where the trait wouldn't
3. **Invariants must be preserved**: If the trait maintains some property, all implementations must maintain it
4. **No new exceptions**: In Rust, this means the error type in `Result<T, E>` should cover all failure modes









### When to Apply the Liskov Substitution Principle (LSP)?
{: .no_toc }

Context: It is 8:20 AM. You replaced an implementation with another one. Tests start failing.

**The question to ask:** *"Can I replace this type with one of its subtypes without surprising the caller?"*

* If using a subtype forces the caller to add special cases, defensive checks, or different logic, LSP is likely violated.
* The Liskov Substitution Principle is not about inheritance syntax, but about **behavioral compatibility**.
* LSP is a thinking tool that helps us say: *"If I have to know the concrete type, then substitution is broken."*










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 4. Interface Segregation Principle (ISP)

### The Principle
{: .no_toc }

> "No client should be forced to depend on methods it does not use."

In other words: don't create fat traits that do everything. Split them into focused, cohesive traits.

### The Problem: The God Trait
{: .no_toc }
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
{: .no_toc }

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
{: .no_toc }

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
{: .no_toc }

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
{: .no_toc }

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
{: .no_toc }

Ask ourself:
- Do all implementors support all methods?
- Could a client need only a subset of the functionality?
- Would testing be easier with smaller interfaces?
- Do different methods serve different use cases/actors?

If yes to any of these, consider splitting the trait.








### When to Apply the Interface Segregation Principle (ISP)?
{: .no_toc }

Context: It is 7:50 AM. The office is empty and the coffee damn hot. You review the interface implemented yesterday and immediately you feel uncomfortable.

**The question to ask:** *"Am I forced to depend on methods I do not use?"*

* If an interface requires a client to implement or know methods that are irrelevant to its use case, the interface is **too broad**.
* The Interface Segregation Principle favors **small, role-focused interfaces** over large, generic ones.
* ISP is a thinking tool that helps us say: *"This interface is making me implement things I don’t care about."*








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 5. Dependency Inversion Principle (DIP)

### The Principle
{: .no_toc }

> "High-level modules should not depend on low-level modules. Both should depend on abstractions."

This is the **cornerstone of Clean Architecture**. It's what allows we to build a system where:
- Business logic doesn't know about databases
- our domain doesn't care about HTTP frameworks
- Core code doesn't depend on external libraries

In Rust terms: **our high-level business logic should depend on traits, and low-level details (I/O, databases, frameworks) should implement those traits.**

### The Problem: Direct Dependencies
{: .no_toc }

Let's build a user registration service:

```rust
// Low-level: PostgreSQL repository
use postgres::{Client, NoTls};

pub struct PostgresUserRepository {
    client: Client,
}

impl PostgresUserRepository {
    pub fn new(connection_string: &str) -> Result<Self, Error> {
        let client = Client::connect(connection_string, NoTls)?;
        Ok(Self { client })
    }

    pub fn save_user(&mut self, user: &User) -> Result<()> {
        self.client.execute(
            "INSERT INTO users (id, email, name) VALUES ($1, $2, $3)",
            &[&user.id, &user.email, &user.name],
        )?;
        Ok(())
    }

    pub fn find_by_email(&mut self, email: &str) -> Result<Option<User>> {
        let rows = self.client.query(
            "SELECT id, email, name FROM users WHERE email = $1",
            &[&email],
        )?;
        // Parse rows...
        Ok(None)
    }
}

// High-level: Business logic
pub struct UserRegistrationService {
    repository: PostgresUserRepository, // Direct dependency on low-level detail!
}

impl UserRegistrationService {
    pub fn register_user(&mut self, email: String, name: String) -> Result<User> {
        // Check if user exists
        if self.repository.find_by_email(&email)?.is_some() {
            return Err(Error::UserAlreadyExists);
        }

        // Create user
        let user = User {
            id: Uuid::new_v4(),
            email,
            name,
        };

        // Save
        self.repository.save_user(&user)?;

        Ok(user)
    }
}
```

**What's wrong?**

1. **High-level logic depends on low-level detail**: `UserRegistrationService` directly depends on `PostgresUserRepository`
2. **Can't test without a database**: Every test needs a real Postgres connection
3. **Can't swap databases**: Want to try MongoDB? we have to rewrite `UserRegistrationService`
4. **Business logic is coupled to infrastructure**: our domain code knows about Postgres-specific errors and types

The dependency arrow points the wrong way:
```
UserRegistrationService --> PostgresUserRepository --> postgres crate
   (high-level)                  (low-level)            (external)
```

### The Solution: Invert the Dependency
{: .no_toc }

```rust
// Define abstraction (owned by high-level module)
pub trait UserRepository {
    fn save_user(&mut self, user: &User) -> Result<(), RepositoryError>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
}

// High-level business logic depends on abstraction
pub struct UserRegistrationService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserRegistrationService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn register_user(&mut self, email: String, name: String)
        -> Result<User, RegistrationError>
    {
        // Business logic doesn't know about Postgres, HTTP, or any implementation detail
        if self.repository.find_by_email(&email)
            .map_err(|_| RegistrationError::DatabaseError)?
            .is_some()
        {
            return Err(RegistrationError::UserAlreadyExists);
        }

        let user = User {
            id: Uuid::new_v4(),
            email,
            name,
        };

        self.repository.save_user(&user)
            .map_err(|_| RegistrationError::DatabaseError)?;

        Ok(user)
    }
}

// Low-level Postgres implementation (in a separate module/crate)
pub struct PostgresUserRepository {
    client: postgres::Client,
}

impl UserRepository for PostgresUserRepository {
    fn save_user(&mut self, user: &User) -> Result<(), RepositoryError> {
        self.client.execute(
            "INSERT INTO users (id, email, name) VALUES ($1, $2, $3)",
            &[&user.id, &user.email, &user.name],
        )
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let rows = self.client.query(
            "SELECT id, email, name FROM users WHERE email = $1",
            &[&email],
        )
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Parse and return...
        Ok(None)
    }
}

// Low-level MongoDB implementation (completely independent)
pub struct MongoUserRepository {
    collection: mongodb::Collection<User>,
}

impl UserRepository for MongoUserRepository {
    fn save_user(&mut self, user: &User) -> Result<(), RepositoryError> {
        self.collection
            .insert_one(user, None)
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let filter = doc! { "email": email };
        self.collection
            .find_one(filter, None)
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))
    }
}

// In-memory implementation for testing
pub struct InMemoryUserRepository {
    users: Vec<User>,
}

impl UserRepository for InMemoryUserRepository {
    fn save_user(&mut self, user: &User) -> Result<(), RepositoryError> {
        self.users.push(user.clone());
        Ok(())
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        Ok(self.users.iter().find(|u| u.email == email).cloned())
    }
}
```

Now the dependency flows correctly:

```
UserRegistrationService --> UserRepository <-- PostgresUserRepository
   (high-level)              (abstraction)        (low-level)
                                  ^
                                  |
                          MongoUserRepository
                             (low-level)
```

Both high-level and low-level depend on the abstraction!

### Testing Becomes Trivial
{: .no_toc }

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_user() {
        // No database needed!
        let repo = InMemoryUserRepository { users: vec![] };
        let mut service = UserRegistrationService::new(repo);

        let user = service.register_user(
            "test@example.com".to_string(),
            "Test User".to_string(),
        ).unwrap();

        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_duplicate_registration() {
        let mut repo = InMemoryUserRepository { users: vec![] };
        repo.users.push(User {
            id: Uuid::new_v4(),
            email: "existing@example.com".to_string(),
            name: "Existing".to_string(),
        });

        let mut service = UserRegistrationService::new(repo);

        let result = service.register_user(
            "existing@example.com".to_string(),
            "Duplicate".to_string(),
        );

        assert!(matches!(result, Err(RegistrationError::UserAlreadyExists)));
    }
}
```

### Real-World: Hexagonal Architecture
{: .no_toc }

DIP is the foundation of Hexagonal/Ports & Adapters architecture:

```rust
// CORE DOMAIN (inner layer - no dependencies)
pub mod domain {
    pub struct Order {
        pub id: OrderId,
        pub items: Vec<LineItem>,
        pub total: Money,
    }

    pub enum OrderError {
        InvalidOrder,
        PaymentFailed,
    }
}

// PORTS (defined by domain - abstractions)
pub mod ports {
    use crate::domain::*;

    // Output port: domain needs to persist orders
    pub trait OrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError>;
        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
    }

    // Output port: domain needs to process payments
    pub trait PaymentGateway {
        fn charge(&self, amount: Money, card: &CreditCard) -> Result<PaymentId, OrderError>;
    }

    // Output port: domain needs to send emails
    pub trait NotificationService {
        fn send_order_confirmation(&self, order: &Order) -> Result<(), OrderError>;
    }
}

// APPLICATION SERVICE (uses domain + ports)
pub mod application {
    use crate::domain::*;
    use crate::ports::*;

    pub struct OrderService<R, P, N>
    where
        R: OrderRepository,
        P: PaymentGateway,
        N: NotificationService,
    {
        repository: R,
        payment: P,
        notifications: N,
    }

    impl<R, P, N> OrderService<R, P, N>
    where
        R: OrderRepository,
        P: PaymentGateway,
        N: NotificationService,
    {
        pub fn place_order(
            &mut self,
            items: Vec<LineItem>,
            card: &CreditCard,
        ) -> Result<Order, OrderError> {
            // Pure business logic - no infrastructure concerns
            let order = Order::new(items)?;

            self.payment.charge(order.total, card)?;
            self.repository.save(&order)?;
            self.notifications.send_order_confirmation(&order)?;

            Ok(order)
        }
    }
}

// ADAPTERS (outer layer - implements ports)
pub mod adapters {
    use crate::ports::*;
    use crate::domain::*;

    // PostgreSQL adapter
    pub struct PostgresOrderRepository {
        pool: sqlx::PgPool,
    }

    impl OrderRepository for PostgresOrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError> {
            // SQL implementation
            Ok(())
        }

        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            // SQL implementation
            Ok(None)
        }
    }

    // Stripe adapter
    pub struct StripePaymentGateway {
        api_key: String,
    }

    impl PaymentGateway for StripePaymentGateway {
        fn charge(&self, amount: Money, card: &CreditCard) -> Result<PaymentId, OrderError> {
            // Stripe API call
            Ok(PaymentId::new())
        }
    }

    // SendGrid adapter
    pub struct SendGridNotificationService {
        api_key: String,
    }

    impl NotificationService for SendGridNotificationService {
        fn send_order_confirmation(&self, order: &Order) -> Result<(), OrderError> {
            // SendGrid API call
            Ok(())
        }
    }
}
```

The beauty: **we can swap any adapter without touching business logic**. Want to switch from Postgres to MongoDB? Implement `OrderRepository` for MongoDB. Want to switch from Stripe to PayPal? Implement `PaymentGateway` for PayPal. The domain is completely isolated.

### Rust-Specific Notes
{: .no_toc }

1. **Generic vs Trait Objects**:
   ```rust
   // Generic: monomorphization, zero-cost, compile-time
   pub struct Service<R: Repository> { repo: R }

   // Trait object: dynamic dispatch, runtime flexibility
   pub struct Service { repo: Box<dyn Repository> }
   ```
   Use generics for performance-critical code, trait objects when we need runtime polymorphism.

2. **Crate Organization**:
   ```
   my-app/
     domain/          (no dependencies - pure Rust)
       Cargo.toml     (no external crates)
       src/
         lib.rs
     application/     (depends on domain)
       Cargo.toml     (depends on domain crate)
       src/
         lib.rs
     adapters/        (depends on domain)
       Cargo.toml     (depends on domain, postgres, http, etc.)
       src/
         postgres.rs
         http.rs
   ```

3. **Dependency direction**:
   - Domain crate: zero dependencies
   - Application crate: depends on domain
   - Adapters crate: depends on domain (NOT on application)
   - Main binary: depends on all, wires them together

4. **The "one binary" concern**: Even though Rust compiles to one binary, the crate structure enforces dependency direction at compile time. we **cannot** accidentally import `postgres` in our domain crate if domain doesn't list it in `Cargo.toml`.







### When to Apply the Dependency Inversion Principle (DIP)?
{: .no_toc }

Context: It is 8:05 AM. Double espresso. Thank God it’s Friday, but the week is not over yet. You want to test your code, but everything depends on concrete details.

**The question to ask:** *"Does my high-level logic depend on details, or do details depend on my logic?"*

* If business rules directly depend on frameworks, databases, or external services, DIP is likely violated.
* The Dependency Inversion Principle is not about abstractions everywhere, but about **protecting policy from implementation details**.
* The Dependency Inversion Principle is a thinking tool that helps us say: *"My core logic should not know how the outside world works."*

















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
<span>Let's avoid building this</span>
</div>

## Conclusion: SOLID in Rust Context

### Key Takeaways
{: .no_toc }

1. **Single Responsibility Principle**: Separate code by the actors that change it. In Rust, this often means separate modules or structs, not cramming everything into methods on one type.
2. **Open-Closed Principle**: Use traits for extension points. Rust's trait system + enums + pattern matching give we powerful tools for open-closed designs.
3. **LSP**: Make sure our trait implementations honor the contract. Rust's type system catches many violations, but we still need to ensure semantic correctness.
4. **ISP**: Don't create god traits. Split them into focused, cohesive interfaces that clients can compose as needed.
5. **DIP**: Depend on traits (abstractions), not concrete types. Structure our crates so high-level business logic doesn't depend on low-level infrastructure.



### SOLID != Architecture
{: .no_toc }
Remember: Uncle Bob is clear that SOLID is about the **mid-level** (modules, classes, functions). It's not the whole story:

- **Component principles** (coming in Part 4 of Clean Architecture) deal with how to organize crates and manage coupling between them
- **Architecture** (Part 5) deals with the big picture: layers, boundaries, the Dependency Rule

SOLID is the **foundation**. Get these principles right at the class/module level, and we'll have solid components. Get solid components, and we can build solid architectures.

### Rust Makes SOLID Easier (Mostly)
{: .no_toc }

Rust's design actually encourages many SOLID principles:

- **Ownership** forces we to think about responsibilities
- **Traits** make abstraction natural
- **Type system** catches interface violations
- **Module system** encourages separation
- **No inheritance** means we can't violate LSP through deep hierarchies

The one challenge: Rust's explicitness can make DIP feel verbose (lots of generics, trait bounds). But that's actually a feature - the compiler is making dependencies explicit and checking them at compile time.



### Next Steps
{: .no_toc }

* **Practice**: Let's refactor some of our own code using these principles. Let's start simple, with one principle at a time.
* Read the rest of the book Clean Architecture:
    * **Part 4 (Component Principles)**: Learn about organizing crates, managing coupling between components
    * **Part 5 (Architecture)**: The big picture - layers, boundaries, the famous Dependency Rule

We should remember these are **principles**, not rules. There are times when violating them is the pragmatic choice. The key is to **know** we're violating them and why.

Now let's write cleaner Rust! 🦀



## References & Further Reading

- [Clean Architecture](https://amzn.eu/d/9CJWwcy) by Robert C. Martin - the source material
- [serodriguez68/clean-architecture](https://github.com/serodriguez68/clean-architecture) - detailed summary of the book
- Rust's trait system: https://doc.rust-lang.org/book/ch10-02-traits.html
- Hexagonal Architecture: https://alistair.cockburn.us/hexagonal-architecture/
- [Rust is not a faster horse](https://www.youtube.com/watch?v=4YU_r70yGjQ) - Understanding how Rust's paradigm differs from OOP
- The [Coffee Shop Order System companion project](https://github.com/40tude/coffee-shop-solid) on GitHub.

<div align="center">
<img src="./assets/img04.webp" alt="" width="600" loading="lazy"/><br/>
<!-- <span>1984</span> -->
</div>


<!--
## Appendix: Quick Reference Card

```rust
// Single Responsibility Principle: One reason to change
struct Employee { /* data */ }
struct PayrollCalculator; // Accounting's responsibility
struct EmployeeRepository; // DBA's responsibility

// Open-Closed Principle: Open for extension, closed for modification
trait ReportFormatter { fn format(&self) -> String; }
struct PdfFormatter;
impl ReportFormatter for PdfFormatter { /* ... */ }

// LSP: Substitutable implementations
trait Storage {
    fn get(&self, key: &str) -> Result<Option<String>, Error>;
}
// All impls must handle errors consistently

// ISP: Focused traits
trait Readable { fn read(&self) -> &str; }
trait Writable { fn write(&mut self, data: &str); }
// Not: trait Document { fn read(); fn write(); fn everything(); }

// DIP: Depend on abstractions
struct Service<R: Repository> { repo: R }
// Not: struct Service { repo: PostgresRepo }
```
-->