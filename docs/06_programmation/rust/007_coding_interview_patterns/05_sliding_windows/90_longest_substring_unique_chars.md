---
# published: false
layout: default
title: "p090 - Longest Substring With Unique Characters"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Longest Substring With Unique Characters

<div align="center">
<img src="../assets/chap_05.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a string determine the lenght of its longest substring of unique char

<span style="color:orange"><b>The point:</b></span>

* Brute force in O(n^3)
    * => dynamic sliding window
* Expand (right) the window until is has duplicates
* Shrink (left) it until the duplicate is removed
* Use a hash set to keep track of chars in the window

**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(m)  |

* O(n) because we traverse the string (len=n)
* O(m) because of the hash set with m unique chars

**About Rust :**
* `use std::collections::HashSet;` 
* `let mut hash_set = HashSet::new();`
* ``hs.insert(bob)``, `hs.contains(&bob)`, `hs.remove(&bob)`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
use std::collections::HashSet;

fn longest_substring_with_unique_chars (s : &str) -> usize{
    
    let s_bytes = s.as_bytes();
    let mut max_len = 0;
    let mut hash_set = HashSet::new();
    let (mut left, mut right) = (0, 0);
    
    while right < s_bytes.len(){
        // if duplicate in the window, shrink it until the duplicate is removed
        while hash_set.contains(&s_bytes[right]){
            hash_set.remove(&s_bytes[left]);
            left +=1;
        }
        // update the max length when no duplicate in the window
        max_len = max_len.max(right - left + 1);
        hash_set.insert(s_bytes[right]);
        right += 1;
    }
    max_len
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{}", longest_substring_with_unique_chars("abcba"))
// } // end of local scope OR end of main()       

```

    3





    ()



## Optimization
* p 93

<span style="color:orange"><b>The point:</b></span>
* Move the left pointer **right after the previous occurence** of the char at the right pointer
* Need to keep track of previous indexes

**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(m)  |



**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

fn longest_substring_with_unique_chars_optimized (s : &str) -> usize {
    
    let s_bytes = s.as_bytes();
    let mut max_len = 0;
    let mut prev_indexes:HashMap<u8, usize> = HashMap::new();
    
    let (mut left, mut right) : (usize, usize) = (0, 0);

    while right < s_bytes.len(){
        let current_char = s_bytes[right];
        // Check if character exists in hash map and its index is >= left
        // if a previous index of the char at the right pointer => it's a duplicate
        if let Some(prev_index) = prev_indexes.get(&current_char) {
            if *prev_index >= left { // .get() returns a reference so we dereference it to read the value
                // move the left pointer to the right of the previous index of the char which is at the right pointer
                left = *prev_index + 1;
            }
        }
        
        // update the max length when no duplicate in the window
        max_len = max_len.max(right - left + 1);
        prev_indexes.insert(current_char, right);
        right += 1;
    }
    max_len
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{}", longest_substring_with_unique_chars_optimized("abcba"));
// } // end of local scope OR end of main()       
```

    3

