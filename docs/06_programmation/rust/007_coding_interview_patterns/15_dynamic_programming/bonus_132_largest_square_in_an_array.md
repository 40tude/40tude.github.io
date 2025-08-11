---
# published: false
layout: default
lang: en-US
title: "bonus132 - Largest Square in a Matrix"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Largest Square in a Matrix

<div align="center">
<img src="../assets/chap_15.webp" alt="" width="300" loading="lazy"/>
</div>

* Determine the area of the largest square of 1's in a binary matrix.



<span style="color:orange"><b>The point:</b></span>

* Squares contain smaller squares inside them
* The length of a square that ends at current cell depends on the lengths of the squares that end at its left, top, and top-left neighboring cells
* `if matrix[i][j] == 1 then max_square(i, j) = 1 + min(max_square(i - 1, j), max_square(i - 1, j - 1), max_square(i, j - 1))`
* Base case : we can set all cells in row 0 and column 0 to 1 in our DP table

**Phases of a DP solution :**
1. Subproblem
1. DP Formula
1. Base Case
1. Populate DP Table




**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(m x n)    | O(m x n)     |

* O(m x n) in time because each cell of the m x n DP table is populated once
* O(m x n) in space because the DP table is m x n


**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
use std::cmp;

fn largest_square_in_a_matrix(matrix: &[Vec<i32>]) -> i32 {

    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let (m, n) = (matrix.len(), matrix[0].len());
    let mut dp = vec![vec![0; n]; m];
    let mut max_len = 0;
    // Base case : If a cell in row 0 is 1, the largest square ending there has a length of 1
    for j in 0..n{
        if matrix[0][j] == 1{
            dp[0][j] = 1;
            max_len = 1;
        }
    }
    // Base case : If a cell in column 0 is 1, the largest square ending there has a length of 1.
    for i in 0..m{
        if matrix[i][0] == 1{
            dp[i][0] = 1;
            max_len = 1;
        }
    }
    // Populate the rest of the DP table.
    for i in 1..m{
        for j in 1..n{
            if matrix[i][j] == 1{
                // The length of the largest square ending here is determined by the smallest square ending at the neighboring cells 
                // (left,top-left, top), plus 1 to include this cell.
                dp[i][j] = 1 + cmp::min(dp[i - 1][j], cmp::min(dp[i - 1][j - 1], dp[i][j - 1]));
                max_len = max_len.max(dp[i][j]);
            }
        }
    }
    max_len*max_len
}

fn main() { // no main() if this code runs in a Jupyter cell
    let matrix = vec![
        vec![1, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0],
        vec![1, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1],
    ];
    println!("{}", largest_square_in_a_matrix(&matrix)); // 9
} // end of local scope OR end of main()
```

## Optimization

<span style="color:orange"><b>The point:</b></span>

* For each cell in the DP table, we only need to access the cell directly above it, the cell to its left, and the top-left diagonal cell
* So we only need access to the previous row and for the cell to its left, we look at the cell to the left in current row 
* We need to maintain two rows: `prev_row` and `curr_row`


**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(m x n)    | O(m)     |

* O(m x n) in time because each cell of the m x n DP table is populated once
* O(m) in space because the DP table is now of size m


**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)







<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





```rust
use std::cmp;

fn largest_square_in_a_matrix_optimized(matrix: &[Vec<i32>]) -> i32 {

    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let (m, n) = (matrix.len(), matrix[0].len());
    let mut prev_row = vec![0; n];
    let mut max_len = 0;
    // Iterate through the matrix.
    for i in 0..m{
        let mut curr_row = vec![0; n];
        for j in 0..n {
            // Base cases: if we’re in row 0 or column 0, the largest square ending
            // here has a length of 1. This can be set by using the value in the input matrix
            if i==0 || j==0{
               curr_row[j] = matrix[i][j]; 
            }else if matrix[i][j] == 1 {
                // curr_row[j] = 1 + min(left, top-left, top)
                curr_row[j] = 1 + cmp::min(curr_row[j - 1], cmp::min(prev_row[j - 1], prev_row[j]));
            }
            max_len = max_len.max(curr_row[j]);
        }
        // Update 'prev_row' with 'curr_row' values for the next iteration.
        prev_row = curr_row;
        // curr_row = vec![0; n]; // not needed here
    }
    max_len*max_len
}

fn main() { // no main() if this code runs in a Jupyter cell
    let matrix = vec![
        vec![1, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0],
        vec![1, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1],
    ];
    println!("{}", largest_square_in_a_matrix_optimized(&matrix)); // 9
} // end of local scope OR end of main()
```

## V2


```rust
use std::cmp;

fn largest_square_in_a_matrix_optimized(matrix: &[Vec<i32>]) -> i32 {

    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let (m, n) = (matrix.len(), matrix[0].len());
    let mut prev_row = vec![0; n];
    let mut max_len = 0;
    // Iterate through the matrix.
    for (i, row) in matrix.iter().enumerate().take(m){
        let mut curr_row = vec![0; n];
        for j in 0..n {
            // Base cases: if we’re in row 0 or column 0, the largest square ending
            // here has a length of 1. This can be set by using the value in the input matrix
            if i==0 || j==0{
                curr_row[j] = row[j]; 
            }else if row[j] == 1 {
                curr_row[j] = 1 + cmp::min(curr_row[j - 1], cmp::min(prev_row[j - 1], prev_row[j]));
            }
            max_len = max_len.max(curr_row[j]);
        }
        // Update 'prev_row' with 'curr_row' values for the next iteration.
        prev_row = curr_row;
    }
    max_len*max_len
}

fn main() { // no main() if this code runs in a Jupyter cell
    let matrix = vec![
        vec![1, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 0],
        vec![1, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1],
    ];
    println!("{}", largest_square_in_a_matrix_optimized(&matrix)); // 9
} // end of local scope OR end of main()
```
