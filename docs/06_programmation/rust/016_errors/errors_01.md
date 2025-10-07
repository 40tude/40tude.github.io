---
published: true
lang: en-US
layout: default
title: Rust Error Handling, Demystified - 01
description: A beginner-friendly conversation on Errors, Results, Options, and beyond.
parent: Rust
#math: mathjax
date               : 2025-09-20 18:00:00
last_modified_date : 2025-09-26 10:00:00
---



# Rust Error Handling, Demystified
{: .no_toc }

A beginner-friendly conversation on Errors, Results, Options, and beyond.
{: .lead }




<!-- <h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>    
</h2> -->




### This is Episode 01
{: .no_toc }



<div align="center">
<img src="./assets/img00.webp" alt="" width="450" loading="lazy"/><br/>
<span>Let's have a beginner-friendly conversation on Errors, Results, Options, and beyond.</span>
</div>





#### Posts 
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/016_errors/errors_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/016_errors/errors_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/016_errors/errors_02.md%})



## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}









<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## `Option<T>` vs. `Result<T, E>`: Choosing the Right Type



**Alice:** OK... I think I get `Result<T, E>`. But what about [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html)? I’ve seen that too. Is `Option<T>` also for error handling?

**Bob:** [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html) is a sibling to `Result<T, E>` in a way. It’s an `enum` that can be `Some(T)` or `None`. It doesn’t carry an error value like `Result<T, E>` does. `None` just means **absence of a value**. We usually use `Option<T>` when an outcome isn’t an error, but just "no value found" or "not applicable". For example, a function that searches for a substring in a string might return an `Option<usize>` – `Some(index)` if found, or `None` if not found. Not finding the substring isn’t really an "error", it’s an expected possibility.



**Alice:** So the difference is that 
* `Result<T, E>` provides the reason for the error (`E`)
* `Option<T>` gives us nothing on failure

**Bob:** In the case on `Option<T>` I would not say "on failure" because "we don't know". Again, if we need to know **why** something went wrong, we must use `Result<T, E>` because `Option::None` carries no data. If we call a function and get a `None`, we only know that there was no result, not why. With `Result::Err`, we usually get an error type or message explaining the issue.


Also, there’s a **semantic difference**. Other developers reading our code will usually interpret a return type of `Option<T>` as "`None` means not found or not present, which might be normal", whereas `Result<T, E>` means "`Err` means an error occurred during the operation". It’s about expectation. So, using the right return type is a form of communication.


Sometimes we even see combinations, like `Result<Option<T>, E>`. This means the operation itself can fail with an error `E`, or it can succeed and return either `Some(T)` (a value was found) or `None` (no value was found). But that’s an advanced usage.






**Alice:** Can you show me a simple comparison?

**Bob:** Sure. Let’s take a trivial scenario: safe division. Suppose we want to divide two numbers, but if the divisor is zero, that’s not a valid operation. We have two design choices: return an `Option<f64>` (where `None` means division by zero was not possible), or return a `Result<f64, String>` to explicitly signify an error. Here’s what both might look like:




```rust
// ex14.rs

// Using Option: No error message, just None if invalid
fn safe_divide_option(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // indicate failure without detail
    } else {
        Some(a / b)
    }
}

// Using Result: Provide an error message on failure
fn safe_divide_result(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Division by zero") // error string explaining the issue
    } else {
        Ok(a / b)
    }
}

fn main() {
    let x = safe_divide_option(10.0, 0.0);
    let y = safe_divide_result(10.0, 0.0);
    println!("Option version: {:?}", x); // None
    println!("Result version: {:?}", y); // Err("Division by zero")
}
```


* In `safe_divide_option`, if `b` is zero we return `None`. The caller must check for `None` but doesn’t get an automatic reason. They just know it didn’t produce a result. 
* In `safe_divide_result`, if `b` is zero we return an `Err` with a message (here a static `&str` slice, but it could be a more complex error type). The caller on receiving an `Err` knows it was an exceptional case and has a message to work with.

Neither approach is wrong here. It depends on how we view division by zero. If we consider it an error (I would vote for), `Result<T, E>` is suitable. If we treat it like "just no valid answer" and move on without an error context, `Option<T>` could suffice.

The key question to ask: **Is the absence of a value an error condition, or is it an expected case?** If it’s normal/expected (like searching in a map for a key that might not be there), use `Option<T>`. If it’s an error (like couldn’t parse config file), use `Result<T, E>` so we can report what went wrong.



**Alice:** Crystal clear, thanks. And I assume we can use the `?` operator with `Option<T>` similarly, as long as our function returns an `Option<T>`?

**Bob:** Yes, and we already touched on that (see `ex11.rs`). If we use `?` on an `Option<T>` and it’s `None`, it will return `None` from our function early. It’s handy when chaining multiple things that might produce no value. 

But remember, we can’t mix `Result<T, E>` and `Option<T>` with `?` without converting. For example, if we have a `Result<T, E>` and we want to use `?` in a function returning `Option<T>`, we would need to convert that `Result<T, E>` into an `Option<T>` (perhaps by ignoring the error or converting error to `None`). Usually, though, we keep to one or the other in a given function.


You can review `ex13.rs` above where we converted `Option<char>` into `Result<char, String>` but here is an additional sample code where the function returns an `Option<T>` to `main()`:

```rust
// ex15.rs

use std::fs::File;
use std::io::Read;

fn read_file_to_string_as_option(path: &str) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    Some(buf)
}

fn main() {
    let existing = "Cargo.toml";
    let missing = "_definitely_missing_.txt";

    println!("--- read_file_to_string_as_option ---");
    match read_file_to_string_as_option(existing) {
        Some(s) => println!("OK: read {} bytes from {existing}", s.len()),
        None => println!("None: could not read {existing}"),
    }
    match read_file_to_string_as_option(missing) {
        Some(s) => println!("OK: read {} bytes from {missing}", s.len()),
        None => println!("None: could not read {missing}"),
    }
}
```


Here is what you should see in the terminal

```
OK: read 167 bytes from Cargo.toml
None: could not read _definitely_missing_.txt
```

* `read_file_to_string_as_option()` read the whole file if possible, otherwise it returns `None`. 
* We decided (don't ask me why) to "intentionally" ignore the error details by converting `Result<T, E>` to `Option<T>` with `.ok()`, so that the `?` operator can be used in the function. Double check:
    * `open()` returns `Result<File, io::Error>`. We convert it to `Option<File>` with `.ok()`, then `?` works with `Option`
    * Same strategy with `read_to_string()` which returns `Result<usize, io::Error>`



**Alice:** I don't get the point, we're losing sight of why the failure is happening!

**Bob:** You are right. We may be asked to design an API acting that way (drop the error and return `None` on failure). It is a choice. Now, if it is really a concern we can add some **observability**. We keep the `Option<T>` API for the caller (so failures collapse to `None`), but we emit/log diagnostics so that the failures are not invisible. See below an example:

```rust
// ex16.rs
use std::fs::File;
use std::io::Read;

fn read_with_logging(path: &str) -> Option<String> {
    let mut file = File::open(path)
        .map_err(|e| {
            eprintln!("[read_with_logging] open error: {e}");
            e
        })
        .ok()?; // Result<File, io::Error> -> Option<File>

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|e| {
            eprintln!("[read_with_logging] read error: {e}");
            e
        })
        .ok()?; // Result<usize, io::Error> -> Option<usize>

    Some(buf)
}

fn main() {
    let existing = "Cargo.toml";
    let missing = "_definitely_missing_.txt";

    match read_with_logging(existing) {
        Some(s) => println!("OK: read {} bytes from {existing}", s.len()),
        None => println!("None: could not read {existing}"),
    }
    match read_with_logging(missing) {
        Some(s) => println!("OK: read {} bytes from {missing}", s.len()),
        None => println!("None: could not read {missing}"),
    }
}
```

You should read the following in the terminal: 

```
OK: read 167 bytes from Cargo.toml
[read_with_logging] open error: Le fichier spécifié est introuvable. (os error 2)
None: could not read _definitely_missing_.txt
```

* With `existing` file, everything works smoothly. At the end, in `main()` we print the number of bytes in the file. Nothing is logged because there is no error.
* With `missing`, `read_with_logging()` log a message then returns immediately. Note how [`.map_err()`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) is used on a `Result<T, E>` and how the calls `read_to_string().map_err().ok()` are daisy chained. 

{: .note-title }
> Side Note
>
> Do not start grumbling... We will discuss `.map_err()` in detail in the Custom Error Types section, later. For now keep in mind that on error, `.map_err()` we log an explanation and propagate (not early return) the error (`e`) to `.ok()?`.



### Summary – `Option<T>` vs `Result<T, E>`

{: .new-title }
> Summary – `Option<T>` vs `Result<T, E>`
>
* **Use `Option<T>` for expected no value scenarios:** If not finding or not having a value is a normal possibility (not an error), `Option<T>` **communicates** that clearly. `None` carries no error info – it just means no result.
* **Use `Result<T, E>` for error scenarios:** If an operation can fail in a way that is considered an error (and especially if we need to know *why* it failed), use `Result<T, E>` so we can provide an error message or error type. `Err(E)` can hold information about what went wrong.
* **Semantic clarity:** Other developers will interpret `Option<T>` and `Result<T, E>` in our APIs as triggers. 
    * `Option<T>` implies the caller should expect the nothing case and it’s not an exceptional error
    * `Result<T, E>` implies the caller should expect the possibility of an error condition that should be handled or propagated. 
Examples:
* A lookup in a map (key might be missing) -> return `Option<T>` (absence is normal if key not present)
* Parsing input (could fail due to external conditions or bad format) -> return `Result<T, E>` with an error explaining the failure
* **Failure is not an option:** It's must be clear in your mind when choosing between `Option<T>` vs `Result<T, E>`




<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/Tid44iy6Rjs?si=eJfnsc8fjGXTqwhq" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>


* **`?` works with both:** We can propagate `None` early from a function returning `Option<T>` using `?` just like we can propagate errors from `Result<T, E>` with `?`. Just ensure the function’s return type matches (`Option<T>` with `Option<T>`, `Result<T, E>` with `Result<T, E>`).




### Exercises – Option<T> vs `Result<T, E>`
1. Can you find `Option<T>` in the std lib documentation?

1. **Design Decisions:** For each of the following scenarios, decide whether `Option<T>` or `Result<T, E>` is more appropriate as a return type and briefly explain why:
* A function `find_user(username: &str) -> ???` that searches a database for a user and either returns a User object or indicates the user was not found.
* A function `read_config(path: &str) -> ???` that reads a configuration file and returns a configuration object. (What if the file is missing or has invalid contents?)
* A function `index_of(text: &str, ch: char) -> ???` that returns the index of a character in a string, or something if the char isn’t present.

1. **Converting `Option<T>` to `Result<T,E>`:** Write a function `get_env_var(name: &str) -> Result<String, String>` that tries to read an environment variable and returns an error message if it’s not set. 
* `std::env::var(name)` actually returns a `Result`, but pretend it gave us an `Option<String>` 
* How would we convert that `Option<T>` to a `Result<T, E>`? 
* We can use `.ok_or(error message)` on the `Option<T>` to turn a `None` into an `Err`

1. **Mixing `Option<T>` and `Result<T,E>`:** Sometimes we have to deal with both. Imagine a function that tries to get a configuration value from either an environment variable or a config file: `fn get_config_value(key: &str) -> Result<Option<String>, ConfigError>`. This returns `Ok(Some(val))` if found, `Ok(None)` if not found in either place, or `Err(e)` if an error occurred (like file read error). 
* Outline how we would implement this: we might first try env var (which gives `Option`), then file (`Result`), and combine them
* Don’t worry about full code. Focus on how you’d handle the types
* This is to think about how to combine `Option<T>` and `Result` logically

























<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


## To `panic!()` or Not to `panic!()`

**Alice:** Alright... Now I understand recoverable errors. But what about unrecoverable ones? When should I actually use [`panic!()`](https://doc.rust-lang.org/std/macro.panic.html) intentionally?

**Bob:** Panicking is basically saying *this is a fatal problem, abort the mission!* We should use `panic!()` for situations where continuing the program could lead to incorrect results, security vulnerabilities, or when the error is totally unexpected and we don’t have a meaningful way to handle it.

Think of it this way: 
* If failure is something we *expect might happen* occasionally (like a file might not be found, user input might be bad, etc.), we should **not** panic — use `Result<T, E>` and handle it. 
* If something happening indicates a bug in **our code** or an impossible situation (like *this array index should never be out of bounds, something is really wrong*), then jumping thru the window (panicking IOW) is acceptable.






**Alice:** So this happen mostly in cases of logic errors or impossible states. Right?

**Bob:** Exactly. For instance, the standard library panics if we attempt out-of-bounds array access, because that’s a bug in **our code** (we miscalculated an index) and there’s no way to recover or proceed sensibly. The program is in a bad state, so it stops. Another example: if we have a function that absolutely requires a valid, non-null pointer (say, something we built using unsafe code), we might panic if it receives a null pointer. Indeed, that situation should never occur if **our code** is correct.


Panic is also often used to indicate *programmer errors* (violating function contracts). If we document that a function must be called with, say, a positive number, we might choose to panic if someone passes a negative, because the caller violated the API contract. This is not something we want to handle at runtime; it should be fixed in the code. [The Rust Book](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling) discusses that: when a function’s contract is violated, a panic(with a clear message) is appropriate since it’s the **caller’s bug**, and we want them to notice and fix it.





**Alice:** And in testing, panics are fine because a failed `assert!()` or `.unwrap()` will just fail the test, right?

**Bob:** Yes, exactly. In tests, we often use panics (e.g., `assert!()` macros or `.unwrap()`) to immediately fail a test when an invariant isn’t met. That’s a valid use of panic. We *want* to stop if something unexpected happens in a test.

Also, small quick-and-dirty code snippets might sprinkle `.unwrap()` for brevity if you’re OK with them crashing on error. But in a robust application or library, you’d use panic very sparingly.

There’s also the consideration of library vs binary (application) code. 
* If you’re writing a library, we should almost never panic on a recoverable error. Indeed, that takes the decision away from the library user (the programmer using our library, the consumer). Instead, return a `Result<T, E>` and let them decide. We only panic in a library if it’s a severe **internal** invariant violation or we literally can’t do anything (and ideally, document that it might panic in that case). 
* In application (binary) code, we control the whole program. We might choose to `panic!()` on certain errors if it simplifies things. Even then we should `panic!()` only when it’s truly unrecoverable or we are OK with the program terminating.




**Alice:** What about using a lot of `.unwrap()` in my code? Is that considered bad?

**Bob:** Frequent use of `.unwrap()` is usually a code smell (except in code examples or tests). Each `.unwrap()` is a potential `panic!()` point. It’s fine if we are 100% sure it can’t fail (like we just checked a condition that guarantees it, or it’s in a context where a crash is acceptable). But if an error is possible and we `.unwrap()`, we are basically ignoring the error and we crash instead of handling it. Often it’s better to handle the error or to propagate it. If we find ourself writing many `.unwrap()`s, we should think about using `?` to propagate or handle errors more gracefully.






To sum up:
* Use `panic!()` (or `.unwrap()`, etc.) for **bugs** and **unexpected** **conditions**. Things that should never happen if **our code** is correct.
* Use `Result<T, E>` for errors that we *expect could happen* in normal usage (and thus might want to recover from).








**Alice:** That’s clear. The Rust Book even has a section title "To panic! or Not to panic!" I think.

**Bob:** Yes, and the summary is pretty much what we discussed. One line [from it](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html): *"when failure is expected, it’s more appropriate to return a `Result<T, E>` than to make a `panic!()` call"*. Only `panic!()` when failure indicates a bug or something so bad that there’s no point in continuing.


One more tip: if we do `panic!()`, let's do it with a helpful message. For example, if a function shouldn’t get a negative number, let's code: 

```rust
panic!("Negative value provided: {}", value);
```

This beats a cryptic panic or (worse) a silent misbehavior. It makes debugging easier by clearly pointing out what went wrong.

And of course, remember that panicking will [unwind the stack](https://doc.rust-lang.org/nomicon/unwinding.html) by default, which cleans up but takes some overhead. In performance-critical or embedded scenarios, sometimes Rust programs are configured to abort immediately on panic!() (no unwind). Remember what we said earlier. If needed, in `Cargo.toml` add the following section:

```toml
[profile.release]
panic = "abort"
```

But that’s an advanced detail. The key point is: `panic!()` = crash. Use with care.





### Summary – Using (or Avoiding) `panic!()`

{: .new-title }
> Summary – Using (or Avoiding) `panic!()
>
* **Expected errors -> `Result<T, E>`, Unexpected errors -> `panic!()`:** If an error condition is something we can anticipate and might want to handle (file not found, invalid input, network timeout), **do not panic**. Use `Result<T, E>` and propagate or handle it. If something is truly unexpected or a bug in **our code** (index out of bounds, violated invariant), a `panic!("msg")` is appropriate to immediately stop the program.
* **Library vs Application code:** 
    * **Libraries** should prefer `Result<T, E>` for errors and avoid panicking, except for internal bugs, because panics in a library will crash the user’s application. 
    * **Applications** (especially very small ones) might use `panic!()`, `.unwrap()`, `.expect()` in places where it’s acceptable for the program to crash (or during development to catch bugs). But even here I'm so no convinced. Indeed we should investigate bugs with a Debugger. For the rest, you will understand my point of view reading the section "Errors from Experimentation to Production". 
* **Use meaningful panic messages:** If we use `panic!()` or `.expect() `, provide context. E.g., `panic!("Negative value provided: {}", value)` is better than a blank panic. This helps debugging by indicating why the panic happened.
* **Minimize `.unwrap()` in code:** Every `.unwrap()` is a potential crash. We use it only when we're sure there's no error (or in test code). Prefer to handle or propagate errors instead. Replacing `.unwrap()` with `?` or proper error handling will make our code more robust.
* **Examples of when to panic:**
    * Out-of-range indexing (bug in **our code**) -> standard library panics (cannot recover safely).
    * Asserting a condition in code (`assert!()` macro) -> panics if the condition is false, useful in tests or to validate internal invariants.
    * Contract violations -> e.g., our function got an invalid argument that should have been prevented by earlier checks. We panic to signal programmer error, after possibly using Rust’s type system to avoid such cases where possible.









### Exercises – Panic vs Result

1. **Spot the Panic:** Take a piece of code (perhaps one of our previous exercise solutions) where we used `.unwrap()` or `.expect()`. What would happen if that line did encounter an error? Is it truly a scenario that should crash the program? Modify the code to handle the error with a `Result<T, E>` if appropriate. If you decide to keep the `.unwrap()`, justify why it is OK (for example, if it’s in a test or if logic guarantees the `Result<T, E>` is `Ok()`).

2. **Design a Robust Function:** Imagine you’re writing a library function `fn send_email(address: &str, body: &str) -> Result<(), SendError>`. 
* Come up with two or three different reasons it might fail (e.g., invalid address format, network outage). 
* For each, decide if it should return an error (`Result::Err`) or panic. Explain your reasoning. Hint: as a library function, it should likely return errors for anything that can go wrong due to external factors or bad input, rather than panicking. Panics should be reserved for something like an invariant violation **inside** the library.

3. **Deliberate Panic:** Write a small program that deliberately panics (for example, by indexing an array out of bounds or using `panic!()` directly with a message). Run it to see what the panic message and backtrace look like. Enable backtrace by running the program with `RUST_BACKTRACE=1` environment variable (under WIN11 you can use `$env:RUST_BACKTRACE=1; cargo run -p u_are_errors --example my_panic_code` in a terminal).





















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


## Custom Error Types and Error Handling in Larger Programs


**Alice:** So far we’ve talked about using the built-in errors (like `std::io::Error` or parsing errors). What about bigger programs where different parts can error in different ways? How should I think about it and then design my own error data types, if necessary?

**Bob:** For me, the key point is that we need to ensure that our custom error type behaves as much like `std::error::Error` as possible. If we can do that, our error can be handled like any standard error, which is pretty cool. As you will see, luckily the `std::error::Error` trait is here to help.  

This said, as our Rust program grows, we might call many operations that can fail, potentially with different error types. We have a few choices:
* Use one catch-all error type everywhere to simplify things. Think to our good old `Box<dyn std::error::Error>` or a crate like `anyhow` in applications. 
* Define our own **custom error type** (usually an `enum` ) that implements `std::error::Error` where we enumerate all possible errors in our context and which is able to convert other errors into our custom type.

Defining a custom error type is common in libraries because, once this is done, the library returns one consistent error type that the users can handle, instead of many disparate types.



**Alice:** How would a custom error type looks like?

**Bob:** As I said, usually it is an `enum`, you know, the Rust's jewel of the crown... 

<div align="center">
<img src="./assets/img40.webp" alt="" width="225" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>


For example, imagine a program that needs to load a configuration file which is in JSON format. Things that could go wrong: file I/O could fail, or JSON parsing could fail. These are two different error types from the std lib or the crate (IO errors and parse errors). We might create an `enum` type definition like this:


```rust
// ex17.rs
use serde::Deserialize;
use std::fmt;
use std::fs::{read_to_string, write};
use std::io::ErrorKind;

#[derive(Debug)]
enum ConfigError {
    Io(std::io::Error),
    Parse(serde_json::Error),
}

// Implement Display for our error to satisfy Error trait.
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "I/O error: {e}"),
            ConfigError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

// Implement the standard Error trait for integration with other error tooling.
// To implement the std::error::Error trait for ConfigError, ConfigError must implement Debug and Display
impl std::error::Error for ConfigError {}
```

* `ConfigError` is our custom error type
* It is an enum (a sum type). A value of this type is exactly one of its variants at a time. Here it has two possible variants:
    * `Io(...)` — a variant that carries one payload of type `std::io::Error`
    * `Parse(...)` — a variant that carries one payload of type `serde_json::Error`
* Keep in mind that [each enum variant is also a constructor of an instance of the enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#:~:text=each%20enum%20variant%20that%20we%20define%20also%20becomes%20a%20function%20that%20constructs%20an%20instance%20of%20the%20enum).
    * Think about: `fn Io(e: std::io::Error) -> ConfigError{...}`

{: .warning-title}
> This is key
>
Each enum variant is also a constructor of an instance of the enum.


* Then in the code above we implement the `Display` trait for our data type `ConfigError`. 
    * This is mandatory. In VSCode, if we hover the word `Error` from `impl std::error::Error` we learn that 
        * to implement the `std::error::Error` trait for `ConfigError`, `ConfigError` must implement `Debug` and `Display`. 
        * `Debug` is easy. It is implemented automatically thanks to the directive `#[derive(Debug)]`. 
        * Now, regarding `Display`, for each variant of the `enum` we explain how to `write!()` it so that they can print nicely. 


{: .warning-title}
> This is key
>
To implement the `std::error::Error` trait for `ConfigError`, `ConfigError` must implement `Debug` and `Display`


* Finally comes the empty implementation of `Error` for `ConfigError`. It is empty because the trait only have default methods which is the case here. In other words, the line officially registers our data type as a standard error, without any additional customization.




{: .note-title }
> Side Note
>
> If you don't feel confident with traits you can read this [series of posts]({%link docs/06_programmation/rust/015_traits/traits_00.md%}).


* Next, when we write the function `load_config()` we make sure it returns `Result<Config, ConfigError>`. See below:

```rust
fn load_config(path: &str) -> Result<Config, ConfigError> {
    let data = read_to_string(path).map_err(ConfigError::Io)?;
    let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
    Ok(cfg)
}
```

Now, fasten your seat belt and stay with me because what follows is a bit rock ‘n’ roll... In any case, **it took me a while** to really realize what was happening. Indeed, inside `load_config()`, if something bad happen we convert the current error into `ConfigError` with the help of `.map_err()`. Here is how:

* If it fails, `std::fs::read_to_string` returns a `Result<String, std::io::Error>`
    * `.map_err(ConfigError::Io)` is then executed
    * However, since you remember (you confirm, you remember) that each enum variant of `ConfigError` is also an initializer of the enum, when `.map_err(ConfigError::Io)` is executed, it calls the function `ConfigError::Io(e: std::io::Error) -> ConfigError` which constructs and returns a `ConfigError`
    * The `ConfigError` (which have the trait `std::error::Error`) is presented in front of the `?` operator
    * The `?` operator bubbles up the `ConfigError` immediately since in our explanations we said that `std::fs::read_to_string` just failed
* The same mechanics is at work on the next line 


* The caller of `load_config()` only have to handle `ConfigError`. Below we show a part of the `load_or_init()` function. The idea is to focus on how this works from the caller point of view:

<!-- It can `match` on whether it is `Io()` or `Parse()` if it wants to distinguish.  -->

```rust
fn load_or_init(path: &str) -> Result<Config, ConfigError> {
    match load_config(path) {
        ...
        Err(ConfigError::Parse(e)) => {
            eprintln!("Invalid JSON in {path}: {e}");
            Err(ConfigError::Parse(e))
        }
        ...
    }
}
```

* This is a `match` on the value returned by `load_config()`
* If the pattern matches `Err(ConfigError::Parse(e))`, the `.json` in invalid
* The function bubbles up (`Err(...)`) the error to the caller (`main()` here)

Let's have a look at the `main()` function.

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
```

* Note that `main()` returns `Result<(), Box<dyn std::error::Error>>`
* This is cool because now we can use the `?` operator in the body of the `main()` at the end of certain lines
* Thanks to `Box<dyn std::error::Error>>`, it works even if the error type from ``write()`` and `load_or_init()` are different (they both implement the `std::error::Error` trait)


Expected output of the `ex17.rs` with ``bad_config.json``: 

```
Invalid JSON in bad_config.json: invalid type: string "not a number", expected u16 at line 1 column 44
Error: Parse(Error("invalid type: string \"not a number\", expected u16", line: 1, column: 44))
error: process didn't exit successfully: `target\debug\examples\ex17.exe` (exit code: 1)
```










Find below `ex17.rs` complete source code because **I hate** partial source code in blog posts that usually never works. 
* Feel free to copy/paste in [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024)
* In VSCode, set a breakpoint and take the time to go through the code line by line (F10). 

<div align="center">
<img src="./assets/img21.webp" alt="" width="560" loading="lazy"/><br/>
<span>Click the image to zoom in</span>
</div>


```rust
// ex17.rs
use serde::Deserialize;
use std::fmt;
use std::fs::{read_to_string, write};
use std::io::ErrorKind;

#[derive(Debug, Deserialize)]
struct Config {
    app_name: String,
    port: u16,
}

#[derive(Debug)]
enum ConfigError {
    Io(std::io::Error),
    Parse(serde_json::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "I/O error: {e}"),
            ConfigError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let data = read_to_string(path).map_err(ConfigError::Io)?;
    let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
    Ok(cfg)
}

fn load_or_init(path: &str) -> Result<Config, ConfigError> {
    match load_config(path) {
        Ok(cfg) => Ok(cfg),

        Err(ConfigError::Io(ref e)) if e.kind() == ErrorKind::NotFound => {
            let default = Config { app_name: "Demo".into(), port: 8086 };
            // Map the write error to ConfigError so `?` compiles.
            write(path, r#"{ "app_name": "Demo", "port": 8086 }"#).map_err(ConfigError::Io)?;
            eprintln!("{path} not found, created with default config");
            Ok(default)
        }

        Err(ConfigError::Io(e)) => {
            eprintln!("I/O error accessing {path}: {e}");
            Err(ConfigError::Io(e))
        }

        Err(ConfigError::Parse(e)) => {
            eprintln!("Invalid JSON in {path}: {e}");
            Err(ConfigError::Parse(e))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
```









**Alice:** Got it. So if I have a module—or more likely, a library—that performs some operations, I should define a custom error type in that module to represent everything that can go wrong. Then I can use `?` to convert sub-errors into my custom error and let them bubble up to `main()`. That way, `main()` only deals with my module’s error type. 

<!-- (or I convert it further to something else or to `Box<dyn Error>` at the final boundary). -->

**Bob:** Exactly. Let’s do a quick mini-example of propagating an error from a module to `main()`. Suppose we have a module `math_utils` with a function that can fail:


```rust
// ex19.rs
mod math_utils {
    // This module could be in a file math_utils.rs
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero { numerator: f64 },
        NegativeLogarithm { value: f64 },
    }

    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MathError::DivisionByZero { numerator } => write!(f, "Cannot divide {} by zero", numerator),
                MathError::NegativeLogarithm { value } => write!(f, "Logarithm of negative number ({})", value),
            }
        }
    }

    impl std::error::Error for MathError {}

    // Functions that return Result<_, MathError>
    pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == f64::EPSILON { Err(MathError::DivisionByZero { numerator: a }) } else { Ok(a / b) }
    }

    pub fn log10(x: f64) -> Result<f64, MathError> {
        if x < 0.0 { Err(MathError::NegativeLogarithm { value: x }) } else { Ok(x.log10()) }
    }
}

use math_utils::{divide, log10};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run() -> Result<()> {
    let my_log = log10(1024.0)?;
    println!("Log10 is {:.3}", my_log);

    let ratio = divide(10.0, 3.0)?;
    println!("Ratio is {:.3}", ratio);

    let bad_ratio = divide(5.0, 0.0)?;
    println!("This won't print because of error above ({})", bad_ratio);
    
    Ok(())
}

fn main() -> Result<()> {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(42);
    }
    Ok(())
}
```

Expected output:

```
Log10 is 3.010
Ratio is 3.333
Error: Cannot divide 5 by zero
error: process didn't exit successfully: `target\debug\examples\ex19.exe` (exit code: 42)
```

If we run this:
* `main()` calls the `run()` function
* There is no problem with `log10()`
* There is no problem with the first `divide()`
* The second `divide()` returns an `Err(MathError::DivisionByZero)` and the `?` bubbles up the error to the caller
* The `println!()` with `bad_ratio` is never executed 
* Back in `main()`, "Ooops, division by zero" is printed, thanks to `Display` implementation for `MathError`   
* Just for the fun, at this point, we return 42 and exit.



We could also catch the error in `main` with a `match` instead, and print something custom. But the point was to illustrate bubbling the error from a module up to `main()`. The key was to define `MathError` and to use it consistently. Each function in the module returns `MathError` on failure, and `run()` and `main()` can deal with `MathError`.






**Alice:** I think I have a much better understanding error handling in Rust now. Thanks.  

**Bob:** It’s a lot to take in at first, but once we get comfortable, we appreciate how Rust’s approach makes us think about errors up front. No more runtime surprises from unhandled exceptions. We decide what to do in each case. And keep in mind, for larger packages, there are crates like `thiserror` to reduce error boilerplate, and `anyhow` for quick-and-easy error handling in applications. Those can be handy, but the fundamentals of `Result<T, E>` and `?` we covered are the building blocks of it all.




### Summary – Custom Errors

{: .new-title }
> Summary – Custom Errors
>
* **Custom error types:** We can define our own error type (often an `enum` because our error can only have a value at a time) to represent errors in our application or library. This allows us to consolidate different error sources (IO, parsing, etc.) into one type and make our functions return that. It improves API clarity. Callers deal with one error type and can match on its variants.  
* **Implementing Error trait:** By implementing `std::error::Error` (which means implementing `fmt::Display` and having `#[derive(Debug)]`), our error type becomes interoperable with the standard ecosystem. It lets us use trait objects (`Box<dyn Error>`) if needed and makes our errors printable and convertible.  
* **Converting errors:** We use pattern matching or helper methods like `.map_err()` (or the `From` trait implementations) to convert underlying errors into our custom error variants. The `?` operator automatically convert errors if our custom error type implements `From` for the error thrown inside the function. This reduces a lot of manual code in propagating errors upward.  
    * Suppose we have an error `enum` `ConfigError { Io(io::Error), Parse(ParseError) }`. If a function reading a config file encounters an `io::Error`, we can do `.map_err(ConfigError::Io)?` to turn it into our error type and return it. The same for parse errors. Now the function returns `Result<Config, ConfigError>`, and the caller only has to handle `ConfigError`.  
* **Using `Box<dyn Error>`:** In application code, if we don’t want to define lots of error types, we can use `Box<dyn Error>` as a catch-all error type (since most errors in std lib implement `Error`). For example, `fn main() -> Result<(), Box<dyn std::error::Error>>` allows us to use `?` with any error that implements `Error` and just propagate it. This is convenient, but in library code you’d usually favor a concrete error type so that the API is self-documented.  





### Exercises – Custom Errors
1. **Define and Use a Custom Error:** Create an enum `MyError` with variants for two different error scenarios (for example, `MyError::EmptyInput` and `MyError::BadFormat(std::num::ParseIntError)`). Implement `std::fmt::Display` for `MyError` to provide human-readable messages. Then write a function `parse_nonempty_int(s: &str) -> Result<i32, MyError>` that returns an error if the input string is empty (`EmptyInput`) or if parsing to int fails (`BadFormat`). Use `?` and appropriate conversions (`map_err`) inside the function. Test it with various inputs (empty string, non-numeric, numeric).  

2. **Combine Two Error Types:** Suppose we have two functions `fn get_data() -> Result<String, io::Error>` and `fn parse_data(data: &str) -> Result<Data, ParseError>`. Write a new function `fn load_data() -> Result<Data, LoadError>` where `LoadError` is our custom enum that has variants for IO and Parse errors. In `load_data`, call `get_data()` and `parse_data()` using `?`, converting their errors into `LoadError` (we can implement `From<io::Error>` and `From<ParseError>` for `LoadError` or use `map_err`). Then try using `load_data()` in a `main` that prints different messages depending on which error occurred (hint: use `match e { LoadError::Io(e) => ..., LoadError::Parse(e) => ... }`).  

3. **Error Propagation in Modules:** Organize a small package with two modules: `network` and `database`. In `network`, create a function `fetch_data()` that might return a network-related error (we can simulate by just returning an `Err` variant like `NetworkError::Offline`). In `database`, create a function `save_data()` that might return a DB-related error (e.g., `DbError::ConnectionLost`). Then in `main`, write a function `run()` that calls `fetch_data` then `save_data`, propagating errors using `?`. Define a combined error type (enum with `Network(NetworkError), Database(DbError)`) to unify them for `run()`. Have `main` call `run()` and handle the unified error. This exercise will give we practice in designing error types and propagating across module boundaries.



























<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


## When and Why to Use `anyhow` and `thiserror` crates



**Alice:** You mentioned external crates like `anyhow` and `thiserror`. When should I reach for them?

**Bob:** Short version:  
- **`anyhow`** in **binaries** when we don’t need a public, fine-grained error type and just want easy error propagation with context.
- **`thiserror`** in **libraries** when we need ergonomic custom error types without writing all `impl` for `Display`, `Error`, and conversions.  






### anyhow - for binaries (mnemonic: A, B, C...**A**nyhow, **B**inaries)

[`anyhow`](https://docs.rs/anyhow/latest/anyhow/) provides a type called `anyhow::Error` which is a dynamic error type (like `Box<dyn Error>` but with some extras such as easy context via `.context(...)`). It’s great for applications where we just want to bubble errors up to `main()`, print a nice message with context, and exit. Here is an example:

```rust
// ex20.rs
use anyhow::{Context, Result};
use std::fs;

// Result alias = Result<T, anyhow::Error>
fn run() -> Result<()> {
    let data = fs::read_to_string("config.json").context("While reading config.json")?; // adds context if it fails
    let cfg: serde_json::Value = serde_json::from_str(&data).context("While parsing JSON")?;
    println!("Config loaded: {cfg}");
    Ok(())
}

fn main() -> Result<()> {
    run()
}
```

Expected output:

```
Error: While reading config.json

Caused by:
    Le fichier spécifié est introuvable. (os error 2)
```

* Notice how adding `.context(...)` makes error messages much more actionable if something fails.
* Otherwise, the key point to understand the previous code is to realize that `Result` is a type alias for `Result<T, anyhow::Error>`.




**Alice:** OK... But could you show me how we should modify one of the previous code, you know, the one where we were reading JSON config file.

**Bob:** Ah, yes, you're right. Let's rework `ex17.rs` to see the impact and benefices. Tadaa!:

```rust
// ex21.rs
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::{read_to_string, write};
use std::io::{self, ErrorKind};

#[derive(Debug, Deserialize)]
struct Config {
    app_name: String,
    port: u16,
}

fn load_config(path: &str) -> Result<Config> {
    let data = read_to_string(path).with_context(|| format!("failed to read config file: {path}"))?;
    let cfg = serde_json::from_str::<Config>(&data).with_context(|| format!("failed to parse JSON in: {path}"))?;
    Ok(cfg)
}

fn load_or_init(path: &str) -> Result<Config> {
    match load_config(path) {
        Ok(cfg) => Ok(cfg),
        Err(err) => {
            if let Some(ioe) = err.downcast_ref::<io::Error>() {
                if ioe.kind() == ErrorKind::NotFound {
                    let default = Config { app_name: "Demo".into(), port: 8086 };
                    let default_json = r#"{ "app_name": "Demo", "port": 8086 }"#;
                    write(path, default_json).with_context(|| format!("failed to write default config to {path}"))?;
                    eprintln!("{path} not found, created with default config");
                    return Ok(default);
                } else {
                    eprintln!("I/O error accessing {path}: {ioe}");
                    return Err(err);
                }
            }
            if let Some(parsee) = err.downcast_ref::<serde_json::Error>() {
                eprintln!("Invalid JSON in {path}: {parsee}");
                return Err(err);
            }
            Err(err)
        }
    }
}

fn main() -> Result<()> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#).context("writing good_config.json")?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#).context("writing bad_config.json")?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
```

Expected output of the `ex21.rs` with ``bad_config.json``: 

```
Invalid JSON in bad_config.json: invalid type: string "not a number", expected u16 at line 1 column 44
Error: failed to parse JSON in: bad_config.json

Caused by:
    invalid type: string "not a number", expected u16 at line 1 column 44
error: process didn't exit successfully: `target\debug\examples\ex21.exe` (exit code: 1)
```



In VSCode, open `ex21.rs` and `ex17.rs` side by side and compare both contents. If you do so and rearrange the source code layout, here is what you should see:

<div align="center">
<img src="./assets/img22.webp" alt="" width="900" loading="lazy"/><br/>
<span>ex17.rs on lhs, ex21.rs on rhs</span>
</div>

* `ex21.rs` is shorter but this is not the point. 
* `ConfigError` and its implementations has disappear because it is no longer needed. 
* Pay attention to `.with_context()` in `load_or_init()`. 
    * It is similar to `.context()` and the string literals. 
    * It takes a closure that returns a String. 
    * It is used here to dynamically `format!()` string with the value of a variable (`path`).
* Also note how the `.context(...)` in `main()` makes error messages much more actionable. 

This is typically what we need in binaries. Ok, let's read the code:

* In the initial version `ex17.rs` we had `fn load_config(path: &str) -> Result<Config, ConfigError> {...}`
* Now we have `fn load_or_init(path: &str) -> Result<Config> {...}` where `Result` is a type alias so that the signature should be read as `fn load_config(path: &str) -> std::result::Result<Config, anyhow::Error>`
* `anyhow` implement `From<E>` for all `E` that implement `std::error::Error + Send + Sync + 'static`
* If any error happen during `read_to_string()` then the `?` operator converts the error from `std::io::Error` to `anyhow::Error` (idem for `serde_json::Error` from `serde_json::from_str`) 




Now the tricky part is in `load_or_init()`:

* Its signature should be read as `fn load_or_init(path: &str) -> Result<Config, , anyhow::Error>`
* On error, we must downcast the `anyhow::Error` and check if it is an `io::Error`. If it is the case we check if it is an `ErrorKind::NotFound`...
* This is not really fun, I agree. 
* In fact I wanted to keep the logic of `load_or_init()` the same. Since it now receives  `Result<Config, , anyhow::Error>` and not a `Result<Config, ConfigError>` we have some work to do to retrieve the 3 kinds of error (not found, access, invalid json).
* Concerning `main()` except the signature there is no change.




For libraries, we should avoid `anyhow::Error` in our public API and prefer a concrete error type (possibly made with `thiserror`) so that downstream users can `match` on variants. Let's talk about it now.






### thiserror - for libraries

[`thiserror`](https://docs.rs/thiserror/latest/thiserror/) is a derive macro crate. Instead of manually implementing by hand `Display` and `Error` and writing `From` conversions (remember `Debug` comes with the directive `#[derive(Debug)]`), we can do something concise like:


```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),   // #[from] automatically implements From
    
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}
```

Now our `load_config()` function can just use the `?` operator and the `#[from]` converts sub-errors automatically. This is excellent for libraries, where we want to expose a stable and descriptive error type to users.


**Alice:** I really don't like code snippet. I like to see all the code. `ex17.rs` is a standalone binary. Could you show me, step by step, how you would split it as a library serving a binary?

**Bob:** Great idea. It is a good opportunity to see code refactoring in practice. Since you want to see all the code each time, I'll need some space but this should not be a problem here. 

First, let's review `ex17.rs` once again:

```rust
// ex17.rs
use serde::Deserialize;
use std::fmt;
use std::fs::{read_to_string, write};
use std::io::ErrorKind;

#[derive(Debug, Deserialize)]
struct Config {
    app_name: String,
    port: u16,
}

#[derive(Debug)]
enum ConfigError {
    Io(std::io::Error),
    Parse(serde_json::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "I/O error: {e}"),
            ConfigError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let data = read_to_string(path).map_err(ConfigError::Io)?;
    let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
    Ok(cfg)
}

fn load_or_init(path: &str) -> Result<Config, ConfigError> {
    match load_config(path) {
        Ok(cfg) => Ok(cfg),

        Err(ConfigError::Io(ref e)) if e.kind() == ErrorKind::NotFound => {
            let default = Config { app_name: "Demo".into(), port: 8086 };
            write(path, r#"{ "app_name": "Demo", "port": 8086 }"#).map_err(ConfigError::Io)?;
            eprintln!("{path} not found, created with default config");
            Ok(default)
        }

        Err(ConfigError::Io(e)) => {
            eprintln!("I/O error accessing {path}: {e}");
            Err(ConfigError::Io(e))
        }

        Err(ConfigError::Parse(e)) => {
            eprintln!("Invalid JSON in {path}: {e}");
            Err(ConfigError::Parse(e))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
```

Here is the content of the terminal

```
Invalid JSON in bad_config.json: invalid type: string "not a number", expected u16 at line 1 column 44
Error: Parse(Error("invalid type: string \"not a number\", expected u16", line: 1, column: 44))
error: process didn't exit successfully: `target\debug\examples\ex17.exe` (exit code: 1)
```




As you say, it is a standalone, all-included, kind of binary. So, as a first step, let's split it into a library and a binary. For demo purpose, we can do this with a single file. In `ex22.rs` (see below) we just define a module inside the source code. If needed, review what we did in `ex19.rs` (the code with `log10()`, do you remember?, September?).

<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/D5XmUnYW5Ks?si=pqdrPrKvEFD3phoV" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>

Here is the code after the first step of refactorization:

```rust
// ex22.rs
mod my_api {
    use serde::Deserialize;
    use std::fmt;
    use std::fs::{read_to_string, write};
    use std::io::ErrorKind;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub app_name: String,
        pub port: u16,
    }

    #[derive(Debug)]
    pub enum ConfigError {
        Io(std::io::Error),
        Parse(serde_json::Error),
    }

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ConfigError::Io(e) => write!(f, "I/O error: {e}"),
                ConfigError::Parse(e) => write!(f, "Parse error: {e}"),
            }
        }
    }

    impl std::error::Error for ConfigError {}

    fn load_config(path: &str) -> Result<Config, ConfigError> {
        let data = read_to_string(path).map_err(ConfigError::Io)?;
        let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
        Ok(cfg)
    }

    pub fn load_or_init(path: &str) -> Result<Config, ConfigError> {
        match load_config(path) {
            Ok(cfg) => Ok(cfg),

            Err(ConfigError::Io(ref e)) if e.kind() == ErrorKind::NotFound => {
                let default = Config { app_name: "Demo".into(), port: 8086 };
                write(path, r#"{ "app_name": "Demo", "port": 8086 }"#).map_err(ConfigError::Io)?;
                eprintln!("{path} not found, created with default config");
                Ok(default)
            }

            Err(ConfigError::Io(e)) => {
                eprintln!("I/O error accessing {path}: {e}");
                Err(ConfigError::Io(e))
            }

            Err(ConfigError::Parse(e)) => {
                eprintln!("Invalid JSON in {path}: {e}");
                Err(ConfigError::Parse(e))
            }
        }
    }
}

use my_api::load_or_init;
use std::fs::write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
```

Hopefully the output is exactly the same:

```
Invalid JSON in bad_config.json: invalid type: string "not a number", expected u16 at line 1 column 44
Error: Parse(Error("invalid type: string \"not a number\", expected u16", line: 1, column: 44))
error: process didn't exit successfully: `target\debug\examples\ex22.exe` (exit code: 1)
```





Now, concerning the refactoring we can observe: 

* We now have a `mod my_api` at the top of the code
* This line declares and brings the content of the namespace `my_api` into the current crate. 
* Since the content of the module `my_api` is in the crate root, the module `my_api` is its child and its symbols can be accessed with the `my_api::blablablabla` syntax. 
* The `use my_api::load_or_init;` statement is a "shortcut" that helps to write `load_or_init("bad_config.json")` rather than the namespace syntax `my_api::load_or_init("bad_config.json")`.



{: .note-title }
> Side Note
>
> If you don't feel 100% confident with modules, crates, files... You can [read this post]({%link docs/06_programmation/rust/013_no_more_mod_rs/no_more_mod_rs.md%})


* `ConfigError` is now public because it is part of `load_or_init()` which is public


In this first step of the refactoring the main idea was to split the code in 2: 
* `my_api` module on one end 
* and a consumer of the API on the other. 

Now that we have our library crate set up, let's explore how to make use of the `thiserror` crate.  So now, we refactor `ex22.rs` into `ex24.rs`. Here it is:

```rust
// ex24.rs
mod my_api {
    use serde::Deserialize;
    use std::fs::{read_to_string, write};
    use std::io::ErrorKind;
    use thiserror::Error;

    type Result<T> = std::result::Result<T, ConfigError>;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub app_name: String,
        pub port: u16,
    }

    #[derive(Debug, Error)]
    pub enum ConfigError {
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        #[error("JSON parse error: {0}")]
        Parse(#[from] serde_json::Error),
    }

    fn load_config(path: &str) -> Result<Config> {
        let data = read_to_string(path).map_err(ConfigError::Io)?;
        let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
        Ok(cfg)
    }

    pub fn load_or_init(path: &str) -> Result<Config> {
        match load_config(path) {
            Ok(cfg) => Ok(cfg),

            Err(ConfigError::Io(ref e)) if e.kind() == ErrorKind::NotFound => {
                let default = Config { app_name: "Demo".into(), port: 8086 };
                write(path, r#"{ "app_name": "Demo", "port": 8086 }"#)?;
                eprintln!("{path} not found, created with default config");
                Ok(default)
            }

            Err(ConfigError::Io(e)) => {
                eprintln!("I/O error accessing {path}: {e}");
                Err(ConfigError::Io(e))
            }

            Err(ConfigError::Parse(e)) => {
                eprintln!("Invalid JSON in {path}: {e}");
                Err(ConfigError::Parse(e))
            }
        }
    }
}

use my_api::load_or_init;
use std::fs::write;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
```

* The code of the client (`main()`) remains unchanged.
* Changes occurs in the API and the biggest one is in `ConfigError` `enum` definition.

```rust
    #[derive(Debug, Error)]
    pub enum ConfigError {
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        #[error("JSON parse error: {0}")]
        Parse(#[from] serde_json::Error),
    }
```

* The directive `#[error...` and `#[from...` make the macro generates concrete implementations at compile time, and then the `?` in `load_config()` uses those implementations via static conversions.
* This is why we no longer need the `impl fmt::Display for ConfigError{...}` nor the `impl Error for ConfigError {}`.  
* The signature of `load_config()` can be simplified 
* Idem for the signature of `load_or_init()`. In addition the `map_err()` can be removed.

At the end we have an API and a consumer. In the API, we delegate to `thiserror` the writing of the implementations. I hope your understand the refactoring process that bring us from `ex17.rs` to `ex24.rs` one step after the other. I hope you enjoyed to read complete code at each step.  







### Summary – `anyhow` & `thiserror`

{: .new-title }
> Summary – `anyhow` & `thiserror`
>
* **`anyhow`**: Binaries. Dynamic error type with great ergonomics and `.context(...)` for adding messages. Best for applications where we just want to bubble errors up and print them, not pattern-`match` on them.  
```rust
use anyhow::{Context, Result};
use std::fs;
fn run() -> Result<String> {
    let data = fs::read_to_string("Cargo.toml").context("while reading Cargo.toml")?; 
    Ok(data)
}
fn main() -> Result<()> {
    let data = run()?;
    println!("Config loaded: {}", data);
    Ok(())
}
```
* **`thiserror`**: Libraries. Derive-based crate to build clear, typed error enums with minimal boilerplate. Best for libraries and public APIs where the caller needs to inspect error kinds.  
```rust
use thiserror::Error;
#[derive(Debug, Error)]
enum ConfigError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
fn load(path: &str) -> Result<String, ConfigError> {
    Ok(std::fs::read_to_string(path)?) // auto-converts into ConfigError::Io
}
fn main() -> Result<(), ConfigError> {
    let content = load("Cargo.toml")?;
    println!("Loaded: {}", content);
    Ok(())
}
```
* **Don’t mix them blindly**: We can use both in the same package (e.g., library crates with `thiserror` exposed, binary crate using `anyhow` internally), but try to keep public APIs typed and internal app code ergonomic.







### Exercises – `anyhow` & `thiserror` 
1. Can you explain why in the API of `ex24.rs` we found `type Result<T> = std::result::Result<T, ConfigError>;` while in the client's code we have `type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;`  

1. **Refactor to `thiserror`:** Take our custom error enum from the previous exercise and replace the manual `Display`/`Error` implementations with a `#[derive(Error)]` and `#[error(...)]` attributes from `thiserror`. If we had conversions from `io::Error` or `serde_json::Error`, add `#[from]` to those variants and remove our manual `From` impls.  

1. **Add Context with `anyhow`:** Write a small binary that reads a file and parses JSON, returning `anyhow::Result<()>`. Add `.context(reading file)` and `.context(parsing JSON)` to the respective fallible operations. Run it with a missing file and with invalid JSON to see the difference in error messages with the added context.  

1. **Design Choice:** Given a package that has both a library crate (`my_lib`) and a binary crate (`my_cli`) in a Cargo workspace, decide how we would structure error handling across both. Hint: `my_lib` exposes typed errors with `thiserror`, while `my_cli` depends on `my_lib` and uses `anyhow` in `main` to convert `my_lib::Error` into `anyhow::Error` using `?` and print user-friendly messages.




















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->



#### Posts 
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/016_errors/errors_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/016_errors/errors_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/016_errors/errors_02.md%})
