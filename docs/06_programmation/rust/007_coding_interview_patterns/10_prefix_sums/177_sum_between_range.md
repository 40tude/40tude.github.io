---
# published: false
layout: default
title: "p177 - Sum Between Range"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Sum Between Range

* Given an array of i32 return the sum of vals between 2 indexes
* You can assume
    * Array is not empty
    * Ranges are valid


<span style="color:orange"><b>The point:</b></span>

* Pre processing in constructor
* ``sum[i, j] = prefix_sums[j] - prefix_sums[i-1]`` .NOT ``i`` but ``i-1``


**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n) & O(1) | O(n)  |

* O(n) because constructor in 0(n) 
* O(1) because ``.sum_range()`` is in O(1)
* O(n) because of space needed to holds up to n sums


**About Rust :**
* No need for `assert!(nums.len()>0, "Invalid length");`
* No need for `assert!(i <= j && j < self.prefix_sums.len(), "Invalid indexes");`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
struct SumBetweenRange {
    prefix_sums : Vec<i32>,
}

impl SumBetweenRange {
    fn new(nums : &[i32]) -> Self {

        let mut prefix_sums = Vec::with_capacity(nums.len());
        let mut current = 0;
    
        for &num in nums {
            current += num;
            prefix_sums.push(current);
        }
        Self { prefix_sums }
    }

    fn sum_range(&self, i : usize, j : usize) -> i32{
        if i == 0 {
            self.prefix_sums[j]
        }else{
            self.prefix_sums[j] - self.prefix_sums[i-1] // NOT i but i-1
        }
    }

}


fn main(){   // no main() if this code runs in a Jupyter cell 
    let input = SumBetweenRange::new(&[3, -7, 6, 0, -2, 5]);

    println!("{:?}", input.sum_range(0, 3)); // 2
    println!("{:?}", input.sum_range(2, 4)); // 4
    println!("{:?}", input.sum_range(2, 2)); // 6
} // end of local scope OR end of main()       
```
