---
# published: false
layout: default
title: "p024 - Largest Container"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Largest Container

<div align="center">
<img src="../assets/chap_01.webp" alt="" width="300" loading="lazy"/>
</div>

* You are given an array of numbers
* Each representing the height of a vertical line
* Return the amount of water which the largest container can hold

<span style="color:orange"><b>The point:</b></span>    
* Use two pointers (start, end).
* We move inward, calculating the area
* We advance the smaller of the two pointers
* Continue until the pointers meet


**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(1)  |

* Because we traverse the n-character string once and allocate a constant number of variables.





**About Rust :**
* `max_water = max_water.max(water);`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->







```rust
fn largest_container(heights:&[i32]) -> i32 {
    let mut max_water = 0;
    let (mut left, mut right) = (0, heights.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative

    while left < right {
        // calculate the water contained between the 2 lines
        let water = heights[left].min(heights[right]) * (right - left) as i32;
        max_water = max_water.max(water);
        // move the pointer with the shorter line inward
        // move both inward in case of equality
        if heights[left] < heights[right] {
            left += 1;
        }
        else if heights[left] > heights[right] {
            right -= 1;
        }
        else{
            left += 1;
            right -= 1;
        }
    }
    max_water
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{}", largest_container(&vec![2, 7, 8, 3, 7, 6]));  // 24
    println!("{}", largest_container(&[1, 8, 6, 2, 5, 4, 8, 3, 7]));  // 49
}
```

    24
    49

