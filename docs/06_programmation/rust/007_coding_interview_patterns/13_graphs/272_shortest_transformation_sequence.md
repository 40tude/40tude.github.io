---
# published: false
layout: default
lang: en-US
title: "p272 - Shortest Transformation Sequence"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Shortest Transformation Sequence


<div align="center">
<img src="../assets/chap_13.webp" alt="" width="300" loading="lazy"/>
</div>

* Given 2 words and a dictionary
* return the shortest transformation sequence fro start to end
* Each word differ from the preceding by one letter
* Each word is in the dictionary
* Assume
    * all words are same len
    * only lowercase English letters
    * no duplicate in dictionary

<span style="color:orange"><b>The point:</b></span>

* Shortest path => BFS
* Level Order Traversal
* Store the words in a dictionary (constant time check for existence)
* 

**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n x L²)   | O(n x L)     |

* O(n x L²) in time because creating hash set take O(n x L) (n= # of words, L=len of a word).Level Order Traversal process n words and we generate 26 x L transformations in O(L) => O(n x L²)
* O(n x L) in space because space taken by the dictionary hash set, visited hash set and queue


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First implementation

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::HashSet;
use std::collections::VecDeque;

fn shortest_transformation_sequence(start: &str, end: &str, dictionary: &[String]) -> usize {
    let dictionary_set: HashSet<String> = HashSet::from_iter(dictionary.iter().cloned());

    if !dictionary_set.contains(start) || !dictionary_set.contains(end) {
        return 0;
    }

    if start == end {
        return 1;
    }

    let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_string());

    let mut visited_set: HashSet<String> = HashSet::new();
    visited_set.insert(start.to_string());

    let mut dist = 0;

    // Use level-order traversal to find the shortest path from start to end
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let curr_word = queue.pop_front();
            // if we found the end word we've reacg it via shortest path
            if curr_word == Some(end.to_string()) {
                return dist + 1;
            }
            // Generate all possible words that have one letter difference to current word
            let curr_word = curr_word.unwrap(); // safe here because queue is not empty
            for i in 0..curr_word.len() {
                for c in lowercase_alphabet.chars() {
                    let next_word = format!("{}{}{}", &curr_word[..i], c, &curr_word[i + 1..]);

                    // if next_word is in dictionary it is a neighbor of current word
                    // if unvisited, add it to the queue to be processed in the next level
                    if dictionary_set.contains(&next_word) && !visited_set.contains(&next_word) {
                        visited_set.insert(next_word.clone());
                        queue.push_back(next_word.clone());
                    }
                }
            }
        }
        dist += 1;
    }
    // If there is no way to reach the end node, then no path exists
    0
}

fn main() {
    let dictionary = ["red", "bed", "hat", "rod", "rad", "rat", "hit", "bad", "bat"].map(|s| s.to_string());

    let start = "red";
    let end = "hit";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 5


    let start = "red";
    let end = "red";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 1

    let start = "Zoubida";
    let end = "red";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 0
}
```

## V2

**About Rust :**
* Use `&str` rather than String (less `.clone()`, less memory allocation, less memory used)
* But... Structures `HashSet<&str>` must be sure that the `&str` they contain remain valid for the entire time the set is used. They must live as long as the set. Otherwise => lifetime problem.
* Keep `queue` and `visited_set` as `Strings`, because we need modifiable words in the loop (Otherwise, with &str, it's a nightmare)
* Watchout `next_word.replace_range(i..=i, &c.to_string());` to avoid `format!()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::{HashSet, VecDeque};

const LOWERCASE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

// This function now uses HashSet<&str> instead of HashSet<String>
fn shortest_transformation_sequence(start: &str, end: &str, dictionary: &[String]) -> usize {
    // Build a set of &str from the dictionary which contains String. No lifetime issue
    let dictionary_set: HashSet<&str> = dictionary.iter().map(|s| s.as_str()).collect();

    if !dictionary_set.contains(start) || !dictionary_set.contains(end) {
        return 0;
    }

    if start == end {
        return 1;
    }

    // queue stores owned Strings since we need to mutate each word
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_string());

    let mut visited_set: HashSet<String> = HashSet::new();
    visited_set.insert(start.to_string());

    let mut dist = 0;

    // Use level-order traversal to find the shortest path from start to end
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let curr_word = queue.pop_front().unwrap(); // Safe unwrap because queue is not empty
            // if we found the end word we've reacg it via shortest path
            if curr_word == end {
                return dist + 1;
            }

            // Generate all possible words that have one letter difference to current word
            for i in 0..curr_word.len() {
                for c in LOWERCASE_ALPHABET.chars() {
                    let mut next_word = curr_word.clone();
                    next_word.replace_range(i..=i, &c.to_string());

                    // if next_word is in dictionary it is a neighbor of current word
                    // if unvisited, add it to the queue to be processed in the next level
                    if dictionary_set.contains(next_word.as_str()) && !visited_set.contains(&next_word) {
                        visited_set.insert(next_word.clone());
                        queue.push_back(next_word);
                    }
                }
            }
        }
        dist += 1;
    }
    // If there is no way to reach the end node, then no path exists
    0
}

fn main() {
    let dictionary = ["red", "bed", "hat", "rod", "rad", "rat", "hit", "bad", "bat"]
        .map(|s| s.to_string());

    let start = "red";
    let end = "hit";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 5

    let start = "red";
    let end = "red";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 1

    let start = "Zoubida";
    let end = "red";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 0
}

```

## Optimization

<span style="color:orange"><b>The point:</b></span>

* initiate the search from `start` and `end` words
* Bidirectional BFS
* Both searches meet when one find the word is in the visited set of the other search


**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n x L²)   | O(n x L)     |

* O(n x L²) in time because 2 level of traversal. BUT... more efficient in practice since there are fewer nodes to traverse when using bidirectional
* O(n x L) in space because space taken by the dictionary hash set, both visited hash sets and both queues

**About Rust :**
<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::{HashSet, VecDeque};

const LOWERCASE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

// This function now uses HashSet<&str> instead of HashSet<String>
fn shortest_transformation_sequence(start: &str, end: &str, dictionary: &[String]) -> usize {
    // Build a set of &str from the dictionary which contains String. No lifetime issue
    let dictionary_set: HashSet<&str> = dictionary.iter().map(|s| s.as_str()).collect();

    if !dictionary_set.contains(start) || !dictionary_set.contains(end) {
        return 0;
    }

    if start == end {
        return 1;
    }

    // queues stores owned Strings since we need to mutate each word
    let mut start_queue: VecDeque<String> = VecDeque::new();
    start_queue.push_back(start.to_string());

    let mut end_queue: VecDeque<String> = VecDeque::new();
    end_queue.push_back(end.to_string());

    let mut start_visited_set: HashSet<String> = HashSet::new();
    start_visited_set.insert(start.to_string());

    let mut end_visited_set: HashSet<String> = HashSet::new();
    end_visited_set.insert(end.to_string());

    let (mut level_start, mut level_end) = (0, 0);

    // Perform a level-order traversal from start and end word
    while !start_queue.is_empty() && !end_queue.is_empty() {
        // Explore the next level of the traversal that start from start word
        // If it meets the other traversal, the shortest pathfrom start to end is found
        level_start +=1;
        if explore_level(&mut start_queue, &mut start_visited_set, &end_visited_set, &dictionary_set){
            return level_start + level_end + 1;
        }
        // Explore the next level of the traversal that start from the end word
        level_end +=1;
        if explore_level(&mut end_queue, &mut end_visited_set, &start_visited_set, &dictionary_set){
            return level_start + level_end + 1;
        }
    }
    // If the traversals never met, then no path exists
    0
}

fn explore_level(queue: &mut VecDeque<String>, visited_set: &mut HashSet<String>, other_visited_set: &HashSet<String>, dictionary_set : &HashSet<&str>) ->bool {
    for _ in 0..queue.len() {
        let curr_word = queue.pop_front().unwrap(); // Safe unwrap because queue is not empty
        // Generate all possible words that have one letter difference to current word
        for i in 0..curr_word.len() {
            for c in LOWERCASE_ALPHABET.chars() {
                let mut next_word = curr_word.clone();
                next_word.replace_range(i..=i, &c.to_string());

                // if next_word has been visited during the other traversal this means both search have met
                if other_visited_set.contains(&next_word){
                    return true;
                }

                if dictionary_set.contains(next_word.as_str()) && !visited_set.contains(&next_word) {
                    visited_set.insert(next_word.clone());
                    queue.push_back(next_word);
                }
            }
        }
    }
    false
}

fn main() {
    let dictionary = ["red", "bed", "hat", "rod", "rad", "rat", "hit", "bad", "bat"]
        .map(|s| s.to_string());

    let start = "red";
    let end = "hit";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 5

    let start = "red";
    let end = "red";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 1

    let start = "Zoubida";
    let end = "red";
    println!("Shortest sequence = {}", shortest_transformation_sequence(start, end, &dictionary)); // 0
}
```
