---
# published: false
layout: default
title: "bonus016 - Longest Chain of Consecutive Numbers"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Longest Chain of Consecutive Numbers

* Find the longest chain of consecutive numbers in an array.
* Two numbers are consecutive if they have a difference of 1.
* I didn't realize that sorting was an option.

Brute force 1:
- We sort the array O(nlog(n)) and iterate through it.

Brute force 2:
- We can consider each value as the start of a sequence.
- This leads to an O(n^3) solution, which is even worse.
- O(n^3) because there is a for loop, a while loop, and inside the while loop, we perform "while (current_num + 1) in nums" in O(n).




<span style="color:orange"><b>The point:</b></span>    

- Optimized approach:
- Instead of performing a linear search, we use a hash set to achieve O(1) lookups.
- We eliminate false starts by skipping values where v-1 exists in the hash set.
- This reduces the complexity from O(n^3) to O(n).


**Complexity Analysis :**

| Time | Space |
|------|-------|
| O(n) | O(n)  |

- Even though there are two loops, the complexity is O(n) because the inner loop runs only on the start of sequences.
- Therefore, the combined number of iterations for both loops is O(n).
- The for loop runs n times, and the while loop runs n times.
- This results in O(n + n) = O(n).
- Space complexity is O(n) because we use a hash set to store the values.






**About Rust :**
* `let num_set: HashSet<i32> = nums.iter().cloned().collect();`
    * `.iter()` provides immutable references to the elements ``&T`` 
    * `nums` remains available afterwards (not consumed)
    * we must transform references into i32 => ``.cloned()``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
use std::collections::HashSet;

fn longest_chain_of_consecutive_numbers(nums: &[i32]) -> i32 {

    if nums.is_empty() {
        return 0;
    }

    let num_set: HashSet<i32> = nums.iter().cloned().collect();
    let mut longest_chain = 0;
    
    for &num in &num_set{
         // If the current number is the smallest in its chain, search for the length of the chain
        if !num_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_chain = 1;
            while num_set.contains(&(current_num + 1)){
                current_num += 1;
                current_chain += 1;
            }
            longest_chain = longest_chain.max(current_chain);
        }
    }
    longest_chain
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", longest_chain_of_consecutive_numbers(&vec![1, 6, 2, 5, 8, 7, 10, 3]));  // 4
    println!("{:?}", longest_chain_of_consecutive_numbers(&[1, 6, 2, 5, 8, 7, 10, 3]));  // 4
// }

```

    4
    4

