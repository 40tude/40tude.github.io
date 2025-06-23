---
# published: false
layout: default
title: "p130 - Valid Parenthesis Expression"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Valid Parenthesis Expression

* Given a string made of the following symbols : `(`, `)`, `[`, `]`, `{`, `}` 
* Determine if it is well formed
* A sequence is valid when every open symbol is paired with a close symbol

<span style="color:orange"><b>The point:</b></span>

* The most recent opening symbol should be the first that gets closed
* LIFO


**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(1)  |

* O(n) because we traverse the string once
* O(1) because in place 

**About Rust :**
* `stack.push()`, `stack.pop()`, `stack.is_empty()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
use std::collections::HashMap;

fn valid_parenthesis_expression(s: &str) -> bool {
    let parenthesis_map = HashMap::from([
        ('(', ')'),
        ('{', '}'),
        ('[', ']'),
    ]);
    let mut stack = Vec::new();

    for c in s.chars() {
        // If current char is an opening symbol, push it
        if parenthesis_map.contains_key(&c) {
            stack.push(c);
        }
        // If current char is a closing symbol check that it closes the current opening
        else {
            match stack.pop() {
                Some(open) => {
                    if parenthesis_map[&open] != c {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }

    // If the stack is empty the expression is valid
    stack.is_empty()
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    println!("{}", valid_parenthesis_expression("([]{})"));  // true
    println!("{}", valid_parenthesis_expression("([]{)}"));  // false
// } // end of local scope OR end of main()       
            
```

    2
    4
    0
    7

