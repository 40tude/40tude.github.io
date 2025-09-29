<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Errors from Experimentation to Production

**Alice:** It was a lot... Many, many thanks because it really helps to organized my thoughts about errors management. There is may one last thing I would like to discuss with you. I know I'm a young Padawan and that my code are mostly experimentation I work on during week-end. Ok, but I'm wondering how errors are managed in more "serious" code. I mean, I would like to learn more so that I will not be lost while reading code from others on GitHub and more importantly, put the good practices in place, up front, so that I can transition happily to production code.

<div align="center">
<img src="./assets/img24.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

**Bob:** ***Help you in this quest, I can.*** And since you already know almost everything you need I propose to follow this path. 
1. First we will recap what we would like. I have nothing to say because you have the answers
2. Then we will we will put ourself in a situation were you start a brand new experimental code.
3. Finally you will transition your code in production ready state. At least we will put in place what we need from the error management point of view.

Do you agree?

**Alice:** This would be perfect. Let's go


### Key Concepts

**Bob** : Have you ever heard about the [Gall’s law]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#galls-law)? No? It translates in words your intuition. Indeed you feel the Force but you also feel that, ideally, your sample code will evolve. The law says (read it with a strong voice...): "A complex system that works is invariably found to have evolved from a simple system that worked..."

So, good news, you are right. You must start with an experimental code that works and which will evolve (may be) in a million dollar class of application.

I can also confirm you are right when you say that you want to put it place, up front, an error management system that scales with your app.

Now, I have a question for you. Without entering technical details, what do you want from the error management standpoint?

**Alice:** Um... I would say...
* The sooner the better. I mean, get help from the rust type and build systems to detect most of errors at compile time...
* The fewer the better. This is obvious. Ideally I don't want error in my code.
* I told you, I really like the `?` operator. It makes the code easy to read. Its my friend. I would like to keep it from proto to prod.
* I would like to have the means to write quickly the experimentation code and to use what we saw with the custom error type in production. `enum` and friends are great but I'm no sure I want to see them in my experimentation.
* What else? An expresso? More seriously I don't see much to say except the fact I want to avoid to rewrite my code when I transitioning to production. 

<div align="center">
<img src="./assets/img25.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

**Bob:** It is really good. You are definitively on the right track. Let's keep all this in mind and let's move to the experimentation phase.





### Experimentation

{: .note-title }
> Side Note
>
> In the workspace, the code used below is in `01_experimentation/examples/` directory. 


**Bob:** It is Saturday night. The house is silent, your young sister is out (you don't want to kow where nor with who). This is the best time to play with Rust.

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

Then... Yes, I know what you want. Let's make sure I can use my friend `?` in `main()`. Since I don't know yet what can of std lib and crate function I will call, I will make sure `main()` can handle and returns all of them. I don't really remember, but it was based on `Box`, `dyn`, blablabla...

**Bob:** It is not a problem. Go back and review `00_u_are_errors\examples\ex08.rs` for example.

**Alice:** Thanks for the nudge. So I would write this:

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
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

Indeed I rewrite your code like this:

```rust
// ex001.rs

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(()) 
}
```

No big change. In fact since we want to use the same code from experimentation to production it is smarter to keep `Error` and `Result<T>` separated. Doing so, even if in production, the `Error` type evolve to something different (e.g. a custom error type) the `Result` type will not be impacted (it will always refers to `Error`) and this is exactly what we want.

By the way do you have any idea of what I just did?

**Alice:** No. You split my line in two and you explained that later if the `Error` type becomes very complicated, this will have no impact of `Result<T>` 











**Bob:** I just added a level of [indirection]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#indirection) which, according to David Wheeler is the way to solve most of problems in software.

So, at this point, we agree to say that `ex001.rs` is by now your official code template. Ok? Ok. 

Do you know what is the BMI?

**Alice:** Yes I do. My sister is always talking about it. I learnt this a statistical value which is more valuable for population than for individual. It indicates if the group is overweight or not. Basically you take a weight (in kg) and divide it by the square of the height (in meters). This give a result in number of kilograms per square meter. If the group is between 18.5 and 24.9 it is OK.

**Bob:** Using your template write a prototype to calculate the BMI.

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

While writing the code, the most difficult part was the line `return Err("Height cannot be 0.0".into());`. I lost some time because initially I wanted to `return Err("Height cannot be 0.0");` but it does'nt work. Indeed `bmi()` returns a `Result<f64>`, this means a `Result<f64, Box<dyn Error>>`. So I have to convert the `&'static str` into a `Box<dyn std::error::Error>` first. I hope I will remember the `.into()`.


**Bob:** Don't worry this will come with practice. Now, in a new experimentation,  I want you to write function that receive a vector of integers written as strings and returns their sum.

**Alice:** If we look from the `main()` standpoint is it what you want to see?

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
* `values: &[&str]` may look weird but no, it is not. In `main()` I pass the vector `numbers` by reference because I borrow it (I don't want to give it) to `sum_strings()`. Now in `main()`, if I press`CTRL+ALT`, I see the exact type of `numbers` (`Vec<&'static str>`). So `sum_strings()`'s parameter is a reference to an array of static strings (`&str`). 
* Then there is a for loop which traverses the vector `values`
* I remembered we used `.parse()` at the beginning of the section "The `Result<T, E>` Type: Handling Recoverable Errors" 
* Pressing `CTRL+ALT`, I see `.parse::<i32>()` returns a `Result<i32, ParseIntError>`
* If `current_val` is Ok I add its value to the running `sum`, otherwise... With the help of `.unwrap()` the code `panic!()`
* At the end of the loop, `sum` is a valid number and I return it with `Ok(sum)`

The code work, but to tell the truth, I'm not really proud of the `.unwrap()` and I know I should avoid the raw loop.

<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/W2tWOdzgXHA?si=eg4uz51KzDt47q2o&amp;start=41" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>

**Bob:** Then?

**Alice:** Now I have this version of `sum_strings()` without any raw loop

```rust
fn sum_strings(values: &[&str]) -> Result<i32> {
    let sum: i32 = values
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .sum();
    Ok(sum)
}
```

But I remember what we said about `.unwrap()`, and `.expect()`. For now I have this version which prints a custom message on error. See below : 

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

Here is what I can see in the terminal

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
    let files = list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}
```

**Bob:** Yes. Now show me the `list_files()` function please.

**Alice:** Here is what I have so far

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
* When OK this is an iterator over the entries within the directory
* If it is an iterator I can daisy chained multiple filter and keep the files of interest
* `.filter_map()`, `.filter()` and `.collect()` operate on an `Iterator<Item = DirEntry>` once the `Result` has been unwrapped by `?`
* These iterator methods do not return a `Result`. They cannot fail in a way that would require error propagation.
* They simply transform the data from one form to another 
* This is why there is no `?` at the end of the steps
    * the first `.filter_map()` silently drops entries that errored
    * the second `.filter()` ask the filesystem whether the entry is a file. If that check errors because it is a directory, it is treated as false and not kept in the list of files.
    * the last `filter_map()` only keeps filenames that are valid UTF-8 and the others are dropped
* The last step is `.collect()` which creates a vector with the filtered names
* Finally the function returns the vector to `main()` with `Ok(files)`        


**Bob:** Did you notice how your template worked fine in 3 different experimentation? I guess we can keep it in our toolbox.

Now in the last sample code, rather than panicking on error after the call to `read_dir()`, could you avoid the `?` here and return a custom message to `main()` explaining what's going here?  

**Alice:** Ok... I start by removing the `?` then... I don't know.

**Bob:** Do you remember the section "`Option<T>` vs. `Result<T, E>`: Choosing the Right Type"? We were discussing about the `Option<T>` and the fact we were using the reason why the failure happened. I told you we can return an `Option<T>` but log the reason of failure. To do so I use .map_err(). Do you remember now? Review `ex16.rs` if needed.

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
        .map_err(|_| "❗Error while reading dir.")? // but `?` is here. We return custom error as static string
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}
```

* The key is to remember `.map_err()` and how it works
* At the exit of `read_dir()`
    * If the Result is an `Ok(value)`, `.map_err()` does nothing. The `?` operator evaluates to `value` and the execution continues
    * If the Result is `Err(e)`, .map_err() applies the closure to `e` and returns `Err(closure(e))`
* Here the closure ignore the actual `io::Error` (`|_|` discards it) and replace it with a static string slice `"Error while reading dir."`
* The `?` operator immediately returns that error from the current function.

Now, let me repeat what happens. Just to make sure my understanding is correct
* The return type of the `list_files()` function is `Result<Vec<String>, Box<dyn std::error::Error>>`
* So when the `Err(&str)` need to be bubbled up, Rust automatically applies `From<&str> for Box<dyn Error>` which is implemented by boxing the `&str` as a string error.
* That’s why we can return a bare "`Error while reading dir.`" and it gets "promoted" into a proper `Box<dyn Error>`.
* The promotion from `&str` to `Box<dyn std::error::Error>` works because std lib includes `impl<'a> From<&str> for Box<dyn Error + 'a>`. I took the time to [read this page](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error%3E:~:text=impl%3C%27a%3E%20From%3C%26str%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E).


{: .warning-title}
> This is key
>
The promotion from `&str` to `Box<dyn std::error::Error>` works because std lib includes an implementation of the From trait which does exactly that. See `impl<'a> From<&str> for Box<dyn Error + 'a>`.  

**Bob:** I'm impressed. Even if it is a little bit overkill because we are supposed to be in an experimentation, I guess that if I ask you to return *also* the reason *why* the error occurred it is a matter of seconds. No?

**Alice:** Now it is easy. Here is the new version of the code

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
* Above, `.map_err()` returns a custom error as formatted `String`.
* * The promotion from `String` to `Box<dyn std::error::Error>` works because std lib includes [`impl<'a> From<String> for Box<dyn Error + 'a>`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C&str%3E-for-Box%3Cdyn+Error%3E:~:str%3E%20for%20Box%3Cdyn%20Error%20+%20'a%3E&text=impl%3C%27a%3E%20From%3CString%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E). 


**Bob:** A Padawan no more, you are. Prove a Jedi Knight you have become... Let's stay on the last example and show me how, in addition to the first version you would return an meaningful error message if the directory is empty.

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
* This one was easier because I remembered about `.into()`
* The code is the same but once the `files` Vector is collected, I check if it is empty
* If it is the case I return an ad hoc message.
* Otherwise, as before, we reach the end of the body of `list_files()` the `files` vector is Ok and I return `Ok(files)`


**Bob:** We are still in the experimentation phase were we take the time to learn, discover, crash and repair things. Can you tell me in detail why and how the .into() works. Take your time, read the documentation...

**Alice:** It turned out to be a real caving expedition, and it took me more time than I had anticipated. Sorry about that. 
* The `.into()` works because std lib includes [`impl<'a> From<&str> for Box<dyn Error + 'a>`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-From%3C%26str%3E-for-Box%3Cdyn+Error%3E:~:text=impl%3C%27a%3E%20From%3C%26str%3E%20for%20Box%3Cdyn%20Error%20%2B%20%27a%3E).
* When I write `"Cannot list empty folder.".into();`
* It starts as a `&'static str`
* Rust sees that the expected type is `Box<dyn Error>`
* It found `impl<'a> From<&str> for Box<dyn Error + 'a>` in the std lib
* But in Rust if we have `From<A> to B` we get `Into<B> for A` for free
* Here this means `Into<Box<dyn Error> for &str` exists
* Then the `static &str` is automatically converted to `Box<dyn Error>`
* The story has a happy ending: they got married and lived happily ever after.

<div align="center">
<img src="./assets/img28.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

**Bob:** It's showtime! Let's move to production phase.
















### Production

<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/LUapZhcsdx8?si=cxsAd5AjKMZfTm1x&amp;start=12" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>


### Summary – Experimentation to Production

{: .new-title }
> Summary – Experimentation to Production
>
* **`derive_more`:** ...  
* **...:** ...  


### Exercises – Experimentation to Production 

1. ...
1. ...






