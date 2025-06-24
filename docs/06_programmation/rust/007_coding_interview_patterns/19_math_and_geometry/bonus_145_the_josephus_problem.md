---
# published: false
layout: default
title: "bonus145 - The Josephus Problem"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# The Josephus Problem

<div align="center">
<img src="../assets/chap_19.webp" alt="" width="300" loading="lazy"/>
</div>

* There are n people standing in a circle, numbered from 0 to `n - 1`. 
* Starting from person 0, count `k` people clockwise and remove the kth person. 
* After the removal, begin counting again from the next person still in the circle. 
* Repeat this process until only one person remains, and return that person’s position.

<span style="color:orange"><b>The point:</b></span>

* Naïve approach in O(n x k)
* subproblem
* recursive : `josephus(n, k) = (josephus(n - 1, k) + k) % n`

**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(n)         | O(1)       |

* O(n) in time because we iterate n sub-problems
* O(1) in space because in place  









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn josephus(n: i32, k: i32) -> i32{
    // If there's only one person, the last person is person 0
    if n == 1{
        return 0;
    }
    // Calculate the position of the last person remaining in the reduced problem with 'n - 1' people
    // We use modulo 'n' to ensure the answer doesn't exceed 'n - 1'
    (josephus(n - 1, k) + k) % n
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", josephus(5, 2)); // 2
    println!("{}", josephus(6, 2)); // 4
} // end of local scope OR end of main()
```


```rust
fn josephus(n: usize, k: usize) -> usize{
    
    match n {
        1 => 0, 
        _ => (josephus(n - 1, k) + k) % n
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", josephus(5, 2)); // 2
    println!("{}", josephus(6, 2)); // 4
} // end of local scope OR end of main()
```

## Optimization

* No recursion
* bottom-up iterative approach
* we only ever need access to the previous value ``res``


**About Rust :**

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn josephus_optimized(n: usize, k: usize) -> usize{
    let mut res = 0;
    for i in 2..n + 1{
        res = (res + k) % i;
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", josephus_optimized(5, 2)); // 2
    println!("{}", josephus_optimized(6, 2)); // 4
} // end of local scope OR end of main()
```

## V2

**About Rust :**
* Avoid raw loop
* `(2..=n).fold(0, |res, i| (res + k) % i)`
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
fn josephus_optimized(n: usize, k: usize) -> usize {
    (2..=n).fold(0, |res, i| (res + k) % i)
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", josephus_optimized(5, 2)); // 2
    println!("{}", josephus_optimized(6, 2)); // 4
} // end of local scope OR end of main()
```
