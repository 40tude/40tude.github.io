---
published: true
lang: en-US
layout: default
title: "Hexagonal Architecture in Rust from the Ground Up"
description: "A gentle introduction with working examples."
parent: "Rust"
nav_order: 33
date:               2026-01-24 15:00:00
last_modified_date: 2026-01-24 15:00:00
---


# Hexagonal Architecture in Rust from the Ground Up
{: .no_toc }

A gentle introduction with working examples.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* For beginners

<!-- **Note**
The [companion project](https://github.com/40tude/coroutines_and_friends) with all the examples is available on GitHub. -->

<div align="center">
<img src="./assets/img00.webp" alt="" width="1024" loading="lazy"/><br/>
<span>2005: When Hexagonal Architecture emerged and War of the Worlds hit theaters.</span>
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

## About ex00.rs
Let's talk with the board: CFO, COO, CEO... They speak "business" they have their own vocabulary, rules, invariants... To tell the truth, they don't care if they received orders via email or by owl. They don't care if orders are tracked in a database or on papyrus... They want to process as many orders as possible, ship products in a matter of minutes and send the invoices within the next second.

If we must write an application for these guys one of the idea is to decouple the business from the external concerns like user interfaces, databases... What  "Uncle Bob's" call the "details" in the book [Clean Architecture](https://www.amazon.fr/dp/0134494164).

Do not misinterpret the word "business" here. We could be talking about "how to operate a nuclear plant", "how to operate a cabaret"...

One option could be to apply the Dependency Inversion Principle. You can read more about DIP on this [page](https://www.40tude.fr/docs/06_programmation/rust/022_solid/solid_04.html).
However real applications are complex: they need storage, payment systems, notification services... How do we scale DIP to handle all of that?

This is the purpose of the Hexagonal Architecture which leverages what we already know about SOLID and DIP but go one step further.

At the heart of Hexagonal Architecture are the concepts of **Ports** and **Adapters**.
* Ports are **interfaces** that define contracts for the communications between the business and the components.
* Adapters are **implementations** of these interfaces.

**Note:**
Think of it that way: "Ciao ragazze! I'm the latest Ferrari cell phone and you know what? If you want to reload my battery, connect an external drive or a ledger, like on any car you will have to use an OBD-II port."

<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
<span>This is an OBD-II connector</span>
</div>

I know this is ridiculous... However keep in mind that this is the phone which impose the port. As a supplier, if you want to get the "Ferrari Compatibility Label" you have no choice than to provide an OBD-II compliant connector for your wired earphones. Good luck!

As on any electronic device there are two types of Ports:
* Output Ports: Describe the methods the business is ready to use to send "Stuff" outside. Here, "Stuff" may be a invoice, notifications... I say "ready to use" because I want to insist on the fact that this is the business, the core, which defines the means to be used. This is nothing else than DIP in action.
* Input Ports: Describe how the business is ready to receive "Stuff" from external systems. Here "Stuff" could be a messages, an order... Again, the core define the methods that external systems will have to use to trigger actions in the core. The keyword here is "will have to use". Yes, this is 4 words but the point is that this is mandatory. If you don't use the input port the business will not take your request/call into account.

Things to keep in mind:
* **Ports = Traits** (Interfaces in plain Rust)
* **Adapters = Implementations** (Rust `impl Xyz for Abc`)

Now... The one million dollar question. Is there a way to demonstrate an Hexagonal Architecture in less than 100 lines of code?

```powershell
cargo run --example ex00
```

Expected output:

```text
[Console] Order #1 confirmed! Total: 4999
Success! Order #1 processed.
```

<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## `ex00.rs` code review

I want to make sur "we learn how to walk before we try to run". So let's take some time to review the code of `ex00.rs` and understand how this works at the simplest level.

**Lines 1-20**
We left the meeting with the board and we now, know their vocabulary. We create a module named `domain` where we recreate the entities we heard about.

First we create the type `Order`. What is an `Order`? An `Order` is an `id` associated with a `total`. We may not agree but, if tomorrow morning we cross the CFO at the cafeteria, we can discuss with him using *his* vocabulary. He will not be lost and more than happy to help us. If needed we will be able to add field to the `struct` later.

Second, we create a type `OrderError`. What is an `OrderError`? It is not yet crystal clear but we know for sure that an error is only one kind of error at a time so we create an empty `enum`. Once we know more about the kind of business they encounter in their domain we will be able to add variants. At the end, we implement the Display trait for the `OrderError`.



**Lines 22-28**
We create a `ports` module. "Remember the Alamo" but remember that **ports = traits**. So we "just" define which methods must be available if something want to be considered as an `OrderNotifier`.

Note that the module `ports` depends on `domain` because the `.process()` method sends a `domain::Order` and return a `Result<(), domain::OrderError>`.



**Lines 30-45**
We said **adapters = implementations**. In this module we define a `ConsoleNotifier`, a concrete data type. Once this is done, since we want that the objects of type `ConsoleNotifier` being used to process orders, we implement the `OrderNotifier` trait for the `ConsoleNotifier`. In our case this consist in defining what will happen when the `.process()` method will be invoked.

Note that in the the module `adapters` we depend on `domain` and `ports` because we implement a trait defined in `ports` which act on objects defined in `domain` (`Order` for example)



**Lines 47-74**
For me, this is where the magic takes place.

Indeed, this is where we define and implement a generic `OrderService` data type over the trait `OrderNotifier`. Realize that this means that *any* concrete type having the `OrderNotifier` trait can be used.

So far, only the `ConsoleNotifier` data type has the trait `OrderNotifier` but, may be tomorrow, we can easily extend the application with a `BellNotifier` (people ring a bell on each order received).

Once the generic `OrderService` is defined, then we write its implementation which consist of 2 methods: `new()` and `.process_order()`. The latter use the `.process()` method that any object having the `OrderNotifier` trait has.

Take some time to *read* and understand each line.

Note that in the the module `application` we depend on `domain` and `ports` because we create a `OrderService` data type which depends on a `OrderNotifier` trait defined in ports which act on object defined in `domain` (`Order` for example)


**Lines 76-86**
Show time! All the pieces fit together. The code involved is very short. This is NOT the problem. My main concerne here is to make sure we all understand is that above, `application` is NOT an executable. For obvious didactic reasons I keep all the modules in one source code, but `application` is NOT an executable but more a lib.

In this context `main()` is:
* Not part of Hexagonal Architecture*
* Not part of the application layer
* Just a delivery mechanism
* A primary adapter

Think of it as a "client", a proof that the architecture works. `main()` belongs to an adapter and does not include any business logic. From the files and folder organization you can keep this model in mind if this help.

```text
my_app/
├── domain/
├── application/
├── adapters/
├── ports/
├── main.rs
```

Now, having this in mind we can *read* the code of `main()`. We create a new `OrderService` based on the data type `ConsoleNotifier` which implements the trait `OrderNotifier`. Then we call `.process_order()` and print the results.






**Notes:**
* Naming is difficult. Order, Invoice, Notifier... I realize things should/could be better. Welcome to the club! Hopefully, intellisense (F2) is our friend.
* I would suggest to start by "fixing the vocabulary". I mean, first, work on the `domain` module and define the datatype of the business you are modeling. Make sure every body agree.
* Then write the `main()` function as you would describe, in plain English, what is going on.
* Then I would work on the `ports`, `adapters` and `application` modules in that order.
* Finalize the `main()` function adding the needed `use...` statements
* Sleep on it
* Review it with a colleague tomorrow morning.


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## `ex00.rs` types organization

* Below I use `Sender` instead of `OrderNotifier`
* The arrows show the dependencies: ports depends on domain, domain depends on no one.

<div align="center">
<img src="./assets/img01.webp" alt="" width="900" loading="lazy"/><br/>
<span>Visual Representation of our Hexagonal Architecture </span>
</div>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## About ex01.rs
The code is the same but the domain is slightly different. I just want to make sure we understand that business/core/domain can be anything.

```powershell
cargo run --example ex01
```

**Expected output:**
```text
[Megaphone] 🎪 Act #1 is ON! Silliness level: 9001
🤡 Success! Clown act #1 scheduled.
```

Skim over the code because, compared to `ex00.rs` I just renamed some variables.


<div align="center">
<img src="./assets/img02.webp" alt="" width="900" loading="lazy"/><br/>
<span>Optional comment</span>
</div>





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Testing `application`

Again, the previous `main()` function was, may be, misleading. So let's go extreme and let's remove it completely. And you know what? Let's remove also the `adapter` that `main()` was using. Remember, adapters are replaceable and owned by the caller, not by the application.

Ok... But what can we do next? Easy, let's write some test. Try this:

```powershell
cargo test --example ex02
```

Expected output:

```text
running 1 test
test tests::process_order_successfully ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Obviously we cannot `cargo test --example ex02` but this simple experimentation is an attempt to demonstrate some of the key ideas are o the Hexagonal Architecture:

* The `domain` and `application` core do not depend on infrastructure
* The `application` is driven by `ports`, `NOT` by a runtime entry point (`main()`)
* We can exercise the system without any real `adapters`

So let's keep in mind that by replacing `main()` with one focused test we prove ourselves that:
* The application core is usable in isolation
* Adapters are just plug-ins
* The system is testable by construction

Let's see the making of and open `ex02.rs`

**Lines 59-82**
This is the test module. If you remember, in `ex00.rs`, `main()` was using a `ConsoleNotifier` which was defined and implemented in the `adapters` module (lines 30-45 of `ex00.rs`). Here we no longer have the `adapters` module.

So, first thing first, at the top of the `tests` module we start by creating our own `TestNotifier` and we implement `OrderNotifier` for it. In other words we are creating our test adapter.

When this is done, we can write the test `process_order_successfully()` where, as before in the `main()` function, we create a new `OrderService` based on the data type `TestNotifier` which implements the trait `OrderNotifier`. Then we call `.process_order()` and check the results.


Now, it should be clear that in an Hexagonal Architecture the `application` never knows whether it’s being used by a CLI, a web server, a test...

The caller decides which adapter to plug in. Finally, realize that tests are not special. They are just another driving adapter.








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## I'm a liar

In order to keep `ex00.rs` as simple as possible I used a trick and I'm not very proud of. So let's look at it in detail because, as you know... The evil is in the details.

To make a long story short, in `ex00.rs` the module application was owning a `Notifier`. Open `ex00.rs`, search the application module. We have:

```rust
pub struct OrderService<N: OrderNotifier> {
    notifier: N,
    next_id: u32,
}
```
This is NOT wrong. This a choice of design with some advantages:
* Simple to understand
* No lifetimes
* No references
* Works perfectly if:
    * there is only one adapter
    * the adapter has no observable state
    * we never need to access the adapter again

For a first contact with hexagonal ideas, it was totally fine. However, this becomes a problem as soon as we want to experiment with more advanced, but typical, hexagonal scenarios:
* Multiples adapters on the same port
* An adapter with internal state
* Observing what an adapter did
* Reusing the same adapter instance elsewhere

With the original design, the adapter is hidden inside the `application`, the outside world cannot see it and accessing it would require:
* exposing internal fields (bad)
* or even worse adding getters just for adapters
* or redesigning later anyway

So now that we understand the basic implementation (`ex00.rs`) let's slightly refine the initial design without changing the behavior.

**Step 1:**
Most important change. In the `application` module, move from

```rust
pub struct OrderService<N: OrderNotifier> {
    notifier: N,
    next_id: u32,
}
```
to

```rust
pub struct OrderService<'a, N: OrderNotifier> {
    notifier: &'a N,
    next_id: u32,
}
```
Now the application has a reference to the `Notifier` (it has a reference to a variable have the trait `OrderNotifier`)

The lifetime `'a` is mandatory. It’s a nudge for the compiler and a promise from us that the referenced notifier will live at least as long as the `OrderService` that uses it.

Then the implementation has to be modified. We go from

```rust
impl<N: OrderNotifier> OrderService<N> {
    pub fn new(notifier: N) -> Self {
        Self {
            notifier,
            next_id: 1,
        }
    }
    // Same as before
}
```

To

```rust
impl<'a, N: OrderNotifier> OrderService<'a, N> {
    pub fn new(notifier: &'a N) -> Self {
        Self {
            notifier,
            next_id: 1,
        }
    }
    // Same as before
}
```
Last change occurs in `main()` we transition from :

```rust
fn main() {
    // Same as before
    let mut service = OrderService::new(ConsoleNotifier);
    // Same as before
}
```
To

```rust
fn main() {
    // Same as before
    let notifier = ConsoleNotifier;
    let mut service = OrderService::new(&notifier);
    // Same as before
}
```

Does this work?
```powershell
cargo run --example ex03
```

Expected output:

```text
[Console] Order #1 confirmed! Total: 4999
Success! Order #1 processed.
```

Let's keep in mind that the `application` is not a container for adapters. It is a consumer of `ports`.






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## More than one adapters on the same port

In VSCode if you compare `ex03.rs` and `ex04.rs` you will realize that
* the `domain` module remains untouched
* `ports` is not modified
* `application` is not modified either

You should see something like this:

<div align="center">
<img src="./assets/img04.webp" alt="" width="900" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>


In a typical hexagonal architecture, first note that it is OK to have more than one adapter to the same port. Second, adding a new adapter to an existing port does not require any changes to the `domain`, `ports`, or `application` modules.

Obviously we must add a new adapter in the `adapters` module. This is done by adding the a `use std::cell::RefCell;` at the top of the module plus the lines below:

```rust
    pub struct InMemoryNotifier {
        messages: RefCell<Vec<String>>,
    }

    impl InMemoryNotifier {
        pub fn new() -> Self {
            Self {
                messages: RefCell::new(Vec::new()),
            }
        }

        pub fn messages(&self) -> Vec<String> {
            self.messages.borrow().clone()
        }
    }

    impl OrderNotifier for InMemoryNotifier {
        fn process(&self, order: &Order) -> Result<(), OrderError> {
            self.messages.borrow_mut().push(format!(
                "Order #{} stored, total = {}",
                order.id, order.total
            ));
            Ok(())
        }
    }

```

The second and last modification is in `main()`. We first bring the brand new `InMemoryNotifier` into the local scope and use it as shown below:

```rust
let memory_notifier = InMemoryNotifier::new();
let mut memory_service = OrderService::new(&memory_notifier);
memory_service.process_order(42).unwrap();

for message in memory_notifier.messages() {
    println!("[Memory] {message}");
}
```

That's all. But does it works? Let's test the 2 adapters on the same port.

```powershell
cargo run --example ex04
```

Expected output:

```text
[Console] Order #1 confirmed! Total: 4999
[Memory] Order #1 stored, total = 42
```










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Tea Time! Let's take a break

Let's step back for a while in order to internalize the shape of the architecture, forget the details and avoiding to be distracted by behavior. At this point I want to make sure we remember that:
* domain contains the entities of the business
* ports = traits, they are stable contracts
* adapters = implementation, they are placeholders
* application borrows adapters
* tests show how the system is driven

Now *read* the template below:

```rust
mod domain {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Stuff {
        pub value: u32,
    }

    #[derive(Debug)]
    pub enum StuffError {}
}

mod ports {
    use crate::domain::{Stuff, StuffError};

    pub trait StuffHandler {
        fn handle(&self, stuff: &Stuff) -> Result<(), StuffError>;
    }
}

mod adapters {
    use crate::domain::{Stuff, StuffError};
    use crate::ports::StuffHandler;

    pub struct MyAdapter;

    impl StuffHandler for MyAdapter {
        fn handle(&self, _stuff: &Stuff) -> Result<(), StuffError> {
            todo!("Adapter implementation goes here");
        }
    }
}

mod application {
    use crate::domain::{Stuff, StuffError};
    use crate::ports::StuffHandler;

    pub struct StuffService<'a, H: StuffHandler> {
        handler: &'a H,
    }

    impl<'a, H: StuffHandler> StuffService<'a, H> {
        pub fn new(handler: &'a H) -> Self {
            Self { handler }
        }

        pub fn process(&self, value: u32) -> Result<Stuff, StuffError> {
            let stuff = Stuff { value };
            self.handler.handle(&stuff)?;
            Ok(stuff)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::StuffService;
    use crate::domain::{Stuff, StuffError};
    use crate::ports::StuffHandler;

    struct TestHandler;

    impl StuffHandler for TestHandler {
        fn handle(&self, _stuff: &Stuff) -> Result<(), StuffError> {
            Ok(())
        }
    }

    #[test]
    fn process_stuff_successfully() {
        let service = StuffService::new(&TestHandler);

        let stuff = service.process(42).unwrap();

        assert_eq!(stuff.value, 42);
    }
}
```

For me the key is in the `application` module when we define (then implement) a generic `StuffService` data type over the trait `StuffHandle`.


```rust
pub struct StuffService<'a, H: StuffHandler> {
    handler: &'a H,
}
```


You can even use the template as it is:

```powershell
cargo test --example ex05
```

Expected output:

```text
running 1 test
test tests::process_stuff_successfully ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Multiple ports, multiple adapters

Why the repository is &mut but the notifier is not

* This is an important teaching moment:
* OrderRepository modifies state (save)
* Therefore it requires &mut R
* OrderNotifier only observes data
* Therefore &N is enough
* Rust forces this distinction, and that’s a good thing.






```powershell
cargo run --example ex06
```

Expected output:

```text
[InMemory] Saving order #1
[Console] Order #1 confirmed! Total: 4999
Success! Order #1 processed.

Retrieving order #1...
[InMemory] Finding order #1
Found: Order #1, total: 4999
```


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## `ex02.rs` code review















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## What should I do tomorrow morning?
