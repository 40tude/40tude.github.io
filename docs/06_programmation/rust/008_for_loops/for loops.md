---
# published: false
layout: default
title: "Who Owns What in Rust Loops? (And Why It Matters)"
parent: "Rust"
#math: mathjax
date               : 2025-06-26 17:00:00
last_modified_date : 2025-07-29 09:00:00
---

# Who Owns What in Rust Loops? (And Why It Matters)

<div align="center">
<img src="./assets/cover.webp" alt="" width="450" loading="lazy"/>
</div>


## TL;DR
In Rust, like in [The Night's Dawn Trilogy](https://en.wikipedia.org/wiki/The_Night%27s_Dawn_Trilogy), everything revolves around possession. If you just want to read data, pass it by reference. If you want to modify it, use a mutable reference. And if you want to consume them, be aware that you won't be able to use them afterwards. The compiler helps you, but you still have to listen to what it's telling you.


## Let's start simple

Copy and paste the code below in [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=790748b4aaa4bcc5e0f423c7966dc9f9) then hit **CTRL+ENTER**.

```rust
fn main() {
    println!("\nTest01 :");

    let the_3_body_problem = ["Ye Wenjie", "Wang Miao", "Shi Qiang", "Mike Evans", "Sophon"];

    for s in the_3_body_problem {
        println!("String is : {}", s);
    }
    println!();

    for s in the_3_body_problem {
        println!("String is : {}", s);
    }
    println!();

    for s in &the_3_body_problem {
        println!("String is : {}", *s); 
        println!("String is : {}", s); 
    }
}
```
Everything should work as expected and you should see

```
Test01 :
String is : Ye Wenjie
String is : Wang Miao
String is : Shi Qiang
String is : Mike Evans
String is : Sophon

String is : Ye Wenjie
String is : Wang Miao
String is : Shi Qiang
String is : Mike Evans
String is : Sophon

String is : Ye Wenjie
String is : Ye Wenjie
String is : Wang Miao
String is : Wang Miao
String is : Shi Qiang
String is : Shi Qiang
String is : Mike Evans
String is : Mike Evans
String is : Sophon
String is : Sophon
```
Now the question is... How and why does this work ?

### What’s going on here?

The array `the_3_body_problem[]` is an array of string slices (`&str`). This is a primitive, copyable type. You can think of this type as being similar to a `u64`: it's lightweight and can be efficiently manipulated by modern processors.

Next, there is the line `for s in the_3_body_problem {...`. Here my recommendation is to view the `for` loop like a function call and to imagine something like : `for(s, the_3_body_problem);`. Having this model in mind we can now apply everything we know about passing arguments in function calls. Here for example, since the array is on the stack (not on the heap) and since the elements it hold are primitive, we "know" that the argument is passed by value. The argument is copied.

Note that `s` is of type string slice (`&str`). This should not be a surprise since the `the_3_body_problem[]` array is an array of `&str`. Every value of the array is copied in `s` and in the body of the loop we pass `s` as an argument to the `println!()` macro.  

### Why the second loop works?
Then we reach the second `for` loop. You may ask why should I bother? The point is to demonstrate that the `the_3_body_problem[]` array is still there, and that it can be used a second loop. Exactly as if after passing it by value in a first function call we would pass it again by value in a second function call.

In the last `for` loop, we become smarter and we pass the `the_3_body_problem[]` array by reference (`&the_3_body_problem`). This is smart because now we do not copy the content of the array (let's say 1MB), we just provide the address of its first element (let's say 8 bytes). 

### Smarter iteration: passing by reference
But... In this case the `s` variable is of type "address of the element in the collection". So here, `s` is of type address of a `&str`, that is `&&str`. And yes, the two ampersands may look weird — but don't panic, it's perfectly normal Rust. Knowing this, if we want to print the actual string value pointed to by `s`, we need to dereference it using `*s`. That’s why the first `println!` uses explicit dereferencing: `*s` turns our `&&str` into a `&str`. However, Rust compiler is smart enough to apply what it calls "deref coercion" and to print something even if we pass an `s` (of type `&&str`)  


The code below is the same as the previous one, but with additional comments

```rust
fn main() {
    println!("\nTest01 :");

    // An array of &str (string slices — a primitive, copyable type)
    let the_3_body_problem = ["Ye Wenjie", "Wang Miao", "Shi Qiang", "Mike Evans", "Sophon"];

    // Since &str implements the Copy trait, the array is implicitly copied rather than moved
    // You can think of a for loop as taking ownership (or borrowing) from the iterable, depending on its type
    // Think to a function call which would look like : `for(s, the_3_body_problem);`
    // It behaves as if each element is passed by value into the loop body
    // This is possible here because the array lives on the stack and holds only Copy elements
    for s in the_3_body_problem {
        // s is of type &str
        println!("String is : {}", s);
    }
    println!();

    // We can reuse the array because it was copied, not moved
    for s in the_3_body_problem {
        println!("String is : {}", s);
    }
    println!();

    // For better efficiency, we can iterate by reference
    for s in &the_3_body_problem {
        // s is of type &&str (reference to a string slice)
        println!("String is : {}", *s); // Explicit dereferencing
        println!("String is : {}", s); // Deref coercion works automatically
    }
}
```





## What happen with an array of non-primitive type?

This is a very good question. We will use the code below in Rust Playground to illustrate the point. Same thing as before, copy, paste then CTRL+ENTER.

```rust
fn main() {
    println!("\nTest02 :");

    // An array of owned Strings (non-primitive type, allocated on the heap)
    let dune = [
        "Paul Atréides (Muad'Dib)".to_string(),
        "Lady Jessica".to_string(),
        "Leto Atréides".to_string(),
        "Chani".to_string(),
        "Stilgar".to_string(),
        "Vladimir Harkonnen".to_string(),
    ];

    // This consumes the array - it's moved into the loop
    for s in dune {
        // s is of type String (ownership is moved here)
        println!("String is : {}", s);
    }
    println!();

    // Uncommenting the following would not compile because `dune` was moved in the previous loop.
    /*
    for s in dune {
        println!("String is : {}", s);
    }
    */

    // To fix that, re-create the array and iterate by reference:
    let dune = [
        "Paul Atréides (Muad'Dib)".to_string(),
        "Lady Jessica".to_string(),
        "Leto Atréides".to_string(),
        "Chani".to_string(),
        "Stilgar".to_string(),
        "Vladimir Harkonnen".to_string(),
    ];

    // Using references to avoid moving
    for s in &dune {
        // s is of type &String
        println!("String is : {}", *s); // Explicit dereference
    }
    println!();

    for s in &dune {
        // s is of type &String
        println!("String is : {}", s); // Deref coercion makes this valid
    }
    println!();
}
```

Everything should work and you should see :


```
Test02 :
String is : Paul Atréides (Muad'Dib)
String is : Lady Jessica
String is : Leto Atréides
String is : Chani
String is : Stilgar
String is : Vladimir Harkonnen

String is : Paul Atréides (Muad'Dib)
String is : Lady Jessica
String is : Leto Atréides
String is : Chani
String is : Stilgar
String is : Vladimir Harkonnen

String is : Paul Atréides (Muad'Dib)
String is : Lady Jessica
String is : Leto Atréides
String is : Chani
String is : Stilgar
String is : Vladimir Harkonnen


```
### The first loop
Now let's comment the code. First thing first, `dune` is an array of non-primitive type. Here I use String which are allocated to the heap.

We then reach the first `for` loop. If we consider the loop as a function call `for(s, dune);` and since the array is an array of non-primitive types, we "know" `dune` will NOT be copied. It will be moved into the `for` loop.

Once in the `for` loop, life is easy. `s` is of type String and we can print it.

What is not so cool is the fact the if we try to uncomment the second loop, then the code no longer compile. Indeed, `dune` was moved in the first loop and it is no longer available.

### How to fix that?

Since `dune` has disappeared, we first re-create it. Then we apply the same technique has before : pass `dune` by reference to the `for` loop. Then everything we said remains true and we can print the strings using `*s` or `s`.










## What if I have a vector of primitive type?

Haha! Looks like you become addict to Rust Playground... Welcome to the club. You know the song now : copy, paste, CTRL+ENTER 



```rust
fn main() {
    println!("\nTest03 :");

    // A vector of &str
    let the_expanse = vec!["James Holden (Jim)", "Naomi Nagata", "Alex Kamal", "Amos Burton", "Chrisjen Avasarala", "Rocinante"];

    // The vector is moved into the loop and no longer accessible afterward.
    for s in the_expanse {
        println!("String is : {}", s);
    }
    println!();

    // The following will not compile because `the_expanse` was moved above.
    /*
    for s in the_expanse {
        println!("String is : {}", s);
    }
    println!();
    */

    // To reuse it, recreate and borrow it instead
    let the_expanse = vec!["James Holden (Jim)", "Naomi Nagata", "Alex Kamal", "Amos Burton", "Chrisjen Avasarala", "Rocinante"];

    for s in &the_expanse {
        // s is of type &&str
        println!("String is : {}", *s); // Explicit dereference
    }
    println!();

    for s in &the_expanse {
        println!("String is : {}", s); // Works due to deref coercion
    }
    println!();
}

```

No surprise, you should see :

```
Test03 :
String is : James Holden (Jim)
String is : Naomi Nagata
String is : Alex Kamal
String is : Amos Burton
String is : Chrisjen Avasarala
String is : Rocinante

String is : James Holden (Jim)
String is : Naomi Nagata
String is : Alex Kamal
String is : Amos Burton
String is : Chrisjen Avasarala
String is : Rocinante

String is : James Holden (Jim)
String is : Naomi Nagata
String is : Alex Kamal
String is : Amos Burton
String is : Chrisjen Avasarala
String is : Rocinante

```


I guess we can, now, start to speed up while commenting the code 
* `the_expanse` is a vector of string slice (`&str`). "Remember Barbara", unlike arrays, which are on the stack, vectors are allocated on the heap.
* A vector does not have a Copy trait. So when we pass `the_expanse` to the `for` loop, it is not copied, it is moved into the loop. 
* The first loop works like a charm
* If we uncomment the second one the code does'nt compile because the vector is no longer accessible
* Before going further, we must recreate a `the_expanse` vector
* It's becoming a habit now, we pass the vector by reference in the last 2 loops
* We can print the string slice using `*s` or `s`.















## And what about a vector of non-primitive type ?

Let's use the code below to illustrate the point.

<!-- 
/*
fn main() {
    let s1 = String::from("Trisolaris");
    let string_ref: &String = &s1; // This is what we get from the iterator : &String

    match string_ref {
        &s => {println!("s: {}", s);} // pattern `&s` veut dire : dé-référencer `item`, stocker le contenu (de type `String`) dans `s`
    }
}
*/ 
-->


```rust

fn main() {
    println!("\nTest04 :");

    // A vector of owned Strings
    let foundation: Vec<String> = vec!["Hari Seldon", "Salvor Hardin", "Hober Mallow", "The Mule", "Bayta Darell"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    // The vector is moved into the loop and no longer accessible afterward
    for s in foundation {
        println!("String is : {}", s); 
    }
    println!();


    // To reuse the vector, recreate and borrow it instead
    let foundation: Vec<String> = vec!["Hari Seldon", "Salvor Hardin", "Hober Mallow", "The Mule", "Bayta Darell"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();


    // Using references
    for s in &foundation {
        // s is of type &String
        println!("String is : {}", *s); // Explicit dereference
    }
    println!();

    for s in &foundation {
        println!("String is : {}", s); // Works due to deref coercion
    }
    println!();

    // The following loop will not compile:
    /*
    for &s in &foundation {
        println!("String is : {}", s);
    }
    */
}
```

You should get the output below :

```
Test04 :
String is : Hari Seldon
String is : Salvor Hardin
String is : Hober Mallow
String is : The Mule
String is : Bayta Darell

String is : Hari Seldon
String is : Salvor Hardin
String is : Hober Mallow
String is : The Mule
String is : Bayta Darell

String is : Hari Seldon
String is : Salvor Hardin
String is : Hober Mallow
String is : The Mule
String is : Bayta Darell

```

### Comments
* We first create `foundation` a vector of `String`
* The vector is on the heap and the elements of the vector are also on the heap
* When we pass the vector to the `for` loop it is moved (not copied)
* After the first loop ,`foundation` is no longer available
* We re-create it
* And we pass it by reference to the `for` loop
* We print the string slice using `*s` or `s`

As explained in the comments, the last `for` loop does'nt compile. 

```rust
for &s in &foundation {
    ...
}
```
Here, we want to be very smart. Indeed, the vector holds non-primitive elements so we may want to use a reference to them before printing. This would show to the rest of the universe how smart we are since this would involve no copy at all. Mouah ah, ah... Master of the world !

Yes, but no. In fact it tooks me a while to understand what happens so let's dedicate a specific section to the topic





## Why does'nt `for &s in &foundation {...` work here?

Copy, paste and CTRL+ENTER the code snippet below

```rust
fn main() {
    println!("\nTest04 bis :");

    let foundation: Vec<String> = vec!["Hari Seldon", "Salvor Hardin", "Hober Mallow", "The Mule", "Bayta Darell"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    // The following loop will not compile:
    for &s in &foundation {
        println!("String is : {}", s);
    }
}
```
The compiler says : 

```
9 |     for &s in &foundation {
  |          -    ^^^^^^^^^^^
  |          |
  |          data moved here
  |          move occurs because `s` has type `String`, which does not implement the `Copy` trait
```

At first glance, you might think: “I know `foundation` holds `String`, so iterating over `&foundation` gives me `&String`, right? So using `&s` should just give me a reference to that reference.” 

But remember — Rust is not just looking at types, it's also deeply concerned with ownership semantics. So let’s follow the white rabbit and really understand what’s happening here.


### Step 1 - Easy, we already know (more or less)
* `foundation` is a `Vec<String>`
* `&foundation` is `&Vec<String>`
* A `for` loop over `&foundation` implicitly calls `.iter()` which provides an `Iterator<Item = &String>`
* At the end, the loop iterates over `&String`

### Step 2 - Brand new stuff
* What does `for &s in ...` really means?
* You can read this section in [The Rust Programming Language](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html#for-loops) book. 
* It is said : "In a `for` loop, the value that directly follows the keyword `for` is a **pattern**..."
* Again `i`, `x`, `s`, `(dr, dc)`... That we use to see in our `for` loops are NOT variables, they are patterns
* So `&s` is a pattern - specifically, a **destructuring pattern**.
* In plain English it says : "I expect to receive a `&T`. Then I will unstructure it to become the owner of the `T` value itself."
* But here, the `&String` is a shared reference. We can't extract the `String` from a simple reference. Indeed the `String` is already owned by someone else. 
* So the Rust compiler says: “You're trying to make a move from a reference - but this type (String) isn't Copy, so I refuse”.


#### Summary
* In a `for` loop, the part right after `for` is not just a variable — it's a pattern.
* Writing `for &s in &collection` means: “I expect to get a reference like `&T`, and I will extract and owned the inner `T`”.
* But this only works if `T` implements `Copy` — otherwise Rust refuses to move it from a shared reference.

### What I did'not understood
At this point you may think that **dereferencing** and **destructuring** are similar. Yes, but no (again). Indeed, in Rust, there's a subtle but important difference between :

**Dereferencing a variable**

``` rust
let s: &String = "Zoubida".to_string(); // create a new reference to the String
let val = *s; // explicit dereferencing

```
* We access what is pointed to by the reference, via a controlled, explicit operation in an instruction.




**Destructuring a value via a pattern**

```rust
let s = String::from("Zoubida");
let &t = &s; // does not compile

```
* `&t` is a destructuring pattern, used in a `let`, `match`, or `for`, which says: "If I get a `&T`, I want to move out the inner `T`".
* And this destructuring **implies a move** of what is referenced, unless `T` implement the Copy trait. This is the key point.


### TL;DR — Deref vs Destructuring
- `*s` (dereferencing): I open the box and **look inside**
- `let &x = something` (destructuring): I open the box and **take what's inside**

Rust allows the first (read), but forbids the second (move from a reference) unless the type implements `Copy`.

**Sumary of the summary** : Pattern matching destructures and attempts to move by default.


So next time you write `for &x in ...`, remember: you're not just accessing the data — you're asking to own it. And Rust will only say yes if it's Copy.

Feel free to read this 2 parts article dedicated to [dereferencing versus destructuring.]({%link docs/06_programmation/rust/009_dereferencing_destructuring/dereferencing_destructuring_01.md%}) 


























## Can we "play" with a vector of even more complex elements ?

No problem, your wish is my command. But before to read further takes 2 minutes and think about what will happen... 

If this is a vector... Where will it be stored? Does the vector implement the Copy trait. Can it be passed by value? Will it be given or borrowed? If the vector is borrowed to the `for` loop, what will be the type of the `for` loop variable ? Will it be a plain value or a reference to ?

OK... Copy, paste and CTRL+ENTER the code below.

```rust

fn main() {
    println!("\nTest05 :");

    const THE_NIGHT_S_DAWN: [&str; 6] = ["Joshua Calvert", "Lagrange Calvert", "Quinn Dexter", "Louise Kavanagh", "Ione Saldana", "Al Capone"];

    // A vector of (index, String) tuples
    let the_night_s_dawn: Vec<(usize, String)> = THE_NIGHT_S_DAWN.iter().enumerate().map(|(i, s)| (i, s.to_string())).collect();

    // Consumes the vector
    for (i, s) in the_night_s_dawn {
        println!("String {} is : {}", i, s);
    }
    println!();

    // The vector was consumed above; we can't reuse it.
    /*
    for (i, s) in the_night_s_dawn {
        println!("String {} is : {}", i, s);
    }
    */

    // New vector (same content)
    let the_night_s_dawn: Vec<(usize, String)> = THE_NIGHT_S_DAWN.iter().enumerate().map(|(i, s)| (i, s.to_string())).collect();

    // Using references
    for (i, s) in &the_night_s_dawn {
        // i is &usize, s is &String
        println!("String {} is : {}", *i, *s); 
        println!("String {} is : {}", i, s); // Works due to deref coercion
    }
    println!();

    // This pattern would fail because it tries to move the String
    /*
    // &vec: yields &(T1, T2)
    // Pattern &(x, y): destructures &tuple, trying to move out x and y
    // But if x or y is non-Copy, this move is forbidden
    for &(i, s) in &the_night_s_dawn {
        println!("String {} is : {}", i, s);
    }
    */

    // The iterator yields references to tuples: &(usize, String)
    // Trying to destructure them with `&(i, s)` implies moving the String,
    // which is forbidden without Copy.
}
```

You should get

```
Test05 :
String 0 is : Joshua Calvert
String 1 is : Lagrange Calvert
String 2 is : Quinn Dexter
String 3 is : Louise Kavanagh
String 4 is : Ione Saldana
String 5 is : Al Capone

String 0 is : Joshua Calvert
String 0 is : Joshua Calvert
String 1 is : Lagrange Calvert
String 1 is : Lagrange Calvert
String 2 is : Quinn Dexter
String 2 is : Quinn Dexter
String 3 is : Louise Kavanagh
String 3 is : Louise Kavanagh
String 4 is : Ione Saldana
String 4 is : Ione Saldana
String 5 is : Al Capone
String 5 is : Al Capone
```

### Comments
Let's comment the code but I'm pretty sure you could do it by yourself...
* `THE_NIGHT_S_DAWN` is a constant. It is an array of 6 string slice.
* We use the constant array to create the `the_night_s_dawn` variable, a vector of tuples consisting of an integer and a String
* As usual, the first `for` loop consumes the vector
* The second, commented `for` loop would not compile
* We re-create the `the_night_s_dawn` vector
* In the third loop `i` and `s` have the type `&usize` and `&String` respectively
* We print the content of the vector leveraging deref coercion or not
* For exactly the same reasons explained in Test04 the last `for` loop would not compile











## A last experiment, just to make sure
OK. This one comes from one solution to the [Coding Interview Patterns problems]({%link docs/06_programmation/rust/007_coding_interview_patterns/13_graphs/268_longest_increasing_path.md%})
 
Copy, paste and CTRL+ENTER the code below. 

```rust
fn main() {
    println!("\nTest06 :");

    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // All these loops work because the tuple elements are primitive types (isize)

    // Iteration by value (copied)
    for (dr, dc) in DIRECTIONS {
        let (r, c) = (42, 101);
        let (new_r, new_c) = (r as isize + dr, c as isize + dc);
        println!("{} - {}", new_r, new_c);
    }
    println!();

    // Iteration by reference
    for (dr, dc) in &DIRECTIONS {
        let (r, c) = (42, 101);

        let (new_r, new_c) = (r as isize + *dr, c as isize + *dc);
        println!("{} - {}", new_r, new_c);
        
        let (new_r, new_c) = (r as isize + dr, c as isize + dc);
        println!("{} - {}", new_r, new_c);
        
    }
    println!();

    // Explicit pattern matching with destructuring
    for &(dr, dc) in &DIRECTIONS {
        let (r, c) = (42, 101);
        let (new_r, new_c) = (r as isize + dr, c as isize + dc);
        println!("{} - {}", new_r, new_c);
    }
    println!();
}
```
This is the expected output :

```
Test06 :
41 - 101
43 - 101
42 - 100
42 - 102

41 - 101
41 - 101
43 - 101
43 - 101
42 - 100
42 - 100
42 - 102
42 - 102

41 - 101
43 - 101
42 - 100
42 - 102
```

### Comments

Again, before reading the explanations below, look at the code and find out the iterable. What is its type? What is the type of its elements... By now you should have enough information to understand what is possible and what is not possible.

* `DIRECTIONS` is a constant. It is an array (on the stack) of 4 tuples. The tuples contains displacement differentials along the rows and columns. Traversing the `DIRECTIONS` array and adding the content of the tuples to a cell whith the coordinates (r, c) we will visit the 4 cells (left, right, up and down) around it.
* The iterable is on the stack and it contains primitive type
* In the first loop the array is copied
* In the second loop it is borrowed and with the help of deref corecion we can dereference (or not) the loop variables (`dr`, `dc`)
* Since the elements are primitve we can unstructure and unreference each element `&(dr, dc)`
* `dr` and `dc` are of type `isize` directly












## What about using an iterator?

We will use the code below.

```rust

fn main() {
    println!("\nTest07 :");

    // Array or vector work
    let star_trek = ["James T. Kirk", "Spock", "Dr. Leonard H. McCoy (Bones)", "Nyota Uhura", "Montgomery Scott (Scotty)"];
    // let star_trek = vec!["James T. Kirk", "Spock", "Dr. Leonard H. McCoy (Bones)", "Nyota Uhura", "Montgomery Scott (Scotty)"];

    // .iter() yields &T
    for s in star_trek.iter() {
        // s is of type &&str (reference to a string slice)
        println!("String is : {}", s); // thanks to deref coercion
        println!("String is : {}", *s); // explicit dereferencing
    }

    println!();

    // The original vector is still available because we only borrowed it
    println!("Original vector: {:?}", star_trek);
}
```

The output should be

```
Test07 :
String is : James T. Kirk
String is : James T. Kirk
String is : Spock
String is : Spock
String is : Dr. Leonard H. McCoy (Bones)
String is : Dr. Leonard H. McCoy (Bones)
String is : Nyota Uhura
String is : Nyota Uhura
String is : Montgomery Scott (Scotty)
String is : Montgomery Scott (Scotty)

```

### Explanation
* Comment/uncoment the line you want to either use an array of a vector
* `.iter()` returns an iterator (`&T`) over the slice and the iterator yields all items from start to end 
* Here the iterable contains string slices (&str) so the type of `s` is `&&str`
* We print the string slice using `*s` or `s`
* So, now, we can either write `for s in &collection` or `for s in collection.iter()`



### Why ?

Why should I use `for s in collection.iter()`? Even though `for s in &collection` and `for s in collection.iter()` are equivalent, using `.iter()` may have some advantages:

1. It makes your intent explicit, especially in generic code
1. It allows chaining with iterator adaptors like .filter() or .map()
1. It aligns visually with `.iter_mut()` and `.into_iter()`, which are required when mutating or consuming the collection












## `.iter_mut()`... What did you say?

Let's play with the code below


```rust
fn main() {
    println!("\nTest08 :");

    let mut space_1999 = vec!["John Koenig".to_string(), "Docteur Helena Russell".to_string(), "Victor Bergman".to_string(), "Alan Carter".to_string()];

    // .iter_mut() yields &mut T. It returns an iterator over a mutable references. Here &mut String
    for s in space_1999.iter_mut() {
        // s is of type &mut String
        s.push_str(" 💥"); // we can mutate the content via the mutable reference
    }

    // Print the modified vector. Both versions are strictly equivalent
    for s in space_1999.iter() {
        // s is of type &String
        println!("Updated string : {}", s);
    }
    println!();

    for s in &space_1999 {
        // s is of type &String
        println!("Updated string : {}", s);
    }
}
```

The expected output :

```
Test08 :
Updated string : John Koenig 💥
Updated string : Docteur Helena Russell 💥
Updated string : Victor Bergman 💥
Updated string : Alan Carter 💥

Updated string : John Koenig 💥
Updated string : Docteur Helena Russell 💥
Updated string : Victor Bergman 💥
Updated string : Alan Carter 💥
```


### Explanation
* We create a **mutable** iterable. Here, `space_1999` is a mutable vector of `String`
* As the name suggest, `.iter_mut()` returns an iterator (`&mut T`) that allows modifying the values of the mutable iterable
* Again, the iterator yields all items from start to end
* In the loop we modify each String by adding another string (" 💥")
* Once this is done we print the content of `space_1999` vector using 2 strictly equivalent variations : `for s in collection.iter()` and `for s in &collection` 




## Is there a way to consume an iterable with an `.iter_xxx()` method ?

Copy, paste and CTRL+ENTER the code below :

```rust

fn main() {
    println!("\nTest09 :");

    let wargames = vec!["Professor Stephen Falken".to_string(), "David Lightman".to_string(), "Joshua".to_string(), "WOPR".to_string()];

    // .into_iter() yields T (moves ownership). It consumes the vector and moves ownership of each String
    for s in wargames.into_iter() {
        // s is of type String (owned)
        println!("String is : {}", s);
    }

    println!();

    // The following line would not compile:
    // println!("{:?}", wargames);
    // because the vector was moved and is no longer accessible
}
```

The output will be

```
Test09 :
String is : Professor Stephen Falken
String is : David Lightman
String is : Joshua
String is : WOPR
```

### Explanation
* We create `wargames`, a vector of `String`
* `.into_iter()` yields `T`
* I have some difficulties with the name `.into_iter()`. Mnemonic : `.into_iter()` → "into the loop, I move ownership in"
* During the loop, the iterator consumes the iterable
* At the end of the loop, the collection is no longer available
* This explains why the commented `println!` would not compile


### What would be the use case ?

OK, the previous code snippet looks somewhat artificial and you want to speak to my supervsior... Look at the code below : 

```rust
fn main() {
    println!("\nTest10 (with .iter()):");

    let original = vec!["Neo", "Trinity", "Morpheus"];

    let modified: Vec<String> = original.iter()
        .map(|s| {
            let mut name = s.to_string(); // clone implicit since &str
            name.push_str(" 🦀");
            name
        })
        .collect();

    println!("Original: {:?}", original); // still available
    println!("Modified: {:?}", modified);
}

```

```
Test10 (with .iter()):
Original: ["Neo", "Trinity", "Morpheus"]
Modified: ["Neo 🦀", "Trinity 🦀", "Morpheus 🦀"]
```


Now run this code

``` rust
fn main() {
    println!("\nTest10 (with .into_iter()):");

    let original = vec!["Neo".to_string(), "Trinity".to_string(), "Morpheus".to_string()];

    let modified: Vec<String> = original.into_iter()
        .map(|mut s| {
            s.push_str(" 🦀");
            s
        })
        .collect();

    // println!("{:?}", original); // does not compile: moved
    println!("Modified: {:?}", modified);
}

```

```
Test10 (with .into_iter()):
Modified: ["Neo 🦀", "Trinity 🦀", "Morpheus 🦀"]
```

### Notes
* Use `.into_iter()` when you're done with a value and want to turn it into something else — typically with `.map(...)` and `.collect()`
* It consumes the original collection and returns owned elements (`T`), so we can move or transform them freely
* `.into_iter()` is ideal when we know the original collection won't be needed anymore. It allows us to move, transform, or mutate each element without borrowing or cloning.



### A last example with `.into_iter()`?

```rust
fn rustify_names(names: Vec<String>) -> Vec<String> {
    // `names` is moved into the function and we now fully own it
    // We use .into_iter() to consume the vector and get owned `String` values
    names
        .into_iter()
        .map(|mut name| {
            name.push_str(" 🦀");
            name
        })
        .collect()
}

fn main() {
    println!("\nTest11 :");

    let simpsons = vec![
        "Homer".to_string(),
        "Marge".to_string(),
        "Bart".to_string(),
        "Lisa".to_string(),
        "Maggie".to_string(),
    ];

    // `simpsons` is moved into the function
    let rustified = rustify_names(simpsons);

    // println!("{:?}", simpsons); // would not compile: simpsons was moved
    println!("Updated names: {:?}", rustified);
}

```

```
Test11 :
Updated names: ["Homer 🦀", "Marge 🦀", "Bart 🦀", "Lisa 🦀", "Maggie 🦀"]

```

















## Bonus point... Do you know in Rust a `for` loop is an expression ?
And not a statement?

Even though we often use a `for` loops for its side effects (like printing), it’s actually an expression that evaluates to the unit type ().

So it’s rarely useful to store the result of a for `loop`, but we can include it in larger expressions, like this:

```rust
fn main() {
    println!("\nTest12 :");

    // The for loop in Rust is an expression that evaluates to the unit type ().
    // This means we can assign it to a variable — although it's not very useful.
    // This compiles and runs just fine. 
    // The compiler might issue a warning if you don’t use it — but that’s all.

    let result = for x in [1, 2, 3] {
        println!("{x}");
    }; 

    // `result` hold the unit value ()
    println!("The result of the for loop is: {:?}", result);

    // However, since `for` always evaluates to (), its value is rarely useful.
    // If you want to accumulate a result, you typically do it inside a block:

    let sum = {
        let mut acc = 0;
        for x in [1, 2, 3] {
            acc += x;
        }
        acc // the value returned by the block
    };
    println!("\nThe sum : {sum}");
}
```

```
Test12 :
1
2
3
The result of the for loop is: ()

The sum : 6

```

### To keep in mind
In Rust, almost everything is an expression — including `if`, `match`, `blocks`, and loops (except `while` loops when they do not return a value with a `break`).


















## Conclusion

The examples use array and Vec. This is fine but but what have been said also apply to other data structures like HashMap, HashSet, BTreeMap...

* Iteration in Rust is powerful, but it requires clarity about ownership.
* Default to using references (`&collection`) in for loops to avoid accidentally moving data.
* Be mindful of the types: whether the collection holds primitive values (like `i32` or `&str`) or heap-allocated ones (like `String`).
* Understand what `.iter()`, `.into_iter()`, and `.iter_mut()` do, especially when working with Vec, HashMap, or arrays:
    * `.iter()` → yields shared references
    * `.into_iter()` → yields owned values (moves ownership)
    * `.iter_mut()` → yields mutable references (for in-place edits)
* If the compiler complains, read the error carefully — it usually gives you a clear hint about what's being moved or borrowed.
* In doubt, ask yourself: “What’s being passed — the value, or a reference? Am I trying to move something I only borrowed?”
* Can you explain the difference between **dereferencing vs destructuring** to your 10 years old kid or sister ?

### Webliography
* Feel free to read this page in [THE book](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html#for-loops)
* Now, [read the bible](https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html#iterator-loops) you can.