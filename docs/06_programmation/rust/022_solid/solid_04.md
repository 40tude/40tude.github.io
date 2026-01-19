---
published: true
lang: en-US
lawet: default
title: "SOLID Principles in Rust: A Practical Guide - 04"
description: "A gentle introduction to SOLID principles using Rust. The focus is on Dependency Inversion Principle."
parent: "Rust"
nav_order: 31
date:               2026-01-12 16:00:00
last_modified_date: 2026-01-19 11:00:00
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
}
```

Expected output:

```powershell
=== Problem: Tight Coupling ===

Order #101 placed
Sending email: Order #101 confirmed
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
// Dependency Inversion Principle - Solution
// =========================

// DOMAIN layer - defines business logic and the abstractions it needs
mod domain {
    // The business logic DEFINES what it needs
    pub trait Notifier {
        fn send(&self, message: &str);
    }

    // Business logic (high-level) DEPENDS ON abstraction
    pub struct OrderService<N: Notifier> {
        notifier: N, // Depends on trait, not concrete class
    }

    impl<N: Notifier> OrderService<N> {
        pub fn new(notifier: N) -> Self {
            Self { notifier } // Injected dependency
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
    use crate::domain::Notifier; // Infrastructure depends on domain

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
- Easy to test with mock implementations



































### Testing Becomes Trivial

Thanks to DIP, we can test the business logic without any infrastructure:
- No email server needed
- No SMS gateway required
- No network calls
- Fast, reliable, isolated tests

We simply create a mock that implements the Notifier trait. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo test -p ex_03_dip
// cargo run -p ex_03_dip

// =========================
// Dependency Inversion Principle - Solution
// =========================

// DOMAIN layer - defines business logic and the abstractions it needs
mod domain {
    // The business logic DEFINES what it needs
    pub trait Notifier {
        fn send(&self, message: &str);
    }

    // Business logic (high-level) DEPENDS ON abstraction
    pub struct OrderService<N: Notifier> {
        notifier: N, // Depends on trait, not concrete class
    }

    impl<N: Notifier> OrderService<N> {
        pub fn new(notifier: N) -> Self {
            Self { notifier } // Injected dependency
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
    use crate::domain::Notifier; // Infrastructure depends on domain

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

// =========================
// TESTING - The real benefit of DIP
// =========================

#[cfg(test)]
mod tests {
    use super::domain::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Mock notifier for testing - no real infrastructure needed!
    struct MockNotifier {
        messages: Rc<RefCell<Vec<String>>>, // Shared ownership for verification
    }

    impl MockNotifier {
        fn new() -> (Self, Rc<RefCell<Vec<String>>>) {
            let messages = Rc::new(RefCell::new(Vec::new()));
            (
                Self {
                    messages: Rc::clone(&messages),
                },
                messages,
            )
        }
    }

    // Implement the domain's trait - that's all we need!
    impl Notifier for MockNotifier {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_order_service_sends_notification() {
        // Arrange: Create service with mock
        let (mock, messages) = MockNotifier::new();
        let service = OrderService::new(mock);

        // Act: Execute business logic
        service.place_order(42);

        // Assert: Verify the notification was sent
        let msgs = messages.borrow();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0], "Order #42 confirmed");
    }

    #[test]
    fn test_multiple_orders() {
        // Arrange
        let (mock, messages) = MockNotifier::new();
        let service = OrderService::new(mock);

        // Act: Place multiple orders
        service.place_order(100);
        service.place_order(101);
        service.place_order(102);

        // Assert: All notifications were sent
        let msgs = messages.borrow();
        assert_eq!(msgs.len(), 3);
        assert!(msgs[0].contains("Order #100"));
        assert!(msgs[1].contains("Order #101"));
        assert!(msgs[2].contains("Order #102"));
    }

    #[test]
    fn test_notification_format() {
        // Arrange
        let (mock, messages) = MockNotifier::new();
        let service = OrderService::new(mock);

        // Act
        service.place_order(999);

        // Assert: Verify exact message format
        let msgs = messages.borrow();
        assert_eq!(msgs[0], "Order #999 confirmed");
    }

    // We could also test error cases, edge cases, etc.
    // All without touching any real infrastructure!
}
```
Use the Test button on the left hand side.

<div align="center">
<img src="./assets/img09.webp" alt="" width="900" loading="lazy"/><br/>
<span>Running tests in Rust Playground</span>
</div>


Expected output:

```
running 3 tests
test tests::test_multiple_orders ... ok
test tests::test_notification_format ... ok
test tests::test_order_service_sends_notification ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```




Key benefits :

1. Fast tests: No network, no I/O, no delays
   - Tests run in milliseconds, not seconds
2. Reliables tests: No flaky external dependencies
   - No "email server down" failures
   - No network timeouts
   - 100% deterministic
3. Isolated tests: Focus on business logic only
   - Test what the service DOES, not HOW it sends notifications
   - Infrastructure bugs don't break domain tests
4. Easy to write: Simple mock, implements one trait
   - No complex mocking framework needed
   - Clear and readable test code

Let's compare this to the "bad example" where `OrderService` depends on `EmailNotifier`:
- We would need a real email server or complex mocking
- Tests would be slow and potentially flaky
- Hard to test edge cases
- Coupling business logic tests to infrastructure details















### Real-World: Hexagonal Architecture

DIP is the foundation of **Hexagonal Architecture** (also called Ports & Adapters), a pattern that keeps our business logic completely independent from external systems.

**The key insight**: Our business logic shouldn't know whether it's using PostgreSQL or MongoDB, Stripe or PayPal, SendGrid or AWS SES. It should only know "I need to store orders" or "I need to process payments".

Here's how it works:

1. **Domain (Core):** Pure business logic with zero dependencies on frameworks or external systems
2. **Ports (Abstractions):** Traits that define what the domain needs (defined BY the domain, not imposed ON it)
3. **Adapters (Infrastructure):** Concrete implementations that plug into the ports

Think of it like a smartphone: the core device defines the ports it needs (USB-C, Lightning), and different adapters can plug in (chargers, headphones, external drives). The phone doesn't care which adapter are used, as long as it implements the correct interface.

Let's see this in action with a simplified order processing system. You can copy and paste the code below in [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run -p ex_04_dip

// =========================
// Hexagonal Architecture - aka Ports & Adapters
// =========================

// DOMAIN Layer (Core Business Logic)
mod domain {
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OrderId(pub u32);

    #[derive(Debug, Clone, Copy)]
    pub struct Money(pub u32); // cents

    #[derive(Debug, Clone)]
    pub struct LineItem {
        pub name: String,
        pub price: Money,
    }

    #[derive(Debug, Clone)]
    pub struct Order {
        pub id: OrderId,
        pub items: Vec<LineItem>,
        pub total: Money,
    }

    #[derive(Debug)]
    pub enum OrderError {
        InvalidOrder,
        PaymentFailed,
        StorageFailed,
        NotificationFailed,
    }

    impl fmt::Display for OrderError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Order {
        pub fn new(id: OrderId, items: Vec<LineItem>) -> Result<Self, OrderError> {
            if items.is_empty() {
                return Err(OrderError::InvalidOrder);
            }

            let total = Money(items.iter().map(|item| item.price.0).sum());

            Ok(Order { id, items, total })
        }
    }
}

// PORTS (Abstractions defined by domain)
mod ports {
    use crate::domain::*;

    // Output port: domain needs to persist orders
    pub trait OrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError>;
        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError>;
    }

    // Output port: domain needs to process payments
    pub trait PaymentGateway {
        fn charge(&self, amount: Money) -> Result<(), OrderError>;
    }

    // Output port: domain needs to send notifications
    pub trait NotificationService {
        fn send_confirmation(&self, order: &Order) -> Result<(), OrderError>;
    }
}

// APPLICATION service (Orchestrates domain + ports)
mod application {
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
        next_id: u32,
    }

    impl<R, P, N> OrderService<R, P, N>
    where
        R: OrderRepository,
        P: PaymentGateway,
        N: NotificationService,
    {
        pub fn new(repository: R, payment: P, notifications: N) -> Self {
            Self {
                repository,
                payment,
                notifications,
                next_id: 1,
            }
        }

        pub fn place_order(&mut self, items: Vec<LineItem>) -> Result<Order, OrderError> {
            // Pure business logic - no infrastructure concerns!
            let order_id = OrderId(self.next_id);
            self.next_id += 1;

            let order = Order::new(order_id, items)?;

            // Use abstractions, not concrete implementations
            self.payment.charge(order.total)?;
            self.repository.save(&order)?;
            self.notifications.send_confirmation(&order)?;

            Ok(order)
        }

        pub fn get_order(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            self.repository.find(id)
        }
    }
}

// ADAPTERS - Implementation #1 (In-Memory)
mod in_memory_adapters {
    use crate::domain::*;
    use crate::ports::*;
    use std::collections::HashMap;

    pub struct InMemoryOrderRepository {
        orders: HashMap<OrderId, Order>,
    }

    impl InMemoryOrderRepository {
        pub fn new() -> Self {
            Self {
                orders: HashMap::new(),
            }
        }
    }

    impl OrderRepository for InMemoryOrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError> {
            println!("[InMemory] Saving order #{:?}", order.id);
            self.orders.insert(order.id, order.clone());
            Ok(())
        }

        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            println!("[InMemory] Finding order #{:?}", id);
            Ok(self.orders.get(&id).cloned())
        }
    }

    pub struct MockPaymentGateway;

    impl PaymentGateway for MockPaymentGateway {
        fn charge(&self, amount: Money) -> Result<(), OrderError> {
            println!("[Mock] Charging ${}.{:02}", amount.0 / 100, amount.0 % 100);
            Ok(())
        }
    }

    pub struct ConsoleNotificationService;

    impl NotificationService for ConsoleNotificationService {
        fn send_confirmation(&self, order: &Order) -> Result<(), OrderError> {
            println!(
                "[Console] Order #{:?} confirmed - Total: ${}.{:02}",
                order.id,
                order.total.0 / 100,
                order.total.0 % 100
            );
            Ok(())
        }
    }
}

// ADAPTERS - Implementation #2 (Simulated External Services)
mod external_adapters {
    use crate::domain::*;
    use crate::ports::*;
    use std::collections::HashMap;

    pub struct PostgresOrderRepository {
        simulated_db: HashMap<OrderId, Order>,
    }

    impl PostgresOrderRepository {
        pub fn new() -> Self {
            Self {
                simulated_db: HashMap::new(),
            }
        }
    }

    impl OrderRepository for PostgresOrderRepository {
        fn save(&mut self, order: &Order) -> Result<(), OrderError> {
            println!("[Postgres] INSERT INTO orders VALUES ({:?}, ...)", order.id);
            self.simulated_db.insert(order.id, order.clone());
            Ok(())
        }

        fn find(&self, id: OrderId) -> Result<Option<Order>, OrderError> {
            println!("[Postgres] SELECT * FROM orders WHERE id = {:?}", id);
            Ok(self.simulated_db.get(&id).cloned())
        }
    }

    pub struct StripePaymentGateway;

    impl PaymentGateway for StripePaymentGateway {
        fn charge(&self, amount: Money) -> Result<(), OrderError> {
            println!(
                "[Stripe API] POST /charges amount=${}.{:02}",
                amount.0 / 100,
                amount.0 % 100
            );
            Ok(())
        }
    }

    pub struct SendGridNotificationService;

    impl NotificationService for SendGridNotificationService {
        fn send_confirmation(&self, order: &Order) -> Result<(), OrderError> {
            println!("[SendGrid API] POST /mail/send to=customer@example.com subject='Order #{:?} Confirmed'",
                order.id);
            Ok(())
        }
    }
}

// Demonstrating Swappable Adapters
fn main() {
    use application::OrderService;
    use domain::{LineItem, Money, OrderId};
    use external_adapters::*;
    use in_memory_adapters::*;

    println!("=== Hexagonal Architecture Demo ===\n");

    // Create test items
    let items = vec![
        LineItem {
            name: "Rust Programming Book".to_string(),
            price: Money(4999), // $49.99
        },
        LineItem {
            name: "Mechanical Keyboard".to_string(),
            price: Money(12999), // $129.99
        },
    ];

    println!("--- Configuration #1: In-Memory Adapters (Testing) ---\n");
    {
        let repo = InMemoryOrderRepository::new();
        let payment = MockPaymentGateway;
        let notifications = ConsoleNotificationService;

        let mut service = OrderService::new(repo, payment, notifications);

        match service.place_order(items.clone()) {
            Ok(order) => println!("Order placed successfully: {:?}\n", order.id),
            Err(e) => println!("Error: {}\n", e),
        }
    }

    println!("--- Configuration #2: External Services (Production) ---\n");
    {
        let repo = PostgresOrderRepository::new();
        let payment = StripePaymentGateway;
        let notifications = SendGridNotificationService;

        let mut service = OrderService::new(repo, payment, notifications);

        match service.place_order(items.clone()) {
            Ok(order) => {
                println!("Order placed successfully: {:?}", order.id);

                // Demonstrate retrieval
                println!();
                if let Ok(Some(retrieved)) = service.get_order(order.id) {
                    println!(
                        "Retrieved order: {} items, total ${}.{:02}",
                        retrieved.items.len(),
                        retrieved.total.0 / 100,
                        retrieved.total.0 % 100
                    );
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
```

Expected output:

```powershell
=== Hexagonal Architecture Demo ===

--- Configuration #1: In-Memory Adapters (Testing) ---

[Mock] Charging $179.98
[InMemory] Saving order #OrderId(1)
[Console] Order #OrderId(1) confirmed - Total: $179.98
Order placed successfully: OrderId(1)

--- Configuration #2: External Services (Production) ---

[Stripe API] POST /charges amount=$179.98
[Postgres] INSERT INTO orders VALUES (OrderId(1), ...)
[SendGrid API] POST /mail/send to=customer@example.com subject='Order #OrderId(1) Confirmed'
Order placed successfully: OrderId(1)

[Postgres] SELECT * FROM orders WHERE id = OrderId(1)
Retrieved order: 2 items, total $179.98
```



The beauty of this architecture:

1. **Zero coupling to infrastructure:** The `OrderService` has no idea whether it's using PostgreSQL or MongoDB, Stripe or PayPal, SendGrid or AWS SES. It only knows the abstractions (traits).

2. **Swap implementations instantly:** Want to switch from Postgres to MongoDB? Write a `MongoOrderRepository` that implements `OrderRepository`. Want to test without hitting real APIs? Use mock implementations. The domain code never changes.

3. **Business logic stays pure:** Look at `OrderService::place_order()`. It's just business rules. No SQL queries, no HTTP calls, no JSON parsing. This makes it easy to understand, test, and maintain.

4. **Dependencies flow inward:** Infrastructure (adapters) depends on domain (ports), not the other way around. This is DIP in action:
   - `PostgresOrderRepository` depends on `OrderRepository` trait (defined in domain)
   - `StripePaymentGateway` depends on `PaymentGateway` trait (defined in domain)
   - Domain depends on... nothing.

5. **Testing becomes trivial:** We can test our business logic with simple in-memory implementations (like we did before). No need for test databases, API mocks, or complex setup.

This is why Hexagonal Architecture is so powerful in real-world applications: **our business logic is completely isolated and protected from the chaos of external systems**. When Stripe changes their API, we only touch the `StripePaymentGateway` adapter. When we migrate from Postgres to MongoDB, we only write a new `MongoOrderRepository`. The core of our application - the business logic - remains stable and unchanged.

**Real-world impact**: At scale, this means our core domain can live for years while adapters come and go. You can refactor infrastructure, switch cloud providers, or adopt new technologies without rewriting your business logic.



**Note:**

In the [solid_test repository](https://github.com/40tude/solid_test) on GitHub the [dip_05](https://github.com/40tude/solid_test/tree/main/dip_05/src) workspace contains a modularized version of the previous sample code. Since we cannot have workspaces in workspace, in this version the different components are distributed among sub directories (adapters, application, domain, port). For example, adapters are split in two groups : in_memory and external.

<div align="center">
<img src="./assets/img10.webp" alt="" width="900" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>



**Note:**

A last version of the same code is available in the [hexagonal architecture](https://github.com/40tude/hexagonal_architecture)  repo on GitHub. In this project components are in their own workspace, the project include testing, a readme file. Looks much more like what we should do in real life.

The output in the console are exactly the same. However, in this version, the components are organized by their type: `adapters-notification`, `adapters-payment`...

<div align="center">
<img src="./assets/img11.webp" alt="" width="900" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>






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

4. **The "one binary" concern**: Even though Rust compiles to one binary, the crate structure enforces dependency direction at compile time. We **cannot** accidentally import `postgres` in our domain crate if domain doesn't list it in `Cargo.toml`.











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
- The [hexagonal architecture](https://github.com/40tude/hexagonal_architecture) demo on GitHub.
- The [Coffee Shop Order System](https://github.com/40tude/coffee-shop-solid)  companion project on GitHub.


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
