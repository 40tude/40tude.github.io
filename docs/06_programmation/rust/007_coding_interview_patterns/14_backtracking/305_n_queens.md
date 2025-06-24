---
# published: false
layout: default
title: "p305 - N Queens"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# N Queens

<div align="center">
<img src="../assets/chap_14.webp" alt="" width="300" loading="lazy"/>
</div>

* There is a chessboard of size `nxn`
* Place `n` queens so that no two queens attack each other
* Return the number of distinct config




<span style="color:orange"><b>The point:</b></span>

* If we can't place a new queen we need to...
* Backtracking
* Place queens on new row (=> no set for tracking rows)
* No need to know position of the queens. Need to know if there is a queen in row, col, diag or square (hash set)


**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n!)       | O(n)  |

* O(n!) in time because 
    * first queen => n choices
    * second queen => n-a. a the number of square attacked by the first queen
    * third queen => n-b. b the number of square attacked by the 2 first queens (`b<a`)
    * this "looks like" to n!
* O(n) in space because n is the max depth of the recursion tree. The hash sets can store up to `n` values.









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* First implementation

**About Rust :**
* `use std::collections::HashSet;`
    * `let my_set = HashSet::new();`
    * ``my_set.contains(&my_value)``
    * ``my_set.insert(my_value)``
    * ``my_set.remove(&my_value)``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashSet;

fn dfs(r: i32, diagonals_set: &mut HashSet<i32>, anti_diagonals_set: &mut HashSet<i32>, cols_set: &mut HashSet<i32>, n: i32, res: &mut i32) {
    // if we have reach the end of the rows, we have placed all queens
    if r == n {
        *res += 1;
        return;
    }

    for c in 0..n {
        let current_diagonal = r - c;
        let current_anti_diagonal = r + c;
        // if there are queens on the current columns, diag or anti diag, skip this square
        if cols_set.contains(&c) || diagonals_set.contains(&current_diagonal) || anti_diagonals_set.contains(&current_anti_diagonal) {
            continue;
        }
        // Place the queen by marking the current col, diag and anti-diag as occupied
        cols_set.insert(c);
        diagonals_set.insert(current_diagonal);
        anti_diagonals_set.insert(current_anti_diagonal);
        // Recursively move to the next row to continue placing queens
        dfs(r + 1, diagonals_set, anti_diagonals_set, cols_set, n, res);
        // Backtrack by removing the current col, diag and anti-diag from the hash sets
        cols_set.remove(&c);
        diagonals_set.remove(&current_diagonal);
        anti_diagonals_set.remove(&current_anti_diagonal);
    }
}

fn n_queens(n: i32) -> i32 {
    let mut res = 0;
    dfs(0, &mut HashSet::new(), &mut HashSet::new(), &mut HashSet::new(), n, &mut res);
    res
}

// no main() if this code runs in a Jupyter cell
fn main() {
    let n = 4;
    print!("{}", n_queens(n)); // 2
} // end of local scope OR end of main()

```

## V2

* Same as above
* Rename some variables
* Remove some comments

**About Rust :**
* `use std::collections::HashSet;`
    * `let my_set = HashSet::new();`
    * ``my_set.contains(&my_value)``
    * ``my_set.insert(my_value)``
    * ``my_set.remove(&my_value)``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::HashSet;

fn dfs(row: i32, cols: &mut HashSet<i32>, diagonals: &mut HashSet<i32>, anti_diagonals: &mut HashSet<i32>, n: i32, count: &mut i32) {
    // If we reach the end, all queens are placed successfully
    if row == n {
        *count += 1;
        return;
    }

    for col in 0..n {
        let diag = row - col;
        let anti_diag = row + col;

        // Check if the position is safe
        if cols.contains(&col) || diagonals.contains(&diag) || anti_diagonals.contains(&anti_diag) {
            continue;
        }

        // Choose: place the queen
        cols.insert(col);
        diagonals.insert(diag);
        anti_diagonals.insert(anti_diag);

        // Explore
        dfs(row + 1, cols, diagonals, anti_diagonals, n, count);

        // Unchoose (backtrack): remove the queen
        cols.remove(&col);
        diagonals.remove(&diag);
        anti_diagonals.remove(&anti_diag);
    }
}

fn n_queens(n: i32) -> i32 {
    let mut cols = HashSet::new();
    let mut diagonals = HashSet::new();
    let mut anti_diagonals = HashSet::new();
    let mut count = 0;

    dfs(0, &mut cols, &mut diagonals, &mut anti_diagonals, n, &mut count);

    count
}

fn main() {
    let n = 4;
    println!("{}", n_queens(n)); // 2
}

```
