---
# published: false
layout: default
title: "bonus052 - Local Maxima"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Local Maxima

<div align="center">
<img src="../assets/chap_06.webp" alt="" width="300" loading="lazy"/>
</div>

* A local maxima is a value greater than both its immediate neighbors. 
* Return any local maxima in an array. 
* You may assume that an element is always considered to be strictly greater than a neighbor that is outside the array.

<span style="color:orange"><b>The point:</b></span>

* if the number (at index i + 1) is greater than the current, there’s definitely a local maxima somewhere to the right of i.
* if the number (at index i + 1) is lower   than the current, i can be a local maxima or the local maxime is on the left of i
* we can then narrow the search toward the direction of the maxima

Brute force linearly search for a local maxima by iteratively comparing each value to its neighbors and returning the first local maxima we find. 
Since we can return any maxima, there’s likely a more efficient approach.
There is no adjacent duplicate => always contains at least one local max

**Checklist**
* **1 - Sorted Search space**
    * [0, n-1]
* **2 - Narrow search space**
    * p 54
    * evaluate val @ mid compare with val @ mid+1
    * update right (or left)
* **3 - Choose an exit condition for the while loop**
    * left < right
* **4 - Return the correct value** 
    * return left (or right)


**Complexity :**

| Time | Space |
|------|-------|
| O(log(n)) | O(1)  |

* O(log(n)) because the search space is of size n
* O(1) because in place 

**About Rust :**
* returns `Option<usize>` because array argument may have no lenght
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn local_maxima_in_array(nums: &[i32]) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let mid = (left + right)/2;
        if nums[mid] > nums[mid + 1] {
            right = mid;
        }else{
            left = mid + 1;
        }
    }
    Some(left)
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", local_maxima_in_array(&[1, 4, 3, 2, 3])); // 1, 4 is also acceptable
    println!("{:?}", local_maxima_in_array(&[])); // None
    println!("{:?}", local_maxima_in_array(&[1, 1, 1])); // any index is acceptable
// }
```

    Some(1)
    None
    Some(2)

