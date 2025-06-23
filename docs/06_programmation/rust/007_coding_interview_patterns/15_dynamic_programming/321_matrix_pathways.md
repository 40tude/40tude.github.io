---
# published: false
layout: default
title: "p321 - Matrix Pathways"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Matrix Pathways

* You are positioned at top-left corner of m x n matrix (row, col)
* Only move downward or to the right
* Determine the # of unique pathways to bottom-right corner
* Assume `m` and `n` >=1

<span style="color:orange"><b>The point:</b></span>

* Think backward
* `matrix_pathways(r, c) = matrix_pathways(r-1, c) + matrix_pathways(r, c-1)`
*  `dp[0][0]` should be 1. row 0 and col 0 should be 1 




**Phases of a DP solution :**
1. Subproblem
1. DP Formula
1. Base Case
1. Populate DP Table




**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(m x n)       | O(m x n) |

* O(m x n) in time because each cell in DP table is populated once
* O(m x n) in space because of size of DP table


**About Rust :**
* First translation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
fn matrix_pathways(m : usize, n : usize) -> usize {
    // Base case. Set cells in row 0 and col 0 to 1
    // We can set all cells in DP table to 1
    let mut dp = vec![vec![1; n]; m];

    // File in the rest of the DP table
    for r in 1..m{
        for c in 1..n{
            // Paths to current cell = paths from above + paths from left
            dp[r][c] = dp[r-1][c] + dp[r][c-1];
        }
    }
    dp[m-1][n-1]
}

// fn main() { // no main() if this code runs in a Jupyter cell
    let (m, n) = (3, 3);
    println!("{}", matrix_pathways(m , n)); // 6
// } // end of local scope OR end of main()
```

    6


## Optimization

<span style="color:orange"><b>The point:</b></span>

* For each cell in the DP table we only need access to cells above and on the left
* Maintain 2 rows : `prev_row` and `curr_row` to access `dp[r-1][c]` and `dp[r][c-1]`



**About Rust :**
* First translation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





```rust
fn matrix_pathways(m : usize, n : usize) -> usize {
    // Initialize `prev_row` as the DP values of row 0 (all 1s)
    let mut prev_row = vec![1; n];

    // Iterate trough the matrix starting from 1
    for _ in 1..m{
        // Set the first cell of curr_row to 1 (set the entire row to 1)
        let mut curr_row = vec![1; n];
        for c in 1..n{
            // Number of unique paths to current cell is 
            // the sum of paths to the cell above it (`prev_row[c]`) and the sum of path to the cell on the left (`curr_row[c-1]`)
            curr_row[c] = prev_row[c] + curr_row[c-1];
        }
        prev_row = curr_row;
    }
    prev_row[n-1]
}

// fn main() { // no main() if this code runs in a Jupyter cell
    let (m, n) = (3, 3);
    println!("{}", matrix_pathways(m , n)); // 6
// } // end of local scope OR end of main()
```

    6

