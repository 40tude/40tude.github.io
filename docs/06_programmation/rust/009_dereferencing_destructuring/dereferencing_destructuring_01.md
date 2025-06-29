---
published: true
layout: default
title: "Rust Dereferencing vs Destructuring — For the Kids 1/2"
parent: "Rust"
#math: mathjax
date               : 2025-06-27 09:00:00
last_modified_date : 2025-06-27 09:00:00
---

<!-- <h2 align="center">
<span style="color:red"><b>This post is still being written.</b></span>    
</h2> -->

# Rust Dereferencing vs Destructuring — For the Kids 1/2
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
## Dereferencing: A smooth start

Copy and paste the code below in the [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=95688f886df53f6776df8a63c0599ccd) then hit CTRL+ ENTER

With the others sample code I will only show the function of interest, not the `main()` function.

```rust
fn dereferencing01() {
    println!("\nDereferencing 01 : 101");
    let my_value = 5; // => my_value: i32
    println!("my_value : {}", my_value);
    let addr_of_my_value = &my_value; // => addr_of_my_value: &i32
    println!("addr_of_my_value : {:p}", addr_of_my_value);
    let content_at_addr_of_my_value = *addr_of_my_value; // content_at_addr_of_my_value => i32
    println!("content_at_addr_of_my_value : {}", content_at_addr_of_my_value);
}
fn main(){
    dereferencing01();
}
```

### Expected output 
{: .no_toc }
{: .no_toc }

```
Dereferencing 01 : 101
my_value : 5
addr_of_my_value : 0x7ffeac669fa4
content_at_addr_of_my_value : 5
```


### Explanations
{: .no_toc }
* `my_value` is a binding (a variable) of type `i32` and it has the value 5. If you are not sure about the difference between binding and variable keep the term variable in mind for now, **but**, stand up, put your right hand on your heart and swear you will read this [page in English]({%link docs/06_programmation/rust/004_mutability/mutability_us.md%}).
* Next come the interesting part. `addr_of_my_value` is a **reference** to `my_value`. Its value is the memory address of `my_value`. I hope you already know that every variable, data structure, code... Is somewhere in the memory of the PC. So they all have a memory address. The syntax is what it is and the `&my_value` means, in plain English : address of `my_value`






***Ah OK. So a reference is a pointer. Right?*** Almost because they are both used to indirectly access and manipulate objects but **no** because they have key differences. Since there is no pointer in Rust (I know `*const T` and `*mut T` exist but let's avoid them for today), let's illustrate de differences between pointers and references using C++. No worries, the syntax is very similar.  

### References and Pointers comparaison in C++
{: .no_toc }


#### **1. Syntax and Declaration**
{: .no_toc }
- **Pointer**: Declared with `*`, can be reassigned to point to different objects (or `nullptr`).
  ```cpp
  int x = 10;
  int* ptr = &x; // ptr is a pointer pointing to x
  int y = 20;
  ptr = &y;      // Now pointer is pointing to y
  ```
- **Reference**: Declared with `&`, must be initialized upon declaration and **cannot be reassigned**.

  ```cpp
  int x = 10;
  int& ref = x; // ref is an alias, another name, for x
  int y = 20;
  ref = y;      // Doesn't make ref point to y; instead, assigns y's value to x 
                // because ref is an alias of x
  ```

#### **2. Nullability**
{: .no_toc }
- **Pointer**: Can be `nullptr` (uninitialized or explicitly set to null).
  ```cpp
  int* ptr = nullptr;   // Valid
  ```
- **Reference**: Must always refer to a valid object (no null references).
  ```cpp
  int& ref;  // Does'nt compile: Must be initialized
  ```

#### **3. Memory Address Handling**
{: .no_toc }
- **Pointer**: Stores the memory address of an object. You can perform arithmetic (e.g., `ptr++`).
  ```cpp
  int arr[3] = {1, 2, 3};
  int* ptr = arr;
  ptr++;  // Moves to next element
  ```
- **Reference**: Acts as an **alias** (another name) for an existing variable. No arithmetic.
  ```cpp
  int x = 10;
  int& ref = x; // ref is just another name for x; no address manipulation.
  ```

#### **4. Indirection & Dereferencing**
{: .no_toc }
- **Pointer**: Requires explicit dereferencing (`*`) to access the value.
  ```cpp
  int x = 10;
  int* ptr = &x;
  cout << *ptr;  // Outputs 10
  ```
- **Reference**: Automatically dereferenced (no need for `*`).
  ```cpp
  int x = 10;
  int& ref = x;
  cout << ref;  // no * needed. ref acts as an alias 
  ```

#### **5. Safety**
{: .no_toc }
- **Pointer**: Risk of dangling pointers (pointing to freed memory).  
- **Reference**: Safer (cannot be null, always bound to an object).  

#### **Summary Table**
{: .no_toc }

| Feature           | Pointer (`*`)                          | Reference (`&`)         |
|-------------------|----------------------------------------|-------------------------|
| Syntax            | `int* ptr = &x;`                       | `int& ref = x;`         |
| Reassignable?     | Yes                                    | No                      |
| Can be null?      | Yes                                    | No                      |
| Memory arithmetic | Yes                                    | No                      |
| Dereferencing     | Explicit dereferencing `cout << *ptr;` | No `cout << ref;`       |

#### To keep in mind in a Rust context
{: .no_toc }

* Syntax 
    * `let ref_to_my_value = &my_value;` 
    * `let content_ref     = *ref_to_my_value;`
* Not dangling. A reference is bound to an object
* Reassignable if `&mut`, not reassignable otherwise 
* No arithmetic on a reference









--- 
## Dereferencing: Mutability

Just to make sure we are on the same page. There are 2 kinds of mutability to consider here :
1. We may want the reference to point to a mutable variable : mutability of the reference
1. We may want the reference to be able to point to different variables (of the same type) : mutability of the binding

If you don't feel confident enough to explain this concept to your kids, read this [page about Mutability]({%link docs/06_programmation/rust/004_mutability/mutability_us.md%}).

In the the code below we focus on the mutability of the reference. Let's run the code!

<!-- 
```rust
fn main(){
    // In Rust, the term “mutable reference” (&mut) refers only to the right to modify the pointed data, 
    // not to the possibility of reassigning the reference itself.
    // The mutability of the binding (let vs let mut) is a distinct and independent notion.

    let my_value = 5; 
    let ref_to_my_value = &my_value; // immutable reference to an immutable variable
    // my_value +=1; // does not compile because my_value is not mut
    // *ref_to_my_value += 1; // does not compile because it refers to &my_value not to &mut my_value
    println!("{}", ref_to_my_value); // 5

    let my_value = 5; 
    // let ref_to_my_value = &mut my_value; // does not compile because my_value in not mut
    println!("{}", my_value); // 5

    let mut my_value = 5; 
    my_value +=1; 
    let ref_to_my_value = &my_value; // immutable reference to a mutable variable
    // my_value +=1; // does not compile compile because my_value is borrowed
    // *ref_to_my_value += 1; // does not compile because it refers to &my_value not &mut my_value
    println!("{}", ref_to_my_value); // 6
    // my_value +=1; // possible as soon as ref_to_my_value goes out of scope (end of borrowing)
    
    let mut my_value = 5; 
    my_value +=1; 
    let ref_to_my_value = &mut my_value; // mutable reference to a mutable variable
                                         // &mut my_value creates a mutable reference (&mut T), which can be used to modify the pointed value.
    *ref_to_my_value += 1; // the 
    println!("{}", ref_to_my_value); // 7
}
``` 
-->



```rust
fn dereferencing02_1() {
    println!("\nDereferencing 02_1 : Mutability of the reference\n");
    let my_value = 5; // immutable variable
    println!("my_value : {}", my_value);
    let ref_to_my_value = &my_value; // immutable reference to immutable variable
    println!("ref_to_my_value : {}", ref_to_my_value);
    println!();

    // *ref_to_my_value = 24; // => does not compile
                              // ref_to_my_value is a `&` ref so the data it refers to cannot be written

    let mut my_mutable_value = 55; // mutable variable
    println!("my_mutable_value : {}", my_mutable_value);
    let ref_to_my_mutable_value = &mut my_mutable_value; // mutable reference to mutable variable
                                                         // &mut my_mutable_value creates a mutable reference
                                                         // which can be used to modify the pointed value
                                                         // my_mutable_value is already a mutable variable
    println!("ref_to_my_mutable_value : {}", ref_to_my_mutable_value);
    println!();
    *ref_to_my_mutable_value += 1;
    println!("ref_to_my_mutable_value : {}", ref_to_my_mutable_value);
    println!("my_mutable_value : {}", my_mutable_value);
    println!();
}
```

### Expected output 
{: .no_toc }

```
Dereferencing 02_1 : Mutability of the reference
my_value : 5
ref_to_my_value : 5
my_mutable_value : 55
ref_to_my_mutable_value : 55
ref_to_my_mutable_value : 56
my_mutable_value : 56
```



### Explanations
{: .no_toc }

* Like in the very first code snippet, we create a variable (immutable variable, `let my_value = 5;`) and print its content
* Then we create an immutable reference to the immutable variable (`let ref_to_my_value = &my_value;`) and print its content
* The commented line does not compile (`*ref_to_my_value = 24;`). With this setup we cannot mutate the content of the reference (and the variable)

Here's how we can address this issue
* We create and print a mutable variable (`let mut my_mutable_value = 55;`)
* Then we create (`let ref_to_my_mutable_value = &mut my_mutable_value;`) and print the content of a mutable reference to a mutable variable. The point I did'nt get at the beginning is that this is the `&mut my_mutable_value` that create a mutable reference which is assigned to a non mutable variable named `ref_to_my_mutable_value` (`let ref_to_my_mutable_value`). 
* We mutate the content of the reference (`*ref_to_my_mutable_value += 1;`)
* Finally we print both, the content of the reference and the variable


Now, let's run this code :

```rust
fn dereferencing02_2() {
    println!("\nDereferencing 02_2 : Mutability of the binding\n");
    let my_value = 5; // immutable variable
    println!("my_value : {}", my_value);
    let other_value = 42;
    println!("other_value : {}", other_value);
    println!();
    let ref_to_my_value = &my_value; // immutable reference to immutable variable
    println!("ref_to_my_value : {}", ref_to_my_value);
    println!();

    // ref_to_my_value = &other_value; // => does not compile
                                       // cannot assign twice to immutable variable `ref_to_my_value`
    
    let ref_to_my_value = &other_value; // => shadowing. Does compile
    println!("ref_to_my_value : {}", ref_to_my_value); // => ref_to_my_value: &i32
    println!();
    let mut mut_ref_to_my_value = &my_value; // mutable reference to immutable variable
    println!("mut_ref_to_my_value : {}", mut_ref_to_my_value);
    mut_ref_to_my_value = &other_value; // mut_ref_to_my_value now reference other_value
    println!("mut_ref_to_my_value : {}", mut_ref_to_my_value);
    let other_value = std::f64::consts::PI; // => other_value: f64
    println!("other_value : {}", other_value);

    // mut_ref_to_my_value = &other_value; // => does not compile: expected `&{integer}`, found `&f64`
}

```

### Expected output 
{: .no_toc }

```
Dereferencing 02_2 : Mutability of the binding
my_value : 5
other_value : 42
ref_to_my_value : 5
ref_to_my_value : 42
mut_ref_to_my_value : 5
mut_ref_to_my_value : 42
other_value : 3.141592653589793
```
### Explanations
{: .no_toc }

In the code we look at the mutability of the binding. We want the reference being able to "point to" different variables.

* We create 2 immutable variables (`let my_value = 5;`, `let other_value = 42;`)
* We create an immutable reference to an immutable variable (`let ref_to_my_value = &my_value;`)
* The commented line does not compile (`ref_to_my_value = &other_value;`). Indeed the reference is immutable. It cannot mutate, it cannot "point to" another variable.

Here's what we can do
* We could "write over" (shadowing) the previous reference (`let ref_to_my_value = &other_value;`)

Here's another option
* We create a mutable binding to an immutable variable (`let mut mut_ref_to_my_value = &my_value;`)
* The binding `mut_ref_to_my_value` can then be mutated and "point to" another variable (`let mut mut_ref_to_my_value = &my_value;` then `mut_ref_to_my_value = &other_value;`)
* Such mutable binding can only points to variable of the same type. The first version of `other_value` was an `i32`. Now if we create an `f64` version of `other_value` (`let other_value = std::f64::consts::PI;`), `mut_ref_to_my_value` cannot point to it. The very last line does not compile.




---
***That is fine but why should I care? I mean, what is the purpose?*** One of the key usage of references is to pass efficiently arguments to functions. Let's see how it works now.

## Dereferencing: Reference as argument

```rust
fn dereferencing03() {
    println!("\nDereferencing 03 : ref as argument\n");
    fn my_function01(v: Vec<i32>) {
        println!("{:?}", v);
    }
    fn my_function02(v: &Vec<i32>) {
        println!("{:?}", *v);
        println!("{:?}", v); // deref coercion in action
    }
    fn my_function03(v: &[i32]) {   // accept reference to vectors or arrays
        // println!("{:?}", *v);    // Does not compile because *v is of type [i32] with no Sized trait 
                                    // Sized trait expected by println!()
                                    // Only references like `&[i32]` implement the `Debug` trait; 
                                    // `[i32]` alone doesn't, as it's dynamically sized)
        println!("{:?}", &*v);      // Overkill ?
        println!("{:?}", v);
    }
    let my_vector = vec![42, 43, 44];
    my_function01(my_vector); // after the call my_vector disappears

    // println!("{:?}", my_vector); // Does not compile

    let my_vector = vec![42, 43, 44]; // must recreate my_vector
    my_function02(&my_vector); // pass a reference
    my_function03(&my_vector);
    let my_array = [142, 143, 144]; // an array on the stack
    my_function03(&my_array);
}
```


### Expected output 
{: .no_toc }
```
Dereferencing 03 : ref as argument
[42, 43, 44]
[42, 43, 44]
[42, 43, 44]
[42, 43, 44]
[42, 43, 44]
[142, 143, 144]
[142, 143, 144]
```


### Explanations
{: .no_toc }
* Yes we can! Yes we can have function definitions inside function definition
* `my_function01` has a unique parameter of type vector of `i32`
* `my_function02` has a unique parameter of type reference to a vector of `i32`
* `my_function03` has a unique parameter of type slice (`&[T]`) of `i32`

#### Pass by value
{: .no_toc }
Then we create `my_vector` (`let my_vector = vec![42, 43, 44];`), a vector of `i32` and we give it as an argument to `my_function01`
* The argument is passed by value
* So it is given to the function and we lost it (RIP)
* After the call to `my_function01` we cannot print `my_vector`

Before to move forward we must recreate `my_vector` (`let my_vector = vec![42, 43, 44];`)

#### Pass by reference
{: .no_toc }
The 2 calls to `my_function02` and `my_function03` are much more interesting
* In both calls `my_vector` is passed by reference (`my_function02(&my_vector);`)
* `my_vector` is **borrowed** to the functions
* It is not given, it does not disappear after the call
* This is why `my_vector` is still available after the first call and can be reused in the second (`my_function03(&my_vector);`)





***This is all good... But why do we do this?*** 
What I will say here is not 100% accurate but bear with me because the idea is to get the concept, then I'll tell you the truth. I promise. 

* In the call to `my_function01` we passed the vector by value. It is like passing the 3 values. We push them on the stack. Call the function. In the function, we pop the values from the stack, then work on them. It was OK because we only had 3 `i32`. What if we had 1_000_000? This would be inefficient. 
* In the calls to `my_function02` and `my_function03` we pass a reference, the address of the `my_vector`. No matter the length of the vector we will always use 8 bytes (on a 64 bit OS, a memory address is on 8 bytes). This is much smarter and much faster.  





***OK... But what is going on in `my_function02`?*** 
* In `my_function02`, `v` is a parameter of type reference to a vector of `i32` (`&Vec<i32>`). If we want to get access to its content, we must dereference it. This is why the first `println!` has `*v` as an argument (`println!("{:?}", *v);`).
* However, like in C++, in the second call to `println!` we can use `v` as a parameter (`println!("{:?}", v);`). The Rust compiler will then apply deref coercion, to transform, on the fly, the `&Vec<i32>` (`v`) into a `Vec<i32>` (`*v`). How does this work here ?
    * `println!` expects a type which implement Debug trait but `&Vec<i32>` doesn’t 
    * The compiler looks for Deref trait implementations on `&Vec<i32>`
    * `Vec<T>` implements `Deref<Target = [T]>`, meaning `&Vec<i32>` can auto-convert to `&[i32]` (a slice of `i32`)
    * And `&[i32]` implements the Debug trait, so the reference to a vector of `i32` is printed as a slice of `i32`.




***And what about `my_function03`?***
* In `my_function03`, `v` parameter is a slice of `i32` (`&[i32]`)
* The commented `println!` does not compile (`println!("{:?}", *v);`). Indeed, `*v` is of type `[i32]` and this type does not implement the Sized trait expected by `println!`
* If we really want to dereference the parameter then we can use the second `println!` but it looks weird (`println!("{:?}", &*v);`)
* The third `println!` is the way to go (`println!("{:?}", v);`)


* Last but not least, we create an array (`my_array`) and pass a reference to it as an argument to `my_function03` (`my_function03(&my_array);`)
    * This demonstrate that `my_function03` works fine when it receives as a parameter either a reference to vector or an array (thanks to deref coercion). 



***You promised to tell the truth about what happens when you pass a vector by value...*** 
In fact, the content of a vector is allocated on the heap (`[42, 43, 44]`) and you can view a vector as a struct with 3 fields (PLC) : 
1. address of the data on the heap (P, pointer)
1. the len of the vector (L, len)
1. the capacity of the vector (C, capacity >=len)

So, yes, I lied. When we pass by value a vector of 100 `i32` we do not passe 100 values. We only pass 3 `i64`(address, len and capacity). Nevertheless what is wrong for a vector is true for an array. An array is just a set of contiguous memory addresses on the stack and if we pass an array by value we pass its content by value.










---
***You mentioned data on the heap. How to dereference this kind of data ?*** You read my mind this is what we will focus on now.

## Dereferencing: Allocations on the heap

```rust
fn dereferencing04() {
    println!("\nDereferencing 04 : Box, Rc and Deref\n");
    // Function that takes a value by reference
    fn print_ref(v: &i32) {
        println!("Value: {}", *v);
        println!("Value: {}", v);
    }
    // Function that takes a Box<i32>
    fn print_box(v: Box<i32>) {
        println!("Boxed value: {}", v);
    }
    // Create a value on the heap using Box
    let b = Box::new(123);
    println!("Address of the heap in the Box : {:p}", b);
    println!("Address of b on the stack      : {:p}", &b);
    // We can dereference Box<T> directly thanks to Deref
    println!("Dereferenced Box: {}", *b);
    // The function expects &i32, but we give it &Box<i32>
    // Thanks to deref coercion, this works
    print_ref(&b);
    // Can also pass the Box directly if signature matches
    print_box(b); // b is moved here
}
```


### Expected output 
{: .no_toc }
```
Dereferencing 04 : Box, Rc and Deref
Address of the heap in the Box : 0x5e603cc1fb10
Address of b on the stack      : 0x7ffc6dbbec38
Dereferenced Box: 123
Value: 123
Boxed value: 123
```


### Explanations
{: .no_toc }
* We first define 2 functions `print_ref` and `print_box`
* Then, in order to allocate memory on the heap we use `Box::new()` (`let b = Box::new(123);`)
* Let's keep in mind this create a unique pointer that own the pointed area.
* Here the required space to store the value 123 (an `i32`, 4 bytes) is allocated on the heap
* `b` a variable of type `Box<i32>` remains on the stack. `b` is a smart pointer which implements **RAII**
* RAII = Ressource Acquisition Is Initialisation. This term is pretty well known in C++. This creates a wrapper around the allocated memory and warranty that the memory will be automatically freed when `b` goes out of scope (even if a `panic` or an early `return` happens)
* Once the variable `b` exists, with a "simple" `println!` we print, the address on the heap which is in the Box (`println!("Address of the heap in the Box : {:p}", b);`).
* Just to make sure we understand that the address of `b` (data type `Box<i32>`) on the stack has nothing to do with the address in the boxe (pointing to the heap) we print the address of `b` so that we can compare (`println!("Address of b on the stack      : {:p}", &b);`).
* Next we dereference the boxe `b` and print the value it points to (`println!("Dereferenced Box: {}", *b);`). This is really cool because once the boxe is created, we use `b` has a regular reference to an `i32`.
* In the call to `print_ref` we pass a reference to the box (`print_ref(&b);`). The box is borrowed and it remains available after the call. 
* In the `print_ref` function the first `println!("Value: {}", *v);` is what we should write but the deref coercion allow us to write the second (`println!("Value: {}", v);`)
* In the last call, make sure to understand that `b` (a variable on the stack) is moved and no longer available right after the call to `print_box`. This is at this point that RAII mechanism will work behind the scene and deallocate the memory on the heap. 



---
***Ok. Now I know how to safely allocate memory on the heap. But can I have 2 boxes pointing to the same area?*** I know what you mean. The heap allocated memory could be a picture of your brand new [Aprilia RSV4](https://www.aprilia.com/en_EN/models/rsv4/) and you would like to make sure your friends can look at it without modifying it. This is not possible with a box directly. So let's find a solution...



## Dereferencing: `Rc<T>` and Reference Count
Indeed, in order to manage memory efficiently we need to be smarter than a box and to include a counter in order to know how many people are currently watching the picture of your motorbike. Let's look at the code below :

```rust
use std::rc::Rc;
fn dereferencing05() {
    println!("\nDereferencing 05 : Rc<T> and Reference Count\n");
    // Function that takes Rc<i32>
    fn print_rc(v: &Rc<i32>) {
        println!("From print_rc : {}", v);
    }
    // Create an Rc pointing to a value on the heap
    let rc1 = Rc::new(999);
    println!("Initial value: {}", rc1); 
    println!("Address in Rc: {:p}", Rc::as_ptr(&rc1));
    println!("Reference count after creation: {}", Rc::strong_count(&rc1)); // 1
    print_rc(&rc1);
    // Create a clone of rc1 — this does not copy the value
    let rc2 = Rc::clone(&rc1);
    println!("\nAfter cloning rc1 into rc2:");
    println!("rc1 points to: {}", rc1);
    println!("rc2 points to: {}", rc2);
    println!("Reference count (rc1): {}", Rc::strong_count(&rc1)); // 2
    println!("Reference count (rc2): {}", Rc::strong_count(&rc2)); // 2
    {
        // Introduce a new scope
        let rc3 = Rc::clone(&rc2);
        println!("\nInside inner scope with rc3:");
        println!("rc3 points to: {}", rc3);
        println!("Reference count: {}", Rc::strong_count(&rc3)); // 3
    } // rc3 goes out of scope here
    println!("\nAfter rc3 is dropped:");
    println!("Reference count (rc1): {}", Rc::strong_count(&rc1)); // 2
}
```


### Expected output 
{: .no_toc }
```
Dereferencing 05 : Rc<T> and Reference Count
Initial value: 999
Address in Rc: 0x56d9e7fceb20
Reference count after creation: 1
From print_rc : 999

After cloning rc1 into rc2:
rc1 points to: 999
rc2 points to: 999
Reference count (rc1): 2
Reference count (rc2): 2

Inside inner scope with rc3:
rc3 points to: 999
Reference count: 3

After rc3 is dropped:
Reference count (rc1): 2
```



### Explanations
{: .no_toc }

* We first create a reference counting pointer, pointing to the memory on the heap (`let rc1 = Rc::new(999);`)
* Again, just to make sure we are in sync : Rc is for single-threaded only
* This said, we print the value and the address on the heap. I really don't like the fact that `Box` and `Rc` don't have similar API. For example why I can't write :

```rust
use std::rc::Rc;

fn main(){
    let b = Box::new(999);
    println!("Dereferenced Box: {}", b); // 999
    println!("Dereferenced Box: {}", *b); // 999
    println!("Address in the Box : {:p}", b); // 0x5eca24b8eb10
    // Does'nt compile use of unstable library feature `box_as_ptr`
    // println!("Address in the Box : {:p}", Box::as_ptr(&b)); 

    println!();
    
    let rc1 = Rc::new(999);
    println!("Initial value: {}", rc1); // 999
    println!("Initial value: {}", *rc1); // 999
    println!("Initial value: {:p}", rc1); // 0x5eca24b8eb40
    println!("Address in Rc: {:p}", Rc::as_ptr(&rc1)); // 0x5eca24b8eb40
}
```
* Finally we demonstrate how to dereference an Rc when we call `print_rc(&rc1);`. Nothing fundamentally new here.

Then it becomes interesting...

* We print the value of the counter of the smart pointer (`println!("Reference count after creation: {}", Rc::strong_count(&rc1)); // 1`). At this point, only one pointer is pointing to area where `999` is stored

Then it becomes *really* interesting... 

* Indeed we clone the smart pointer we had (`let rc2 = Rc::clone(&rc1);`). It is important to keep in mind that no copy or worst, no deep copy happens. When applied to a reference-counted smart pointer, `.clone()` simply add 1 to the counter. The `.clone()` operation is fast and cheap.
* Once rc2 is created we print the value it points to and the current value of the reference counter (`println!("Reference count (rc2): {}", Rc::strong_count(&rc2)); // 2`). This should not be a surprise but the counter of `rc1` and `rc2` are equal (otherwise we would be in be trouble)

In a last experiment we create a scope (`{` and `}`) where we create another clone (`let rc3 = Rc::clone(&rc2);`).
* Before the end of the scope we display the counter of one of the clones (`println!("Reference count: {}", Rc::strong_count(&rc3)); // 3`)
* After the scope we display the counter of one of the clones (`println!("Reference count (rc1): {}", Rc::strong_count(&rc1)); // 2`). It goes from 3 to 2 since `rc3` no longer exists.










---
***Well, well, well... I know you will NOT talk about references in a multithreaded context but... What if I need to mutate the heap allocated memory ?*** This could be the case where the allocated memory represents your bank account where your company transfer your salary and where you would like to check the total amount available. To do so, we need to be even smarter than before...

## Dereferencing: `Rc<RefCell<T>>` for shared mutation (single-thread)

However, the good news it that instead of learning a new smart pointer, we will reuse what we already know about `Rc` (reference-counted smart pointer) and add interior mutability to the heap allocated memory. This is done using ``RefCell``. First thing first, let's run the code below :  

```rust
use std::cell::RefCell;
use std::rc::Rc;
fn dereferencing06() {
    println!("\nDereferencing 06 : Rc<RefCell<T>> for shared mutation (single-thread)\n");
    // Rc enables multiple ownership, RefCell enables interior mutability
    let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));
    println!("shared_vec: {:?}", shared_vec);
    println!("Reference count: {}", Rc::strong_count(&shared_vec));
    // Clone the Rc to get multiple owners
    let a = Rc::clone(&shared_vec);
    let b = Rc::clone(&shared_vec);
    println!("Reference count: {}", Rc::strong_count(&shared_vec)); // 3
    // Mutate the shared vector from owner `a`
    {
        let mut vec_ref = a.borrow_mut(); // borrow `a` as mutable
        vec_ref.push(4);
        println!("a pushed 4: {:?}", vec_ref);
    }
    // Read from the shared vector via owner `b`
    {
        let vec_ref = b.borrow(); // borrow `b` as immutable
        println!("b sees the vector: {:?}", vec_ref);
    }
    // Shows that the compiler doesn't see borrow conflicts, but the runtime does.
    {
        let _first = shared_vec.borrow_mut();
        // let _second = shared_vec.borrow_mut(); // panics at runtime
    }
    // Reference count stays at 3 until `a`, `b`, and `shared_vec` go out of scope
    println!("Reference count: {}", Rc::strong_count(&shared_vec)); // 3
}
```


### Expected output 
{: .no_toc }
```
Dereferencing 06 : Rc<RefCell<T>> for shared mutation (single-thread)
shared_vec: RefCell { value: [1, 2, 3] }
Reference count: 1
Reference count: 3
a pushed 4: [1, 2, 3, 4]
b sees the vector: [1, 2, 3, 4]
Reference count: 3
```


### Explanations
{: .no_toc }

* Let's say we want to share a vector `shared_vec` of`i32`
* We would write : `let shared_vec = Rc::new(vec![1, 2, 3]);`
* Since we want to add interior mutability (ability to mutate the content of the vector) we write : `let shared_vec = Rc::new(RefCell::new(vec![1, 2, 3]));`
* Everthing we said about cloning and inspecting the counter remains valid. After all, ``shared_vec`` is a `Rc<Vec<i32>>`
* This is true but ``shared_vec`` is a `Rc<RefCell<Vec<i32>>>` and this is what allow us to mutate the heap allocated memory safely (almost)

Then we create the first scope (more on this in few lines)
* Inside the scope, we create `vec_ref` (`let mut vec_ref = a.borrow_mut();`). To make a long story short it borrow the wrapped object (`vec![1, 2, 3]`) with the ability to mutate it (`.borrow_mut()`). 
* Then we push a value in the borrowed vector (`vec_ref.push(4);`) and print it
* And the scope ends right after the `}` `vec_ref` goes out of scope and no one is borrowing the heap allocated vector

We then create a second scope
* We create a new version `vec_ref` (`let vec_ref = b.borrow();`). This time it borrows `b` without the ability to mutate it (`.borrow()` not `.borrow_mut()`).
* The print shows that the mutably shared vector has been mutated

***Why do we need scopes here ?***
This is the one million dollars question... And you should have the answer. Make a test. Remove them and the code will panic on line `let vec_ref = b.borrow();`. Can you say why? 

Without the scopes, when we reach the line `let vec_ref = b.borrow();`, on stage we have one borrower with mutability capabilities and we would like to add a borrower with read-only capabilities. No way. You know the rule : Only one writer or multiple readers. 

One thing to keep in mind. The conflict among borrowers happen at runtime **not** at compile time

This is what is demonstrated in the last scope. 
* If you uncomment the second line `// let _second = shared_vec...`
* The code panic at runtime







---
***Any tips and trick to share ?*** Here are a few common traps and surprises you might encounter (I did)

## Rust Gotchas: Dereferencing Edition

1. **References are not pointers (exactly)**  
   They behave similarly but are *not* the same. You can't do pointer arithmetic, and they must always be valid and non-null.

2. **`&mut` vs `mut`**  
* `mut x`: you're allowed to modify `x`.  
* `&mut x`: you're allowed to modify the *value* `x` points to (&mut is a compound operator in Rust, it is a single “logical keyword”, which reads "mutable reference to")
* But `let mut x = &y;` only means that `x` (the reference) can change to point elsewhere — not that `y` is mutable!

3. **Shadowing vs reassignment**  
   You can "reassign" an immutable reference via *shadowing* (`let x = &y;` again), but trying to reassign without `let` won’t compile.

4. **Boxing isn't cloning**  
   `Box::new(value)` allocates `value` on the heap. It does *not* create a deep copy when passed by value — it moves ownership unless you explicitly `.clone()` the inner value.

5. **Rc<T> cloning doesn’t clone the value**  
   It just increments the reference counter. Great for sharing read-only access — but not safe across threads or for mutation without `RefCell`.

6. **Deref coercion looks like magic**  
   But it's not: it follows well-defined `Deref` rules. Still, don’t rely on it blindly — sometimes explicit `*` helps readability and prevents surprises.

7. **Borrow checker checks borrows at compile time... until `RefCell` enters the game**  
   `RefCell<T>` defers checks to *runtime*. That means your code compiles, but can still panic if you violate borrow rules (e.g., two mutable borrows).
