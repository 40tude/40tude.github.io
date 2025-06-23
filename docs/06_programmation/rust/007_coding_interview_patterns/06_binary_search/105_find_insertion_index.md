---
# published: false
layout: default
title: "p105 - Find insertion index"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Find insertion index

* Given a sorted array with unique values.
    * If the array contains the target retrun its index
    * Otherwise return the insrtion index(index where the target would be if it were inserted)

<span style="color:orange"><b>The point:</b></span>

* Combine both cases finding the first value >= the target
* Search the first value that match a condition. The condition is  ``the number is >= the target`` (see top p 106)

**Checklist**
* **1 - Sorted search space**
    * Z! [0, n] and NOT [0, n-1] because if the target is NOT in the array its insertion index is n
* **2 - Narrow search space**
    * p 108 
    * ``if num[mid] <  target => left = mid+1``
    * ``if num[mid] >= target => right = mid``
* **3 - Choose an exit condition for the while loop**
    * we exit when left = right => the condition is ``while left < right``
* **4 - Return the correct value**
    * left

**Complexity :**

| Time | Space |
|------|-------|
| O(log(n)) | O(1)  |

* O(log(n)) because the search space is of size n+1
* O(1) because in place 

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn find_insertion_index(nums:&[i32], target:i32) -> usize{
    let(mut left, mut right) = (0, nums.len());
    while left < right{
        let mid = left + (right - left)/2;
        if nums[mid] >= target {
            right = mid;
        } else{
            left = mid + 1;
        }
    }
    left
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{}", find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 4));  // 2
    println!("{}", find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 6));  // 4
    println!("{}", find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 0));  // 0
    println!("{}", find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 42)); // 7
// } // end of local scope OR end of main()       
            
```

    2
    4
    0
    7

