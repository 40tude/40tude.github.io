---
# published: false
layout: default
title: "p411 - Reverse 32-Bit integer"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Reverse 32-Bit integer

<div align="center">
<img src="../assets/chap_19.webp" alt="" width="300" loading="lazy"/>
</div>

* Reverse the digits of a i32
* If overflow (out of [-2^31, 2^31-1]) return 0
* You can only store int in the 32 bit range


<span style="color:orange"><b>The point:</b></span>

* Reminder and i32 div
* Edge cases : `reversed>INT_MAX//10` (`reversed<INT_MAX//10` if neg) 


**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(1)         | O(1)       |

* O(log(n)) in time because we traverse each digit and the number of digits is around log(n) (log10(n)). We can consider O(1) since i32
* O(1) in space  





**About Rust :**
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn reverse_32_bit_integer(mut n : i32) -> i32{
    let mut reversed_n = 0;

    // keep looping until we've added all digits
    while n != 0 {
        let digit = n % 10;
        n /= 10;
        // if reversed_n > i32::MAX/10 || reversed_n < i32::MIN/10 {
        if !(i32::MIN/10..=i32::MAX/10).contains(&reversed_n) {
            return 0
        }
        reversed_n = reversed_n * 10 + digit;
    }
    reversed_n
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", reverse_32_bit_integer(420)); // 24
    println!("{}", reverse_32_bit_integer(-15)); // -51
    println!("{}", reverse_32_bit_integer( 1534236469)); // 0
    println!("{}", reverse_32_bit_integer(-1563847412)); // 0
} // end of local scope OR end of main()
```

## V2


**About Rust :**
* Hide the compare to i32::MAX and i32::MIN
    * Not sure it's such a good idea
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn reverse_32_bit_integer(mut n: i32) -> i32 {
    let mut reversed_n : i32 = 0;

    while n != 0 {
        let digit = n % 10;  // Extract the last digit
        n /= 10;             // Remove the last digit from n
        reversed_n = match reversed_n.checked_mul(10).and_then(|v| v.checked_add(digit)) {
            Some(val) => val, // Safe to proceed
            None => return 0, // Overflow detected, return 0 immediately
        };
    }
    reversed_n
}

fn main() { // no main() if this code runs in a Jupyter cell 
    println!("{}", reverse_32_bit_integer(420)); // 24
    println!("{}", reverse_32_bit_integer(-15)); // -51
    println!("{}", reverse_32_bit_integer( 1534236469)); // 0
    println!("{}", reverse_32_bit_integer(-1563847412)); // 0
} // end of local scope OR end of main()

```
