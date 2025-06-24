---
# published: false
layout: default
title: "bonus080 - Sort K-Sorted Array"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Sort K-Sorted Array

* Given an i32 array where each element is at most ``k`` positions away from its sorted position, sort the array in a non-decreasing order.

<span style="color:orange"><b>The point:</b></span>

* The input is partially sorted (k-sorted)
* Think backward
* For any index i, the element that belongs at index i in the sorted array is located within the range [i-k, i+k]
* It is the smallest value available in [i, i+k]

**Complexity :**

| Time               | Space |
|--------------------|-------|
| O(n log(k))               | O(k)  |

* O(log(n)) because 
    * We perform heapify on a min_heap of size k+1 in O(k)
    * push and pop operations on approximately n-k values using the heap. Since the heap can grow up to k+1 each push and pop operation takes O(log(k)). Therefore, this loop takes O(nlog(k)) in the worst case.
    * The final while-loop where we pop k+1 values runs in O(k log(k))
    * Overall time complexity = O(k) + O(n log(k)) + O ( k log(k)) = O (n log(k))
        * Since k is upper-bounded by n in each operation above. This is because the heap can only ever contain at most n values. we push object to the heap in O(log(n)). 
* O(k) because the heap can grow up to k+1

**About Rust :**
* min-heap => Reverse
* `nums.iter().take(k + 1).map(|&x| Reverse(x)).collect::<Vec<_>>()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




## V1
* First implementation


```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_a_k_sorted_array(nums:&mut[i32], k:usize) -> Vec<i32> {
    // Populate a min-heap with the first k + 1 (see `.take(k + 1)`) values in 'nums'.
    let mut min_heap = BinaryHeap::from(
        nums.iter().take(k + 1).map(|&x| Reverse(x)).collect::<Vec<_>>() // min_heap (must be "reversed" element by element)
    );
    
    // Replace elements in the array with the minimum from the heap at each iteration.
    let mut insert_index = 0;
    for i in k+1 ..nums.len() {
        if let Some(Reverse(top)) = min_heap.pop(){ 
            nums[insert_index] = top;
            insert_index += 1;
            min_heap.push(Reverse(nums[i]));
        }
    }
    
    while !min_heap.is_empty(){
        // Pop the remaining elements from the heap to finish sorting the array.
        if let Some(Reverse(top)) = min_heap.pop(){ 
            nums[insert_index] = top;
            insert_index +=1;
        }
    }
    nums.to_vec()
}

fn main(){   // no main() if this code runs in a Jupyter cell 

    println!("{:?}", sort_a_k_sorted_array(&mut[5, 1, 9, 4, 7, 10], 2)); //[1, 4, 5, 7, 9, 10]
    
} // end of local scope OR end of main()       
```

## V2
* No longer work in place
* Create a `result` vector to be returned
    * No `mut` when calling the function
* Use a slice in the ``for`` loop
    * `for &num in &nums[k + 1..] {`


```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_a_k_sorted_array(nums: &[i32], k: usize) -> Vec<i32> {
    // Populate a min-heap with the first k + 1 values in 'nums'.
    let mut min_heap = BinaryHeap::from(
        // min_heap (must be "reversed" element by element)
        nums.iter().take(k + 1).map(|&x| Reverse(x)).collect::<Vec<_>>() 
    );

    // Replace elements in the array with the minimum from the heap at each iteration.
    let mut result = Vec::with_capacity(nums.len());
    for &num in &nums[k + 1..] {
        if let Some(Reverse(top)) = min_heap.pop() {
            result.push(top);
        }
        min_heap.push(Reverse(num));
    }

    while let Some(Reverse(top)) = min_heap.pop() {
        // Pop the remaining elements from the heap to finish sorting the array.
        result.push(top);
    }
    result
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    println!("{:?}", sort_a_k_sorted_array(&[5, 1, 9, 4, 7, 10], 2)); //[1, 4, 5, 7, 9, 10]
} // end of local scope OR end of main()       
```

## V3
* In the `for` loop
    * Replace `if let` with `result.push(min_heap.pop().unwrap().0);`
* <span style="color:lime"><b>Preferred solution?</b></span> 


```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_a_k_sorted_array(nums: &[i32], k: usize) -> Vec<i32> {
    
    // Populate a min-heap with the first k + 1 values from nums
    let mut min_heap = BinaryHeap::from(
        // min_heap (must be "reversed" element by element)
        nums.iter().take(k + 1).map(|&x| Reverse(x)).collect::<Vec<_>>()
    );
    
    // Replace elements in the array with the minimum from the heap at each iteration.
    let mut result = Vec::with_capacity(nums.len());
    for &num in nums.iter().skip(k + 1) {
        result.push(min_heap.pop().unwrap().0);
        min_heap.push(Reverse(num));
    }

    while let Some(Reverse(top)) = min_heap.pop() {
        // Pop the remaining elements from the heap to finish sorting the array.
        result.push(top);
    }

    result
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    println!("{:?}", sort_a_k_sorted_array(&[5, 1, 9, 4, 7, 10], 2)); //[1, 4, 5, 7, 9, 10]
} // end of local scope OR end of main()       
```
