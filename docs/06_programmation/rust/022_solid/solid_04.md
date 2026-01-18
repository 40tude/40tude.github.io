---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide - 04"
description: "A gentle introduction to SOLID principles using Rust. The focus is on Dependency Inversion Principle."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-15 11:00:00
---


# SOLID Principles in Rust: A Practical Guide
{: .no_toc }

A gentle introduction to SOLID principles using Rust. The focus is on Dependency Inversion Principle.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 04
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


## Dependency Inversion Principle (DIP)

### The Principle

> "High-level modules should not depend on low-level modules. Both should depend on abstractions."

This is the **cornerstone of Clean Architecture**. It's what allows us to build a system where:
- Business logic doesn't know about databases
- Our domain doesn't care about HTTP frameworks
- Core code doesn't depend on external libraries

In the book Database and Web are considered as "details".
<!-- The author emphasizes that the flow of data goes in the opposite direction of the dependencies, hence the name Dependency Inversion Principle. -->

In Rust terms: **our high-level business logic should depend on traits, and low-level details (I/O, databases, frameworks) should implement those traits.**







### The Problem: Direct Dependencies

Let's start with a Bad implementation. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_01_dip

// =========================
// Dependency Inversion Principle - Problem
// =========================

mod bad_example {
    // Infrastructure component (low-level)
    pub struct EmailNotifier;

    impl EmailNotifier {
        pub fn send(&self, message: &str) {
            println!("Sending email: {}", message);
        }
    }

    // Business logic (high-level) DEPENDS ON infrastructure
    pub struct OrderService {
        notifier: EmailNotifier, // BAD - Direct dependency on concrete class
    }

    impl OrderService {
        pub fn new() -> Self {
            Self {
                notifier: EmailNotifier, // BAD -  Hardcoded dependency
            }
        }

        pub fn place_order(&self, order_id: u32) {
            println!("Order #{} placed", order_id);
            self.notifier
                .send(&format!("Order #{} confirmed", order_id));
        }
    }
}


fn main() {
    println!("=== Problem: Tight Coupling ===\n");
    let bad_service = bad_example::OrderService::new();
    bad_service.place_order(101);

    println!("\nWhat if we want to add SMS notifications?");
    println!("We'd have to modify OrderService itself!");
}
```

Expected output:

```powershell
=== Problem: Tight Coupling ===

Order #101 placed
Sending email: Order #101 confirmed

What if we want to add SMS notifications?
We'd have to modify OrderService itself!
```

**What's wrong?**
1. `OrderService` (business logic) is tightly coupled to `EmailNotifier` (infrastructure)
2. We can't switch to SMS or Slack notifications without modifying `OrderService`
3. Testing `OrderService` requires a real `EmailNotifier` (no mocking possible)
4. The dependency arrow points **FROM** business logic **TO** infrastructure (wrong direction!)







### The Solution: Invert the Dependency

Now let's see how we can make our high-level business logic should depend on traits, and low-level details (I/O, databases, frameworks, infrastructure) implement those traits. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):


```rust
// cargo run -p ex_02_dip

// =========================
// Dependency Inversion Principle - Fix
// =========================

// DOMAIN layer - defines business logic and the abstractions it needs
mod domain {
    // The business logic DEFINES what it needs
    pub trait Notifier {
        fn send(&self, message: &str);
    }

    // High-level business logic
    pub struct OrderService<N: Notifier> {
        notifier: N,
    }

    impl<N: Notifier> OrderService<N> {
        pub fn new(notifier: N) -> Self {
            Self { notifier }
        }

        pub fn place_order(&self, order_id: u32) {
            println!("Order #{} placed", order_id);
            self.notifier
                .send(&format!("Order #{} confirmed", order_id));
        }
    }
}

// INFRASTRUCTURE layer - adapts to domain requirements
mod infrastructure {
    use crate::domain::Notifier;

    pub struct EmailNotifier;
    pub struct SmsNotifier;

    // Infrastructure IMPLEMENTS what the domain needs
    impl Notifier for EmailNotifier {
        fn send(&self, message: &str) {
            println!("Sending email: {}", message);
        }
    }

    impl Notifier for SmsNotifier {
        fn send(&self, message: &str) {
            println!("Sending SMS: {}", message);
        }
    }
}

fn main() {
    use domain::OrderService;
    use infrastructure::{EmailNotifier, SmsNotifier};

    println!("=== Dependency Inversion Principle ===\n");

    let email_service = OrderService::new(EmailNotifier);
    email_service.place_order(201);

    println!();

    let sms_service = OrderService::new(SmsNotifier);
    sms_service.place_order(202);
}
```


Expected output:

```powershell
=== Dependency Inversion Principle ===

Order #201 placed
Sending email: Order #201 confirmed

Order #202 placed
Sending SMS: Order #202 confirmed
```


In the second source code:
1. The `DOMAIN module` defines the business logic (`OrderService`) and declares
   the abstraction it needs (`Notifier` trait). It doesn't know or care about
   email, SMS, or any specific implementation.
2. The `INFRASTRUCTURE module` contains concrete implementations (EmailNotifier,
   SmsNotifier) that **must adapt** to the interface defined by the domain.
3. Notice the dependency direction:
   - infrastructure imports from domain (`use crate::domain::Notifier`)
   - domain imports **nothing** from infrastructure
   - This is the "inversion": infrastructure depends on domain, not the reverse
4. The `main()` function is the composition root where we wire everything together.

Benefits:
- Business logic is isolated and testable
- We can add new notifiers (Slack, Push, etc.) without touching the domain
- Infrastructure components are pluggable and interchangeable



































### Testing Becomes Trivial

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

Context: It is 8:05 AM. Double espresso. Thank God it’s Friday, but the week is not over yet. You want to test your code, but everything depends on concrete details.

**The question to ask:** *"Does my high-level logic depend on details, or do details depend on my logic?"*

* If business rules directly depend on frameworks, databases, or external services, DIP is likely violated.
* The Dependency Inversion Principle is not about abstractions everywhere, but about **protecting policy from implementation details**.
* The Dependency Inversion Principle is a thinking tool that helps us say: *"My core logic should not know how the outside world works."*

















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Conclusion: SOLID in Rust Context

<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
<span>Let's avoid building this</span>
</div>

### Key Takeaways

1. **Single Responsibility Principle**: Separate code by the actors that change it. In Rust, this often means separate modules or structs, not cramming everything into methods on one type.
2. **Open-Closed Principle**: Use traits for extension points. Rust's trait system + enums + pattern matching give we powerful tools for open-closed designs.
3. **LSP**: Make sure our trait implementations honor the contract. Rust's type system catches many violations, but we still need to ensure semantic correctness.
4. **ISP**: Don't create god traits. Split them into focused, cohesive interfaces that clients can compose as needed.
5. **DIP**: Depend on traits (abstractions), not concrete types. Structure our crates so high-level business logic doesn't depend on low-level infrastructure.






### SOLID != Architecture

Remember: Uncle Bob is clear that SOLID is about the **mid-level** (modules, classes, functions). It's not the whole story:

- **Component principles** (coming in Part 4 of Clean Architecture) deal with how to organize crates and manage coupling between them
- **Architecture** (Part 5) deals with the big picture: layers, boundaries, the Dependency Rule

SOLID is the **foundation**. Get these principles right at the class/module level, and we'll have solid components. Get solid components, and we can build solid architectures.







### Rust Makes SOLID Easier (Mostly)

Rust's design actually encourages many SOLID principles:

- **Ownership** forces we to think about responsibilities
- **Traits** make abstraction natural
- **Type system** catches interface violations
- **Module system** encourages separation
- **No inheritance** means we can't violate LSP through deep hierarchies

The one challenge: Rust's explicitness can make DIP feel verbose (lots of generics, trait bounds). But that's actually a feature - the compiler is making dependencies explicit and checking them at compile time.






### Next Steps

* **Practice**: Let's refactor some of our own code using these principles. Let's start simple, with one principle at a time.
* Read the rest of the book Clean Architecture:
    * **Part 4 (Component Principles)**: Learn about organizing crates, managing coupling between components
    * **Part 5 (Architecture)**: The big picture - layers, boundaries, the famous Dependency Rule

We should remember these are **principles**, not rules. There are times when violating them is the pragmatic choice. The key is to **know** we're violating them and why.

Now let's write cleaner Rust! 🦀













<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## References & Further Reading

- [Clean Architecture](https://amzn.eu/d/9CJWwcy) by Robert C. Martin - the source material
- [serodriguez68/clean-architecture](https://github.com/serodriguez68/clean-architecture) - detailed summary of the book
- Rust's trait system: https://doc.rust-lang.org/book/ch10-02-traits.html
- Hexagonal Architecture: https://alistair.cockburn.us/hexagonal-architecture/
- [Rust is not a faster horse](https://www.youtube.com/watch?v=4YU_r70yGjQ) - Understanding how Rust's paradigm differs from OOP
- The code of the posts is available in the [solid_test repo on GitHub](https://github.com/40tude/solid_test)
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


## Next Step
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/022_solid/solid_00.md%}): Introduction + Single Responsibility Principle
* [Episode 01]({%link docs/06_programmation/rust/022_solid/solid_01.md%}): Open-Closed Principle
* [Episode 02]({%link docs/06_programmation/rust/022_solid/solid_02.md%}): Liskov Substitution Principle
* [Episode 03]({%link docs/06_programmation/rust/022_solid/solid_03.md%}): Interface Segregation Principle
* [Episode 04]({%link docs/06_programmation/rust/022_solid/solid_04.md%}): Dependency Inversion Principle + Conclusion
