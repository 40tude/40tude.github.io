---
# published: false
layout: default
lang: en-US
title: "p253 - Count Islands"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Count Islands

<div align="center">
<img src="../assets/chap_13.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a binary matrix (1=land, 0=water) 
* Return the number of islands (connected land in the 4 directions)


<span style="color:orange"><b>The point:</b></span>

* One one cell of land is found, identify the rest using any graph traversal algo
* Here we use DFS
* Must mark cells as visited (val from 1 to -1)



**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(m x n)    | O(m x n)     |

* O(m x n) in time because the matrix is visited at most twice (search for land, DFS)
* O(m x n) in space because the size of the recursive stack (can grow up to m x n) 









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First implementation

**About Rust :**
* Keep in mind the input matrix is modified
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
fn count_islands(matrix: &mut [Vec<i32>]) -> usize {
    if matrix.is_empty() || matrix[0].is_empty() {
        0
    } else {
        let mut count = 0;
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                // If a land cell is found, perform DFS to explore the rest of the island
                if matrix[r][c] == 1 {
                    dfs(r as i32, c as i32, matrix);
                    count += 1;
                }
            }
        }
        count
    }
}

fn dfs(r: i32, c: i32, matrix: &mut [Vec<i32>]) {
    matrix[r as usize][c as usize] = -1;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    // Recursively call DFS on each neightbors
    for d in dirs {
        let next_r = r + d.0;
        let next_c = c + d.1;
        if is_within_bounds(next_r, next_c, matrix) && matrix[next_r as usize][next_c as usize] == 1
        {
            dfs(next_r, next_c, matrix);
        }
    }
}

fn is_within_bounds(r: i32, c: i32, matrix: &[Vec<i32>]) -> bool {
    r >= 0 && r < matrix.len() as i32 && c >= 0 && c < matrix[0].len() as i32
}

fn main() {
    let mut matrix = vec![
        vec![1, 1, 0, 0],
        vec![1, 1, 0, 0],
        vec![0, 0, 1, 1],
        vec![0, 0, 1, 1],
    ];
    println!("Number of islands = {}", count_islands(&mut matrix)); // 2
}
```

## V2

**About Rust :**
* `dfs()` works with `isize` indices. This allow to make calculations without complains from the compiler.
* In `dfs()` see how and where `r_usize` and `c_usize` are defined
* <span style="color:lime"><b>Preferred solution?</b></span>     
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn count_islands(matrix: &mut [Vec<i32>]) -> usize {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut count = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            // If a land cell is found, perform DFS to explore the rest of the island
            if matrix[r][c] == 1 {
                dfs(r as isize, c as isize, matrix);
                count += 1;
            }
        }
    }
    count
}

fn dfs(r: isize, c: isize, matrix: &mut [Vec<i32>]) {
    if !is_within_bounds(r, c, matrix) {
        return;
    }

    let (r_usize, c_usize) = (r as usize, c as usize);
    if matrix[r_usize][c_usize] != 1 {
        return;
    }

    // Mark the cell as visited
    matrix[r_usize][c_usize] = -1;

    // Recursively call DFS on each neighbors
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    // for (dr, dc) in dirs.iter() {       // .iter() create an iterator over references
    //                                     // this may seems overkill here because tuple implement copy but this is idiomatic
    //                                     // what would happen if the element of the array do not implement copy ?
        
    //     dfs(r + *dr, c + *dc, matrix);  // I dereference just to show I know I use references
    //                                     // here r + dr would be accepted by the compiler
    // }

    for (dr, dc) in &dirs {                 // traverse the array by reference. No copy
        dfs(r + dr, c + dc, matrix);        // r + *dr could be used
    }
}

fn is_within_bounds(r: isize, c: isize, matrix: &[Vec<i32>]) -> bool {
    // Rust compiler evaluates expressions as written
    // It may be useful to put : 
    //      * at the beginning, tests that are often false (or lite in terms of processing)
    //      * at the end, tests that are often true        (or heavy in terms of processing)
    r >= 0 && c >= 0 && (r as usize) < matrix.len() && (c as usize) < matrix[0].len()
}

fn main() {
    let mut matrix = [
        vec![1, 1, 0, 0],
        vec![1, 1, 0, 0],
        vec![0, 0, 1, 1],
        vec![0, 0, 1, 1],
    ];
    println!("Number of islands = {}", count_islands(&mut matrix)); // 2
}
```
