---
# published: false
layout: default
lang: en-US
title: "bonus073 - Maximums of Sliding Window"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Maximums of Sliding Window

<div align="center">
<img src="../assets/chap_07.webp" alt="" width="300" loading="lazy"/>
</div>

* A sliding window of size ``k`` slides through an integer array from left to right. 
* Create a new array that records the largest number found in each window as it slides through.
    * Input: nums = [3, 2, 4, 1, 2, 1, 1], k = 4
    * Output: [4, 4, 4, 2]

<span style="color:orange"><b>The point:</b></span>

* A stack to push values onto during each enqueue call (enqueue_stack).
* A stack to pop values from during each dequeue call (dequeue_stack).

**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(n)  |

* O(n) because we slide over the array in linear time. values of ``nums`` are pushed and poped in the deque at most once
for each element. Stack operations are in O(1)
* O(k) because the deque can store up to ``k`` elements (size of returned ``res`` in NOT taken into account) 

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
use std::collections::VecDeque;

pub fn maximums_of_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    
    let mut res = Vec::new();
    if k == 0 {
        return res; // Edge case: empty window size, no max possible
    }
    
    let mut dq: VecDeque<(i32, usize)> = VecDeque::new(); // (value, index)
    let mut left = 0;

    for (right, &num) in nums.iter().enumerate() {
        // Ensure the values of the deque maintain a monotonic decreasing order
        // by removing candidates ≤ the current candidate.
        while let Some(&(val, _)) = dq.back() {
            if val <= num {
                dq.pop_back();
            } else {
                break;
            }
        }

        // Add the current candidate.
        dq.push_back((num, right));

        // If the window is of length 'k', record the maximum of the window.
        if right >= k - 1 {
            // Remove elements from the front if they are outside the window.
            if let Some(&(_, idx)) = dq.front() {
                if idx < left {
                    dq.pop_front();
                }
            }

            // The maximum of the current window is at the front of the deque.
            if let Some(&(max_val, _)) = dq.front() {
                res.push(max_val);
            }

            // Advance the left bound of the window.
            left += 1;
        }
    }

    res
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", maximums_of_sliding_window(&[3, 2, 4, 1, 2, 1, 1], 4)); // [4, 4, 4, 2]
    println!("{:?}", maximums_of_sliding_window(&[3, 2, 4], 2));             // [3, 4]
    println!("{:?}", maximums_of_sliding_window(&[], 4));                    // []
    println!("{:?}", maximums_of_sliding_window(&[3, 2, 4, 1, 2, 1, 1], 0)); // []
} // end of local scope OR end of main()       
```

## Unit tests work in Rust Playground
* NOT in Jupyter cell


```rust
use std::collections::VecDeque;

pub fn maximums_of_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    
    let mut res = Vec::new();
    if k == 0 {
        return res; // Edge case: empty window size, no max possible
    }
    
    let mut dq: VecDeque<(i32, usize)> = VecDeque::new(); // (value, index)
    let mut left = 0;

    for (right, &num) in nums.iter().enumerate() {
        // Ensure the values of the deque maintain a monotonic decreasing order
        // by removing candidates ≤ the current candidate.
        while let Some(&(val, _)) = dq.back() {
            if val <= num {
                dq.pop_back();
            } else {
                break;
            }
        }

        // Add the current candidate.
        dq.push_back((num, right));

        // If the window is of length 'k', record the maximum of the window.
        if right >= k - 1 {
            // Remove elements from the front if they are outside the window.
            if let Some(&(_, idx)) = dq.front() {
                if idx < left {
                    dq.pop_front();
                }
            }

            // The maximum of the current window is at the front of the deque.
            if let Some(&(max_val, _)) = dq.front() {
                res.push(max_val);
            }

            // Advance the left bound of the window.
            left += 1;
        }
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let nums = vec![3, 2, 4, 1, 2, 1, 1];
        let result = maximums_of_sliding_window(&nums, 4);
        assert_eq!(result, vec![4, 4, 4, 2]);

        let nums2 = vec![3, 2, 4];
        let result2 = maximums_of_sliding_window(&nums2, 2);
        assert_eq!(result2, vec![3, 4]);
    }

    #[test]
    fn test_empty_input() {
        let nums: Vec<i32> = vec![];
        let result = maximums_of_sliding_window(&nums, 4);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_zero_window() {
        let nums = vec![3, 2, 4, 1];
        let result = maximums_of_sliding_window(&nums, 0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_window_larger_than_input() {
        let nums = vec![1, 2];
        let result = maximums_of_sliding_window(&nums, 5);
        assert_eq!(result, vec![]); // No complete window can be formed
    }

    #[test]
    fn test_exact_window_size() {
        let nums = vec![5, 1, 3, 7];
        let result = maximums_of_sliding_window(&nums, 4);
        assert_eq!(result, vec![7]);
    }
}

```
