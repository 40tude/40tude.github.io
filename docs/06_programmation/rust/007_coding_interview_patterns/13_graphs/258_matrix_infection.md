---
# published: false
layout: default
title: "p258 - Matrix infection"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Matrix infection

* Given a matrix of
    * 0 empty
    * 1 uninfected
    * 2 infected 
* Each sec, every infected cell infects adjacent neighbors cells (4 directions)
* Return the number of sec to infecs all cells. -1 if this is not possible

<span style="color:orange"><b>The point:</b></span>

* Level order traversal problem
* BFS & queue
* Multi-source BFS
* During the patrix traversal  
    * add to the queue all infected cells 
    * count the number of 1
* During the traversal decrement count as more cells are contaminated 



**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(m x n)    | O(m x n)     |

* O(m x n) in time because in worst case, every cell is visited during level-order traversal
* O(m x n) in space because the size of the queue (can grow up to m x n) 









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First translation

**About Rust :**
* Check the `for (dr, dc) in &directions{` (and the 2 others variants). 
* Read `253_count_islands.ipynb` and `.iter()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::VecDeque;

fn matrix_infection(matrix: &mut [Vec<i32>]) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return -1; 
    }

    let (mut ones, mut seconds) = (0, 0);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Count the number of uninfected cells, add infected cells to the queue
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            match matrix[r][c] {
                0 => {},                               // Empty cell
                1 => ones += 1,                        // Uninfected cell
                2 => queue.push_back((r, c)),          // Infected cell
                _ => panic!("Unexpected value"),
            }
        }
    }

    // Directions for 4-neighbor connectivity (up, down, left, right)
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Use level-order traversal to determine how long it takes
    while !queue.is_empty() && ones > 0 {
        // 1 sec passes with each each level of the matrix explored
        seconds += 1; 
        let level_size = queue.len();
        for _ in 0..level_size { // loop by level
            // let (r, c) = queue.pop_front().unwrap();
            if let Some((r, c)) = queue.pop_front() {
                // Infect any neighboring 1s and add them to the queue
                // for (dr, dc) in directions{      // by value, implicit copy
                // for &(dr, dc) in &directions {   // by reference + only copy what is needed : for &Trio(a, b, _) in &data{ //only a and b copied)
                for (dr, dc) in &directions{        // by reference
                    let new_r = r as isize + dr;   
                    let new_c = c as isize + dc;

                    if is_within_bounds(new_r, new_c, matrix) {
                        let (new_r_usize, new_c_usize) = (new_r as usize, new_c as usize);
                        if matrix[new_r_usize][new_c_usize] == 1 {
                            matrix[new_r_usize][new_c_usize] = 2;
                            ones -= 1;
                            queue.push_back((new_r_usize, new_c_usize)); // push neighbor
                        }
                    }
                }
            }
        }
    }
    // if there are still unifected cells left return -1, otherwise the time passed
    if ones == 0 { seconds } else { -1 }
}

fn is_within_bounds(r: isize, c: isize, matrix: &[Vec<i32>]) -> bool {
    r >= 0 && c >= 0 && (r as usize) < matrix.len() && (c as usize) < matrix[0].len()
}

fn main() {
    let mut matrix = vec![
        vec![1, 1, 1, 0],
        vec![0, 0, 2, 1],
        vec![0, 1, 1, 0]
    ];
    println!("{} sec.", matrix_infection(&mut matrix)); // 3

    let mut empty = vec![];
    println!("{} sec.", matrix_infection(&mut empty));        // -1

    let mut all_infected = vec![vec![2,2], vec![2,2]];
    println!("{} sec.", matrix_infection(&mut all_infected)); // 0
    
    let mut impossible = vec![vec![1,0], vec![0,1]];
    println!("{} sec.", matrix_infection(&mut impossible));   // -1
}

```
