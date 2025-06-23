---
# published: false
layout: default
title: "bonus128 - Phone Keypad Combinations"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Phone Keypad Combinations

* You are given a string containing digits from 2 to 9 inclusive. Each digit maps to a set of letters
as on a traditional phone keypad
* Return all possible letter combinations the input digits (string) could represent.




<span style="color:orange"><b>The point:</b></span>

* Three letters per digit [2..8] 4 letter for 9
* State space tree start with "" (level i=0). Then 3 branches (one per letter, level i=1), then 4 branches (level i=2)
* Backtracking
* Hash map to map digit (K) to chars (V)
*


**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n!)       | O(n)  |

* O(n!) in time because 
    * first queen => n choices
    * second queen => n-a. a the number of square attacked by the first queen
    * third queen => n-b. b the number of square attacked by the 2 first queens (`b<a`)
    * this "looks like" to n!
* O(n) in space because n is the max depth of the recursion tree. The hash sets can store up to `n` values.









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* Translation

**About Rust :**
* `let keypad_map = HashMap::from([('2', "abc"), ('3', "def"), ('4', "ghi"), ('5', "jkl"), ('6', "mno"), ('7', "pqrs"), ('8', "tuv"), ('9', "wxyz")]);`
* `my_&str.chars()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

fn backtrack(i: usize, curr_combination: &mut Vec<char>, digits: &str, keypad_map: &HashMap<char, &str>, res: &mut Vec<String>) {
    // Termination condition: if all digits have been considered
    // Add the current combination to the output list
    if curr_combination.len() == digits.len() {
        res.push(curr_combination.iter().collect());
        return;
    }

    // PYTHON = for letter in keypad_map[digits[i]]...
    if let Some(letters) = keypad_map.get(&digits.chars().nth(i).unwrap()) { // Safe unwrap because digits are valid
        for letter in letters.chars() {
            // Add the current letter.
            curr_combination.push(letter);

            // Recursively explore all paths that branch from this combination.
            backtrack(i + 1, curr_combination, digits, keypad_map, res);

            // Backtrack by removing the letter we just added.
            curr_combination.pop();
        }
    }
}

fn phone_keypad_combinations(digits: &str) -> Vec<String> {
    let index = 0;
    let mut curr_combination = Vec::new();
    let keypad_map = HashMap::from([('2', "abc"), ('3', "def"), ('4', "ghi"), ('5', "jkl"), ('6', "mno"), ('7', "pqrs"), ('8', "tuv"), ('9', "wxyz")]);
    let mut res = Vec::new();
    backtrack(index, &mut curr_combination, digits, &keypad_map, &mut res);
    res
}

fn main() { // no main() if this code runs in a Jupyter cell
    let digits = "69";
    print!("{:?}", phone_keypad_combinations(digits)); // ["mw", "mx", "my", "mz", "nw", "nx", "ny", "nz", "ow", "ox", "oy", "oz"]
} // end of local scope OR end of main()

```

## V2

* In `backtrack()`
    * avoid `.nth(i)` in O(n)
    * Use ``digits.get(i)`` instead to get an access in O(1)
* In `main()` 
    * ``digits.chars().collect()`` is called once to convert the string in ``Vec<char>``


**About Rust :**
```rust
if let Some(&digit) = digits.get(i) {
        if let Some(letters) = keypad_map.get(&digit) {
```
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

fn backtrack(i: usize, curr_combination: &mut Vec<char>, digits: &[char], keypad_map: &HashMap<char, &str>, res: &mut Vec<String>) {
    // Termination condition: if all digits have been considered
    // Add the current combination to the output list
    if curr_combination.len() == digits.len() {
        res.push(curr_combination.iter().collect());
        return;
    }

    // PYTHON = for letter in keypad_map[digits[i]]...
    if let Some(&digit) = digits.get(i) {
        if let Some(letters) = keypad_map.get(&digit) {
            for letter in letters.chars() {
                // Add current letter
                curr_combination.push(letter);
                // Recursively explore all paths that branch from this combination.
                backtrack(i + 1, curr_combination, digits, keypad_map, res);
                // Backtrack by removing the letter we just added.
                curr_combination.pop();
            }
        }
    }
}

fn phone_keypad_combinations(digits: &str) -> Vec<String> {
    let index = 0;
    let mut curr_combination = Vec::new();
    let digits_chars: Vec<char> = digits.chars().collect(); // collect chars once
    let keypad_map = HashMap::from([('2', "abc"), ('3', "def"), ('4', "ghi"), ('5', "jkl"), ('6', "mno"), ('7', "pqrs"), ('8', "tuv"), ('9', "wxyz")]);
    let mut res = Vec::new();
    
    backtrack(index, &mut curr_combination, &digits_chars, &keypad_map, &mut res);
    res
}

fn main() { // no main() if this code runs in a Jupyter cell
    let digits = "69";
    print!("{:?}", phone_keypad_combinations(digits)); // ["mw", "mx", "my", "mz", "nw", "nx", "ny", "nz", "ow", "ox", "oy", "oz"]
} // end of local scope OR end of main()

```
