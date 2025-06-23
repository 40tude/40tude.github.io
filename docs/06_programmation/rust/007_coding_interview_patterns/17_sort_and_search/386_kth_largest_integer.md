---
# published: false
layout: default
title: "p386 - Kth Largest Integer"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Kth Largest Integer

* Return the Kth largest integer in an array
* no duplicates
* at least one element in the array
* 1<=k<=n





## Min-Heap

<span style="color:orange"><b>The point:</b></span>

* Sort in reverse and return `array[k-1]`. Takes O(nlog(n))
* We need to order the k largest integers and select the min
* min-heap


**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n log(n))    | O(log(n)) |

* O(n log(n)) in time because it use merge sort. The split happens log2(n). Merging takes n operation => O(n log(n))
* O(log(n)) in space because of depth of the recursive call stack (can grow up to log2(n))





**About Rust :**
* Use a `BinaryHeap` which is a max-heap by default
* To mimic a min-heap, values are stored in reverse (`std::cmp::Reverse`) 
    * `else if num > min_heap.peek().unwrap().0`
        * Accesses the i32 integer stored in struct Reverse
        * Mandatory because Reverse is a struct tuple.
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->







```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn kth_largest_integer_min_heap(nums: &Vec<i32>, k: usize) -> i32 {
    // Create a min-heap using Reverse
    let mut min_heap = BinaryHeap::new();

    for &num in nums {
        // Ensure the heap has at least `k` i32
        if min_heap.len() < k {
            // Push Reverse(num) to simulate a min-heap
            min_heap.push(Reverse(num));
        } else if num > min_heap.peek().unwrap().0 {
            // If num is greater than the smallest integer in the heap, pop off the smallest value and push `num`
            min_heap.pop();
            min_heap.push(Reverse(num));
        }
    }

    // Return the kth largest element (unwrap the Reverse)
    min_heap.peek().unwrap().0
}

fn main() { // no main() if this code runs in a Jupyter cell
    let vals = vec![5, 2, 4, 3, 1, 6];
    println!("{:?}", kth_largest_integer_min_heap(&vals, 3)); // 4
} // end of local scope OR end of main()

```

## Quickselect

<span style="color:orange"><b>The point:</b></span>

* Quickselect leverage the parition step of quicksort
* It positions a value in its sorted position
* Instead of finding kth largest we will find the (n-k)th smallest i32
* Read again `379_sort_array.ipynb`

**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n)           | O(log(n))      |

* O(n) in time because on average the problem size is reduced by half during recursion. A linear partition is performed during the recursive calls => O(n) + O(n/2) + O(n/4)... => O(n)
    * Worst case : O(n²)
* O(log(n)) in space on average because of depth of the recursive call stack (can grow up to log2(n))
    * Worst case : O(n)










<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* First translation

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
// # Cargo.toml
// [dependencies]
// rand = "0.9.1"

// If in a Jupyter cell
// :dep rand = "0.9.1"

// No reference to rand needed in Rust playground

use rand::Rng;

fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = nums[right]; // The pivot is the value at the 'right' index
    let mut low = left; // 'low' starts at 'left'

    // Move all numbers less than pivot to the left
    for i in left..right {
        if nums[i] < pivot {
            nums.swap(low, i);
            low += 1;
        }
    }

    // After partition low is where the pivot should be => swap the pivot number with the number at low pointer
    nums.swap(low, right);
    low
}


fn quickselect(nums : &mut [i32], left : usize, right : usize, k : usize)->i32{
    let n = nums.len();
    
    // Base case : if subarray has 0 or 1 element it is sorted
    if left >= right {
        return nums[left];
    }

    // Choose a pivot at a random index
    let mut rng = rand::rng(); 
    let random_index = rng.random_range(left..=right); 

    // Swap the random pivot with rightmost element to position pivot on rightmost index
    nums.swap(random_index, right);

    let pivot_index = partition(nums, left, right);

    if pivot_index < n-k{
        quickselect(nums, pivot_index+1, right, k)
    }else if pivot_index > n-k{
        quickselect(nums, left, pivot_index.saturating_sub(1), k)
    }else{
        nums[pivot_index]
    }
}

fn kth_largest_integer_quickselect(nums : &mut [i32], k : usize) -> i32{
    quickselect(nums, 0, nums.len()-1, k)
}

fn main() { // no main() if this code runs in a Jupyter cell
    let mut vals = vec![5, 2, 4, 3, 1, 6];
    println!("{:?}", kth_largest_integer_quickselect(&mut vals, 3)); // 4
} // end of local scope OR end of main()
```

## V2

* See `17_sort_and_search\379_sort_array.ipynb`
* Random generator created once and passed has an argument

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
// # Cargo.toml
// [dependencies]
// rand = "0.9.1"

// If in a Jupyter cell
// :dep rand = "0.9.1"

// No reference to rand needed in Rust playground

use rand::Rng;

fn partition(nums: &mut [i32]) -> usize {
    let pivot = nums[nums.len() - 1];
    let mut low = 0;
    // Move all numbers less than pivot to the left (=> numbers greater than pivot are on the right)
    for i in 0..nums.len() - 1 {
        if nums[i] < pivot {
            nums.swap(low, i);
            low += 1;
        }
    }
    // After partition low is where the pivot should be => swap the pivot number with the number at low pointer
    nums.swap(low, nums.len() - 1);
    low
}

fn quickselect(nums : &mut [i32], rng: &mut impl Rng, k : usize)->i32{
    // Choose a pivot at a random index
    let random_index = rng.random_range(0..nums.len());

    // Swap the random pivot with rightmost element to position pivot on rightmost index
    nums.swap(random_index, nums.len() - 1);

    let mid = partition(nums);
    let target = nums.len() - k; // k-th largest is at len - k (0-based)

    if mid == target {
        nums[mid]
    } else if mid < target {
        // Search in the right part
        quickselect(&mut nums[mid + 1..], rng, k)
    } else {
        // Search in the left part
        quickselect(&mut nums[..mid], rng, k)
    }
}

fn kth_largest_integer_quickselect(nums : &mut [i32], k : usize) -> i32{
    let mut rng = rand::rng();
    quickselect(nums, &mut rng, k)
}

fn main() { // no main() if this code runs in a Jupyter cell
    let mut vals = vec![5, 2, 4, 3, 1, 6];
    println!("{:?}", kth_largest_integer_quickselect(&mut vals, 3)); // 4
} // end of local scope OR end of main()

```
