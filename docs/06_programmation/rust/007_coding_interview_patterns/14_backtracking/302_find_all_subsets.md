---
# published: false
layout: default
title: "p302 - Find All Subsets"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Find All Subsets

* Return all possible subset of a given set of i32
* Each subset can be ordered
* Subset can be returns in any order

<span style="color:orange"><b>The point:</b></span>

* Starting with [], each subset is created by including/excluding each number of the input
* backtracking

**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n x 2^n)  | O(n)  |

* O(n x 2^n) in time because the state space has a depth of n (and 2 branches each time). For each of the 2^n subsets we add a copy of each in O(n) => O(n x 2^n) 
* O(n) in space because n is the max depth of the recursion tree. We keep track of curr_subset data structure in O(n). The result array is not taken into account.



**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)






<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn backtrack(i: usize, curr_subset: &mut Vec<i32>, nums: &[i32], res: &mut Vec<Vec<i32>>) {
    // base case : if all elements have been considered, add the current subset to res
    if i == nums.len() {
        res.push(curr_subset.clone()); // cannot push curr_subset (it is used afterward) so we push a clone of it
        return;
    }
    // include the current element and recursively explore all paths that branch from this subset
    curr_subset.push(nums[i]);
    backtrack(i + 1, curr_subset, nums, res);
    // exclude the current element and recursively  explore all paths that branch from this subset
    curr_subset.pop();
    backtrack(i + 1, curr_subset, nums, res);
}

fn find_all_subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    backtrack(0, &mut vec![], nums, &mut res);
    res
}

// no main() if this code runs in a Jupyter cell
fn main() {
    let nums = vec![4, 5, 6];
    let subsets = find_all_subsets(&nums);
    for subset in subsets {
        print!("{:?} - ", subset); // [4, 5, 6] - [4, 5] - [4, 6] - [4] - [5, 6] - [5] - [6] - [] - 
    }
} // end of local scope OR end of main()

```
