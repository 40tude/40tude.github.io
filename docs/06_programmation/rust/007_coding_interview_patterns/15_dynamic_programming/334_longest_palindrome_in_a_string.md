---
# published: false
layout: default
lang: en-US
title: "p334 - Longest Palindrome in a String"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Longest Palindrome in a String

<div align="center">
<img src="../assets/chap_15.webp" alt="" width="300" loading="lazy"/>
</div>

* Return the longest palindromic substring within a given string

<span style="color:orange"><b>The point:</b></span>

* Brute force is very inefficient (O(n^3))
* Palindromes contains shorter palindromes
* String from `i` to `j` is a palindrome if
    1. `s[i] = s[j]`
    1. `s(i+1 : j-1)` is a palindrome
* `dp[i][j] = true if s[i]==s[j] && dp[i+1][j+1]`
* Base case
    * All substrings of len 1 are palindromes
    * Substring of len 2 are palindromes is both chars are the same


**Phases of a DP solution :**
1. Subproblem
1. DP Formula
1. Base Case
1. Populate DP Table




**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n²)       | O(n²)        |

* O(n²) in time because each cell of the n x n DP table is populated once
* O(n²) in space because of the size of the DP table


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn longest_palindrome_in_a_string(s: &str) -> String {
    // Convert strings to vectors of chars to allow indexed access
    let s: Vec<char> = s.chars().collect();

    if s.is_empty() {
        return "".to_string();
    }
    let mut dp = vec![vec![false; s.len()]; s.len()];

    // Base case. A single char is a palindrome
    // for i in 0..s.len() {
    //     dp[i][i] = true;
    // }
    for (i, row) in dp.iter_mut().enumerate() { // see Clippy
        row[i] = true;
    }
    
    let (mut max_len, mut start_index) = (1, 0);
    // Base case : a substring of len 2 is a palindrome if both chars are the same
    for i in 0..s.len() - 1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = true;
            max_len = 2;
            start_index = i
        }
    }

    // find palindromic substrings of len 3 or more
    for substring_len in 3..=s.len() {
        // Iterate trough each substring of len `substring_len`
        for i in 0..s.len() - substring_len {
            let j = i + substring_len - 1;
            // If first and last char are the same
            // and if the inner string is a palindrome then this is a palindrome
            if s[i] == s[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                max_len = substring_len;
                start_index = i;
            }
        }
    }
    s[start_index..start_index + max_len].iter().collect() // .collect() transforms the iterator into String
}

fn main() {
    // no main() if this code runs in a Jupyter cell
    let s = "abccbaba";
    println!("{}", longest_palindrome_in_a_string(s)); // abccba

    let s = "";
    println!("{}", longest_palindrome_in_a_string(s)); // ""

    let s = "a";
    println!("{}", longest_palindrome_in_a_string(s)); // a

    let s = "ca";
    println!("{}", longest_palindrome_in_a_string(s)); // c

    let s = "aa";
    println!("{}", longest_palindrome_in_a_string(s)); // aa
} // end of local scope OR end of main()

```

## Optimization

<span style="color:orange"><b>The point:</b></span>

* Base cases are the centers of palindromes
* Let's expand outward
* 2 cases to consider :
    * single char : 
    * two chars :  

**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(m x n)       | O(1)      |


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





```rust
fn expand_palindrome(mut left: usize, mut right: usize, s: &[char]) -> (usize, usize) {
    while left > 0 && right + 1 < s.len() && s[left - 1] == s[right + 1] {
        left -= 1;
        right += 1;
    }
    (left, right - left + 1)
}

fn longest_palindrome_in_a_string(s: &str) -> String {
    // Convert strings to vectors of chars to allow indexed access
    let s: Vec<char> = s.chars().collect();

    let (mut max_len, mut start) = (0, 0);

    for center in 0..s.len() {
        // Check odd len palindromes
        let (odd_start, odd_len) = expand_palindrome(center, center, &s);
        if odd_len > max_len {
            start = odd_start;
            max_len = odd_len;
        }
        // Check for even len palindromes
        if center < s.len() - 1 && s[center] == s[center + 1] {
            let (even_start, even_len) = expand_palindrome(center, center + 1, &s);
            if even_len > max_len {
                start = even_start;
                max_len = even_len
            }
        }
    }
    s[start..start + max_len].iter().collect() // .collect() transforms the iterator into String
}

fn main() {
    // no main() if this code runs in a Jupyter cell
    let s = "abccbaba";
    println!("{}", longest_palindrome_in_a_string(s)); // abccba

    let s = "";
    println!("{}", longest_palindrome_in_a_string(s)); // ""

    let s = "a";
    println!("{}", longest_palindrome_in_a_string(s)); // a

    let s = "ca";
    println!("{}", longest_palindrome_in_a_string(s)); // c

    let s = "aa";
    println!("{}", longest_palindrome_in_a_string(s)); // aa
} // end of local scope OR end of main()

```
