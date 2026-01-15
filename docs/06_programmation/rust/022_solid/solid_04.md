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


**Note:**
1. The code of this post are also available in the [solid_test repo on GitHub](https://github.com/40tude/solid_test)
1. In addition, feel free to play, break, rebuild the [Coffee Shop Order System companion project](https://github.com/40tude/coffee-shop-solid) on GitHub












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

This is the **cornerstone of Clean Architecture**. It's what allows we to build a system where:
- Business logic doesn't know about databases
- our domain doesn't care about HTTP frameworks
- Core code doesn't depend on external libraries

In Rust terms: **our high-level business logic should depend on traits, and low-level details (I/O, databases, frameworks) should implement those traits.**











### The Problem: Direct Dependencies

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