---
# published: false
layout: default
title: "p175 - Introduction"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Introduction

**Complexity :**

| Time     | Space |
|----------|-------|
| O(n)     | O(n)  |

* O(n) because we sum over n values
* O(n) because of space used to holds the n sums


**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## V1


```rust
fn compute_prefix_sum(nums:&[i32])->Vec<i32>{
    let mut prefix_sums = Vec::with_capacity(nums.len());
    if !nums.is_empty() {
        prefix_sums.push(nums[0]);
        for i in 1..nums.len(){
            prefix_sums.push(prefix_sums[i-1] + nums[i] )  
        }
    }
    prefix_sums
}

fn main(){   
    println!("{:?}", compute_prefix_sum(&[10, 15, 20, 10, 5])); //[10, 25, 45, 55, 60]
    println!("{:?}", compute_prefix_sum(&[])); //[]
}   
```

## V2
* Better


```rust
fn compute_prefix_sum(nums: &[i32]) -> Vec<i32> {
    let mut prefix_sums = Vec::with_capacity(nums.len());
    let mut current = 0;
    
    for &num in nums {
        current += num;
        prefix_sums.push(current);
    }
    prefix_sums
}

fn main(){   
    println!("{:?}", compute_prefix_sum(&[10, 15, 20, 10, 5])); //[10, 25, 45, 55, 60]
    println!("{:?}", compute_prefix_sum(&[])); //[]
}   
```
