---
published: true
lang: en-US
layout: default
title: "Functions, Methods, Closures, Iterators, and Coroutines in Rust"
description: "A short, practical reminder with working examples."
parent: "Rust"
nav_order: 32
date:               2026-01-22 11:00:00
last_modified_date: 2026-01-22 11:00:00
---


# Function, Method, Closure, Coroutine and Iterator in Rust
{: .no_toc }

A quick reminder.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>




<!--
TODO
*
-->


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* For beginners
* **Function**: Executes once, returns once
* **Method**: Function tied to a type
* **Closure**: Function that captures environment variables
* **Coroutine**: Function that can suspend and resume, maintaining state across suspension points
* **Iterator**: Produces a sequence of values lazily, one at a time












<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ### This is Episode 00
{: .no_toc }

#### The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/022_solid/solid_00.md%}): Introduction + Single Responsibility Principle
* [Episode 01]({%link docs/06_programmation/rust/022_solid/solid_01.md%}): Open-Closed Principle
* [Episode 02]({%link docs/06_programmation/rust/022_solid/solid_02.md%}): Liskov Substitution Principle
* [Episode 03]({%link docs/06_programmation/rust/022_solid/solid_03.md%}): Interface Segregation Principle
* [Episode 04]({%link docs/06_programmation/rust/022_solid/solid_04.md%}): Dependency Inversion Principle + Conclusion
 -->





<div align="center"> -->
<img src="./assets/img00.webp" alt="" width="1024" loading="lazy"/><br/>
<span>1992: When Batman returned and Windows 3.1 relied on cooperative multitasking of the coroutines.</span>
</div>














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc} -->











<!-- ## Coroutines: Concept and Differences

### What is a Coroutine?

A **coroutine** is a function that can **suspend its execution** and later **resume** from where it left off, preserving its state between suspensions. Unlike regular functions that run to completion, coroutines can yield control back to the caller and be resumed later.

### Key Differences -->





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->



## Function
- Runs from start to finish in one go
- Returns a single value
- Stack is destroyed when it returns
- No state preservation between calls

**Example:** You can copy and paste the code below into the [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run --example ex00
fn regular_function(x: i32) -> i32 {
    println!("Regular function computing {} * 2", x);
    x * 2
}

fn main() {
    let result = regular_function(5);
    println!("Result: {}\n", result);
}
```


**Expected output:**

```powershell
Regular function: computing 5 * 2
Result: 10
```





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Method
- A function associated with a type/object
- Same execution model as functions (runs to completion)
- Can access `self` data


**Example:** You can copy and paste the code below into the [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run --example ex01
struct Calculator {
    multiplier: i32,
}

impl Calculator {
    fn multiply_method(&self, x: i32) -> i32 {
        println!("Method computing {} * {}", x, self.multiplier);
        x * self.multiplier
    }
}

fn main() {
    let calc = Calculator { multiplier: 3 };
    let result = calc.multiply_method(7);
    println!("Result: {}\n", result);
}
```

**Expected output:**

```powershell
Method computing 7 * 3
Result: 21
```




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Closure
- A function that **captures variables** from its environment (current block)
- Still runs to completion like a regular function
- The captured state exists across multiple calls to the closure


**Example 1:** You can copy and paste the code below into the [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run --example ex02
fn demonstrate_closure() {
    let captured_value = 10;

    let closure = |x: i32| {
        println!("Closure computing {} + captured {}", x, captured_value);
        x + captured_value
    };

    println!("Closure result: {}", closure(5));
}

fn main() {
    demonstrate_closure();
}
```


**Expected output:**

```powershell
Closure computing 5 + captured 10
Closure result: 15
```

**Example 2:** You can copy and paste the code below into the [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run --example ex03
pub fn shift_all<F>(data: &mut [i32], mut mutator: F)
where
    F: FnMut(i32) -> i32,
{
    for v in data {
        *v = mutator(*v);
    }
}

fn main() {
    let bias = 42;
    let add_bias = |n| n + bias;

    let mut my_data = vec![1, 3, 5, 7];
    shift_all(&mut my_data, add_bias);

    println!("{:?}", my_data);
}
```
**Expected output:**

```powershell
[43, 45, 47, 49]
```




**Example 3:** You can copy and paste the code below into the [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run --example ex04
// Generator-style coroutine (using iterator)

fn generator_style(start: i32, count: i32) -> impl Iterator<Item = i32> {
    (0..count).map(move |i| {
        println!("Generator: yielding value {}", start + i);
        start + i
    })
}

fn main() {
    let mut iterator = generator_style(100, 3);
    println!("First value: {:?}", iterator.next());
    println!("Second value: {:?}", iterator.next());
    println!("Third value: {:?}", iterator.next());
}
```

**Expected output:**

```powershell
Generator: yielding value 100
First value: Some(100)
Generator: yielding value 101
Second value: Some(101)
Generator: yielding value 102
Third value: Some(102)

```











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Coroutine
- Can **pause and resume** execution
- Maintains its execution state (local variables, instruction pointer)
- Can yield multiple values over time
- Often used for async operations, generators, or cooperative multitasking
- Primarily implemented through `async`/`await`, which compile to state machines that can be suspended at `.await` points.


**Example:** You can copy and paste the code below into the [Rust Playground](https://play.rust-lang.org/):

```rust
// cargo run --example ex05
// async with tokio & reqwest

use std::time::Instant;

#[tokio::main]
async fn main() {
    // Sequential requests
    println!("Sequential requests:");
    let start_sequential = Instant::now();
    fetch_sequential().await;
    let duration_sequential = start_sequential.elapsed();
    println!("Total sequential time: {:?}\n", duration_sequential);

    // Async requests
    println!("Async requests:");
    let start_async = Instant::now();
    asynchronously().await;
    let duration_async = start_async.elapsed();
    println!("Total async time: {:?}\n", duration_async);
}

async fn fetch_sequential() {
    let urls = vec!["https://www.google.com", "https://www.microsoft.com"];

    for url in urls {
        let start = Instant::now();
        match fetch_url(url).await {
            Ok(_) => {
                let duration = start.elapsed();
                println!("  {} - {:?}", url, duration);
            }
            Err(e) => {
                println!("  {} - Error: {}", url, e);
            }
        }
    }
}

async fn asynchronously() {
    let urls = vec!["https://www.google.com", "https://www.microsoft.com"];

    // Create a vector of futures
    let mut tasks = vec![];

    for url in urls {
        let url = url.to_string();
        let task = tokio::spawn(async move {
            let start = Instant::now();
            let result = fetch_url(&url).await;
            let duration = start.elapsed();
            (url, result, duration)
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        match task.await {
            Ok((url, result, duration)) => match result {
                Ok(_) => println!("  {} - {:?}", url, duration),
                Err(e) => println!("  {} - Error: {}", url, e),
            },
            Err(e) => println!("  Task error: {}", e),
        }
    }
}

async fn fetch_url(url: &str) -> Result<(), reqwest::Error> {
    let _response = reqwest::get(url).await?;
    Ok(())
}
```

**Expected output:**

```powershell
Sequential requests:
  https://www.google.com - 147.6548ms
  https://www.microsoft.com - 125.9957ms
Total sequential time: 274.657ms

Async requests:
  https://www.google.com - 72.3342ms
  https://www.microsoft.com - 116.1267ms
Total async time: 116.6558ms
```






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Iterator
- Represents a sequence of values produced one at a time
- Maintains internal iteration state between calls
- Returns the next value on each call, or signals completion
- Often used in for / foreach loops
- Does not necessarily store all values in memory
- Can be implemented by collections, generators, or custom types
- May be finite or infinite



**Example:** You can copy and paste the code below into the [Rust Playground](https://play.rust-lang.org/):

```rust
struct Counter {
    current: u64,
}

impl Iterator for Counter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current += 1;
        Some(value) // Never returns None → infinite iterator
    }
}

fn main() {
    let counter = Counter { current: 0 };

    for n in counter.take(5) {
        println!("{}", n);
    }
}
```



**Expected output:**

```powershell
0
1
2
3
4
```























## Are Iterator based on Coroutine?

**No, an iterator is not a coroutine** in the traditional sense.

Iterator can behaves like a coroutine but but:
- **Iterators** simulate this through state stored in struct fields
- **Coroutines** achieve this through actual execution suspension


### Iterator
- **Pull-based**: The consumer calls `.next()` to get values
- **Synchronous**: Computation happens immediately when `.next()` is called
- **State machine**: Implemented as a struct with state fields
- **No suspension points**: Code doesn't "pause" - each `.next()` runs fresh logic
- **Simple trait**: Just needs `fn next(&mut self) -> Option<Item>`

### Coroutine (async/await)
- **Push-based** (in async context): The runtime drives execution
- **Asynchronous**: Can wait for I/O, timers, etc.
- **State machine**: Compiler generates one from async fn
- **Suspension points**: Can `.await` and resume later
- **Complex**: Involves `Future`, `Poll`, `Waker`, executors

### Philosophical Difference

**A mind model for Iterator**

> Each call is a **fresh execution** of the `next()` method. There's no "pausing" mid-function.

```
User calls next() → Execute logic → Return value → DONE
User calls next() → Execute logic → Return value → DONE
User calls next() → Execute logic → Return value → DONE
```

**A mind model for Coroutine**

> A **single execution** that suspends and resumes, preserving the exact position and local variables.

```
Start execution → Compute → PAUSE (await)
                           ↓
Resume → Compute more → PAUSE (await)
                           ↓
Resume → Compute more → DONE (return)
```



### Visual Comparison

```rust
// Iterator: each next() call executes independently
struct Counter { count: i32, max: i32 }

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        // This code runs fresh each time
        // No "pausing" - just checking state and updating
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Coroutine: execution can suspend and resume
async fn async_counter(max: i32) -> Vec<i32> {
    let mut results = vec![];
    for i in 0..max {
        // This is a suspension point - execution can pause here
        some_async_operation().await;
        results.push(i);
    }
    results
}
```



<!-- ### Summary

| Feature | Iterator | Coroutine |
|---------|----------|-----------|
| Execution model | Re-enter function each time | Suspend/resume single execution |
| Async support | No | Yes |
| Implementation | Manual state in struct | Compiler-generated state machine |
| Control flow | Pull (caller driven) | Push (runtime driven) |
| Use case | Sequences, collections | Async I/O, concurrency | -->