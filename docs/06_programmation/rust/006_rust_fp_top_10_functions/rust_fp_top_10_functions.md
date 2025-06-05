---
# published: false
layout: default
title: "Rust and Functional Programming: A Beginner’s Top 10 Functions"
parent: "Rust"
#math: mathjax
date               : 2025-06-05 09:00:00
last_modified_date : 2025-06-05 09:00:00
---

# Rust and Functional Programming: A Beginner’s Top 10 Functions

## TL;DR

These functions come from the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait in Rust and embody the principles of functional programming.

* Transformations without mutation
* Operation compositions (chaining is done using dot `.`)
* Immutability
* Expressions (returns values) rather than statements 

<div align="center">
<img src="./assets/img_00.webp" alt="" width="225" loading="lazy"/>
</div>


## 1. `map`

Transforms each element of an iterator.

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled); // [2, 4, 6]
}
```


## 2. `filter`

Keep elements matching a condition.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let even: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even); // [2, 4]
}
```


## 3. `fold`

Accumulates a value from an iterator (uses an initial value). A Swiss army knife.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum); // 10
}
```


## 4. `reduce` 

Like `.fold()`, but without init value (it uses the first element as initial value).

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let product = numbers.iter().copied().reduce(|a, b| a * b);
    println!("{:?}", product); // Some(24)
}
```


## 5. `for_each`

Applies a function immediately (not lazy like `.map()`) to each element, mainly for side effects like printing, logging, etc.

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    numbers.iter().for_each(|x| println!("{}", x * 10));
    // 10
    // 20
    // 30
}
```


## 6. `chain`

Concatenates two iterators.

```rust
fn main() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let chained: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("{:?}", chained); // [1, 2, 3, 4]
}
```


## 7. `zip`

Associates the elements of two iterators together.

```rust
fn main() {
    let names = vec!["Alice", "Bob"];
    let ages = vec![30, 40];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("{:?}", pairs); // [("Alice", 30), ("Bob", 40)]
}
```


## 8. `flatten`

Flattens an iterator of iterators.

```rust
fn main() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flat: Vec<_> = nested.into_iter().flatten().collect();
    println!("{:?}", flat); // [1, 2, 3, 4]
}
```


## 9. `filter_map`

Filters and transforms in a single pass.

```rust
fn main() {
    let strings = vec!["42", "abc", "93"];
    let numbers: Vec<_> = strings.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("{:?}", numbers); // [42, 93]
}
```


## 10. `any` / `all`

Checks whether **at least one** (`any`) or **all** (`all`) elements satisfy a condition.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let any_even = numbers.iter().any(|x| x % 2 == 0);
    let all_even = numbers.iter().all(|x| x % 2 == 0);
    println!("Any even? {}", any_even); // true
    println!("All even? {}", all_even); // false
}
```


