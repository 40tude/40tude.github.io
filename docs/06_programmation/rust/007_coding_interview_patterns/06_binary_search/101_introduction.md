---
# published: false
layout: default
title: "p101 - Introduction"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Introduction

<div align="center">
<img src="../assets/chap_06.webp" alt="" width="300" loading="lazy"/>
</div>

If sorted data set => binary tree
1. Define search space
    * sorted
    * set left and right ptr
1. Narrowing the search space
    * move ptr inward until only one element remains
    * moves are based on the **value @ mid point** (1/2 of search space)
    * if the searched value is on the right of mid, move left ptr to the right (resp right ptr to the left)
        1. left = mid+1 if value @ mid **cannot** be the searched value (resp. right = mid-1)
        1. left = mid   if value @ mid **could** be the searched value (resp. right = mid)
    * mid = left + (right-left)//2 (avoid overflow)
1. Choose an exit condition for the **while loop**
    * ``while left <  right`` ends when ``left = right`` (right=left=searched value)
    * ``while left <= right`` ends when ``left > right`` (left surpassed right) 
1. Return the correct value
    * return either value @ left or right

## Checklist 
* 1 - **Sorted search space**
* 2 - **Narrowing search space**
* 3 - **Choose an exit condition for the while loop**
* 4 - **Return the correct value**


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

