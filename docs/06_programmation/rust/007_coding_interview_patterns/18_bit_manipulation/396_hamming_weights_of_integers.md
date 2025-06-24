---
# published: false
layout: default
title: "p396 - Hamming Weights of Integer"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Hamming Weights of Integer

* The Hamming weight of an integer is the number of bits set to 1
* Given a positive integer `n` return an array where the ith element is the Hamming weight of integer `i` for all integers from `0` to `n`

<span style="color:orange"><b>The point:</b></span>

* Check if LSB and shift right




**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(nlog(n))   | O(1)       |

* O(nlog(n)) in time because for each integer from 0 to n, counting the number of bits set is in log(n) => n log(n)
* O(1) in space because in place  









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

**About Rust :**
* Basic implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn count_set_bits(mut x : u32) -> usize{
    let mut count:usize = 0;
    // count each bit set to 1 until x equal 0
    while x>0{
        if x&1 == 1 {
            count +=1; // Increment count if LSB is 1
        }
        x >>= 1; // right shift
    }
    count
}

fn hamming_weights_of_integers(n: u32) -> Vec<usize>{
 let mut out_vect = Vec::new();
 for x in 0..=n{
    out_vect.push(count_set_bits(x));
 }   
 out_vect
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{:?}", hamming_weights_of_integers(7)); // [0, 1, 1, 2, 1, 2, 2, 3]
} // end of local scope OR end of main()
```

## V2

**About Rust :**
* `(0..=n).map(count_set_bits).collect()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn count_set_bits(mut x: usize) -> usize {
    let mut count = 0;
    // count each bit set to 1 until x equal 0
    while x != 0 {
        count += x & 1; // Add LSB to count
        x >>= 1;        // Right shift by 1
    }
    count
}

fn hamming_weights_of_integers(n: usize) -> Vec<usize> {
    // Create a Vec by mapping each number in the range 0..=n to its Hamming weight
    (0..=n).map(count_set_bits).collect()
}


fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{:?}", hamming_weights_of_integers(7)); // [0, 1, 1, 2, 1, 2, 2, 3]
} // end of local scope OR end of main()
```

## Dynamic Programming

<span style="color:orange"><b>The point:</b></span>

* when we reach x we have all result for all integers from 0 to x-1
* optimal substructure
* the number  of bits in `x` is the number of bits in `(x>>1)` plus the LSB from `x`


**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(n)         | O(1)       |

* O(n) in time because we populate each element of DP once
* O(1) in space because no extra space required  


**About Rust :**
* `let mut dp = vec![0; n + 1];`
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn hamming_weights_of_integers(n: usize) -> Vec<usize> {
    // Base case : the number of bits set in 0 is 0
    // Set dp[0] to 0 while setting all dp content to 0
    let mut dp = vec![0; n + 1];

    for x in 1..=n{
        dp[x] = dp[x>>1] + (x&1);
    }
    dp
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{:?}", hamming_weights_of_integers(7)); // [0, 1, 1, 2, 1, 2, 2, 3]
} // end of local scope OR end of main()
```
