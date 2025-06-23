---
# published: false
layout: default
title: "p032 - Pair Sum - Unsorted"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Pair Sum - Unsorted

- Here the list is unsorted
    - see 01_two_pointers\12_pair_sum_**sorted**.ipynb
- Given an array and a target sum, return the indexes of any pair of numbers that add up to the target sum.
- Avoid brute force with 2 nested loops O(n²) and use the two pointers technique
- Avoid sorting the array because O(nlogn)

<span style="color:orange"><b>The point:</b></span>    
- complement of x is y = target - x
- look for indices NOT for the values by themselves
    - pass 1 = fill a hash map with for each val its index
    - pass 2 = look for each val, if it scomplement is in the hash map


**Complexity Analysis :**

One pass


| Time | Space |
|------|-------|
| O(n) | O(n)  |

- Time, because we go through le list only once
- Space, the hash map can grow up to n





**About Rust :**
* `use std::collections::HashMap;`  ``my_hashmap.insert(key, value);``
* `for (i, &num) in nums.iter().enumerate()`
    * `.iter()` provides immutable references to the elements ``&T`` 
    * `nums` remains available afterwards (not consumed)
* `if let Some(&my_value) = my_hash_map.get(&my_key)`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




## Two passes


```rust
use std::collections::HashMap;

fn pair_sum_unsorted_two_pass(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    
    let mut num_to_index = HashMap::new();

    // First pass: fill the hash map with `num` as the key and its index `i` as the value
    for (i, &num) in nums.iter().enumerate() {
        num_to_index.insert(num, i);
    }

    // Second pass: for each number, check if its complement exists in the map and is at a different index
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        // If the complement is in the hash map and its index is different from `i`, returns the pair
        if let Some(&j) = num_to_index.get(&complement) {
            if i != j {
                return Some((i, j));
            }
        }
    }

    // No valid pair found
    None
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", pair_sum_unsorted_two_pass(&[5, 2, 3, 4, 1], 42));  // () 
    println!("{:?}", pair_sum_unsorted_two_pass(&[5, 2, 3, 4, 1], 6));  // (0,4) 
    let bob = vec![5, 2, 3, 4, 1];    
    println!("{:?}", pair_sum_unsorted_two_pass(&bob, 6));  // (0, 4)
// }
    
```

    None
    Some((0, 4))
    Some((0, 4))


## One pass

* We can have **one** pass if we fill the hash map and look for the complement at the same time
* <span style="color:lime"><b>Preferred solution?</b></span> 



```rust
use std::collections::HashMap;

fn pair_sum_unsorted_one_pass(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    
    let mut complement_to_index = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = complement_to_index.get(&complement) {
            return Some((j, i));
        }
        complement_to_index.insert(num, i);
    }
    None
}

// fn main() {
    println!("{:?}", pair_sum_unsorted_one_pass(&[1, 2, 3, 4, 5], 6));  // Some((1, 3))
    println!("{:?}", pair_sum_unsorted_one_pass(&[1, 2, 3, 4, 5], 42)); // None
    let bob = vec![1, 2, 3, 4, 5];    
    println!("{:?}", pair_sum_unsorted_one_pass(&bob, 6));  // (0, 4)
// }
```

    Some((1, 3))
    None
    Some((0, 4))


## V3

* A more functional way?



```rust
use std::collections::HashMap;

fn pair_sum_unsorted_one_pass(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    
    let mut seen = HashMap::new();

    nums.iter().enumerate().find_map(|(i, &num)| {
        let complement = target - num;
        seen.get(&complement).map(|&j| (j, i)).or_else(|| {
            seen.insert(num, i);
            None
        })
    })
}

// fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{:?}", pair_sum_unsorted_one_pass(&[1, 2, 3, 4, 5], 6));  // Some((1, 3))
    println!("{:?}", pair_sum_unsorted_one_pass(&[1, 2, 3, 4, 5], 42)); // None
    let bob = vec![1, 2, 3, 4, 5];    
    println!("{:?}", pair_sum_unsorted_one_pass(&bob, 6));  // Some((1, 3))
// }
```

    Some((1, 3))
    None
    Some((0, 4))

