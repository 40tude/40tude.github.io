---
# published: false
layout: default
title: "p339 - Maximum Subarray Sum"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Maximum Subarray Sum

* Given an array of `i32` return the sum of the subarray (contiguous cells) with the largest sum
* Assume array len >= 1

<span style="color:orange"><b>The point:</b></span>

* Brute force is in O(n²)
* There are negative numbers
* Contiguous cells
* curr_sum = max(curr_sum + num, num)
* **Kadane**'s algorithm





**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n)        | O(1)         |

* O(n) in time because we iterate input array once
* O(1) in space because the algo acts in place


**About Rust :**
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn maximum_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        0
    } else {
        // Set sum variables to MIN so that negatives sum can be considered
        let (mut max_sum, mut current_sum) = (i32::MIN, i32::MIN);
        for &num in nums {
            // Either add the current number to the running sum or start a new subarray at the current number
            current_sum = (current_sum + num).max(num);
            max_sum = max_sum.max(current_sum);
        }
        max_sum
    }
}

fn main() { // no main() if this code runs in a Jupyter cell
    let vals = vec![3, 1, -6, 2, -1, 4, -9];
    println!("{}", maximum_subarray_sum(&vals)); // 5
} // end of local scope OR end of main()

```

## DP

**Phases of a DP solution :**
1. Subproblem
1. DP Formula
1. Base Case
1. Populate DP Table


<span style="color:orange"><b>The point:</b></span>

* Start with the end in mind
* Every subarray ends at a certain index
* So each index is the end of several subarrays
* One of which will have the max subarray sum
* `max_subarray(i) = max (max_subarray(i-1) + nums[i], nums[i])`
* Recursive
* Base case : when there is one element there is one subarray

**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n)           | O(n)      |

* O(n) in time because we iterate DP array once
* O(1) in space because the DP array size


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





```rust
fn maximum_subarray_sum_dp(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        0
    } else {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        let mut max_sum = dp[0];
        for i in 1..nums.len() {
            dp[i] = (dp[i - 1] + nums[i]).max(nums[i]);
            max_sum = max_sum.max(dp[i]);
        }
        max_sum
    }
}

fn main() {    // no main() if this code runs in a Jupyter cell
    let vals = vec![3, 1, -6, 2, -1, 4, -9];
    println!("{}", maximum_subarray_sum_dp(&vals)); // 5
} // end of local scope OR end of main()

```

## Optimization

<span style="color:orange"><b>The point:</b></span>

* We only need to access `dp[i-1]` to calculate value at `i`
* No need for DP array
* One variable is enough

**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n)        | O(1)         |

* O(n) in time because we iterate input array once
* O(1) in space because the algo acts in place



**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn maximum_subarray_sum_dp_optimized(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        0
    } else {
        let (mut current_sum, mut max_sum) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            current_sum = (current_sum + nums[i]).max(nums[i]);
            max_sum = max_sum.max(current_sum);
        }
        max_sum
    }
}

fn main() {    // no main() if this code runs in a Jupyter cell
    let vals = vec![3, 1, -6, 2, -1, 4, -9];
    println!("{}", maximum_subarray_sum_dp_optimized(&vals)); // 5
} // end of local scope OR end of main()

```

## V2

* Clippy does'nt like the `for loop` as it is

**About Rust :**
* `for &num in nums.iter().skip(1) {...`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn maximum_subarray_sum_dp_optimized(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        0
    } else {
        let (mut current_sum, mut max_sum) = (nums[0], nums[0]);
        for &num in nums.iter().skip(1) {
            current_sum = (current_sum + num).max(num);
            max_sum = max_sum.max(current_sum);
        }
        max_sum
    }
}

fn main() {    // no main() if this code runs in a Jupyter cell
    let vals = vec![3, 1, -6, 2, -1, 4, -9];
    println!("{}", maximum_subarray_sum_dp_optimized(&vals)); // 5
} // end of local scope OR end of main()

```
