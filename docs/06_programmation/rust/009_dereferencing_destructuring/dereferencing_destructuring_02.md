---
published: true
layout: default
title: "Rust Dereferencing vs Destructuring — For the Kids 2/2"
parent: "Rust"
#math: mathjax
date               : 2025-06-27 09:00:00
last_modified_date : 2025-06-27 09:00:00
---

<h2 align="center">
<span style="color:red"><b>This post is still being written.</b></span>    
</h2>

# Rust Dereferencing vs Destructuring — For the Kids 2/2
{: .no_toc }

<div align="center">
<img src="./assets/cover.webp" alt="" width="450" loading="lazy"/>
<p>Thanks Chat GPT</p>
</div>

---
## TL;DR
{: .no_toc }

* **Dereferencing**: accessing the value behind a reference or smart pointer (e.g., `*x`, or implicit via `Deref`). Used to read or mutate the underlying data, respecting Rust’s borrowing rules (`&T`, `&mut T`).
* **Destructuring**: breaking apart composite values (tuples, structs, enums) using pattern matching syntax. Can move or borrow parts depending on context.

---
## The post is in 2 parts
{: .no_toc }

* [Rust Dereferencing vs Destructuring — For the Kids 1/2]({%link docs/06_programmation/rust/009_dereferencing_destructuring/dereferencing_destructuring_01.md%})
* [Rust Dereferencing vs Destructuring — For the Kids 2/2]({%link docs/06_programmation/rust/009_dereferencing_destructuring/dereferencing_destructuring_02.md%})

---
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}


---

## Introduction

If you're learning Rust and the concepts of ownership, borrowing, and references still feel unfamiliar or intimidating — you're not alone.

Coming from languages like Python or C++, it's easy to assume that Rust's `&`, `*`, and smart pointers behave the same way. But Rust has its own philosophy, built around memory safety and enforced by strict compile-time rules.

This article aims to clarify the difference between **dereferencing** and **destructuring** — two concepts that are often confused, especially outside of `match` expressions.

### Why the Confusion?

At first glance, dereferencing (using `*`) and destructuring (in `let`, `if let`, or `match` patterns) can *look* similar when working with references. Consider the following lines:

```rust
let r = &Some(5);
if let Some(val) = r {
    println!("val = {val}");
}
```
No explicit `*r` — yet the pattern matches. How?

Now look at this one-liner:

```rust
let Some(x) = &Some(42);
```
Is this dereferencing, destructuring, or both?

And then:

```rust
let b = Box::new((42, "hello"));
let (x, y) = *b;
let (x, y) = b; // Doesn't compile
```
All three examples seem simple — but do you really understand why they behave this way?

If you already know the answers, maybe this article isn’t for you. But if you’ve ever hesitated, been surprised by a compilation error, or struggled to explain why one line works and another doesn’t… then you’re in the right place.

This article won’t just define dereferencing and destructuring — it will show you how Rust treats them, how the compiler helps (or confuses) you, and when the distinction truly matters.


### What This Article Covers  
1. **Dereferencing**: How to access values through references and smart pointers (Box, Rc, RefCell), and how mutability affects this.

2. **Destructuring**: How to unpack values in let, match, and function or closure parameters — including when working with references.

No multithreading knowledge required. For a threaded use case, see this [post on Multithreaded Mandelbrot sets (in French)]({% link docs/06_programmation/rust/003_mandelbrot_multithread/mandelbrot_multithread.md %}).

Whether you're just starting with Rust or adjusting your mental model, this post is for you.



<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->
<!-- ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ -->



---
Now that we’ve seen in Part 1 how to follow pointers, it’s time to open the box and peek inside with destructuring!

## Patterns and Destructuring Patterns in Rust

***What is Destructuring?*** **Destructuring** is the act of using a pattern to **break a value apart** and extract its inner pieces. As we will see, we are not just assigning a value, we are **unpacking** it. However before diving into **destructuring**, it’s important to understand what a **pattern** is. 

In Rust, a **pattern** is a syntax that **matches the shape of a value**. You’ve probably already seen patterns in `match` statements, `if let`, or `while let` — but patterns are **everywhere**: in `let` bindings, function and closure parameters, and `for` loops.

***OK... But what is a pattern?***

A **pattern** tells the compiler: "I expect a value of a certain shape — and I want to extract pieces from it." Ok, let's not waste time and go and see some code.




---
## Destructuring: A smooth start

Too often we, me first, associate the concept to ``match`` but this is too restrictive. Let's start with some let statements. Copy and paste the code below in the Rust Playground then hit CTRL+ ENTER

```rust
fn destructuring01() {
    println!("\nDestructuring 01 : 101\n");

    let (x, y) = (1, 2); // (x, y) is a pattern
    println!("{x}, {y}");

    let (x, y) = (1, 3.14); // tuple => we can have different data type
    println!("{x}, {y}");

    let [a, b, c] = [10, 20, 30]; // [a, b, c] is a pattern
    println!("{a}, {b}, {c}");

    let x = 42; // `x` is a very simple pattern: it matches any value and binds it to the name `x`
    println!("{x}");

    let ((x1, y1), (x2, y2)) = ((1, 2), (3, 4)); // nested destructuring
    println!("{x1}, {y1}, {x2}, {y2}");
}

fn main(){
    destructuring01();
}
```

### Expected output 
{: .no_toc }
```
Destructuring 01 : 101

1, 2
10, 20, 30
42
1, 2, 3, 4

```

### Explanations
{: .no_toc }
* As I said, destructuring is the act of using a pattern to break a value apart and extract its inner pieces. In this context, a pattern is a syntax that matches the shape of a value. 
* The first `let` statement matches `(x, y)` to `(1, 2)`. Once this is OK shape wise, it extracts the value 1 and affect it to `x` and do the same with 2 and `y`. I told you. A smooth start.
    * I hope why `let` is a statement and not an expression. If not, read this [Computer Science Vocabulary page ]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}) the this [one](https://doc.rust-lang.org/stable/reference/statements.html?highlight=statement#statements). 
* The second `let` is similar to the first one except that `x` and `y` have different data type 
* The third `let` is similar to the first one but since we match arrays, `a`, `b` and `c` have the same data type
* The fourth might be surprising. If the notion of **binding** is not crystal clear, you can read this  
[page about Mutability]({%link docs/06_programmation/rust/004_mutability/mutability_us.md%})
* The last `let` statement shows nested destructuring, where like with Russian Dolls, match act at different levels.













---
## Destructuring: Partial Destructuring
```rust
fn destructuring02() {
    println!("\nDestructuring 02 : partial destructuring\n");

    let (x, ..) = (1, 2, 3); // ignore the rest
    println!("x : {}", x);

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    let pt = Point3D { x: 1, y: 2, z: 3 };

    let Point3D { x, .. } = pt;
    println!("x coordinates: {}", x);
}



```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }





















---
## Destructuring: `struct` with `let`
```rust
fn destructuring03() {
    println!("\nDestructuring 03 : a struct with let\n");

    struct Scientist {
        name: String,
        field: String,
    }

    let hari = Scientist {
        name: "Hari Seldon".to_string(),
        field: "Psychohistory".to_string(),
    };

    let Scientist { name, field } = hari;
    println!("{name} works in {field}");
}


```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }











---
## Destructuring: `enum` with let
```rust
fn destructuring04() {
    println!("\nDestructuring 04 : enum with let\n");

    enum Role {
        Emperor,
        Trader(String),
        Scientist { name: String, field: String },
    }

    let characters = vec![
        Role::Emperor,
        Role::Trader("Hober Mallow".to_string()),
        Role::Scientist {
            name: "Hari Seldon".to_string(),
            field: "Psychohistory".to_string(),
        },
    ];

    for role in characters {
        match role {
            Role::Emperor => println!("The Emperor rules... vaguely."),
            Role::Trader(name) => println!("A trader named {name}"),
            Role::Scientist { name, field } => {
                println!("Scientist {name} specializes in {field}")
            }
        }
    }
}


```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }





---
## Destructuring: `tuples` with `let` 1/2
```rust
fn destructuring05() {
    println!("\nDestructuring 05 : tuples with let 1/2\n");

    let (name, age) = ("Salvor Hardin", 42); // tuple destructuring
    let Some(x) = Some(5) else { todo!() }; // enum destructuring

    fn print_coords((x, y): (i32, i32)) {
        println!("{x}, {y}");
    }

    let (my_x, my_y) = (28, 56);
    print_coords((my_x, my_y));
}


```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }







---
## Destructuring: `tuples` with `let` 2/2
```rust
// When destructuring, the pattern on the left-hand side must match the shape of the value on the right.
// In this case, a 2-element tuple is matched by a 2-element pattern.
fn destructuring06() {
    println!("\nDestructuring 06 : tuples with let 2/2\n");

    let pair = ("Hari Seldon", 12050);

    // Destructuring the tuple into two separate variables
    let (name, year) = pair;

    println!("{} was born in year {}", name, year);

    // You can also ignore parts of a tuple using _
    let (_, just_the_year) = pair;
    println!("We only care about the year: {}", just_the_year);
}


```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }




---
## Destructuring: function & closure parameters


```rust

fn destructuring07() {
    println!("\nDestructuring 07 : function & closure parameters\n");

    // --- Function with destructured parameters ---
    fn print_coordinates((x, y): (i32, i32)) {
        println!("Function received: x = {}, y = {}", x, y);
    }

    let point = (10, 20);
    print_coordinates(point);

    // --- Destructuring in a let binding ---
    let (a, b) = point;
    println!("Destructured binding: a = {}, b = {}", a, b);

    // --- Destructuring in a closure ---
    let points = vec![(1, 2), (3, 4), (5, 6)];

    println!("\nClosure with destructuring:");
    points.iter().for_each(|&(x, y)| {
        println!("Point: x = {}, y = {}", x, y);
    });
}
```

### Expected output 
{: .no_toc }
```
Destructuring 01 : function parameters

x = 10, y = 20
a = 10, b = 20
```

### Explanations
{: .no_toc }








---
## Destructuring: in `for` loops with `.enumerate()`
```rust
// In a for loop, the variable immediately after for is a pattern.
// That’s why we can destructure tuples directly inside the loop.”
fn destructuring08() {
    println!("\nDestructuring 08 : in for loops with enumerate()\n");

    let characters = vec!["Hari", "Salvor", "Hober"];

    for (index, name) in characters.iter().enumerate() {
        // (index, name) a pattern that destructures the (usize, &str) tuple
        println!("Character #{index} is {name}");
    }

    // Underscore can be used to ignore parts
    for (_, name) in characters.iter().enumerate() {
        println!("Name only: {name}");
    }
}


```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }











---

<!-- 
### Destructuring in `for` loops
{: .no_toc }

One of the less obvious places where destructuring happens is in `for` loops.

This code looks innocent:

```rust
for s in &foundation {
    println!("{}", s);
}
```

But what happens if we write:

```rust
for &s in &foundation {
    println!("{}", s);
}
```

Suddenly it doesn’t compile. Why?

Because `&s` is a **destructuring pattern** — not a reference! It says:

> “I expect a reference here — and I want to dereference it and bind the content to `s`.”

If `&foundation` gives you `&String`, and you write `for &s in ...`, you’re saying:

> “Take the `&String`, dereference it, and bind the result to `s`.”

But a `&String` cannot be dereferenced into a `String` — only a `&str` or clone would work. That’s why the compiler gets upset.

 -->




## Destructuring: `for` loop over array slices
```rust
// This line might look like we're referencing s, but &[x, y] is a pattern, not a reference. The compiler matches each &[i32; 2] and destructures it in-place
fn destructuring09() {
    println!("\nDestructuring 09 : for loop over array slices\n");

    let coordinates = vec![[1, 2], [3, 4], [5, 6]];

    // Each element is a reference to an array: &[i32; 2]
    // Destructuring pattern applied to &[i32; 2]
    for &[x, y] in &coordinates {
        // &[x, y] pattern that matches a reference to a 2-element array
        println!("x: {}, y: {}", x, y);
    }

    // Alternative: without destructuring
    for coord in &coordinates {
        println!("coord[0]: {}, coord[1]: {}", coord[0], coord[1]);
    }
}


```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }






















---
## Destructuring: destructuring pattern in for loop

This is the part that broke my brain when I first encountered it.

When iterating over a vector of strings by reference (&Vec<String>), I naively thought that writing for &s in &foundation meant “give me the reference and then give me the value.” But that’s not what’s happening.

In Rust, the expression after for is always a pattern — and here, &s is a destructuring pattern, not a reference.

It tries to match a &String (which is what &foundation yields) with the pattern &s, which would only work if s were a String. But in Rust, you can’t implicitly copy or clone a String, so it fails to compile.

Lesson learned: in a for loop, if you write &s, you’re telling the compiler: “I want to destructure a reference and bind the value inside it.” It’s not the same as taking a reference.


```rust
fn destructuring10() {
    println!("\nDereferencing 10 : destructuring pattern in for loop\n");

    let foundation: Vec<String> = vec!["Hari Seldon", "Salvor Hardin", "Hober Mallow", "The Mule", "Bayta Darell"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    // The following loop will not compile
    // In a for loop, the value that directly follows the keyword for is a pattern
    // So `s`is NOT variable, &s is not a reference, &s is a pattern - specifically, a destructuring pattern.

    // for &s in &foundation {
    //     println!("String is : {}", s);
    // }

    for s in &foundation {
        println!("String is : {}", s);
    }
}

```

### Expected output 
{: .no_toc }

```


```




### Explanations
{: .no_toc }







---
## Destructuring: Option<T> in a for loop
```rust
// Patterns can be used in loops to filter and destructure in a single step. Here, &Some(score) is not a reference — it’s a pattern that matches a reference to an Option and destructures it if it’s Some
fn destructuring11() {
    println!("\nDestructuring 11 : Option<T> in a for loop\n");

    let maybe_scores = vec![Some(10), None, Some(30)];

    // The pattern is a reference to an Option, so we match &Some(x)
    for &opt in &maybe_scores {
        match opt {
            Some(score) => println!("Score: {}", score),
            None => println!("No score"),
        }
    }

    // Alternative: filter out None before the loop
    for score in maybe_scores.iter().filter_map(|opt| opt.as_ref()) {
        println!("Got a score (filter_map): {}", score);
    }

    // Using if-let inside the loop body
    // Using if-let inside the loop body
    for maybe in &maybe_scores {
        if let Some(score) = maybe {
            println!("Score via if-let: {}", score);
        }
    }

    // Rather than going through a Vec<Option<T>>, and ignoring the None in the loop, we can avoid the if let by flattening the Some directly in the iterator.
    for score in maybe_scores.iter().flatten() {
        println!("Score via flatten: {}", score);
    }
}


```

### Expected output 
{: .no_toc }
```
```

### Explanations
{: .no_toc }








## Conclusion

| Aspect           | Dereferencing               | Destructuring                            |
| ---------------- | --------------------------- | ---------------------------------------- |
| Syntax           | `*x`                        | `let (a, b) = x`                         |
| Semantics        | Access pointed value        | Extract elements of a structure          |
| Applicable to    | `&T`, `Box<T>`, etc.        | `tuple`, `struct`, `enum`, `array`, etc. |
| Requires traits? | Yes: `Deref`                | No (structural pattern matching)         |

* Summarizes the distinction.
    * Dereferencing is peeling a wrapper off, destructuring is breaking the thing into pieces.
    * The word pattern refers to the left-hand side of an assignment, match or for.
    * Destructuring occurs as soon as you “break a structure into pieces”, thanks to this pattern.

* Encourages experimentation (with `for`, `let`, `if let`, `while let`, function parameters).
* Bonus: suggest a toy implementation to play with `Deref` and `Drop` to see the effects.

* Thinking that destructuring is only possible in `match`
* Incorrect understanding of pattern in `for`





### Webliography
{: .no_toc }
* Patterns in the [Rust Reference](https://doc.rust-lang.org/stable/reference/patterns.html#grammar-PatternNoTopAlt)
* let statement in the [Rust Reference](https://doc.rust-lang.org/stable/reference/statements.html#grammar-LetStatement) 





















































