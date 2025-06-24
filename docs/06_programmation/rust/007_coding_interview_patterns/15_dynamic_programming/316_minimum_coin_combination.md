---
# published: false
layout: default
title: "p316 - Minimum Coin Combination"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Minimum Coin Combination

* You are given an array of coin values and a target amount
* Return the minimum number of coins needed
* If not possible return -1
* Assume unlimited supply of each coin

## Top-Down

<span style="color:orange"><b>The point:</b></span>

* Brute force impossible because infinite number of each coin
* If we peek one coin of value v, then the next subproblem to solve is to reach target-v 
* Recursive
* min_coin_combination(target) = 1 + min(min_coin_combination (target - coin_i)) and coin_i is one of the coins value
* Base case when target = 0 => return 0
* Memoization to solve overlapping subproblems




**Phases of a DP solution :**
1. Subproblem
1. DP Formula
1. Base Case
1. Populate DP Table




**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(target x n)  | O(target) |

* O(target x n) in time thanks to the memoization. Each subproblem is solved once. There are at most target subproblem and we iterate over n coins => O(target x n)
* O(target) in space because of size o memoization structure stores up to target key-values pairs (size of recursive call stack is target/m where m is the smallest coin value)


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
use std::collections::HashMap;

fn top_down_dp(coins: &[usize], target : usize, memo : &mut HashMap<usize, usize>) -> usize{
    if target == 0{
        0
    }else if let Some(&nb_ways) = memo.get(&target) {    
        nb_ways
    } else {
        // Initialize min_coins to a large number
        let mut min_coins = usize::MAX;
        for coin in coins{
            if *coin <= target {
                min_coins = min_coins.min(1+top_down_dp(coins, target-coin, memo));
            }
        }
        memo.insert(target, min_coins);
        min_coins
    }
}

fn min_coin_combination_top_down(coins : &[usize], target : usize) -> Option<usize> {
    let mut memo = HashMap::new();
    let res = top_down_dp(coins, target, &mut memo);
    match res{
        usize::MAX=>None,
        _ => Some(res)
    }
}

fn main() { // no main() if this code runs in a Jupyter cell

    let coins = vec![1,2,3];
    let target = 5; // Some(2)
    println!("{:?}", min_coin_combination_top_down(&coins, target)); // Some(2)

    let coins = vec![42, 18];
    let target = 5; // None
    println!("{:?}", min_coin_combination_top_down(&coins, target)); // None
} // end of local scope OR end of main()
```

    Some(2)
    None


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

