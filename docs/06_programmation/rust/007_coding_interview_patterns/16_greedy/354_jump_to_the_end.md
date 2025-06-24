---
# published: false
layout: default
title: "p354 - Jump to the End"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Jump to the End

* Given an array of u32
* Originally at index0
* Numbers represent the **maximum** jump distance from the current index 
* Determine if it is possible to reach the end of the array


<span style="color:orange"><b>The point:</b></span>

* From index i, furthest index is i + index[i]
* 0 is a dead end
* If we can reach the last index from any earlier index, the later becomes the new destination => start from the end


**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(1)  |

* O(n) in time because we iterate over each cell
* O(1) in space because in place









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* First implementation

**About Rust :**
* ``for i in (0..nums.len()).rev() {``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn jump_to_the_end(nums: &[usize]) -> bool {
    if nums.is_empty() {
        return false; 
    }
    // Set initial destination to the last index
    let mut destination = nums.len() - 1;

    // Traverse the array in reverse to see if the destination can be reach by earlier indexes
    for i in (0..nums.len()-1).rev() {
        // if we can reach the destination from current index, the current index becomes the new destination
        if i + nums[i] >= destination {
            destination = i;
        }
    }
    //  If the destination is index 0 we can jump to the end from idx 0
    destination == 0
}

// no main() if this code runs in a Jupyter cell
fn main() {
    let nums = vec![];
    println!("{:?}", jump_to_the_end(&nums)); // false

    let nums = vec![3, 2, 0, 2, 5];
    println!("{:?}", jump_to_the_end(&nums)); // true

    let nums = vec![2, 1, 0, 3];
    println!("{:?}", jump_to_the_end(&nums)); // false
} // end of local scope OR end of main()

```

## V2

* A more functional way?

**About Rust :**
* Use ``fold`` to carry the "destination" 
* <span style="color:lime"><b>Preferred solution?</b></span> 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


### Note about `.fold()`
* This is an iterator method for **reducing a collection to a single value** by applying an accumulation function.
    * Cycle through each element,
    * At each step, combine the current accumulator with the element,
    * Generate a new accumulator value
    * At the end, we have a single value.
* More functional because there is no ``mut``, no ``for`` loop, no `while` loop

#### Syntax 

```rust
iterator.fold(init, |accumulator, item| {
    // return new accumulator
})

```

#### Simple example 

Find a max

```rust

let nums = vec![3, 7, 2, 9, 5];
let max = nums.iter().fold(usize::MIN, |acc, &x| acc.max(x));
println!("{}", max); // 9

```


```rust
fn jump_to_the_end(nums: &[usize]) -> bool {
    if nums.is_empty() {
        return false;
    }
    
    (0..nums.len()) // a range
        .rev() // transforms the range into an iterator reading values in reverse order
        .fold(nums.len() - 1, |destination, i| { // destination is the accumulator, i is the current item
            if i + nums[i] >= destination {
                i           // destination becomes i
            } else {
                destination // otherwise destination stays the same
            }
        }) == 0
}

fn main() { // no main() if this code runs in a Jupyter cell
    let nums = vec![];
    println!("{:?}", jump_to_the_end(&nums)); // false

    let nums = vec![3, 2, 0, 2, 5];
    println!("{:?}", jump_to_the_end(&nums)); // true

    let nums = vec![2, 1, 0, 3];
    println!("{:?}", jump_to_the_end(&nums)); // false
} // end of local scope OR end of main()

```

## V3

* Another functional way?

**About Rust :**
* Use ``.enumerate()``
* Use ``fold`` to carry the "destination" 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn jump_to_the_end(nums: &[usize]) -> bool {
    assert!(!nums.is_empty(), "Input cannot be empty");

    nums.iter()
        .enumerate()
        .rev()
        .fold(nums.len() - 1, |destination, (i, &num)| {
            if i + num >= destination {
                i
            } else {
                destination
            }
        }) == 0
}

fn main() { // no main() if this code runs in a Jupyter cell
    // let nums = vec![];
    // println!("{:?}", jump_to_the_end(&nums)); // false

    let nums = vec![3, 2, 0, 2, 5];
    println!("{:?}", jump_to_the_end(&nums)); // true

    let nums = vec![2, 1, 0, 3];
    println!("{:?}", jump_to_the_end(&nums)); // false
} // end of local scope OR end of main()
```

## What about performances ?
* I did a bench with Cargo bench
    * Check `16_greedy\assets\bench_jump` and read the `README.md` file
* The third version is the looser. The first two are similar

```
for loop                time:   [2.0784 µs 2.0833 µs 2.0892 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

fold with range         time:   [2.0918 µs 2.1007 µs 2.1113 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

fold with iter enumerate
                        time:   [4.5125 µs 4.5231 µs 4.5351 µs]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

```
