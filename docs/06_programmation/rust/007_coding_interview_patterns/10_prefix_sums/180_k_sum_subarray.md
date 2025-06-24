---
# published: false
layout: default
title: "p180 - K-Sum Subarray"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# K-Sum Subarray

<div align="center">
<img src="../assets/chap_10.webp" alt="" width="300" loading="lazy"/>
</div>

* Find the number of subarrays in an array of i32 that sum to `k`



<span style="color:orange"><b>The point:</b></span>

* prepend the prefix-sum array with 0
* iterate from 1 because 0 was added

**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n²)        | O(n)  |

* O(n²) because we iterate through each value in 2 nested loops. Brute force would be O(n3)
* O(n) because of space taken by the hash map



**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn k_sum_subarrays(nums : &[i32], k:i32) -> usize {
    let n = nums.len();

    // Populate prefix sum array, setting its first element to 0
    let mut prefix_sums = Vec::with_capacity(n+1);
    prefix_sums.push(0);
    let mut current = 0;
    for &num in nums {
        current += num;
        prefix_sums.push(current);
    }
    
    // loop trough valid pairs of prefix_sum values to find all subarray summing to `k`  
    let mut count = 0;
    for j in 1..n+1{
        for i in 1..j+1{
            if prefix_sums[j] - prefix_sums[i-1] == k {
                count += 1 ;
            }
        }
    }
    count
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    println!("{:?}", k_sum_subarrays(&vec![1, 2, -1, 1, 2], 3));    // 3
    println!("{:?}", k_sum_subarrays(&[1, 2, -1, 1, 2], 3));        // 3
} // end of local scope OR end of main()       
```

## Optimization

<span style="color:orange"><b>The point:</b></span>

* store encountered prefix sum in a hash map
* we can then in constant time check if ``curr_prefix_sum - 1`` was encountered

**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) because we iterate through each value 
* O(n) because of space taken by the hash map

**About Rust :**
* `count += *prefix_sum_map.get(&(my_key)).unwrap_or(&0);` 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

fn k_sum_subarrays_optimized(nums : &[i32], k:i32) -> usize {
    // Initialize the hash map with 0 to handle subarrays
    // that sum to `k` from the start of the array
    let mut prefix_sum_map = HashMap::new();
    prefix_sum_map.insert(0, 1);

    // Populate prefix sum array, setting its first element to 0
    // let mut prefix_sums = Vec::with_capacity(nums.len()+1);
    // prefix_sums.push[0];
    let mut curr_prefix_sum = 0;
    let mut count = 0;
    for &num in nums {
        // update running prefix by adding current number
        curr_prefix_sum += num;

        // If a subarray with sum `k` exists, 
        // increment `count` by the number of times it has been found
        count += *prefix_sum_map.get(&(curr_prefix_sum - k)).unwrap_or(&0);

        // Udate the frequency of `curr_prefix_sum` in the hash map
        let freq = *prefix_sum_map.get(&curr_prefix_sum).unwrap_or(&0);
        prefix_sum_map.insert(curr_prefix_sum, freq+1);
    }
    count
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    println!("{:?}", k_sum_subarrays_optimized(&vec![1, 2, -1, 1, 2], 3));  // 3
    println!("{:?}", k_sum_subarrays_optimized(&[1, 2, -1, 1, 2], 3));      // 3
} // end of local scope OR end of main()       
```
