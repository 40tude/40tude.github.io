---
# published: false
layout: default
lang: en-US
title: "p020 - Is Palindrome Valid"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Is Palindrome Valid

<div align="center">
<img src="../assets/chap_01.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a string determine if it's a palindrome after removing all non-alphanumeric characters

<span style="color:orange"><b>The point:</b></span>    
* Use two pointers (start, end).
* They must match. We skip non-alphanumeric characters.
* We must land on the same character or two identical characters before left > right.


**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(1)  |

* Because we traverse the n-character string once and allocate a constant number of variables.





**About Rust :**
* `let chars: Vec<char> = s.chars().collect();`
* `chars[left].is_alphanumeric()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->






```rust
fn is_palindrome_valid(s: &str) -> bool{
    let s = s.to_lowercase();                   // s.to_ascii_lowercase();
    let chars: Vec<char> = s.chars().collect(); // transforms the view into a vector of chars we can iterate on. 
                                                // s.chars() is an iterator
                                                // .collect() transforms the iterator in a collection (otherwise nothing happens)

    let (mut left, mut right) = (0, chars.len().saturating_sub(1)); // right = len - 1 or 0 if len-1 is negative
    
    while left < right{
        //skip non alphanumeric characters
        while left < right && !chars[left].is_alphanumeric(){
            left += 1;
        }
        //skip non alphanumeric characters
        while left < right && !chars[right].is_alphanumeric(){
            right -= 1;
        }
        //if does'nt match return false otherwise go forward
        if chars[left] != chars[right]{
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{:?}", is_palindrome_valid("abbabba")); // true
    println!("{:?}", is_palindrome_valid("abbabbu")); // false
    println!("{:?}", is_palindrome_valid("a man, a plan, a canal: panama")); // true
    println!("{:?}", is_palindrome_valid("Madam, in Eden, I'm Adam")); // true
    println!("{:?}", is_palindrome_valid("")); // true
// }
```

    true
    false
    true
    true
    true


## An idea
* No need to create a vector of chars locally
* However, with huge strings, `Vec<char>` might be faster because of O(1) access 


```rust
fn is_palindrome_valid(s: &str) -> bool {
    let s = s.to_lowercase();
    let mut chars = s.chars().filter(|c| c.is_alphanumeric());
    
    while let (Some(left_char), Some(right_char)) = (chars.next(), chars.next_back()) {
        if left_char != right_char {
            return false;
        }
    }
    true
}
// fn main(){
    println!("{:?}", is_palindrome_valid("abbabba"));
    println!("{:?}", is_palindrome_valid("abbabbu"));
    println!("{:?}", is_palindrome_valid("a man, a plan, a canal: panama")); // true
    println!("{:?}", is_palindrome_valid("Madam, in Eden, I'm Adam"));
    println!("{:?}", is_palindrome_valid(""));
// }
```

    true
    false
    true
    true
    true

