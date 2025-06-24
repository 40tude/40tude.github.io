---
# published: false
layout: default
title: "p401 - Swap Odd and Even Bits"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Swap Odd and Even Bits

<div align="center">
<img src="../assets/chap_18.webp" alt="" width="300" loading="lazy"/>
</div>

* Given an u32 return an integer where all the  bits in even position are swapped with their adjacent odd bits

<span style="color:orange"><b>The point:</b></span>

* bit in even position swapped with the bit in the next odd position (adjacent)
* all bits in even position need to shift by 1 on the left
* all bits in odd position need to shift by 1 on the right
* use a mask (*smoking!*) + bitwise-and




**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(1)         | O(1)       |




**About Rust :**
* Basic implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn swap_odd_and_even_bits(n : u32) -> u32{
    let even_mask = 0x5555_5555;
    let odd_mask = 0xAAAA_AAAA;
    let even_bits = n & even_mask;
    let odd_bits = n & odd_mask;
    // shift the even bit to the lef, the odd bit to the right
    // merge
    (even_bits<<1) | (odd_bits>>1)
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", swap_odd_and_even_bits(41)); // 22
    println!("{}", swap_odd_and_even_bits(23)); // 43
} // end of local scope OR end of main()
```
