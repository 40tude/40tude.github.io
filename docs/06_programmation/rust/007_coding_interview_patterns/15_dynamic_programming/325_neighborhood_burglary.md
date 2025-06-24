---
# published: false
layout: default
title: "p325 - Neighborhood Burglary"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Neighborhood Burglary

* You are plan to rob houses in a street
* Each house store a certain amount of money
* When 2 adjacent houses are robed, the alarm is set
* Return the max amount of money without triggering the alarms


<span style="color:orange"><b>The point:</b></span>

* Greedy algo where we start with largest amount fail because it may miss the long term objective
* Start by the end
* Start by the last house (i). What is the amount of money stolen so far ?
    * If i is skipped the result is the one at home i-1. `profit = total_stolen(i-1)`
    * If we rob, we can rob i-1. `profit = house(i) + total_stolen(i-2)`
* `dp[i] = dp[i-1].max(house[i] + dp[i-2])` => set val for index 0 and 1
* Best case = 1 house. `dp[0] = house[0]`




**Phases of a DP solution :**
1. Subproblem
1. DP Formula
1. Base Case
1. Populate DP Table




**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n)           | O(n)      |

* O(n) in time because each cell in DP table is populated once
* O(n) in space because of size of DP table


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn neighborhood_burglary(houses : &[usize]) -> usize {
    // Handle cases where size is less than 2
    if houses.is_empty(){
        0
    }else if houses.len()==1{
        houses[0]
    }else{
        let mut dp = vec![0; houses.len()];
        // Base case. Only one house
        dp[0] = houses[0];
        // Base case. Only 2 houses
        dp[1] = houses[0].max(houses[1]);
        // Fill the rest of DP array
        for i in 2..houses.len(){
            dp[i] = dp[i-1].max(houses[i] + dp[i-2]);
        }
        dp[houses.len()-1]
    }
}

// fn main() { // no main() if this code runs in a Jupyter cell
    let houses = vec![200, 300, 200, 50];
    println!("{}", neighborhood_burglary(&houses)); // 400
// } // end of local scope OR end of main()
```

    400


## V2

**About Rust :**
* Prefer `match` to `if`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn neighborhood_burglary(houses: &[usize]) -> usize {
    match houses.len() {
        0 => 0,
        1 => houses[0],
        _ => {
            let mut dp = vec![0; houses.len()];
            dp[0] = houses[0];
            dp[1] = houses[0].max(houses[1]);
            for i in 2..houses.len() {
                dp[i] = dp[i - 1].max(houses[i] + dp[i - 2]);
            }
            dp[houses.len() - 1]
        }
    }
}

fn main() { // no main() if this code runs in a Jupyter cell
    let houses = vec![200, 300, 200, 50];
    println!("{}", neighborhood_burglary(&houses)); // 400
} // end of local scope OR end of main()
```

## Optimization

<span style="color:orange"><b>The point:</b></span>

* Only need to access `dp[i - 1]` and `dp[i - 2]`
* No need  to store the entire DP array
* `prev_max_profit` to store `dp[i-1]` and `prev_prev_max_profit` to store `dp[i-2]`

**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n)           | O(1)      |


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





```rust
fn neighborhood_burglary(houses: &[usize]) -> usize {
    match houses.len() {
        0 => 0,
        1 => houses[0],
        _ => {
            // Initialize the variables with base cases 
            let mut prev_max_profit = houses[0].max(houses[1]);
            let mut prev_prev_max_profit = houses[0];

            for house in houses.iter().skip(2){
                let curr_max_profit = prev_max_profit.max(house+ prev_prev_max_profit);
                // Update the values for next iteration
                prev_prev_max_profit = prev_max_profit;
                prev_max_profit = curr_max_profit;
            }
            prev_max_profit
        }
    }
}

fn main() { // no main() if this code runs in a Jupyter cell
    let houses = vec![200, 300, 200, 50];
    println!("{}", neighborhood_burglary(&houses)); // 400
} // end of local scope OR end of main()
```
