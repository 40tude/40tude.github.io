---
# published: false
layout: default
lang: en-US
title: "p268 - Longest Increasing Path"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Longest Increasing Path

<div align="center">
<img src="../assets/chap_13.webp" alt="" width="300" loading="lazy"/>
</div>

* Find the longest strickly increasing path in a matrix of u32
* A path is a sequence of cells adjacent in the 4 directions

<span style="color:orange"><b>The point:</b></span>

* strickly => does not include 2 cells of same value
* DAG = directed acyclic graph
* DFS to traverse the matrix
* memoization


**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(m x n)    | O(m x n)     |

* O(m x n) in time because each cell is visited at most twice
* O(m x n) in space because the size of the recursice call stack during DFS and because of memoization table (both can grow up to m x n) 









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First implementation

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
fn longest_increasing_path(matrix: &[Vec<i32>]) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut res = 0;
    let (m, n) = (matrix.len(), matrix[0].len()); // m = row, n = col
    let mut memo = vec![vec![0; n]; m];
    // find the longest increasing path starting at each cell
    // the max is of these equal to the overall LIP
    for r in 0..m {
        for c in 0..n {
            res = res.max(dfs(r, c, matrix, &mut memo));
        }
    }
    res
}

fn dfs(r: usize, c: usize, matrix: &[Vec<i32>], memo: &mut [Vec<i32>]) -> i32 {
    if memo[r][c] != 0 {
        memo[r][c]
    } else {
        let mut max_path = 1;
        let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // the longest path starting at the current cell is equal to the longest path of its larger neighboring cells plus 1
        for (dr, dc) in &dir { // by reference
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;
            

            if is_within_bounds(new_r, new_c, matrix) {
                let (new_r_usize, new_c_usize) = (new_r as usize, new_c as usize);
                if matrix[new_r_usize][new_c_usize] > matrix[r][c] {
                    max_path = max_path.max(1 + dfs(new_r_usize, new_c_usize, matrix, memo));
                }
            }
        }
        memo[r][c] = max_path;
        max_path
    }
}

fn is_within_bounds(r: isize, c: isize, matrix: &[Vec<i32>]) -> bool {
    r >= 0 && c >= 0 && (r as usize) < matrix.len() && (c as usize) < matrix[0].len()
}

fn main() {
    let matrix = vec![vec![1, 5, 8], vec![3, 4, 4], vec![7, 9, 2]];
    println!("LIP = {}", longest_increasing_path(&matrix)); // 5

    let empty_matrix: Vec<Vec<i32>> = Vec::new();
    println!("LIP = {}", longest_increasing_path(&empty_matrix)); // 0
}
```

## V2


**About Rust :**
* ``dir[]`` is now a constant
* The calls to ``is_within_bounds()`` has been suppressed  
* length is `usize` not `i32`
* Watch out : `for &(dr, dc) in &DIRS`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn longest_increasing_path(matrix: &[Vec<i32>]) -> usize {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut res = 0;
    let (m, n) = (matrix.len(), matrix[0].len()); // m = row, n = col
    let mut memo = vec![vec![0; n]; m];
    // find the longest increasing path starting at each cell
    // the max is of these equal to the overall LIP
    for r in 0..m {
        for c in 0..n {
            res = res.max(dfs(r, c, matrix, &mut memo));
        }
    }
    res
}

fn dfs(r: usize, c: usize, matrix: &[Vec<i32>], memo: &mut [Vec<usize>]) -> usize {
    if memo[r][c] != 0 {
        return memo[r][c];
    } 

    let mut max_path = 1;

    // the longest path starting at the current cell is equal to the longest path of its larger neighboring cells plus 1
    for &(dr, dc) in &DIRS {    // Take a reference to the table (&DIRECTIONS)
                                // Destructure and dereference each element (&(dr, dc))
                                // dr and dc are isize directly
        let (new_r, new_c) = (r as isize + dr, c as isize + dc);
        
        // no call to is_within_bounds()
        if new_r >= 0 && new_c >= 0 {
            let (new_r_usize, new_c_usize) = (new_r as usize, new_c as usize);
            if new_r_usize < matrix.len() && new_c_usize < matrix[0].len() && matrix[new_r_usize][new_c_usize] > matrix[r][c] {
                max_path = max_path.max(1 + dfs(new_r_usize, new_c_usize, matrix, memo));
            }
        }
    }
    memo[r][c] = max_path;
    max_path
}

fn main() {
    let matrix = vec![vec![1, 5, 8], vec![3, 4, 4], vec![7, 9, 2]];
    println!("LIP = {}", longest_increasing_path(&matrix)); // 5

    let empty_matrix: Vec<Vec<i32>> = Vec::new();
    println!("LIP = {}", longest_increasing_path(&empty_matrix)); // 0
}
```
