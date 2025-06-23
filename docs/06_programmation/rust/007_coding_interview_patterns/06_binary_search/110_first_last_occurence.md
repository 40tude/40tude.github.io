---
# published: false
layout: default
title: "p110 - First and Last Occurence of a Number"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# First and Last Occurence of a Number

* Given an array sorted in non-decreasing order return the first and last indexes of a target number, `[-1, -1]` otherwise

<span style="color:orange"><b>The point:</b></span>
* Sorted array => think about binary search
* We must find 2 occurrences
    * find lower bound
    * find upper bound
* In upper-bound binary search => bias the midpoint to the right. `mid = 1 + (left+right)//2`

### Checklist : Find lower bound
* **1 - Sorted search space**
    * [0, n-1]
* **2 - Narrow search space**
    * we are seeking for the leftmost occurrence
    * if value @ mid > target => target is on the left of mid => move right ptr inward => right = mid - 1
    * if value @ mid < target => target in on the right of mid => move left ptr inward => left = mid + 1
    * if value @ mid == target => 2 options : mid = lowerbound or mid is not yet the lowerbound (lowerbound is still on the left)
        * right = mid
* **3 - Choose an exit condition for the while loop**
    * exit when left==right  => while left < right
* **4 - Return the correct value**
    * return left (or right)

### Checklist : Find upper bound 
* **1 - Sorted search space**
    * [0, n-1]
* **2 - Narrow search space**
    * we are seeking for the rightmost occurrence
    * if value @ mid > target => target is on the left of mid => move right ptr inward => right = mid - 1
    * if value @ mid < target => target in on the right of mid => move left ptr inward => left = mid + 1
    * if value @ mid == target => 2 options : mid = upperbound or mid is not yet the upperbound (upperbound is still on the right)
        * left = mid
        * Infinite loop see p 113
        * bias the midpoint to the right. `mid = 1 + (left+right)//2`
* **3 - Choose an exit condition for the while loop**
    * exit when left==right => while left < right
* **4 - Return the correct value**
    * return right (or left)



**Complexity :**

| Time | Space |
|------|-------|
| O(log(n)) | O(1)  |

* O(log(n)) because the search space is of size n
* O(1) because in place 

**About Rust :**
* I assume the array is NOT empty (one can check `nums.is_empty()` otherwise) 
* I spent way too much time on a issue I did'nt realize when I was using Python
    * Indeed with `right = mid - 1;` the index `right` may become negative.
    * And in Python it is OK to look at ``nums[-1]`` and if this happen, `if nums[right] == target` "works" in Python and panic in Rust
    * This explain the `right = mid.checked_sub(1).unwrap_or(0);` 
* `lower_bound_binary_search(nums: &[i32], target: i32)` and `upper_bound_binary_search(nums:&[i32], target:i32)` returns `Option<usize>`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn lower_bound_binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > target{
            // right = mid - 1; // right can be negative in Python, not in Rust
            // right = mid.checked_sub(1).unwrap_or(0); // avoid underflow and exclude midpoint from the search space
            right = mid.saturating_sub(1); // avoids underflow. If mid == 0, then mid.saturating_sub(1) returns 0, with no panic nor Option
        }else if nums[mid] < target{
            left = mid + 1;  // exclude midpoint from the search space
        }else{
            right = mid; // narrow the search space toward the left
        }
    }
    if nums[left] == target { Some(left) } else { None }
}

fn upper_bound_binary_search(nums:&[i32], target:i32) -> Option<usize>{
    let(mut left, mut right) = (0, nums.len() - 1);   
    while left < right{
        let mid = 1 + left + (right - left) / 2; // biasing midpoint to the right
        if nums[mid] > target{
            // right = mid - 1; // right can be negative in Python, not in Rust
            // right = mid.checked_sub(1).unwrap_or(0); // avoid underflow and exclude midpoint from the search space
            right = mid.saturating_sub(1); 
        }else if nums[mid] < target{
            left = mid + 1; // exclude midpoint from the search space
        }else{
            left = mid; // narrow the search space toward the left
        }
    }
    if nums[right] == target { Some(right) } else { None }
}

fn first_last_occurrences(nums:&[i32], target:i32) -> (Option<usize>, Option<usize>){
    (lower_bound_binary_search(nums, target), upper_bound_binary_search(nums, target))
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 4)); // (Some(3), Some(5))
    println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 7)); // (None, None)
    println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 11)); // (Some(12), Some(12))
    println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 42)); // (None, None)
    println!("{:?}", first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 6, 8, 9, 10, 11], 0)); // (None, None)
} // end of local scope OR end of main()       

```

    (Some(3), Some(5))
    (None, None)
    (Some(12), Some(12))
    (None, None)
    (None, None)



```rust
    // prettier printing ?
    let (lower, upper) = first_last_occurrences(&[1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], 4); // (3, 5)
    match (lower, upper) {
        (Some(l), Some(u)) => println!("({}, {})", l, u),
        _ => println!("(None, None)"),
    }
```


```rust
    // To check if the array is empty
    if nums.is_empty() {
        return None;
    }

```
