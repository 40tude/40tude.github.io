---
# published: false
layout: default
lang: en-US
title: "p116 - Cutting Wood"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Cutting Wood

<div align="center">
<img src="../assets/chap_06.webp" alt="" width="300" loading="lazy"/>
</div>

* Given an array representing the heights of trees and ``k`` an integer representing the lenght of wood to be cut
* H the height of the cutting machine which cut top part of the trees  
* Determine the highest H value to cut at least k m of wood


<span style="color:orange"><b>The point:</b></span>

* binary search problem where the search space **does not** encompass the input array
* p 118  
    1. we move form the space of trees heights to the space of heights of cutting tool (H) 
    1. this space is effectively sorted (True, False) so that we can use binary search
    1. we search for the upper bound where the condition is True
* search for upper bound => biased mid = 1 + (left+right)//2



**Checklist**

* **1 - Sorted search space**
    * H [0, height of tallest tree]

* **2 - Narrow search space**
    * if val @ mid < target => right = mid - 1
    * if val @ mid >= target => left = mid 
* **3 - Choose an exit condition for the while loop**
    * when left == right
* **4 - Return the correct value**
    * left or right


**Complexity :**

| Time | Space |
|------|-------|
| O(log(n)) | O(1)  |

* O(log(n)) because the search space is of size n
* O(1) because in place 





**About Rust :**
* `let max_height = heights.iter().max().copied().unwrap_or(0); `
* `heights.iter().max()` provide an `Option<&i32>` then `.copied()` converts `Option<&i32>` in `Option<i32>`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn cuts_enough_wood(heights:&[i32], k:i32, h:i32) -> bool{
    let mut wood_collected = 0;
    for height in heights{
        if *height > h {
            wood_collected += height - h;
        }
    }
    wood_collected >= k
}

fn upper_bound_binary_search(heights:&[i32], k:i32) -> i32{
    let max_height = heights.iter().max().copied().unwrap_or(0); 
    let (mut left, mut right) = (0, max_height);
    while left < right{
        let mid = 1 + left + (right - left) / 2 ;
        if cuts_enough_wood(heights, k, mid){
            left = mid;
        }else{
            right = mid - 1;
        }    
    }    
    right 
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", upper_bound_binary_search(&[2, 6, 3, 8], 7)); // 3
    println!("{:?}", upper_bound_binary_search(&[2, 6, 3, 8], 0)); // 8
    println!("{:?}", upper_bound_binary_search(&[2, 6, 3, 8], 1_000)); // 0
// } // end of local scope OR end of main()       

```

    3
    8
    0

