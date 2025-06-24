---
# published: false
layout: default
title: "bonus141 - Dutch National Flag"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Dutch National Flag

* Given an array of 0s, 1s, and 2s representing red, white, and blue, respectively
* Sort the array in place so it resembles the Dutch national flag, with all reds (0s) coming first, followed by whites (1s), and finally blues (2s).


<span style="color:orange"><b>The point:</b></span>

* Using in-built function => O(nlog(n))
* position all 0s to the left, all 2s to the right (the 1s comes for free in the middle)
* Use 2 pointers (left, right) that go inward, swap value when the encounter a 0 or a 2  

**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n)           | O(1)      |

* O(n) in time because we iterate through each element of `nums` once.
* O(1) in space because in place




<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* First implementation

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
fn dutch_national_flag(nums: &mut[i32]) {

    let mut i = 0;
    let (mut left, mut right) = (0, nums.len() - 1);
    
    while i <= right {
        if nums[i] == 0{
            // Swap 0s with the element at the left pointer.
            nums.swap(i, left);
            left += 1;
            i += 1;
        } else if nums[i] == 2{
            // Swap 2s with the element at the right pointer.
            nums.swap(i, right);
            right -= 1
        } else{
            i += 1;
        }
    }
} 


fn main() { // no main() if this code runs in a Jupyter cell
    let mut vals = vec![0, 1, 2, 0, 1, 2, 0]; 
    dutch_national_flag(&mut vals);
    println!("{:?}", vals); // [0, 0, 0, 1, 1, 2, 2]
    
} // end of local scope OR end of main()
```

## V2

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn dutch_national_flag(nums: &mut [i32]) {
    let mut i = 0;
    let (mut left , mut right) = (0, nums.len() - 1);
    
    while i <= right {
        match nums[i] {
            0 => {
                // Swap 0s with the element at the left pointer
                nums.swap(i, left);
                left += 1;
                i += 1;
            },
            2 => {
                // Swap 2s with the element at the right pointer
                nums.swap(i, right);
                // Do not increment i, as the swapped value needs to be checked
                right -= 1;
            },
            _ => {
                // If it's 1, just move forward
                i += 1;
            },
        }
    }
}

fn main() {
    let mut vals = vec![0, 1, 2, 0, 1, 2, 0];
    dutch_national_flag(&mut vals);
    println!("{:?}", vals); // Output: [0, 0, 0, 1, 1, 2, 2]
}

```
