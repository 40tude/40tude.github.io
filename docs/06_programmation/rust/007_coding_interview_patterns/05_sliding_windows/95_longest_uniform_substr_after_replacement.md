---
# published: false
layout: default
title: "p095 - Longest Uniform Substring After Replacements"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Longest Uniform Substring After Replacements

* Given a string determine the length of its longest substring of similar char replacing up to `k` chars

**Example**
* k = 2
* a a ~~b~~ [c] c ~~d~~ [c] c a => a a c c c c c a

<span style="color:orange"><b>The point:</b></span>

* When we find a valid window with a certain length, no shorter windows can provide a longer uniform substring

* **Step 1**
    * Uniform string => replace all char except the most frequent one
    * num_char_to_replace = len(str) - highest_freq
    * A string can be made uniform if num_char_to_replace <= k
    * Use a hash map to keep track of freq of each char
* **Step 2**
    * use dynamic sliding window
    * to find the longest substring where num_char_to_replace <= k
        * if condition met => expand to the right
        * if condition not met shrink by the left until it meet the condition again
    * see p 97 the way to update highest_freq when we would like to shrink by the left but where we slide the window of the current size 

**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(m)  |

* O(n) because we traverse the string (len=n)
* O(m) because there are m unique chars in the hash map freqs


**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
use std::collections::HashMap;

fn longest_uniform_substring_after_replacement (s : &str, k: usize) -> usize{
    
    let mut freqs: HashMap<u8, i32> = HashMap::new();
    let(mut highest_freq, mut max_len) = (0, 0);
    let (mut left, mut right) = (0, 0);
    let s_bytes = s.as_bytes(); 

    while right < s_bytes.len(){
        // update the freq of the char under right ptr and highest freq of the current window
        *freqs.entry(s_bytes[right]).or_insert(0) += 1;
        highest_freq = highest_freq.max(freqs[&s_bytes[right]] as usize);
        
        // find the # of replacements. the window length = right - left + 1
        let num_chars_to_replace = right - left + 1 - highest_freq;
        
        if num_chars_to_replace > k {
            // remove the char under the left ptr then move the left ptr
            if let Some(freq) = freqs.get_mut(&s_bytes[left]) {
                *freq -= 1; // .get_mut() returns a mutable reference so we dereference it to modify the value
            }
            left += 1;
        }
        // assign the window length to max_len
        max_len = right - left + 1;
        right += 1;
    }
    max_len
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{}", longest_uniform_substring_after_replacement("aabcdcca", 2)) // 5
// } // end of local scope OR end of main()       


```

    5





    ()


