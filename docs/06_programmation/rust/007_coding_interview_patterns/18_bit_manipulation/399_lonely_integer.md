---
# published: false
layout: default
title: "p399 - Lonely Integer"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Lonely Integer

<div align="center">
<img src="../assets/chap_18.webp" alt="" width="300" loading="lazy"/>
</div>

* Given an integer array where each number occurs twice except for one, find the unique (*to rule them all?*)

<span style="color:orange"><b>The point:</b></span>

* xor all elements together




**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(nlog(n))   | O(1)       |

* O(nlog(n)) in time because for each integer from 0 to n, counting the number of bits set is in log(n) => n log(n)
* O(1) in space because in place  




**About Rust :**
* Basic implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn lonely_integer(nums : &[i32]) -> i32{
    let mut res = 0;
    // xor each element of the array
    // duplicate values cancel each other out (x^x=0)
    for num in nums{
        res ^=num;
    }
    // res store the lonely i32 because it is not canceled by any duplicate
    res
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let nums = vec![1,3,3,2,1];
    println!("{}", lonely_integer(&nums)); // 2
} // end of local scope OR end of main()
```
