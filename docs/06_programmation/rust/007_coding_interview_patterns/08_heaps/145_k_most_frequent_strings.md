---
# published: false
layout: default
lang: en-US
title: "p145 - K Most Frequent Strings"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# K Most Frequent Strings

<div align="center">
<img src="../assets/chap_08.webp" alt="" width="300" loading="lazy"/>
</div>

* Find the ``k`` most frequent string in an array
* Return them sorted by desc freq
* If 2 strings have same freq sort them in lexicographical order

<span style="color:orange"><b>The point:</b></span>

* Use a max-heap


**Complexity :**

| Time               | Space |
|--------------------|-------|
| O(n + klog(n))     | O(n)  |
| O(n + u + klog(u)) | O(n)  |

* O(n + klog(n)) because it takes O(n) to count freq. We pop off the heap k times => O(logn(n)) 
* More precisely : O(n + u + klog(u)) : O(n) to count freq, O(u) to build a heap with u unique elements, klog(u) for extractions
* Space : O(n) because hashmap and heap store at most n pairs 

**About Rust :**
* In Rust, a BinaryHeap is max-heap by default (while in Python a heap is min-heap by default)
* Ord implies PartialOrd, so we must implement PartialOrd if we implement Ord
* Ord, total order, means that all pairs of elements are comparable with each other, and that the comparison is consistent
* PartialOrd : Not all elements are necessarily comparable. Example : Floating numbers (f32, f64) with NaN. Result is Some(Ordering) or None
* In ``struct Pair``, if there is a ``&str`` we **MUST** confirm to the compiler that the reference will live as long as the Pair => lifetime specifier
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## First implementation with Strings


```rust
use std::collections::{BinaryHeap, HashMap}; // In Rust, BinaryHeap is a max-heap by default
use std::cmp::Ordering;

// Define a pair struct to hold the string and its frequency
#[derive(Eq, PartialEq)]    // BinaryHeap => Eq, PartialEq, Ord, PartialOrd must be implemented
                            // Here leverage Eq and PartialEq which compare all fields
struct Pair {
    string: String,
    freq: usize,
}

// BinaryHeap => we must explain how to compare 2 objects
// Implement custom ordering for Pair
// In Rust, Ord implies PartialOrd, so we must implement PartialOrd if we implement Ord.
// Total order means that all pairs of elements are comparable with each other, and that the comparison is consistent.
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {        // Ordering::Less Ordering::Equal or Ordering::Greater
        // Prioritize lexicographical order for strings with equal frequencies
        if self.freq == other.freq {
            // Reverse the lexicographical order to match Python's heapq behavior
            // (min-heap via __lt__)
            other.string.cmp(&self.string) // rather than self.string.cmp(&other.string) 
        } else {
            // Primary comparison: frequency (ascending order)
            self.freq.cmp(&other.freq)
        }
    }
}

// Implement PartialOrd in terms of Ord
// PartialOrd : Not all elements are necessarily comparable.
// Example : Floating numbers (f32, f64) with NaN
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Returns the k most frequent strings using a max-heap
fn k_most_frequent_strings_max_heap(strs: &[String], k: usize) -> Vec<String> {
    // Count the frequency of each string using a HashMap
    let mut freqs: HashMap<&String, usize> = HashMap::new();
    for s in strs {
        *freqs.entry(s).or_insert(0) += 1;
    }

    // Build the heap with custom Pair items
    let mut max_heap: BinaryHeap<Pair> = freqs
        .into_iter()
        .map(|(s, freq)| Pair {
            string: s.clone(),
            freq,
        })
        .collect();

    // Extract the top k elements
    let mut result = Vec::new();
    for _ in 0..k {
        if let Some(pair) = max_heap.pop() {
            result.push(pair.string);
        }
    }

    result
}


fn main(){     // no main() if this code runs in a Jupyter cell 
    let strs = vec!["go".to_string(),
                    "coding".to_string(), 
                    "byte".to_string(), 
                    "byte".to_string(), 
                    "go".to_string(), 
                    "interview".to_string(), 
                    "go".to_string()];

    println!("{:?}", k_most_frequent_strings_max_heap(&strs, 2));  // ["go", "byte"]
} // end of local scope OR end of main()       
```

## Max-Heap : Use ``&str`` but does NOT work

* <span style="color:red"><b>Does not work (yet)</b></span> 
* In ``struct Pair``, if there is a ``&str`` we **MUST** confirm to the compiler that the reference will live as long as the Pair
    * This is done with 
```rust
struct Pair<'a> {
    string: &'a str,
    freq: usize,
}

```
* Once the lifetime is specified in the ``Pair`` it must be propagated to the rest of the code
    * See compiler errors


```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

// Define a pair struct to hold the string slice and its frequency
#[derive(Eq, PartialEq)]
struct Pair {
    string: &str,
    freq: usize,
}

// Implement custom ordering for Pair
impl Ord for Pair{
    // Will be call as A.cmp(B) where A and B are 2 pairs
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize lexicographical order for strings with equal frequencies
        if self.freq == other.freq {
            // Reverse lex order to match Python's min-heap behavior
            other.string.cmp(self.string)
        } else {
            self.freq.cmp(&other.freq)
        }
    }
}

// Implement PartialOrd in terms of Ord
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Returns the k most frequent strings using a max-heap
fn k_most_frequent_strings_max_heap<'a>(strs: &[&str], k: usize) -> Vec<&str> {
    // Count the frequency of each string using a HashMap
    let mut freqs: HashMap<&str, usize> = HashMap::new();
    for &s in strs {
        *freqs.entry(s).or_insert(0) += 1;
    }

    // Build the heap with custom Pair items
    let mut max_heap: BinaryHeap<Pair> = freqs
        .into_iter()
        .map(|(s, freq)| Pair { string: s, freq })
        .collect();

    // Extract the top k elements
    let mut result = Vec::new();
    for _ in 0..k {
        if let Some(pair) = max_heap.pop() {
            result.push(pair.string);
        }
    }

    result
}


fn main(){     // no main() if this code runs in a Jupyter cell 
    let strs = vec!["go", "coding", "byte", "byte", "go", "interview", "go"];
    println!("{:?}", k_most_frequent_strings_max_heap(&strs, 2));  // 
} // end of local scope OR end of main()       
            
```

## Max-Heap : Use ``&str`` (with lifetime specifier)

* <span style="color:lime"><b>Preferred solution?</b></span> 


```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

// Define a pair struct to hold the string slice and its frequency
#[derive(Eq, PartialEq)]
struct Pair<'a> {
    string: &'a str,
    freq: usize,
}

// Implement custom ordering for Pair
impl<'a> Ord for Pair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize lexicographical order for strings with equal frequencies
        if self.freq == other.freq {
            // Reverse lex order to match Python's min-heap behavior
            other.string.cmp(self.string)
        } else {
            self.freq.cmp(&other.freq)
        }
    }
}

// Implement PartialOrd in terms of Ord
impl<'a> PartialOrd for Pair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Returns the k most frequent strings using a max-heap
fn k_most_frequent_strings_max_heap<'a>(strs: &'a [&'a str], k: usize) -> Vec<&'a str> {
    
    // Count the frequency of each string using a HashMap
    let mut freqs: HashMap<&'a str, usize> = HashMap::new();
    for &s in strs {
        *freqs.entry(s).or_insert(0) += 1;
    }

    // Build the heap with custom Pair items
    let mut max_heap: BinaryHeap<Pair> = freqs
        .into_iter()
        .map(|(s, freq)| Pair { string: s, freq })
        .collect();

    // Extract the top k elements
    let mut result = Vec::new();
    for _ in 0..k {
        if let Some(pair) = max_heap.pop() {
            result.push(pair.string);
        }
    }

    result
}


fn main(){     // no main() if this code runs in a Jupyter cell 
    let strs = vec!["go", "coding", "byte", "byte", "go", "interview", "go"];

    println!("{:?}", k_most_frequent_strings_max_heap(&strs, 2));  // ["go", "byte"]
} // end of local scope OR end of main()       
```

    2
    4
    0
    7


## Min-Heap

<span style="color:orange"><b>The point:</b></span>

* Use a min-heap to save heap space (store k and not n strings)

**Complexity :**

| Time               | Space |
|--------------------|-------|
| O(n log(k))        | O(n)  |

* O(n log(k)) because it takes O(n) to count freq. Populate the heap with n words. Each op is O(log(k)). Extract/pop k strings in O(klog(k)). Reverse the array in O(k). O(n) + O(nlog(k)) + O(klog(k))+ O(k) = O(nlog(k)) 
* Space : O(k) because hashmap and heap store at most n pairs 



```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

// Define a pair struct to hold the string slice and its frequency
#[derive(Eq, PartialEq)]
struct Pair<'a> {
    str_val: &'a str,
    freq: usize,
}

// Implement custom ordering for Pair
impl<'a> Ord for Pair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize lexicographical order for strings with equal frequencies
        if self.freq == other.freq {
            self.str_val.cmp(other.str_val) // rather than other.string.cmp(&self.string)
        } else {
            other.freq.cmp(&self.freq) // rather than self.freq.cmp(&other.freq)
        }
    }
}

// Implement PartialOrd in terms of Ord
impl<'a> PartialOrd for Pair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Returns the k most frequent strings using a max-heap
fn k_most_frequent_strings_min_heap<'a>(strs: &[&'a str], k: usize) -> Vec<&'a str> {
    
    // Count frequency of each string
    let mut freqs: HashMap<&str, usize> = HashMap::new();
    for s in strs {
        *freqs.entry(*s).or_insert(0) += 1;
    }

    let mut min_heap: BinaryHeap<Pair> = BinaryHeap::new();

    // Build the min-heap with top k frequent strings
    for (&s, &f) in freqs.iter() {
        min_heap.push(Pair { str_val: s, freq: f });
        if min_heap.len() > k {
            min_heap.pop();
        }
    }

    // Extract the top k elements
    let mut res = Vec::with_capacity(k);
    while let Some(p) = min_heap.pop() {
        res.push(p.str_val);
    }
    res.reverse(); // in place
    res
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    let strs = vec!["go", "coding", "byte", "byte", "go", "interview", "go"];

    println!("{:?}", k_most_frequent_strings_min_heap(&strs, 2));  // ["go", "byte"]
} // end of local scope OR end of main()       
```
