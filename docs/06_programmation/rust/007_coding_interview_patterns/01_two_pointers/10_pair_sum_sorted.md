---
# published: false
layout: default
title: "p010 - Pair Sum - Sorted"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Pair Sum - Sorted

<div align="center">
<img src="../assets/book_cover.png" alt="" width="225" loading="lazy"/>
</div>


* Given a sorted array and a target sum, return the indexes of any pair of numbers that add up to the target sum.
* Avoid brute force with 2 nested loops O(n²) and use the two pointers technique

<span style="color:orange"><b>The point:</b></span>    
* Leverage the fact that the numbers are sorted
* Use 2 ptrs, one on the left the other on the right
* Sum them up. If below move the left ptr inward. If above move the right ptr inward


**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(1)  |

* O(n) because we traverse the list
* O(1) in space because no growing data structure are created



**About Rust :**
* For this exercice only
    * I show different ways to manage results in Rust
        * ``Result<T, E>``, `Option<T>`, ``assert_eq!``, the `?` operator 
    * I show how to include testing in the code
        * Running tests is not possible in Jupyter Notebook but works in a project and in [Rust Playground](https://play.rust-lang.org/) 
* ``pair_sum_sorted()`` first parameter is `&[i32]`
    * The function can be called with a reference to an array or a vector
* ``pair_sum_sorted()`` returns indexes as ``usize`` not ``i32``
* ``saturating_sub(1)`` 
    * ``let mut right= nums.len().saturating_sub(1);``
    * ``right = len - 1`` or ``0`` if ``len-1`` is negative because `right is usize`
* ``struct PairNotFoundError;`` custom type
    * When returning ``Result<T, E>`` 
* ``.map_or()``
    * When returning `Option<T>` or ``Result<T, E>``
    * See `println!("Result : {:?}", res.map_or("No solution".to_string(), |v| format!("{:?}", v)));`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





## Base implementation

* `right = nums.len().saturating_sub(1)` right = len - 1 or 0 if len-1 is negative


```rust
// the 1st parameter is a slice (=> the function can be called with a reference to an array OR a vector) 
fn pair_sum_sorted(nums: &[i32], target: i32) -> Vec<usize> {
    
    let (mut left, mut right) = (0, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative
    
    while left < right {
        let sum = nums[left] + nums[right];
        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            return vec![left, right];
        }
    }
    vec![]
}

// fn main() {
    // Does not print, just assert. May be enough most of the time in the context of this book
    assert_eq!(pair_sum_sorted(&[-5, -2, 3, 4, 6], 7), [2, 3]); // an array as argument

    let res = pair_sum_sorted(&[-5, -2, 3, 4, 6], 7); 
    println!("{:?}", res); // [2, 3]        :? => use the debug format

    let res = pair_sum_sorted(&vec![1, 1, 1], 2); // a vector as argument
    println!("{:?}", res); // [0, 2] or any valid pair

    let res = pair_sum_sorted(&[1, 1, 1], 42);
    println!("{:?}", res); // [] 
// }

```

    [2, 3]
    [0, 2]
    []


## Returns a `Result<T, E>` 
* ``pair_sum_sorted()`` continue to accept reference to array or vector 
* When found, the 2 indices are returned in a tuple.
* Return an error otherwise


```rust

// Define a custom error type
#[derive(Debug)]
struct PairNotFoundError;

// Function that returns a Result
// the 1st parameter is a slice (=> the function can be called with a reference to an array OR a vector) 
fn pair_sum_sorted(nums: &[i32], target: i32) -> Result<(usize, usize), PairNotFoundError> {
    
    let (mut left, mut right) = (0, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative
    
    while left < right {
        let sum = nums[left] + nums[right];
        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            // Success: return indices wrapped in Ok (a tuple)
            return Ok((left, right));
        }
    }
    // Error: no valid pair found
    Err(PairNotFoundError)
}

// fn main() {
    let nums = [-5, -2, 3, 4, 6];
    let target = 7; 
    match pair_sum_sorted(&nums, target) {
        Ok(indices) => println!("Found pair at indices: {:?}", indices),
        Err(_) => println!("No pair found that sums to the target."),
    }
    
    let nums = vec![1, 1, 1];
    let target = 2;
    match pair_sum_sorted(&nums, target) {
        Ok(indices) => println!("Found pair at indices: {:?}", indices),
        Err(_) => println!("No pair found that sums to the target."),
    }
    
    let res = pair_sum_sorted(&[1, 1, 1], 2);
    // Use map_or to provide a default string if no solution
    println!("Result : {:?}", res.map_or("No solution".to_string(), |v| format!("{:?}", v)));
// }
```

    Found pair at indices: (2, 3)
    Found pair at indices: (0, 2)
    Result : "(0, 2)"


## Returns an `Option<T>` 
* <span style="color:lime"><b>Preferred solution?</b></span>
* Because not finding a pair may not be considered as an error
* ``pair_sum_sorted()`` continue to accept reference to array or vector 
* When found, the 2 indices are returned in a tuple.
* Return None otherwise


```rust
// Function returns an Option instead of a Result
// the 1st parameter is a slice (=> the function can be called with a reference to an array OR a vector) 
fn pair_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {

    let (mut left, mut right) = (0, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative

    while left < right {
        let sum = nums[left] + nums[right];
        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            // Success: return indices wrapped in Some (a tuple)
            return Some((left, right));
        }
    }
    // No valid pair found, return None
    None
}

// fn main() { // no main() if this code runs in a Jupyter cell
    let nums = [-5, -2, 3, 4, 6];
    let target = 7; 
    match pair_sum_sorted(&nums, target) {
        Some(indices) => println!("Found pair at indices: {:?}", indices),
        None => println!("No pair found that sums to the target."),
    }

    let nums = vec![1, 1, 1];
    let target = 2;
    match pair_sum_sorted(&nums, target) {
        Some(indices) => println!("Found pair at indices: {:?}", indices),
        None => println!("No pair found that sums to the target."),
    }

    let res = pair_sum_sorted(&[1, 1, 1], 42);
    // Use map_or to provide a default string if no solution
    println!("Result : {:?}", res.map_or("No solution".to_string(), |v| format!("{:?}", v)));
// }

```

    Found pair at indices: (2, 3)
    Found pair at indices: (0, 2)
    Result : "No solution"


## Use the ? operator 
* ``pair_sum_sorted()`` returns an Option because not finding a pair may not be considered as an error
* Continue to accept reference to array or vector 
* When found, the 2 indices are returned in a tuple.
* Return None otherwise
* We need an intermediate function to demonstrate the propagation of the Option when the `?` operator is used


```rust
// the 1st parameter is a slice (=> the function can be called with a reference to an array OR a vector) 
fn pair_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {

    let (mut left, mut right) = (0, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative

    while left < right {
        let sum = nums[left] + nums[right];
        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            // Success: return indices wrapped in Some (a tuple)
            return Some((left, right));
        }
    }
    // No valid pair found, return None
    None
}

// Function that uses ? to propagate the Option
fn intermediate_fn (nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let indices = pair_sum_sorted(nums, target)?; // If None, early return
    Some(indices) // If Some, wrap and return
}


// fn main(){     // no main() if this code runs in a Jupyter cell 
    let nums = [0, 1, 2, 4, 5, 6];
    let target = 42;
    match intermediate_fn(&nums, target) {
        Some(indices) => println!("Found a pair : {:?}", indices),
        None => println!("No pair found."),
    }
// } 
```

    No pair found.





    ()



## How to include testing ?

**About Rust :**
* Testing does **NOT** work in a  Jupyter Notebook
    * For what I know (june 2025)
* `#[allow(dead_code)]` is mandatory under Rust Playground
    * Otherwise, unnecessary warnings are issued by the compiler
    * In a "regular" Rust project, you can comment those lines 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

### Why `#[allow(dead_code)]` is needed in Rust Playground ?
* Rust (via cargo test) compiles tests in a temporary crate, injecting the contents of the source file into a lib crate, even if the file is ``main.rs``.
* When a function (such as `pair_sum_sorted()`) is used only in tests, then the compiler sees it as unused in the main crate (lib), even if it is invoked in `#[cfg(test)]`.
* This is a limitation of Rust's dead_code analysis system: 
    * calls from `#[cfg(test)]` blocks are not enough to prevent the dead_code warning
    * as the compiler first analyzes the “out-of-test” code to decide what is used.
* So the compiler considers 
    * as dead_code everything that is not called in the “normal path” of execution.
    * and calls from `#[cfg(test)]` do not prevent a warning, even if the function is actually being tested.




```rust
// Function returns an Option instead of a Result
// The 1st parameter is a slice (=> the function can be called with a reference to an array OR a vector) 
#[allow(dead_code)] // Rust Playground : avoid some warnings during compilation for testing
fn pair_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {

    let (mut left, mut right) = (0, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative

    while left < right {
        let sum = nums[left] + nums[right];
        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            // Success: return indices wrapped in Some (a tuple)
            return Some((left, right));
        }
    }
    // No valid pair found, return None
    None
}

#[allow(dead_code)] // Rust Playground : avoid some warnings during compilation for testing
// fn main() { // no main() if this code runs in a Jupyter cell
    let nums = [-5, -2, 3, 4, 6];
    let target = 7; 
    match pair_sum_sorted(&nums, target) {
        Some(indices) => println!("Found pair at indices: {:?}", indices), // Found pair at indices: (2, 3)
        None => println!("No pair found that sums to the target."),
    }

    let nums = vec![1, 1, 1];
    let target = 2;
    match pair_sum_sorted(&nums, target) {
        Some(indices) => println!("Found pair at indices: {:?}", indices), // Found pair at indices: (0, 2)
        None => println!("No pair found that sums to the target."),
    }

    let res = pair_sum_sorted(&[1, 1, 1], 42);
    // Use map_or to provide a default string if no solution
    println!("Result : {:?}", res.map_or("No solution".to_string(), |v| format!("{:?}", v))); // Result : "No solution"
// } // end of local scope OR end of main()

#[cfg(test)]
mod my_tests {
    use super::*;

    #[test]
    fn find_pair_should_return_indices_when_sum_matches_target() {
        let nums = [-5, -2, 3, 4, 6];
        let target = 7;
        // Compare directly with Some to avoid panicking with unwrap
        assert_eq!(Some((2, 3)), pair_sum_sorted(&nums, target));
    }

    #[test]
    fn should_return_none_when_no_valid_pair_exists() {
        let nums = [1, 1, 1];
        let target = 42;
        assert_eq!(None, pair_sum_sorted(&nums, target));
    }
}
```

    Found pair at indices: (2, 3)
    Found pair at indices: (0, 2)
    Result : "No solution"

