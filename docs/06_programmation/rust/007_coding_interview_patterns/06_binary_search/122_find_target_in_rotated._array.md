---
# published: false
layout: default
title: "p122 - Find the Target in a Rotated Sorted Array"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Find the Target in a Rotated Sorted Array

<div align="center">
<img src="../assets/chap_06.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a rotated sorted array of unique numbers, return the index of a target value (-1 otherwise)
* A rotated sorted array is an ascending sorted array where a portion slide from the front to the back
* [1, 2, 3, 4, 5] => [3, 4, 5, 1, 2]

<span style="color:orange"><b>The point:</b></span>

* Use the sorted subarray [left, mid] or [mid, right] to determine where to target is


Brute force takes linear time
We know the array is sorted and rotated

**Checklist**

* **1 - Sorted Search space**
    * [0, n-1]
* **2 - Narrow search space**
    * p 124
    * [left:mid] is sorted
        1. if the target is in [left, mid[ then right = mid - 1
        1. otherwise (if target is not in [left, mid[) then left = mid + 1
    * [mid:right] is sorted
        1. if the target is in ]mid, right] then left = mid + 1
        1. otherwise (if target is not in ]mid, left]) then right = mid -1
* **3 - Choose an exit condition for the while loop**
* **4 - Return the correct value**


**Complexity :**

| Time | Space |
|------|-------|
| O(log(n)) | O(1)  |

* O(log(n)) because the search space is of size n
* O(1) because in place 





**About Rust :**
* `find_the_target_in_rotated_sorted_array` returns an ``Option<usize>``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn find_the_target_in_rotated_sorted_array(nums:&[i32], target:i32) -> Option<usize> {
    
    let (mut left, mut right) = (0, nums.len() - 1);
    
    while left < right {
        let mid = left + (right - left) / 2 ;
        if nums[mid] == target{
            return Some(mid);
        }
        // if the left subarray is sorted check if the target falls in this range
        // yes => search in the left subarray
        // no => search in the right subarray
        else if nums[left]<=nums[mid]{
            if nums[left] <= target && target < nums[mid]{
                right = mid -1; 
            }else{
                left = mid + 1;
            }
        // if the right subarray is sorted check if the target falls in this range
        // yes => search in the right subarray   
        // no => search in the left subarray
        }else if nums[mid] < target && target <= nums[right]{
            left = mid + 1;
        }else{
            right = mid - 1;
        }
    }
    if nums[left] == target{Some(left)}else{None}
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", find_the_target_in_rotated_sorted_array(&[8, 9, 1, 2, 3, 4, 5, 6, 7], 1));  // Some(2)
    println!("{:?}", find_the_target_in_rotated_sorted_array(&[8, 9, 1, 2, 3, 4, 5, 6, 7], 0));  // None
    println!("{:?}", find_the_target_in_rotated_sorted_array(&[8, 9, 1, 2, 3, 4, 5, 6, 7], 42)); // None
} // end of local scope OR end of main()       

```

    Some(2)
    None
    None

