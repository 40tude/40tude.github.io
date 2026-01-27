---
published: true
lang: en-US
layout: default
title: Rust Error Handling, Demystified - 04
description: A beginner-friendly conversation on Errors, Results, Options, and beyond.
parent: Rust
#math: mathjax
nav_order: 22
date               : 2025-09-20 18:00:00
last_modified_date : 2026-01-27 11:00:00
---



# Rust Error Handling, Demystified
{: .no_toc }

A beginner-friendly conversation on Errors, Results, Options, and beyond.
{: .lead }




<!-- <h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2> -->



### This is Episode 04
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
* [Episode 03]({%link docs/06_programmation/rust/016_errors/errors_03.md%})
* [Episode 04]({%link docs/06_programmation/rust/016_errors/errors_04.md%})
* [Episode 05]({%link docs/06_programmation/rust/016_errors/errors_05.md%})


## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Errors from Experimentation to Production

**Alice:** It was a lot... Again, many, many thanks because it really helps to organized my thoughts about errors management.

There is, may be, one last thing I would like to discuss with you. I know, I'm still a young Padawan, and most of my projects are just experiments I tinker with on weekends. Ok, ok, ok... But I'm wondering how errors are managed in more "serious" code. I mean, I would like to learn more so that I will not be "lost" while reading code from others on GitHub. More importantly, I would like to put in place the good practices, up front, so that I can transition happily to production.

<div align="center">
<img src="./assets/img24.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

**Bob:** *Help you in this quest, I can.* And since you already know almost everything you need to know, I propose we follow this path:

1. First, we’ll recap what we’d like to see — and actually live with — when it comes to error management. Kind of like a wish list, if you will. I don’t have much to add here, since you already have the answers.
2. Then we will put ourself in a situation were you start few experimental projects. It will be a good opportunity to write some code, check our knowledge and put in place good practices.
3. Finally you will transition your projects in production ready state. At least we will put in place what we need from the error management point of view.

Do you agree?

**Alice:** This would be perfect. Let's go.


### Key Concepts
{: .no_toc }

**Bob:** Have you ever heard about the [Gall’s law]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#galls-law)? No? It translates in words your intuition. Indeed you feel the Force but you also feel that, ideally, your sample code will evolve. The law says (read it with a strong voice like in the film The Ten Commandments): "A complex system that works is invariably found to have evolved from a simple system that worked..."

<div align="center">
<img src="./assets/img29.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

So, good news, you are right. You must start with an experimental code that works and which will evolve (may be) in a million dollar class of application.

I can also confirm you are right when you say that you want to put it in place, up front, an error management system that scales with your app.

Now, I have a question for you. Without entering in the technical details, what do *you* want from the error management standpoint?

**Alice:** Um... I would say...
* The sooner the better. I mean, get help from the rust type and build systems to detect most of errors at compile time. You know what I mean.
* The fewer the better. This is obvious. Ideally I don't want error in my code.
* I told you, I really like the `?` operator. It makes the code easy to read. It is my friend. I would like to keep it in the transition from prototype to production.
* I want to be able to prototype experimentation code quickly, while still applying the lessons we learned with the custom error type in production. `enum` and related features are powerful, but I’m not sure I want to bother with them in my experimental code.
* I also remember what we said. If I write a library it should return the errors to the consumer and let him decide. It should almost never `panic!()`.
* Library should expose one error data type in their API even if internally it use `anyhow` and different options. I'm not sure I'm very clear on this point...
* What else? An espresso? More seriously, I don’t have much to add, except that I’d like to avoid rewriting my code when transitioning to production.

<div align="center">
<img src="./assets/img25.webp" alt="" width="225" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

**Bob:** It's really good. You are definitively on the right track. Let's keep all this in mind and let's move to the experimentation phase.





### Experimentation
{: .no_toc }

{: .note-title }
> Side Note
>
> In the [workspace](https://github.com/40tude/err_for_blog_post), the source code discussed below are in the `01_experimentation/examples/` directory.


**Bob:** It is Saturday night. The house is silent, your young sister is out (you don't want to kow where nor with who). This is the best time to play with Rust. No?

<div align="center">
<img src="./assets/img26.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

Based on what you just learnt, can you write your version of "Hello, World!"?

**Alice:** "Hello, World!"... It is a very simple code. I would start with:

```rust
fn main() {
    println!("Hello, world!");
}
```

Then... Yes, I know what you want. Let's make sure I can use my friend `?` in `main()`. Since I don't know yet what kind of std lib and crate functions I will call, I make sure `main()` can handle and returns all of them. I don't really remember, but it was based on `Box`, `dyn`, blah blah blah...

**Bob:** It is not a problem. Go back and review `00_u_are_errors\examples\ex08.rs` in [Episode 01]({%link docs/06_programmation/rust/016_errors/errors_01.md%}) for example.

**Alice:** Thanks for the nudge. So I would write the code like this:

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(()) // we must return a Result whose value here is Ok(())
}
```

But then I can imagine that other functions in `main.rs` will need to return the same `Result`. So in order to simplify the writing of the functions signature I write:

```rust
// ex000.rs
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
```

**Bob:** Pretty cool. Now, I want you to *trust in me, just in me...*.

<div align="center">
<img src="./assets/img27.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

Let me rewrite your code like this:

```rust
// ex001.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
```

No big change.
1. Since we want to use the same code from experimentation to production it is smarter to keep `Error` and `Result<T>` type aliases on 2 type alias declarations. Doing so, even if in production, the `Error` type evolve to something different (e.g. a custom error type) the `Result` type will not be impacted (it will always refers to `Error`) and this is exactly what we want.
1. Do you see the `pub` word? Here it does not really matter because the code is monolithic. Tomorrow, if you want to make sure `Result<T>` (and `Error`) can be used elsewhere it is better to anticipate and to give them a public access modifier upfront.

By the way do you have any idea of what I did?

**Alice:** No. You split my line in two and you explained that later if the `Error` type becomes very complicated, this will have no impact on `Result<T>`



**Bob:** I just add what we call a level of [indirection]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#indirection) which, according to [David Wheeler](https://en.wikipedia.org/wiki/David_Wheeler_(computer_scientist)), is THE way to solve most of problems in computer science.

So, at this point, we agree to say that `ex001.rs` is by now your official code template. Ok? Ok, let's move on.

Do you know what BMI is?

**Alice:** Yes I do. My young sister is always talking about it. I read this a statistical value which is more valuable for population than for individuals. It indicates if the group is overweight or not. Basically you take a weight (in kg) and divide it by the square of the height (in meters). This give a result in number of kilograms per square meter. If the group is between 18.5 and 24.9 it is OK.

**Bob:** Using your code template write a prototype to calculate the BMI.

**Alice:** Here is what I have so far.

```rust
// ex100.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let my_bmi = bmi(70.0, 1.7)?;
    println!("BMI: {my_bmi:.2}");
    Ok(())
}

fn bmi(w: f64, h: f64) -> Result<f64> {
    if h.abs() < f64::EPSILON {
        return Err("Height cannot be 0.0".into());
    }
    Ok(w / (h * h))
}
```

While writing the code, the most difficult part was the line
```rust
return Err("Height cannot be 0.0".into());
```

I lost some time because initially I wanted to write

```rust
return Err("Height cannot be 0.0");
```

But this does'nt work. Indeed `bmi()` returns a `Result<f64>`, this means a `Result<f64, Box<dyn Error>>`. So I have to convert the `&'static str` into a `Box<dyn std::error::Error>` first. I hope that now on, I will remember the `.into()`.


**Bob:** Don't worry this will come with practice. Now, for a new experiment, I want you to write a function that receives a vector of integers written as strings and returns their sum as an `i32`.

**Alice:** If we look at it from the perspective of the `main()` function, is the code below  what you have in mind?

```rust
// ex200.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let numbers = vec!["10", "20", "89", "30"];

    let total = sum_strings(&numbers)?;
    println!("The total is: {total}");
    Ok(())
}
```
**Bob:** Yes, keep going.

**Alice:** My first idea for `sum_strings()` is the code below

```rust
fn sum_strings(values: &[&str]) -> Result<i32> {
    let mut sum = 0;
    for s in values {
        let current_val = s.parse::<i32>();
        sum += current_val.unwrap();
    }
    Ok(sum)
}
```






* It returns a `Result<32>` so that I can use `?` in `main()`
* `values: &[&str]` may look weird but no, it is not. In `main()` I pass the vector `numbers` by reference because I borrow it (I don't want to give it) to `sum_strings()`. Now in `main()`, if I press`CTRL+ALT`, I see the exact type of `numbers` (`Vec<&'static str>`). So `sum_strings()`'s parameter is a reference to an array (`&[...]`) of static strings (`&str`).
* Then, there is a `for` loop which traverses the vector `values`
* I remembered we used `.parse()` at the beginning of the section ["The `Result<T, E>` Type: Handling Recoverable Errors"]({%link docs/06_programmation/rust/016_errors/errors_01.md%}#the-resultt-e-type-handling-recoverable-errors)
* Pressing `CTRL+ALT`, I see `.parse::<i32>()` returns a `Result<i32, ParseIntError>`
* If `current_val` is Ok I add its value to the running `sum`, otherwise... With the help of `.unwrap()` the code `panic!()`
* At the end of the loop, `sum` is a valid number and I return it with `Ok(sum)`

The code work, but to tell the truth, I'm not really proud of the `.unwrap()` and I know I should avoid the raw loop.

<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/W2tWOdzgXHA?si=eg4uz51KzDt47q2o&amp;start=41" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>

**Bob:** Then?

**Alice:** Now, I have this version of `sum_strings()` without any raw loop

```rust
fn sum_strings(values: &[&str]) -> Result<i32> {
    let sum: i32 = values
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .sum();
    Ok(sum)
}
```

But I remember what we said about `.unwrap()`, and `.expect()`. Finally I have this version which prints a custom message on error. See below:

```rust
// ex200.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let numbers = vec!["10", "20", "oops", "30"];

    let total = sum_strings(&numbers)?;
    println!("The total is: {total}");
    Ok(())
}

fn sum_strings(values: &[&str]) -> Result<i32> {
    let sum: i32 = values
        .iter()
        .map(|s| s.parse::<i32>().expect(&format!("Failed to parse '{}' as integer", s)))
        .sum();
    Ok(sum)
}
```

Here is what I can see in the terminal when "oops" is in the initial vector.

```
thread 'main' panicked at 01_experimentation\examples\ex200.rs:19:59:
Failed to parse 'oops' as integer: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\examples\ex200.exe` (exit code: 101)
```


**Bob:** This is pretty cool for a young Padawan. Last but not least I would like you to use your template and write an application that print the names of the files in a directory. Easy? No?

**Alice:** Same test. Just to make sure... From the point of view of `main()` is it what you expect?

```rust
// ex300.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let files = list_files(".")?;
    println!("{files:#?}");
    Ok(())
}
```

**Bob:** Yes. Now, show me the `list_files()` function please.

**Alice:** Here is what I have so far (no raw loop):

```rust
fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}
```
* I looked around in the documentation and on the web how to list files in a directory with Rust.
* Then I met [`read_dir()`](https://doc.rust-lang.org/std/fs/fn.read_dir.html) which returns an `io::Result<ReadDir>`
* When OK it can be used as an iterator over the entries within the directory (there is an `impl Iterator for ReadDir`)
* If it is an iterator I can daisy chain multiple filters and keep the files of interest
* `.filter_map()`, `.filter()` and `.collect()` operate on an `Iterator<Item = DirEntry>` once the `Result` has been unwrapped by `?` right after `read_dir()`
* These iterator methods do not return a `Result`. They cannot fail in a way that would require error propagation.
* They simply transform the data from one form to another
* This is why there is no `?` at the end of the steps
    * the first `.filter_map()` silently drops entries that errored
    * the second `.filter()` ask the filesystem whether the entry is a file. If that check errors because it is a directory, it is treated as false and not kept in the list of files.
    * the last `filter_map()` only keeps filenames that are valid UTF-8 while the others are dropped
* The last step is `.collect()` which creates a vector with the filtered filenames
* Finally the function returns the vector to `main()` with `Ok(files)`


**Bob:** Did you notice how your template worked fine in 3 different experiments? I guess we can **keep it in our toolbox**.

Now in the last sample code, rather than panicking on error after the call to `read_dir()`, could you avoid the `?` and return a custom message to `main()` explaining what's happen?

**Alice:** Ok... I start by removing the `?` then... I don't know!

**Bob:** Do you remember the section "`Option<T>` vs. `Result<T, E>`: Choosing the Right Type" in [Episode 02]({%link docs/06_programmation/rust/016_errors/errors_02.md%})? We were discussing about the `Option<T>` and the fact we were loosing the reason why the failure happened. I told you we can return an `Option<T>` but log the reason of failure. To do so I used `.map_err()`. Do you remember? Review `ex16.rs` then come back here.

**Alice:** I get it. Here is my new version of the code

```rust
// ex301.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let files = list_files("")?;
    println!("{files:#?}");
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path) // no `?` here
        .map_err(|_| "❗Error while reading dir.")? // but `?` is here. On error, return a static string
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}
```

* You are right. The key is to remember `.map_err()` and how it works. Let me rephrase my understanding... At the exit of `read_dir()`
    * If the `Result` is an `Ok(value)`, `.map_err()` does nothing. The `?` operator evaluates to `value` and the execution continues
    * If the `Result` is `Err(e)`, `.map_err()` applies the closure to `e` and returns `Err(closure(e))`
        * Here the closure ignores the actual `io::Error` (`|_|` discards it) and replaces it with a static string slice `"Error while reading dir."`
        * The `?` operator immediately returns that error from the current function.




Now, let me repeat the details of the operations. Just to make sure...

* The return type of the `list_files()` function is `Result<Vec<String>, Box<dyn std::error::Error>>`
* So when the `Err(&str)` need to be bubbled up, Rust needs to find a way to transform the `&str` into a `Box<dyn std::error::Error>`
* The promotion from `&str` to `Box<dyn std::error::Error>` is possible because std lib includes `impl<'a> From<&str> for Box<dyn Error + 'a>`. I took the time to [read this page](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error%3E:~:text=impl%3C%27a%3E%20From%3C%26str%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E).
* This explains why we can return a bare "`Error while reading dir.`" and how it gets "promoted" into a proper `Box<dyn Error>`.

{: .warning-title}
> This is key
>
The promotion from `&str` to `Box<dyn std::error::Error>` works because std lib includes an implementation of the `From` trait which does exactly that. See `impl<'a> From<&str> for Box<dyn Error + 'a>`.

**Bob:** I'm truly impressed. Now, even if it is a little bit overkill because we are supposed to be in an experiment, if I ask you to return *also* the reason *why* the error occurs I guess it is a matter of seconds. No?

**Alice:** You're right. Now it is much easier. Here is the new version of the code

```rust
// ex302.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let files = list_files("")?;
    println!("{files:#?}");
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)
        .map_err(|why| format!("❗Error while reading dir. Reason = {why}"))?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}
```
* Above, `.map_err()` now returns a custom error as formatted `String`.
* The promotion from `String` to `Box<dyn std::error::Error>` works because std lib includes [`impl<'a> From<String> for Box<dyn Error + 'a>`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C&str%3E-for-Box%3Cdyn+Error%3E:~:str%3E%20for%20Box%3Cdyn%20Error%20+%20'a%3E&text=impl%3C%27a%3E%20From%3CString%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E).




**Bob:** *A Padawan no more, you are. Prove a Jedi Knight you have become...* Let's go back to the first experiment and show me how you would return an meaningful error message if the directory is empty.

**Alice:** Here is my code

```rust
// ex303.rs
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let files = list_files("./01_experimentation/empty")?;
    println!("{files:#?}");
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    if files.is_empty() {
        return Err("Cannot list empty folder.".into());
    }
    Ok(files)
}
```
* This time it's easier because I remember about `.into()`
* I keep the initial code but once the `files` vector is collected, I check if it is empty.
* If it is I return an ad hoc message.
* Otherwise, as before, we reach the end of the body of `list_files()`, the `files` vector is Ok and I return `Ok(files)`


**Bob:** We are still in the experimentation phase where we can take the time to learn, discover, crash and repair things. Can you tell me, in detail, why and how the `.into()` works? Take your time, read the documentation before to anser.

**Alice:** It turned out to be a real caving expedition, and it took me more time than I had anticipated. Sorry about that.

<div align="center">
<img src="./assets/img30.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

I focus on the lines below:

```rust
    if files.is_empty() {
        return Err("Cannot list empty folder.".into());
    }
```
The `.into()` works because std lib includes [`impl<'a> From<&str> for Box<dyn Error + 'a>`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error%3E:~:text=impl%3C%27a%3E%20From%3C%26str%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E) and here is why:
* When I write `"Cannot list empty folder.".into();`
* It starts as a `&'static str`
* The compiler knows that the expected type is `Box<dyn Error>`
* It founds `impl<'a> From<&str> for Box<dyn Error + 'a>` in the std lib
* But in Rust if we have `From<A> to B` then we get `Into<B> for A` for free
* Here this means `Into<Box<dyn Error> for &str` exists
* Then the `static &str` is automatically converted to `Box<dyn Error>`

The story has a happy ending: they got married and lived happily ever after.


{: .warning-title}
> This is key
>
In Rust if the trait `From<A> for B` exists, then we get the trait `Into<B> for A` for free.


<div align="center">
<img src="./assets/img28.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>






### Summary – Experimentation
{: .no_toc }

{: .new-title }
> Summary – Experimentation
>
* `main()` return any kind of error that implements the `Error` trait
* `?` can be used in `main()`
* In our functions we return custom messages (`.into()`, `.map_err()`...)
* Let's keep this code fragment in mind:
    ```rust
    pub type Error = Box<dyn std::error::Error>;
    pub type Result<T> = std::result::Result<T, Error>;
    fn main() -> Result<()> {
        let files = list_files("")?;
        println!("{files:#?}");
        Ok(())
    }
    fn list_files(path: &str) -> Result<Vec<String>> {
        let files: Vec<String> = std::fs::read_dir(path)
            .map_err(|why| format!("❗Error while reading dir. Reason = {why}"))?
            // REST OF THE CODE ;
        if files.is_empty() {
            return Err("Cannot list empty folder.".into());
        }
        Ok(files)
    }
    ```



**Bob:** It's showtime! Let's transition to production.















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->



#### Posts
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/016_errors/errors_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/016_errors/errors_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/016_errors/errors_02.md%})
* [Episode 03]({%link docs/06_programmation/rust/016_errors/errors_03.md%})
* [Episode 04]({%link docs/06_programmation/rust/016_errors/errors_04.md%})
* [Episode 05]({%link docs/06_programmation/rust/016_errors/errors_05.md%})


