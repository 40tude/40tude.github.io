---
# published: false
layout: default
title: "bonus046 - Matrix Search"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Matrix Search

<div align="center">
<img src="../assets/chap_06.webp" alt="" width="300" loading="lazy"/>
</div>

* Determine if a target value exists in a matrix. 
* Each row of the matrix is sorted in non-decreasing order
* The first value of each row is greater than or equal to the last value of the previous row


<span style="color:orange"><b>The point:</b></span>
* all values in a given row are greater than or equal to all values in the previous row. 
* think about flatten C array in memory where cell(r, c) map to r*n + c
* i in flatten array => r = i//n c = I%n



Brute force does not take advantage of the sorted rows
Building a flatten ordered array take O(m x n) in time and O(m x n) in space 



**Checklist**
* **1 - Sorted Search space**
    * [0, m x n-1]
* **2 - Narrow search space**
    * p 48
    * mid = (left + right) // 2
    * r = mid//n c = mid%n
    * matrxi[r][c] < target => left  = mid + 1
    * matrxi[r][c] > target => right = mid - 1
* **3 - Choose an exit condition for the while loop**
    * while left <= right
* **4 - Return the correct value** 
    * true or false


**Complexity :**

| Time | Space |
|------|-------|
| O(log(m x n)) | O(1)  |

* O(log(m x n)) because the binary search space is of size mxn
* O(1) because in place 


**About Rust :**
* `fn matrix_search(matrix: &[Vec<i32>], target: i32)` rather than `fn matrix_search(matrix: &Vec<Vec<i32>>, target: i32)`
* `match matrix[r][c].cmp(&target)`
* <span style="color:lime"><b>Preferred solution?</b></span> 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn matrix_search(matrix: &[Vec<i32>], target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    
    let(m, n) = (matrix.len(), matrix[0].len());
    let (mut left, mut right) = (0, m * n - 1);
    
    while left <= right {
        let mid = left + (right - left) / 2;
        let (r, c) = (mid / n, mid % n);
        match matrix[r][c].cmp(&target) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => right = mid - 1,
            std::cmp::Ordering::Less => left = mid + 1,
        }
    }
    false
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", matrix_search(&[vec![2, 3, 4, 6], vec![7, 10, 11, 17], vec![20, 21, 24, 33]], 21)); // true 
    println!("{:?}", matrix_search(&[vec![2, 3, 4, 6], vec![7, 10, 11, 17], vec![20, 21, 24, 33]], 22)); // false
    println!("{:?}", matrix_search(&[vec![]], 1)); // false
    println!("{:?}", matrix_search(&[], 1)); // false
// }
```

    true
    false
    false
    false


## First implementation

**About Rust :**
* This code panic with empty matrix
* No need for `&vec![...`, ``&[`` is enough
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
fn matrix_search(matrix: &[Vec<i32>], target: i32) -> bool{
    let(m, n) = (matrix.len(), matrix[0].len());
    let (mut left, mut right) = (0, m * n - 1);
    while left <= right{
        let mid = left + (right - left) / 2;
        let (r, c) = (mid/n, mid%n);
        if matrix[r][c] == target{
            return true;
        }else if matrix[r][c] > target{
            right = mid - 1;
        }else{
            left = mid + 1;
        }
    }
    false
}
// fn main() {
    println!("{:?}", matrix_search(&vec![vec![2, 3, 4, 6], vec![7, 10, 11, 17], vec![20, 21, 24, 33]], 21)); // true 
    println!("{:?}", matrix_search(&vec![vec![2, 3, 4, 6], vec![7, 10, 11, 17], vec![20, 21, 24, 33]], 22)); // false  
    println!("{:?}", matrix_search(&[vec![]], 1)); // false
    println!("{:?}", matrix_search(&[], 1)); // false
// }
```

    
    thread '<unnamed>' panicked at src/lib.rs:4:37:
    attempt to subtract with overflow
    stack backtrace:
       0: rust_begin_unwind
                 at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/std/src/panicking.rs:692:5
       1: core::panicking::panic_fmt
                 at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/core/src/panicking.rs:75:14
       2: core::panicking::panic_const::panic_const_sub_overflow
                 at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/core/src/panicking.rs:178:21
       3: <unknown>
       4: <unknown>
       5: <unknown>
       6: evcxr::runtime::Runtime::run_loop
       7: evcxr::runtime::runtime_hook
       8: evcxr_jupyter::main
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


    true
    false

