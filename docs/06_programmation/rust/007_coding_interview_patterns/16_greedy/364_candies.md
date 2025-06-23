---
# published: false
layout: default
title: "p364 - Candies"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Candies

* Children sitting in a row
* Distribute candies while abiding the rules
    * at least one candie per kid
    * if 2 children sit next to each other, the child with higher rating must receive more candies
* Determine the minimum number of candies needed


<span style="color:orange"><b>The point:</b></span>
* There's no rule that says: “If ratings are equal, then candies must be equal.”
* pass 1 : ensure that any kid with a higher rating than the one on their left receives more candies
* pass 2 : ensure that any kid with a higher rating than the one on their right receives more candies

**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we iterate twice over the children
* O(n) in space because the place needed for the candies









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* Translation

**About Rust :**
* `for i in range(n-2, -1, -1)` -> `for i in (0..=n-2).rev() {`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn candies(ratings: &[i32]) -> i32 {
    if ratings.is_empty() {
        return -1;
    }
    let n = ratings.len();
    // Ensure each child has 1 candy
    let mut candies = vec![1; n];
    // Pass 1 : ensure that any kid with a higher rating than the one on their left receives more candies
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    //  Pass 2 : ensure that any kid with a higher rating than the one on their right receives more candies
    for i in (0..=n-2).rev() {              // ! n-2 included 
        if ratings[i] > ratings[i + 1] {
            // If the kids already have more candies, do nothing
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }
    candies.iter().sum()
}

fn main() {
    // no main() if this code runs in a Jupyter cell
    let ratings = vec![4, 3, 2, 4, 5, 1];
    println!("{}", candies(&ratings)); // 12

    // i = 1: 3  > 1, so candies[1] > candies[0] → ok with [1, 2, 1]
    // i = 2: 3 == 3, no constraint → we can leave 1
    let ratings = vec![1, 3, 3];
    println!("{}", candies(&ratings)); // 4
} // end of local scope OR end of main()

```

## V2

* I don't like `ratings` as an array of ``i32``. Should be `u32`.
* `candies()` should return an `Option<u32>`

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn candies(ratings: &[u32]) -> Option<u32> {
    if ratings.is_empty() {
        return None;
    }
    let n = ratings.len();
    // Ensure each child has 1 candy
    let mut candies = vec![1; n];
    // Pass 1 : ensure that any kid with a higher rating than the one on their left receives more candies
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    //  Pass 2 : ensure that any kid with a higher rating than the one on their right receives more candies
    for i in (0..=n-2).rev() {              // ! 0..=n-2 
        if ratings[i] > ratings[i + 1] {
            // If the kids already have more candies, do nothing
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }
    Some(candies.iter().sum())
}

fn main() {
    // no main() if this code runs in a Jupyter cell
    let ratings = vec![4, 3, 2, 4, 5, 1];
    println!("{:?}", candies(&ratings)); // Some(12)

    // i = 1: 3  > 1, so candies[1] > candies[0] → ok with [1, 2, 1]
    // i = 2: 3 == 3, no constraint → we can leave 1
    let ratings = vec![1, 3, 3];
    println!("{:?}", candies(&ratings)); // Some(4)

    let ratings = vec![];
    println!("{:?}", candies(&ratings)); // None
} // end of local scope OR end of main()

```

## V3

* `let n = ratings.len();` can be removed
* I don't like the ``n-2`` in the second pass
* rewrite of the for loops

**About Rust :**
* <span style="color:lime"><b>Preferred solution?</b></span> 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn candies(ratings: &[u32]) -> Option<u32> {
    if ratings.is_empty() {
        return None;
    }
    // Ensure each child has 1 candy
    let mut candies = vec![1; ratings.len()];
    // Pass 1: ensure that any kid with a higher rating than the one on their left receives more candies
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }
    // Pass 2: ensure that any kid with a higher rating than the one on their right receives more candies
    for i in (0..ratings.len()-1).rev() {
        if ratings[i] > ratings[i + 1] {
            // If the kid already has more candies, do nothing
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }
    Some(candies.into_iter().sum()) // into_iter swalow candies
}

fn main() {
    // no main() if this code runs in a Jupyter cell
    let ratings = vec![4, 3, 2, 4, 5, 1];
    println!("{:?}", candies(&ratings)); // Some(12)

    // i = 1: 3  > 1, so candies[1] > candies[0] → ok with [1, 2, 1]
    // i = 2: 3 == 3, no constraint → we can leave 1
    let ratings = vec![1, 3, 3];
    println!("{:?}", candies(&ratings)); // Some(4)

    let ratings = vec![];
    println!("{:?}", candies(&ratings)); // None
} // end of local scope OR end of main()

```

## V4 (like Aprilia, Ducati...)

* A more functional way?

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
fn candies(ratings: &[u32]) -> Option<u32> {
    if ratings.is_empty() {
        return None;
    }
    let mut candies = vec![1; ratings.len()];
    
    // Pass 1: left to right
    ratings.windows(2)          // provides consecutive &[u32] slice of size 2
        .enumerate()            // .enumerate() provide pairs (usize, &[u32]) where the first element is the index of the pair
        .for_each(|(i, w)| {    // for_each() receives a tuple (i, w)
            if w[1] > w[0] {
                candies[i + 1] = candies[i] + 1;
            }
        });

    // Pass 2: right to left
    ratings.windows(2)
        .enumerate()
        .rev()                  // .rev() reverses iteration to pass from right to left.
        .for_each(|(i, w)| {
            if w[0] > w[1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        });

    Some(candies.into_iter().sum())
}

fn main() { // no main() if this code runs in a Jupyter cell
    
    let ratings = vec![4, 3, 2, 4, 5, 1];
    println!("{:?}", candies(&ratings)); // Some(12)

    let ratings = vec![1, 3, 3];
    println!("{:?}", candies(&ratings)); // Some(4)

    let ratings = vec![];
    println!("{:?}", candies(&ratings)); // None
} // end of local scope OR end of main()

```
