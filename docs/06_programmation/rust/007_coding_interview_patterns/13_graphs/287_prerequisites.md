---
# published: false
layout: default
title: "p287 - Prerequisites"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Prerequisites

* `n` the number of courses from 0 to `n-1`
* An array of prerequisites pairs (directed)
* Determine if it is possible to enroll all courses

<span style="color:orange"><b>The point:</b></span>

* Impossible if there is a cycle in the graphical representation
* No arrow pointing to courses without prerequisites (in_degree = # of directed edges incoming to the node)
* When there are not any courses with an in_degree = 0 there is no solution
* Topological sorting
* Kahn's algorithm performs topological sort



**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n+e)      | O(n+e)       |

* O(n+e) in time because 
    * creating adjacency list and recording `in_degrees` take O(e) because we iterate through each prerequisite once. 
    * Because adding all course with in_degree==0 takes O(n)
* O(n+e) in space because adjacency list is in O(n+e), `in_degrees` array and the queue take up O(n) in space

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First implementation

**About Rust :**
* See au the Python's `defaultdict` is simulated here
    * `graph.entry(prerequesite).or_default().push(course);`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::{HashMap, VecDeque};

fn prerequisites(n: usize, prerequesites: Vec<(usize, usize)>) -> bool {
    let mut graph : HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degrees = vec![0; n];

    // Represent the graph as and adjacency list and record the in_degree of each course
    for (prerequesite, course) in prerequesites{
        // Ensure the key exists and push the course (see defaultdict in Python)
        graph.entry(prerequesite).or_default().push(course);
        in_degrees[course] +=1;
    }

    let mut queue: VecDeque<usize> = VecDeque::new();

    // Add all courses with an in-degree of 0 to the queue
    for (i, &deg) in in_degrees.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
        }
    }
    
    let mut enrolled_courses = 0;

    // Perform topological sort
    while !queue.is_empty(){
        let node = queue.pop_front().unwrap(); // safe because queue is not empty
        enrolled_courses +=1;
        // Avoid panic in the loop on graph[&node] by using get() and iterating over an empty slice if the key is absent
        for &neighbor in graph.get(&node).unwrap_or(&vec![]) {
            in_degrees[neighbor] -=1;
            // if in_degree of a neighboring course becomes 0, add it to the queue
            if in_degrees[neighbor]==0{
                queue.push_back(neighbor);
            }
        }
    }
    // return true if all courses enrolled
    enrolled_courses == n
}

fn main() {
    let n = 3;
    let prereq_list = vec![(0,1), (1, 2), (2, 1)];
    println!("{}", prerequisites(n, prereq_list)); // false
}
```

## V2


**About Rust :**
* `prerequisites` argument is passed by reference 
    * allow to reveive content of a vector or content of an array by reference
* whatchout : `for &(prerequesite, course) in prerequisites  {...`
* `while let Some(node) = queue.pop_front() {` to simplify the while loop
* <span style="color:lime"><b>Preferred solution?</b></span>     
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::{HashMap, VecDeque};

fn prerequisites(n: usize, prerequisites: &[(usize, usize)]) -> bool {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new(); 
    let mut in_degrees = vec![0; n];

    // Represent the graph as and adjacency list and record the in_degree of each course
    for &(prerequesite, course) in prerequisites  {
        // Ensure the key exists and push the course (see defaultdict in Python)
        graph.entry(prerequesite).or_default().push(course);
        in_degrees[course] += 1;
    }

    let mut queue = VecDeque::new();

    // Add all courses with an in_degree of 0 to the queue
    for (i, &deg) in in_degrees.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
        }
    }

    let mut enrolled_courses = 0;

    // Perform topological sort
    while let Some(node) = queue.pop_front() {
        enrolled_courses += 1;
        // Avoid panic
        for &neighbor in graph.get(&node).map_or(&[][..], Vec::as_slice) {
            in_degrees[neighbor] -= 1;
            // if in_degree of a neighboring course becomes 0, add it to the queue
            if in_degrees[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }
    // return true if all courses enrolled
    enrolled_courses == n
}

fn main() {
    let n = 3;
    let prereq_list = vec![(0, 1), (1, 2), (2, 1)]; 
    println!("{}", prerequisites(n, &prereq_list)); // false
}

```
