---
# published: false
layout: default
title: "bonus039 - Find the Median From Two Sorted Arrays"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Find the Median From Two Sorted Arrays

* Given two sorted integer arrays, find their median value as if they were merged into a single sorted sequence 
    * once merge they would be sorted

<span style="color:orange"><b>The point:</b></span>

* Use the sorted subarray [left, mid] or [mid, right] to determine where to target is


Brute force
* merge arrays then find the median => O((m+n)*log(m+n))
* O(m+n) is the merge is sorted



<!-- **Checklist**

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
 -->



**Complexity :**

| Time | Space |
|------|-------|
| O(log(min(m,n))) | O(1)  |

* O(log(min(m,n))) because the binary search on the smaller array
* O(1) because in place



**About Rust :**
* `let (mut left, mut right) = (0, m);`
* `let l2_index = half_total_len.saturating_sub(l1_index + 1).saturating_sub(1); `
* `let l1 = nums1.get(l1_index).map_or(f64::NEG_INFINITY, |&x| x as f64);`
* <span style="color:lime"><b>Preferred solution?</b></span> 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn find_the_median_from_two_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    // Optimization: ensure 'nums1' is the smaller array.
    let (nums1, nums2) = if nums2.len() < nums1.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };

    let (m, n) = (nums1.len(), nums2.len());
    let half_total_len = (m + n) / 2;
    // m and NOT m-1 mostly because right is usize and cannot be negative if m==0
    let (mut left, mut right) = (0, m); 

    loop {
        let l1_index = (left + right) / 2;
        // usize cannot be negative. Without saturating_sub, an underflow would cause a panic.
        // saturating_sub returns 0 if the result is negative, thus avoiding crashes.
        // let l2_index = half_total_len - (l1_index + 1) - 1;
        let l2_index = half_total_len.saturating_sub(l1_index + 1).saturating_sub(1); 
        
        // Get values with bounds checking
        // .get() return an Option (Some(&value))
        // .map_or(default, closure) => return f64::NEG_INFINITY if None or apply closure if Some(x) 
        let l1 = nums1.get(l1_index).map_or(f64::NEG_INFINITY, |&x| x as f64);
        let r1 = nums1.get(l1_index + 1).map_or(f64::INFINITY, |&x| x as f64);
        
        let l2 = nums2.get(l2_index).map_or(f64::NEG_INFINITY, |&x| x as f64);
        let r2 = nums2.get(l2_index + 1).map_or(f64::INFINITY, |&x| x as f64);
        
        if l1 > r2 {
            right = l1_index;
        } else if l2 > r1 {
            left = l1_index + 1;
        } else {
            return if (m + n) % 2 == 0 {
                (l1.max(l2) + r1.min(r2)) / 2.0
            } else {
                r1.min(r2)
            };
        }
    }    
}

// fn main() {
    println!("{:?}", find_the_median_from_two_sorted_arrays(&[1, 3, 7], &[]));  
    println!("{:?}", find_the_median_from_two_sorted_arrays(&[1, 3, 7], &[0, 2, 5, 6, 8]));  // 4.0
    println!("{:?}", find_the_median_from_two_sorted_arrays(&[0, 2, 5, 6, 8], &[1, 3, 7]));  // 4.0
    println!("{:?}", find_the_median_from_two_sorted_arrays(&[0, 2, 5, 6, 8], &[1, 3, 7, 9]));  // 5.0
// }
```

    3.0
    4.0
    4.0
    5.0


## First implementation

**About Rust :**
* The code panic if one of the slice is empty
* The code generate a warning under Rust Playground because of the `l1_index < 0` make no sense since `l1_index` `is usize` 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn find_the_median_from_two_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64{

    // Optimization: ensure 'nums1' is the smaller array.
    let (nums1, nums2) = if nums2.len() < nums1.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };

    let (m, n) = (nums1.len(), nums2.len());
    let half_total_len = (m + n) / 2;
    let (mut left, mut right) = (0, m - 1);

    // A median always exists in a non-empty array, so continue binary search until it’s found.
    loop{
        let l1_index = (left + right) / 2;
        let l2_index = half_total_len - (l1_index + 1) - 1;
        
        // Set to -infinity or +infinity if out of bounds.
        let l1 = if l1_index < 0 { f64::NEG_INFINITY } else { nums1[l1_index] as f64};
        let r1 = if l1_index >= m - 1 { f64::INFINITY } else { nums1[l1_index + 1] as f64};
        
        let l2 = if l2_index < 0 { f64::NEG_INFINITY } else { nums2[l2_index] as f64};
        let r2 = if l1_index >= n - 1 { f64::INFINITY } else { nums2[l2_index + 1] as f64};
        
        
        if l1 > r2{ // If 'l1 > r2', then 'l1' is too far to the right. Narrow the search space toward the left.
            right = l1_index - 1;
        } else if l2 > r1{ // If 'l2 > r1', then 'r1' is too far to the left. Narrow the search space toward the right.
            left = l1_index + 1;
        } else{ // If both 'l1' and 'l2' are less than or equal to both 'r1' and 'r2', we found the correct slice.
            if (m + n) % 2 == 0 {
                return (l1.max(l2) + r1.min(r2)) / 2.0;
            } else {
                return r1.min(r2);
            }
        }
    }    
}


// fn main(){     // no main() if this code runs in a Jupyter cell 
    // println!("{:?}", find_the_median_from_two_sorted_arrays(&[1, 3, 7], &[]));  
    println!("{:?}", find_the_median_from_two_sorted_arrays(&[1, 3, 7], &[0, 2, 5, 6, 8]));  // 4.0
    println!("{:?}", find_the_median_from_two_sorted_arrays(&[0, 2, 5, 6, 8], &[1, 3, 7]));  // 4.0
    println!("{:?}", find_the_median_from_two_sorted_arrays(&[0, 2, 5, 6, 8], &[1, 3, 7, 9]));  // 5.0
// } // end of local scope OR end of main()       
            
```

    4.0
    4.0
    5.0





    ()


