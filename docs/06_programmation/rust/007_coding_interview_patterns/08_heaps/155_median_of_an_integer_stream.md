---
# published: false
layout: default
title: "p155 - Median of an Integer Stream"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Median of an Integer Stream

<div align="center">
<img src="../assets/chap_08.webp" alt="" width="300" loading="lazy"/>
</div>

* Design a structure where we can add i32 from a data stream and get the median
    * `add(num:i32)` : add an int to the structure
    * ``get_median() -> f64`` : returns the median from all integers so far


<span style="color:orange"><b>The point:</b></span>

* Use 2 heaps 
    * max-heap manages the left half (max val <= min val of min-heap). Default in Rust
    * min-heap manages the right half 
    * both have the same number of values (except when n is odd)

**Complexity :**

| Time               | Space |
|--------------------|-------|
| O(log(n))               | O(1)  |

* O(log(n)) because we push object to the heap in O(log(n)). If rebalancing required we pop and push in O(log(n)) again
* O(1) because access to the top of the heap is in O(1) 

**About Rust :**
* max-heap is the default in Rust 
* `.get()` returns an ``Option<T>`` and don't panic when called on an empty structure 

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## First version


```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MeadianCalc {
    left_half : BinaryHeap<i32>, // max-heap (default in Rust)
    right_half : BinaryHeap<Reverse<i32>>, // min_heap (must be "reversed")
}

impl MeadianCalc {
    fn new() -> Self {
        Self {
            left_half : BinaryHeap::new(),
            right_half : BinaryHeap::new(),
        }
    }

    fn add (&mut self, n:i32){
        // Push to left_half if it's empty or if n is less than or equal to the max of left_half
        // Otherwise "reverse" push on the right_half 
        if let Some(&top) = self.left_half.peek() { // peek returns an Option<&T>
            if n <= top {
                self.left_half.push(n);
            } else {
                self.right_half.push(Reverse(n));
            }
        } else {
            self.left_half.push(n);
        }

        // Rebalance
        if self.left_half.len() > self.right_half.len() + 1 {
            self.right_half.push(Reverse(self.left_half.pop().unwrap()));
        } else if self.left_half.len() < self.right_half.len() {
            self.left_half.push(self.right_half.pop().unwrap().0);
        }
    }

    fn get (&self) -> Option<f64>{
        match (self.left_half.peek(), self.right_half.peek()) {
            (Some(&left_top), Some(&Reverse(right_top))) if self.left_half.len() == self.right_half.len() => {
                Some((left_top as f64 + right_top as f64) / 2.0)
            },
            (Some(&left_top), _) => Some(left_top as f64),
            _ => None,
        }
    }
}

fn main(){   // no main() if this code runs in a Jupyter cell 

    let mut my_med = MeadianCalc::new();
    println!("{:?}", my_med.get()); // None
    
    my_med.add(3);
    println!("{:?}", my_med.get()); // Some(3.0)

    my_med.add(6);
    println!("{:?}", my_med.get()); // Some(4.5)

    my_med.add(42);
    println!("{:?}", my_med.get()); // Some(6.0)
} // end of local scope OR end of main()       
```

## Other test
* Check ``.add()`` and the `n <= *self.left_half.peek().unwrap()`
* <span style="color:lime"><b>Preferred solution?</b></span> 


```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MeadianCalc {
    left_half : BinaryHeap<i32>, // max-heap (default in Rust)
    right_half : BinaryHeap<Reverse<i32>>, // min_heap
}

impl MeadianCalc {
    fn new() -> Self {
        Self {
            left_half : BinaryHeap::new(),
            right_half : BinaryHeap::new(),
        }
    }

    fn add (&mut self, n:i32){
        // Push to left_half if it's empty or n is less than or equal to the max of left_half
        if self.left_half.is_empty() || n <= *self.left_half.peek().unwrap() {
            self.left_half.push(n);
        } else {
            self.right_half.push(Reverse(n));
        }

        // Rebalance
        if self.left_half.len() > self.right_half.len() + 1 {
            self.right_half.push(Reverse(self.left_half.pop().unwrap()));
        } else if self.right_half.len() > self.left_half.len() {
            self.left_half.push(self.right_half.pop().unwrap().0);
        }
    }

    fn get (&self) -> Option<f64>{
        match (self.left_half.peek(), self.right_half.peek()) {
            (Some(&left_top), Some(&Reverse(right_top))) if self.left_half.len() == self.right_half.len() => {
                Some((left_top as f64 + right_top as f64) / 2.0)
            },
            (Some(&left_top), _) => Some(left_top as f64),
            _ => None,
        }
    }

    
}

fn main(){   // no main() if this code runs in a Jupyter cell 

    let mut my_med = MeadianCalc::new();
    println!("{:?}", my_med.get());     // None
    
    my_med.add(3);
    println!("{:?}", my_med.get());     // Some(3.0)

    my_med.add(6);
    println!("{:?}", my_med.get());     // Some(4.5)

    my_med.add(42);
    println!("{:?}", my_med.get());     // Some(6.0)
} // end of local scope OR end of main()       
```
