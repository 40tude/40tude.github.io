---
# published: false
layout: default
lang: en-US
title: "p311 - Climbing Stairs"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Climbing Stairs

<div align="center">
<img src="../assets/chap_15.webp" alt="" width="300" loading="lazy"/>
</div>

* Determine the number of distinct ways to climb a staircase of n steps by taking either 1 or 2 steps at a time

## Top-Down

<span style="color:orange"><b>The point:</b></span>

* To reach stair `i` we come from stair `i-1` or stair ``i-2``
* To solve `climbing_stairs(n)` we need to solve 2 subproblems `climbing_stairs(n-1)` and `climbing_stairs(n-2)`
* Recursive
* Overlapping subproblems is solved with **memoization** (store result of subproblems as they are solve, avoid to recalculate). Hash map (constant time access)




**Phases of a DP solution :**
1. Subproblem
1. DP Formula
1. Base Case
1. Populate DP Table




**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n)         | O(n)      |

* O(n) in time because of memoization(O(2^n) otherwise because the depth of recursion tree is n and branching factor is 2). 
* O(n) in space because of depth of the recursive call stack (can grow up to n)


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
use std::collections::HashMap;

fn climbing_stairs_top_down(n :usize) -> usize {
    let mut memo = HashMap::new();
    climbing_stairs(n, &mut memo)
}

fn climbing_stairs(n : usize, memo : &mut HashMap<usize, usize>) -> usize{
    // base case 
    // with a 1-step staircase => only 1 way to climb
    // with a 2-steps staircase => only 2 ways to climb
    if n<=2{
        n
    } else if let Some(&nb_ways) = memo.get(&n) {    
        nb_ways
    } else {
        // The number of ways to climb to the n-th step is the sum of ways to reach n-1 and n-2
        let nb_ways = climbing_stairs(n-1, memo) + climbing_stairs(n-2, memo);
        memo.insert(n, nb_ways);
        nb_ways
    }
}

// fn main() { // no main() if this code runs in a Jupyter cell

    println!("{}", climbing_stairs_top_down(4)); // 5
    
// } // end of local scope OR end of main()
```

    5


## Bottom-Up

<span style="color:orange"><b>The point:</b></span>

* Any Top-Down + memoization solution can be implemented as Bottom-Up + a DP array

**Complexity :**

| Time         | Space     |
|--------------|-----------|
| O(n)         | O(n)      |

* O(n) in time because iterate over n elements of DP array
* O(n) in space because space taken by DP array


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





```rust
fn climbing_stairs_bottom_up(n : usize) -> usize{
    // base case 
    // with a 1-step staircase => only 1 way to climb
    // with a 2-steps staircase => only 2 ways to climb
    if n<=2{
        n
    }else{
        let mut dp:Vec<usize> = vec![0; n+1]; // Allocation
        dp[1] = 1; // Initialization
        dp[2] = 2;

        // Starting from step 3, calculate the number of ways to reach each step until n-th
        for i in 3..n+1{
            dp[i] = dp[i-1] + dp[i-2];
        }
        dp[n]
    }
}

// fn main() { // no main() if this code runs in a Jupyter cell

    println!("{}", climbing_stairs_bottom_up(4)); // 5
    
// } // end of local scope OR end of main()
```

    5


## Optimization - Bottom-Up 

<span style="color:orange"><b>The point:</b></span>

* No need to store the entire DP array 
* Only 2 values `dp[i-1]` (`one_step_before`) and `dp[i-2]` (`two_step_before`)

**About Rust :**
* First implementation
* <span style="color:lime"><b>Preferred solution?</b></span>     
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
fn climbing_stairs_bottom_up_optimized(n : usize) -> usize{
    if n<=2{
        n
    } else{
        let (mut one_step_before, mut two_step_before) = (2, 1);

        // Starting from step 3, calculate the number of ways to reach each step until n-th
        for _ in 3..n+1{
            let current = one_step_before + two_step_before;
            two_step_before = one_step_before;
            one_step_before = current;
        }
        one_step_before
    }
}

// fn main() { // no main() if this code runs in a Jupyter cell

    println!("{}", climbing_stairs_bottom_up_optimized(4)); // 5
    
// } // end of local scope OR end of main()
```

    5

