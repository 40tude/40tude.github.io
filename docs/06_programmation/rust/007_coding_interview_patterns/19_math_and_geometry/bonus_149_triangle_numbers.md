---
# published: false
layout: default
lang: en-US
title: "bonus149 - Triangle Numbers"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Triangle Numbers

<div align="center">
<img src="../assets/chap_19.webp" alt="" width="300" loading="lazy"/>
</div>

* Consider a triangle composed of numbers where the top of the triangle is 1. 
* Each subsequent number in the triangle is equal to the sum of three numbers above it
    1. its top-left number, 
    1. its top number
    1. its top-right number. 
* If any of these three numbers don’t exist, assume they are equal to 0.
* Given a value representing a row of this triangle, return the position of the first even number in this row. 
* Assume the first number in each row is at position 1.


<span style="color:orange"><b>The point:</b></span>

* The triangle is symmetric (can exclude right half)
* Pascal's triangle
* We only care about the parity of each number (NOT the values)
* Even numbers only begin appearing from row 3 onward




**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(1)         | O(1)       |




<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

**About Rust :**
* Basic implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn triangle_numbers(n: usize) -> usize{

    // If n is an odd-numbered row, the first even number always starts at position 2
    if n % 2 != 0{
        return 2;
    // If n is a multiple of 4, the first even number always starts at position 3.
    }else if n % 4 == 0{
        return 3;
    }
    // For all other rows, the first even number always starts at position 4.
    return 4
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", triangle_numbers(4)); // 3
} // end of local scope OR end of main()
```

## V2

**About Rust :**
* No ``return``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn triangle_numbers(n: usize) -> usize {
    if n % 2 == 1 {
        2
    } else if n % 4 == 0 {
        3
    } else {
        4
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", triangle_numbers(4)); // 3
} // end of local scope OR end of main()
```

## V3

**About Rust :**
* Using ``match`` with tuples and `_`
* No ``return``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn triangle_numbers(n: usize) -> usize {
    match (n % 2, n % 4) {
        (1, _) => 2,    // n is odd
        (0, 0) => 3,    // n divisible by 4
        (0, _) => 4,    // n even but not divisible by 4
        _ => unreachable!(), // impossible case
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", triangle_numbers(4)); // 3
} // end of local scope OR end of main()
```

# V4

**About Rust :**
* Compile-time evaluation
    * See ``constexpr`` in C++
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
const fn triangle_numbers(n: usize) -> usize {
    if n % 2 == 1 {
        2
    } else if n % 4 == 0 {
        3
    } else {
        4
    }
}

fn main() {
    const N: usize = triangle_numbers(4); // compile-time evaluation
    println!("{}", N); // 3
}
```
