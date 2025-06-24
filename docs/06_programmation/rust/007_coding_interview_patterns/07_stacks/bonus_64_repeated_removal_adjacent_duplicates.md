---
# published: false
layout: default
title: "bonus064 - Repeated Removal of Adjacent Duplicates"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Repeated Removal of Adjacent Duplicates

<div align="center">
<img src="../assets/chap_07.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a string, continually perform the following operation: 
* Remove pairs of adjacent duplicates from the string. 
* Continue performing this operation until the string no longer contains pairs of adjacent duplicates.
* Return the final string.

<span style="color:orange"><b>The point:</b></span>
* understand that both chars of the pair vanish (we don't even keep one char)
* build the string character by character
* immediately remove each pair that get formed as we build the string


**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(n)  |

* O(n) because we travers the string and join up to n chars at the end. push/pop contributes 0(1) time
* O(n) because the stack can store up to ``n`` chars 

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
fn repeated_removal_of_adjacent_duplicates(s: &str) -> String{
    let mut stack = Vec::new();
    for c in s.chars(){
        if !stack.is_empty() && c == *stack.last().unwrap(){
            // Current character is the same as the one on top of the stack,
            // This is a pair => pop the top character to make both disappear
            stack.pop();
        } else{
            // Otherwise, push the current character onto the stack.
            stack.push(c);
        }
    }
    // Return the remaining characters as a string.
    stack.into_iter().collect()
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", repeated_removal_of_adjacent_duplicates("aacabba")); // "c"
    println!("{:?}", repeated_removal_of_adjacent_duplicates("aaa")); // "a"
    println!("{:?}", repeated_removal_of_adjacent_duplicates("")); // ""
} // end of local scope OR end of main()       
```
