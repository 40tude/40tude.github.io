---
# published: false
layout: default
lang: en-US
title: "p035 - Verify Sudoku Board"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Verify Sudoku Board

<div align="center">
<img src="../assets/chap_02.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a partially completed 9x9 Sudoku board, validate it.

<span style="color:orange"><b>The point:</b></span>    

We use a hash set for each row, column, and square.
* The `cell (j, i)` belongs to `row j`, `column i`, and square `[j // 3][i // 3] (i, j ∈ [0, 8])`

Create
* 9 hash sets for rows
* 9 hash sets for columns
* and 9 hash sets for squares.

Iterate over each cell and add the value to each corresponding hash set.
* If a value is already present in any hash set, we return False.

We'll iterate over each cell:
    - Check if the number already exists in the respective row, column, or square.
    - If any duplicate is found, the Sudoku board is invalid.


**Complexity Analysis :**

| Time | Space |
|------|-------|
| O(n²) | O(n²)  |

- Because we iterate over each cell of the board once and perform operations on hash sets in O(1).
- We allocate three arrays (rows, cols, grids) of size n (9 here), each containing n (9 here) elements.





**About Rust :**
* `use std::collections::HashSet;`
* `if my_vect_of_hashset[r].contains(&val)`
    * a reference as argument
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





```rust
use std::collections::HashSet;

// IMO, a 9x9 2D array better expresses what a Sudoku board is (rather than a list of lists)
fn verify_sudoku_board(board: &[[u8; 9]; 9]) -> bool {
    let mut row_sets: Vec<HashSet<u8>> = vec![HashSet::new(); 9];
    let mut col_sets: Vec<HashSet<u8>> = vec![HashSet::new(); 9];
    let mut grid_sets: Vec<Vec<HashSet<u8>>> = vec![vec![HashSet::new(); 3]; 3];

    for r in 0..9 {
        for c in 0..9 {
            let num = board[r][c];
            if num == 0 {
                continue;
            }

            if row_sets[r].contains(&num)
                || col_sets[c].contains(&num)
                || grid_sets[r / 3][c / 3].contains(&num)
            {
                return false;
            }

            row_sets[r].insert(num);
            col_sets[c].insert(num);
            grid_sets[r / 3][c / 3].insert(num);
        }
    }
    true
}
                
                
// fn main(){     // no main() if this code runs in a Jupyter cell 
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    
    println!("{:?}", verify_sudoku_board(&board));  // true
    
    
    let board = [
        [3, 0, 6, 0, 5, 8, 4, 0, 0],
        [5, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 7, 0, 0, 0, 0, 3, 1],
        [1, 0, 2, 5, 0, 0, 3, 2, 0],
        [9, 0, 0, 8, 6, 3, 0, 0, 5],
        [0, 5, 0, 0, 9, 0, 6, 0, 0],
        [0, 3, 0, 0, 0, 8, 2, 5, 0],
        [0, 1, 0, 0, 0, 0, 0, 7, 4],
        [0, 0, 5, 2, 0, 6, 0, 0, 0],
    ];
    
    println!("{:?}", verify_sudoku_board(&board))  // false
// }
```

    true
    false





    ()



## V2 

**About Rust :**
* To please Clippy
* Use iterators rather than explicit index in the for loops
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashSet;

// IMO, a 9x9 2D array better expresses what a Sudoku board is (rather than a list of lists)
fn verify_sudoku_board(board: &[[u8; 9]; 9]) -> bool {
    let mut row_sets: Vec<HashSet<u8>> = vec![HashSet::new(); 9];
    let mut col_sets: Vec<HashSet<u8>> = vec![HashSet::new(); 9];
    let mut grid_sets: Vec<Vec<HashSet<u8>>> = vec![vec![HashSet::new(); 3]; 3];

    for (r, row) in board.iter().enumerate() {
        for (c, &num) in row.iter().enumerate() {
            if num == 0 {
                continue;
            }

            if row_sets[r].contains(&num)
                || col_sets[c].contains(&num)
                || grid_sets[r / 3][c / 3].contains(&num)
            {
                return false;
            }

            row_sets[r].insert(num);
            col_sets[c].insert(num);
            grid_sets[r / 3][c / 3].insert(num);
        }
    }
    true
}                
                
fn main(){     // no main() if this code runs in a Jupyter cell 
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    
    println!("{:?}", verify_sudoku_board(&board));  // true
    
    
    let board = [
        [3, 0, 6, 0, 5, 8, 4, 0, 0],
        [5, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 7, 0, 0, 0, 0, 3, 1],
        [1, 0, 2, 5, 0, 0, 3, 2, 0],
        [9, 0, 0, 8, 6, 3, 0, 0, 5],
        [0, 5, 0, 0, 9, 0, 6, 0, 0],
        [0, 3, 0, 0, 0, 8, 2, 5, 0],
        [0, 1, 0, 0, 0, 0, 0, 7, 4],
        [0, 0, 5, 2, 0, 6, 0, 0, 0],
    ];
    
    println!("{:?}", verify_sudoku_board(&board))  // false
}
```
