---
published: true
lang: en-US
layout: default
title: "Option in Rust: 15 Examples from Beginner to Advanced - 00"
description: "Learn Rust's `Option<T>` through runnable Playground examples - progressive guide from `if let` to advanced combinators"
parent: "Rust"
date:               2025-11-29 01:00:00
last_modified_date: 2025-12-05 10:00:00
---

# `Option<T>` in Rust: 15 Examples from Beginner to Advanced
{: .no_toc }

A Code-First Guide with Runnable Examples
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>




## TL;DR
{: .no_toc }

* 3 ski slopes: 🟢 Beginner, 🔵 Intermediate, 🔴 Advanced
* Basic extraction: `if let`, `match`, `let...else`
* Safe unwrapping: `unwrap_or()`, `unwrap_or_else()`
* Chaining: `map()`, `and_then()`, `filter()`
* Advanced: `as_ref()`, `take()`, `flatten()`, combining Options



<!--

Example 01: `Option<T>` as a Return Value
```rust
fn get_selection(&self) -> Option<String> {
    Some("lorem ipsum".to_string())
    // None
}
```

Example 02: Conditional Pattern Matching
```rust
if let Some(my_txt) = my_editor.get_selection() {
    println!("Selection: {my_txt}");
} else {
    println!("No text selected");
}
```
If the pattern `Some(my_txt)` successfully matches the `Option<T>` returned by `my_editor.get_selection()`, then **bind its contents** to `my_txt` and run the first block; otherwise, run the `else` block."


Example 03: `match` Expression with Early Return
```rust
let my_file_name = match &editor.path_to_file {
    Some(path) => path.file_name(),
    None => return,
};
```
"Match on `&editor.path_to_file`. If it contains a value, **bind a reference to that value** to `path`, then call the method `file_name()` on `path` and bind the result to `my_file_name`. If `None`, return early."


Example 04: "Modern" Early Return
```rust
let Some(my_path) = &editor.path_to_file else {
    return;
};
```
"Let the pattern `Some(my_path)` match on `&editor.path_to_file`. If it doesn't match (i.e., it's `None`), execute the `else` block which returns early."



Example 05: Early Return Propagation
```rust
let text = s?;
chars.next()?;
```
"If this `Option<T>` is `None`, immediately return `None` from the function. Otherwise, unwrap the `Some(v)` value and continue."




Example 06: Providing Defaults
```rust
Option<T>.unwrap_or(v) or Option<T>.unwrap_or(my_function())
Option<T>.unwrap_or_else(||my_closure())
```

* `Option<T>.unwrap_or(v)` = "Give me the value inside the `Option<T>` **OR** if the option is `None`, give me the value `v` (where `v` can be the result of a function)."
* `Option<T>.unwrap_or_else(||my_closure())` = "Give me the value inside the `Option<T>` **OR** if the `Option<T>` is `None`, call the closure and give me its returned value."



Example 07: Transforming Values Inside `Option<T>`

```rust
let result = name.map(|n| n.trim().to_string())
```

"If the `Option<T>` is `Some(v)`, apply the transformation to the inner value and wrap the result in `Some(w)`. If `None`, skip the transformation and return `None`."







Example 08: Chaining `Option<T>`

```rust
let chain_result = Some("49")
    .and_then(|s| parse_positive(s)) // Some(49)
    .and_then(|n| if n < 50 { Some(n * 2) } else { None });
```
"If the `Option<T>` is `Some(v)`, apply the transformation that returns an `Option<U>` and **flatten the result**. If `None`, skip and return `None`."





Example 09: Pattern Matching with Guards
```rust
match age {
    Some(a) if a < 18 => "Minor",
    ...
```
"If the `Option<T>` is `Some(v)` **AND** if the extracted value (`v`) satisfies this predicate then execute the code after the `=>`."









Example 10: Borrowing Instead of Moving
```rust
let len = path.as_ref().map(|p| p.as_os_str().len());
path.as_mut().map(|p| p.push("documents"));
```
"`as_ref()` converts `Option<T>` to `Option<&T>`, so that we can peek inside without consuming. `as_mut()` gives `Option<&mut T>` for peek and poke. Both leave the original `Option<T>` intact."



Example 11: Extracting Value and Leaving `None`
```rust
if let Some(_f) = self.file.take() {...}
```
"Give me the value inside `Some(v)` + replace the `Option<T>` with `None` + and return the value as `Option<T>`"





Example 12: Conditional Mapping
```rust
let result = name.map(|n| n.trim()).filter(|n| !n.is_empty()).map(|n| n.to_uppercase());
```
"If `Option<T>` is `Some(v)` and the condition is true, keep it as `Some(v)`. Otherwise, return `None`. It's like `map()` but it can remove values."





Example 13: Working with Collections of Options
```rust
let out: Vec<i32> = in
    .iter()
    .filter_map(|&s| parse_number(s))
    .collect();

```
"`.flatten()` converts `Vec<Option<T>>` to `Vec<T>` by discarding `None` while `.filter_map(|x| optional_transform(x))` combines `.map()` and `.flatten()` in one step"



Example 14: Combining Multiple Options
```rust
fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    Some(a? + b?)
}
```
"`Some(a? + b?)` offers a concise early-return logic to `Options<T>` 2 or more option. If all `Options<T>` are `Some(v)` the processing takes place, otherwise, if any is `None`, early reply `None`."




Example 15: Converting `Option<&T>` to `Option<T>`
```rust

```
"`.copied()` duplicates the value inside `Option<&T>` to produce `Option<T>` (requires the `Copy` trait). `.cloned()` does the same but uses the `Clone` trait instead - works for non-Copy types like `String`."

-->










<!-- ###################################################################### -->
<!-- ###################################################################### -->
### This is Episode 00
{: .no_toc }

#### The Posts Of The Saga
{: .no_toc }
* 🟢 [Episode 00]({%link docs/06_programmation/rust/020_some/some_00.md%}): Intro + Beginner Examples
* 🔵 [Episode 01]({%link docs/06_programmation/rust/020_some/some_01.md%}): Intermediate Examples
* 🔴 [Episode 02]({%link docs/06_programmation/rust/020_some/some_02.md%}): Advanced Examples + Advises + Cheat Sheet...



<div align="center">
<img src="./assets/img00.webp" alt="" width="300" loading="lazy"/><br/>
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
## Introduction

Personally, while still learning Rust I always struggle with the `Some(v)` syntax. I can **read** and understand most of the code I get, but the next day when I need to write a line of code to handle an `Option<T>` I don't feel comfortable. I've been dealing with this problem since the beginning and I can't seem to get rid of it. What's worse, I can understand code written by others or code generated by Claude/ChatGPT, but I can't write it myself with ease.

In short, this post is a kind of therapy during which I will try to heal myself 😁.









<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## How to Use This Guide?

I want to try something new here so this article uses a sort of **inverted pedagogy**: code first, explanation second and cherry on the cake... You don't have to read it all all at once since the post is split in 3 different levels:

- 🟢 **Beginner**: Basic examples (`if let`, `match`)
- 🔵 **Intermediate**: Combinators and chaining (`map`, `and_then`, `?`)
- 🔴 **Advanced**: Ownership tricks (`as_ref`, `take`, `flatten`)

Each Example section is organized as follow:

1. **Real-world context:** Few words about the context before we look at the code
1. **Runnable Example:**
    1. Copy the code
    1. Paste it into [Rust Playground](https://play.rust-lang.org/)
    1. Press Ctrl+E (or click "Run")
    1. Experiment by modifying the code. Break everything, spend some time in the Playground. Play!
1. **Read it Aloud:** This section is **MUCH MORE IMPORTANT** than you may think in first approximation. Indeed, I'm convinced that too often, in programming (like in Maths) we look at the code (at the formula) but we don't **read** the code (nor the formula). However the code tells us a story and this is this story that I try to tell in the section.
1. **Comments:** my comments about the code, my understanding or some other stuff I may have in mind.
1. **Key Points:** what to keep in mind.
1. **Find More Examples:** don't trust me. Check by yourself. Search in the code of largely used crates, not in ad hoc projects nor in toy projects (see below how to)






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Learning from existing code
The idea is to learn from others and to "see" how do they do. I propose to clone the projects below. Copy the line below. Yes, you can copy more than one line at a time in Powershell.

```powershell
cd $env:TMP
git clone https://github.com/BurntSushi/ripgrep
git clone https://github.com/microsoft/edit
git clone https://github.com/sharkdp/hexyl
git clone https://github.com/ajeetdsouza/zoxide
git clone https://github.com/aovestdipaperino/turbo-vision-4-rust
```
Other crates include but are not limited to : [`bat`](https://github.com/sharkdp/bat), [`fd`](https://github.com/sharkdp/fd), [`tokei`](https://github.com/XAMPPRocky/tokei), [`hyperfine`](https://github.com/sharkdp/hyperfine)... Stay away from projects that are either too large or too complex. Don't worry, we will not build or install these projects (for example edit require the nightly build Rust compiler), we will only search in their code base.

Later in this post, at the end of each Example, I will share the regular expression to use to find the lines of code corresponding to the example. Once you have this regular expression in hands (let's say `Some\(.+\)$`) you can either:

**In VSCode:**
* Open the project of interest
* `CTRL+SHIFT+F`, enable regex (`ALT+R`), type: `Some\(.+\)$`

**In Powershell:** copy'n paste the line below in a terminal.

```Powershell
Get-ChildItem -Path "$env:TMP/hexyl/src" -Filter *.rs -Recurse |
    ForEach-Object {
        # Read file content with line numbers
        Select-String -Path $_.FullName -Pattern 'Some\(.+\)$' -SimpleMatch:$false |
            ForEach-Object {
                # Output file path and line number
                [PSCustomObject]@{
                    File      = $_.Path
                    LineNumber = $_.LineNumber
                    LineText   = $_.Line.Trim()
                }
            }
    }
```

Above we look into the codebase of `hexyl`. This work the same way for `zoxyde/src` or `turbo-vision-4-rust/src`.

In the case of:
* `edit` use the path: `$env:TMP/edit/crates/edit/src`
* `ripgrep` use the path: `$env:TMP/ripgrep/crates`

Based on what I saw, it seems `ripgrep` is the project that cover most if not all the examples of this post. I strongly recommend you open it now in you favorite IDE. This said... Let's dive in!

<div align="center">
<img src="./assets/img01.webp" alt="" width="600" loading="lazy"/><br/>
</div>










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 🟢 - Example 01 - `Option<T>` as a Return Value

### Real-world context
{: .no_toc }

I want to start with this use case because, for me, this is, by far, the easiest. Indeed, we can easily explain in plain English that a function might search for a file and, if it can’t find it, simply returns "nothing". If it succeeds, it returns something—like the first line of the file. Using `Option<T>` makes sense for any function that might not succeed (without throwing an error or crashing) but also doesn’t always have a meaningful value to return. This pattern is common for operations like searching, parsing, or handling optional configuration. On the other hand it is not too complicated to imagine a `struct` where some of its fields may, at one point, contain nothing. Think about an editor with no file loaded (see Example 02 and Code snippet 04)

Easy to explain, easy to translate. The easiest, I told you.

### Runnable Example
{: .no_toc }

Copy and paste in [Rust Playground](https://play.rust-lang.org/)

```rust
struct Editor {
}

impl Editor {
    fn get_selection(&self) -> Option<String> {
        // Simulate selection: uncomment one to test both cases
        Some("lorem ipsum".to_string())
        // None
    }
}

fn main() {
    let my_editor = Editor {};

    let selection = my_editor.get_selection();
    if selection.is_some(){
        println!("Selection: {}", selection.unwrap());
    }else{
        println!("No text selected");
    }
}
```

### Read it Aloud
{: .no_toc }

In `main()`, the code says : "`get_selection()` returns an `Option<String>` which contains the selected text as a `String` or `None`. The `if selection.is_some()` checks if the return value contains something and if so, executes the first block. Otherwise, it executes the else branch."


### Comments
{: .no_toc }
* It is important to realize that `get_selection()` returns a type `Option<String>` **NOT** a type `String`
* In the playground, replace `Some("lorem ipsum".to_string())` with `Some("lorem ipsum".into())`. Does it work? Why?
* Do you agree on the fact that `selection.is_some()` does **NOT** extract the value from the `Option<T>` but just check if there is a value in the `Option<T>`?
* Take your time and **read** the [documentation](https://doc.rust-lang.org/std/option/enum.Option.html).
* Is it clear that once we checked the `Option<T>` contains something then `unwrap()` extract this thing?
* Duplicate the `println!("Selection: {}", selection.unwrap());`at the very end of `main()`. Does it works? Why?


### Key Points
{: .no_toc }

1. **Pattern**: The function returns an `Option<T>` (here `Option<String>`). It can be `None` or `Some(v)`.
2. **When to use**: When we need to express the fact that a function or a method can return nothing or something.
3. **Usage**: When a struct, an application, a function has an optional parameter or return value, `Option<T>` should be used.


### Find More Examples
{: .no_toc }

* Regular expression to use either in VSCode ou Powershell: `Some\(.+\)$`
* Make the test. Now! Open `edit` project in VSCode and search for `Some\(.+\)$` (`CTRL+SHIFT+F` then `Alt+R`).

<div align="center">
<img src="./assets/img02.webp" alt="" width="900" loading="lazy"/><br/>
</div>












<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 🟢 - Example 02 - Conditional Pattern Matching - `if let Some(v) = Option<T>`

### Real-world context
{: .no_toc }

Updating configuration, processing optional user input, conditional initialization.

### Runnable Examples
{: .no_toc }




**Code snippet 00**

Copy and paste in [Rust Playground](https://play.rust-lang.org/). In the first three versions of the earlier code, within `main()`, we review several alternative approaches before using the `if let Some(...) = Option<T>` pattern.


```rust
struct Editor {
}

impl Editor {
    fn get_selection(&self) -> Option<String> {
        // Simulate selection: uncomment one to test both cases
        Some("lorem ipsum".into())
        // None
    }
}

fn main() {
    let my_editor = Editor {};

    let msg = my_editor.get_selection();
    match msg {
        Some(my_txt) => println!("Selection: {my_txt}"),
        None => println!("No text selected")
    };

}
```



**Code snippet 01**

```rust
struct Editor {
}

impl Editor {
    fn get_selection(&self) -> Option<String> {
        // Simulate selection: uncomment one to test both cases
        Some("lorem ipsum".into())
        // None
    }
}

fn main() {
    let my_editor = Editor {};

    match my_editor.get_selection() {
        Some(my_txt) => println!("Selection: {my_txt}"),
        None => println!("No text selected")
    }
}
```



**Code snippet 02**


```rust
struct Editor {
}

impl Editor {
    fn get_selection(&self) -> Option<String> {
        // Simulate selection: uncomment one to test both cases
        Some("lorem ipsum".into())
        // None
    }
}

fn main() {
    let my_editor = Editor {};

    let msg = match my_editor.get_selection() {
        Some(my_txt) => format!("Selection: {my_txt}"),
        None => format!("No text selected")
    };
    println!("{msg}");
}
```






**Code snippet 03**

Copy and paste in [Rust Playground](https://play.rust-lang.org/). In this version of the previous code we use `if let Some()` pattern.

```rust
struct Editor {
}

impl Editor {
    fn get_selection(&self) -> Option<String> {
        // Simulate selection: uncomment one to test both cases
        Some("lorem ipsum".into())
        // None
    }
}

fn main() {
    let my_editor = Editor {};

    // The `if let Some(...) = Option<T>` conditional pattern matching
    // => unwrap and use the selection only if Some()
    if let Some(my_txt) = my_editor.get_selection() {
        println!("Selection: {my_txt}");
    } else {
        println!("No text selected");
    }

}
```




**Code snippet 04**

Copy and paste in [Rust Playground](https://play.rust-lang.org/)

```rust
use std::path::PathBuf;

struct Editor {
    path_to_file: Option<PathBuf>,
}

impl Editor {
    fn set_path(&mut self, path: PathBuf) {
        self.path_to_file = Some(path);
    }
}

fn main() {
    let mut my_editor = Editor { path_to_file: None };

    // Uncomment one to test both cases
    let new_path = Some(PathBuf::from(r"tmp/my_file.txt"));
    // let new_path = None;

    if let Some(path) = new_path {
        my_editor.set_path(path);
    }

    println!("The file: {:?}", my_editor.path_to_file);
}
```



### Read it Aloud
{: .no_toc }

**Code snippet 00:** The code says: "In `main()`, `msg` is an `Option<String>` and we use a `match` expression to cover explicitly all the possible values returned by `my_editor.get_selection()`."

**Code snippet 01:** The code says: "In `main()`, on the return of `my_editor.get_selection()` we use a `match` expression to cover explicitly all the possible values."

**Code snippet 02:** Here the code tells us this story: "The result of the `match` expression is used to initialize the `msg` variable which receive a formatted string before to be printed."

**Code snippet 03:** Here the plot unfolds as follows: "If the pattern `Some(my_txt)` successfully matches the `Option<T>` returned by `my_editor.get_selection()`, then **bind its contents** to `my_txt` and run the first block; otherwise, run the `else` block."


**Code snippet 04:**
* Here is the context of the story
    * The `Editor` may have a path to a file (or not). This explains why the field `path_to_file` is of type `Option<PathBuf>`.
    * When created, `my_editor` does not point to any particular file (see the `path_to_file: None`).
    * Then a `new_path` variable is created. It is an `Option<PathBuf>` containing a path to a file.

That being said, the story goes like this: "If `new_path` contains a value, **bind it** to `path` and call the method `set_path()` with it as an argument. Otherwise, skip the block entirely."









### Comments
{: .no_toc }

* Code snippet 03. `if let Some(x) = expression` is **NOT** a boolean expression. This is a **conditional pattern matching**. Rust try to match the pattern `Some(...)` with the value on the right. If the match succeeds, the `if` block is executed; otherwise, it is ignored.
    * Personally, that's what trips me up every time because I can't help reading it as a Boolean expression.
* Code snippet 03. `if let Some(x) = expression` is **NOT** a boolean expression, see it is as a shortened version of a `match`.
* Code snippet 03. When we **read** a line like `if let Some(...) = Option<T>{...}` we should **say** "If the pattern `Some(...)` successfully matches the `Option<T>` then blablabla..."
* Code snippet 02. `match` is an expression. It evaluates to a single value. Look, there is a `;` at the end of the line `let msg = ...`
* Code snippet 02. Do you see the difference between the second and third code snippet? In the latter, `msg` receive the result of `format!` and then it is printed. Again `match` is an expression, **NOT** a statement.
* Code snippet 04. Uncomment the line `let new_path = None;` (comment the line above). Is the behavior of the code crystal clear?
* Code snippet 04. Add this line `println!("{}", new_path.is_some());` at the very end of the code. What happens? Why?.
* Each time one of our data type have a field which may contain something or nothing we should use an `Option<T>`. See Example 01.


### Key Points
{: .no_toc }

1. **Pattern**: `if let Some(x) = expression` is **NOT** a boolean expression, it is a **conditional pattern matching**
1. **Pattern**: Conditionally execute code only when `Option<T>` has a value
1. **Ownership**: As with the `unwrap()` (see Example 01), the value inside `Some()` is **moved** into `path` (not a reference)
<!-- 1. **Alternative**: Could use `if new_path.is_some() { ... }` but wouldn't extract the value cleanly -->

### Find More Examples
{: .no_toc }

Regular expression to use either in VSCode ou Powershell: `if let Some\(.+\) = `








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 🟢 - Example 03 - `match` Expression with Early Return

### Real-world context
{: .no_toc }

Anything that might fail and requires early exit: File operations, network requests, database queries...

### Runnable Example
{: .no_toc }

Copy and paste in [Rust Playground](https://play.rust-lang.org/)


```rust
use std::path::PathBuf;

struct Editor {
    path_to_file: Option<PathBuf>,
}

fn save_file(editor: &Editor) {
    let my_file_name = match &editor.path_to_file {
        Some(path) => path.file_name(),
        None => return,
    };

    if let Some(name) = my_file_name {
        println!("Saving file: {:?}", name);
    }
}

fn main() {
    let editor = Editor {
        // Uncomment one to test both cases
        path_to_file: Some(PathBuf::from(r"tmp/my_file.txt")),
        // path_to_file: None,
    };
    save_file(&editor);
}
```


### Read it Aloud
{: .no_toc }

* In `save_file()` the code says: "Match on `&editor.path_to_file`. If it contains a value, **bind a reference to that value** to `path`, then call the method `file_name()` on `path` and bind the result to `my_file_name`. If `None`, return early."




### Comments
{: .no_toc }

* `save_file()` has a reference to the Editor as a parameter (borrow)
* In the Playground, remove the reference in front of `&editor.path_to_file`. What happens? Why?
* It is important to understand that we don't directly “bind” the file name, we bind the result of the extraction, which is itself an `Option<T>` (because a path might not have a file name, for example `/` or `.` ).
* Ideally we should write `save_file()` as below:

    ```rust
    fn save_file(editor: &Editor) {
        // Extract file name from path, converting OsStr to String (Option<String>)
        let file_name = match &editor.path_to_file {
            Some(path) => path.file_name().map(|s| s.to_string_lossy().to_string()),
            None => return,
        };

        if let Some(name) = file_name {
            println!("Saving file: {}", name); // No :?
        }
    }
    ```
* At the end of `main()` add the 2 lines below. What do need to do to compile. Why?
    ```rust
    editor.path_to_file = Some(PathBuf::from(r"tmp/my_file2.txt"));
    save_file(&editor);
    ```


<div align="center">
<img src="./assets/img04.webp" alt="" width="600" loading="lazy"/><br/>
</div>



### Key Points
{: .no_toc }

1. **Pattern**: `match` with early return avoids deep nesting
2. **When to use**: When `None` means "abort this operation"
3. **Modern alternative**: See next example with `let...else`

### Find More Examples
{: .no_toc }

Regular expression to use either in VSCode ou Powershell: `match .+ \{\s*Some\(.+\) => .+,\s*None => return`












<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 🟢 - Example 04 - "Modern" Early Return - `let...else`

### Real-world context
{: .no_toc }

Same as match early return, but more concise (modern Rust style).

### Runnable Example
{: .no_toc }

Copy and paste in [Rust Playground](https://play.rust-lang.org/)

```rust
use std::path::PathBuf;

struct Editor {
    path_to_file: Option<PathBuf>,
}

fn save_file(editor: &Editor) {
    let Some(my_path) = &editor.path_to_file else {
        eprintln!("No path to file available");
        return;
    };

    if let Some(name) = my_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
    {
        println!("Saving file: {}", name);
    }
}

fn main() {
    let mut editor = Editor {
        // Uncomment one to test both cases
        // path_to_file: Some(PathBuf::from(r"tmp/my_file.txt")),
        path_to_file: None,
    };
    save_file(&editor);

    editor.path_to_file = Some(PathBuf::from(r"tmp/my_file2.txt"));
    save_file(&editor);
}
```

### Read it Aloud
{: .no_toc }

* At the begining of `save_file()` the code says: "Let the pattern `Some(my_path)` match on `&editor.path_to_file`. If it doesn't match (i.e., it's `None`), execute the `else` block which returns early."




### Comments
{: .no_toc }
* Compare the end of `save_file()` in Example 03 with Example 04. In the latter we extract file name from path, converting OsStr to String. It was mentioned in the comments of Example 03.



### Key Points
{: .no_toc }

1. **Pattern**: `let Some(var) = expr else { ... }` replaces match with early return
2. **Readability**: More concise than match when you only care about the `None` case
3. **Requirement**: The else block must diverge (return, break, continue, panic)
4. **Modern**: Introduced in Rust 1.65 (2022) - idiomatic for new code

### Find More Examples
{: .no_toc }

Regular expression to use either in VSCode ou Powershell: `let Some\(.+\) = .+ else`. Try it with `ripgrep` for example.






























<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Posts Of The Saga
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/020_some/some_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/020_some/some_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/020_some/some_02.md%})
