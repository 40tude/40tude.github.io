---
# published: false
layout: default
title: "p014 - Triplet Sum"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-24 16:00:00
---

# Triplet Sum

<div align="center">
<img src="../assets/rust_logo.webp" alt="" width="225" loading="lazy"/>
</div>


* Given an array of int, return all triplets such that a+b+c =0
* 1 2 3 and 2 3 1 are considered duplicates

<span style="color:orange"><b>The point:</b></span>    
* Sort the array
* b + c = -a for all values of a


**Complexity :**

| Time | Space |
|------|-------|
| O(n²) | O(n)  |

* We sort the array => O(nlogn)
* For each element in the array, we call pair_sum_sorted_all_pairs(), which runs in O(n)
* Total complexity: O(nlogn) + O(n²) = O(n²)
* O(n) space complexity due to sorting algorithm
* It seems we do not consider space used to store the triplets (not sure I understood why)

Here, log is indeed base-2 logarithm
* log2(128)     = 7
* log2(256)     = 8
* log2(512)     = 9
* log2(1024)    = 10
* log2(1MB)     = 20




**About Rust :**
* ``pairs.push(vec![nums[left], nums[right]]);`` to push a vec in a vec
* Custom types ``type Pair = (i32, i32);`` and ``type Triplet = (i32, i32, i32);``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
// pair_sum_sorted_all_pairs() receives a slice (read only, no copy)
fn pair_sum_sorted_all_pairs(nums: &[i32], start: usize, target: i32) -> Vec<Vec<i32>> {
    let mut pairs = Vec::new();
    let (mut left, mut right) = (start, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            pairs.push(vec![nums[left], nums[right]]);
            left += 1;
            // The list is sorted
            // To avoid duplicate, skip "b" if it is same as previous number
            while left < right && nums[left] == nums[left - 1] {
                left += 1;
            }
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    pairs
}

// nums must be mutable because of nums.sort(); 
// nums is a "copy" that we can modify in the function
// However, since Vec does not implement the Copy trait, the argument, the vector is moved 
// It is no longer available in main() once the call to triplet_sum() happens
// Here, in main() after the call to triplet_sum() I don't use the argument so it is OK to "give" it to the function 
fn triplet_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut triplets = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        // Triplet will never sum to 0
        if nums[i] > 0 {
            break;
        }
        // The list is sorted
        // To avoid duplicate, skip "a" if it is same as previous number
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        // Find pair with sum = -a (meaning -num[i])
        let pairs = pair_sum_sorted_all_pairs(&nums, i + 1, -nums[i]);
        // There is no "+" overload for Vec<T> in Rust
        for pair in pairs {
            let mut triplet = vec![nums[i]]; // Create a vector with 1 element
            triplet.extend(pair);   // add the 2 elements of the tuple named pair to triplet
                                    // how is it possible? pair is a tuple (i32, i32), it implements IntoIterator.
                                    // we can use it in extend(...), which accepts any IntoIterator<Item = T>.
            triplets.push(triplet); // add it to the list of triplets (with an s)
        }
    }
    triplets
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    let res_list = triplet_sum(vec![0, -1, 2, -3, 1]); // [[-3, 1, 2], [-1, 0, 1]]
    println!("{:?}", res_list);

    let res_list = triplet_sum(vec![0, 0, 1, -1, 1, -1]); // [[-1, 0, 1]]
    println!("{:?}", res_list);
// }


```

    [[-3, 1, 2], [-1, 0, 1]]
    [[-1, 0, 1]]


## Use custom types

**About Rust :**
* <span style="color:lime"><b>Preferred solution?</b></span>
* Pair and Triplet
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
type Pair = (i32, i32);
type Triplet = (i32, i32, i32);

// pair_sum_sorted_all_pairs() receives a slice (read only, no copy)
// Return a vector of Pair
fn pair_sum_sorted_all_pairs(nums: &[i32], start: usize, target: i32) -> Vec<Pair> {
    let mut pairs = Vec::new();
    let (mut left, mut right) = (start, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            pairs.push((nums[left], nums[right]));
            left += 1;
            // The list is sorted
            // To avoid duplicate, skip "b" if it is same as previous number
            while left < right && nums[left] == nums[left - 1] {
                left += 1;
            }
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    pairs
}

// Return de vector of Triplet
fn triplet_sum(mut nums: Vec<i32>) -> Vec<Triplet> {
    let mut triplets = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        if nums[i] > 0 {
            break;
        }
        // The list is sorted
        // To avoid duplicate, skip "a" if it is same as previous number
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        // Find pair with sum = -a (meaning -num[i])
        let pairs = pair_sum_sorted_all_pairs(&nums, i + 1, -nums[i]);
        // Rust allows direct unpacking of tuples in a loop
        // (b, c) successively takes the values of each (i32, i32) of the pairs vector.
        for (b, c) in pairs {
            triplets.push((nums[i], b, c));
        }
    }
    triplets
}

// fn main() {
    let res_list = triplet_sum(vec![0, -1, 2, -3, 1]);
    println!("{:?}", res_list);

    let res_list = triplet_sum(vec![0, 0, 1, -1, 1, -1]);
    println!("{:?}", res_list);
// }

```

    [(-3, 1, 2), (-1, 0, 1)]
    [(-1, 0, 1)]


## What if we really need to pass a reference ?
* to ``triplet_sum()``
* Cases where arrays/vectors are huge
* We need to make sure the caller knows that **the array is modified** (sorted) once it comes back from triplet_sum()
    * This is what happens in the Python (check the book)
* This may be an issue in a concurrent context


```rust
type Pair = (i32, i32);
type Triplet = (i32, i32, i32);

// pair_sum_sorted_all_pairs() receives a slice (read only, no copy)
// return a vector of Pairs
fn pair_sum_sorted_all_pairs(nums: &[i32], start: usize, target: i32) -> Vec<Pair> {
    let mut pairs = Vec::new();
    let (mut left, mut right) = (start, nums.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative
    
    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            pairs.push((nums[left], nums[right]));
            left += 1;
            // The list is sorted
            // To avoid duplicate, skip "b" if it is same as previous number
            while left < right && nums[left] == nums[left - 1] {
                left += 1;
            }
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    pairs
}

// We need to pass a &mut Vec<T> because of the sort
// Return de vector of Triplet
fn triplet_sum(nums: &mut Vec<i32>) -> Vec<Triplet> {
    let mut triplets = Vec::new();
    nums.sort();

    for i in 0..nums.len() {
        if nums[i] > 0 {
            break;
        }
        // The list is sorted
        // To avoid duplicate, skip "a" if it is same as previous number
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        // Find pair with sum = -a (meaning -num[i])
        let pairs = pair_sum_sorted_all_pairs(nums, i + 1, -nums[i]);
        // Rust allows direct unpacking of tuples in a loop
        // (b, c) successively takes the values of each (i32, i32) of the pairs vector.
        for (b, c) in pairs {
            triplets.push((nums[i], b, c));
        }
    }
    triplets
}

fn main() {
    let mut my_vec1 = vec![0, -1, 2, -3, 1];
    let res_list = triplet_sum(&mut my_vec1);
    println!("{:?}", res_list); // [(-3, 1, 2), (-1, 0, 1)]
    println!("{:?}\n", my_vec1); // my_vec1 is still available but it has been modified [-3, -1, 0, 1, 2]

    let mut my_vec2 = vec![0, 0, 1, -1, 1, -1];
    let res_list = triplet_sum(&mut my_vec2);
    println!("{:?}", res_list); // [(-1, 0, 1)]
    println!("{:?}", my_vec2);  // [-1, -1, 0, 0, 1, 1]
}






```

    [(-3, 1, 2), (-1, 0, 1)]
    [-3, -1, 0, 1, 2]
    
    [(-1, 0, 1)]
    [-1, -1, 0, 0, 1, 1]

