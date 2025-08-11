---
# published: false
layout: default
lang: en-US
title: "p247 - Introduction"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Introduction

<div align="center">
<img src="../assets/chap_13.webp" alt="" width="300" loading="lazy"/>
</div>



## First attempt
* Use objects and methods
* Use an "inner" module named `graph` where classes `Graph` (public) and `GraphNode` (hidden) are defined (among other things their public API)
    * This module should be a different file `graph.rs` for example
    * Here the code must be monolithic so the module in defined/scoped (I don't know the term) in `main.rs`
* About the classes defined
    * A `GraphNode` is a value/id and a vector of neighbors 
    * A `Graph` is a vector of GraphNodes
    * No pointer, nor boxe or whatsoever
    * Indeed, at this point the graph is read only (not mutable) so there is no need for `Rc` nor `RefCell`
    * Keep in mind that the id of each node is also its value. This helps to keep the code simple
    * If this is a problem, rather than :

```rust
struct GraphNode {
    val: usize,
    neighbors: Vec<usize>,
}
``` 

One could write

```rust
struct GraphNode<T> {
    id: usize,
    neighbors: Vec<usize>,
    data: T,
}

```
<!-- * <span style="color:lime"><b>Preferred solution?</b></span> -->
* Pay some attention to `for &neighbor in &self.nodes[node].neighbors {...`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust

mod graph {
    use std::collections::{HashSet, VecDeque};
    
    struct GraphNode {
        val: usize,
        neighbors: Vec<usize>,
    }

    impl GraphNode {
        fn new(val: usize, neighbors: Vec<usize>) -> Self {
            Self { val, neighbors }
        }

        fn process(&self) {
            println!("Processing node {}", self.val);
        }
    }

    pub struct Graph {
        nodes: Vec<GraphNode>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self { nodes: Vec::new() }
        }

        pub fn from_adjacency_list(adj_list: &[Vec<usize>]) -> Self {
            let mut graph = Graph::new();
            for (i, neighbors) in adj_list.iter().enumerate() {
                graph.nodes.push(GraphNode::new(i, neighbors.clone()));
            }
            graph
        }

        pub fn print(&self, node: usize) {
            let mut visited = HashSet::new();
            self.print_recursive(node, &mut visited);
        }

        fn print_recursive(&self, node: usize, visited: &mut HashSet<usize>) {
            if visited.contains(&node) {
                return;
            }
            visited.insert(node);
            println!("Node {} has neighbors {:?}", node, &self.nodes[node].neighbors);
            for &neighbor in &self.nodes[node].neighbors {
                self.print_recursive(neighbor, visited);
            }
        }

        pub fn dfs(&self, start: usize) {
            let mut visited = HashSet::new();
            self.dfs_recursive(start, &mut visited);
        }

        fn dfs_recursive(&self, start: usize, visited: &mut HashSet<usize>) {
            if visited.contains(&start) {
                return;
            }
            visited.insert(start);
            self.nodes[start].process();
            for &neighbor in &self.nodes[start].neighbors {
                self.dfs_recursive(neighbor, visited);
            }
        }

        pub fn bfs(&self, start: usize) {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while let Some(current) = queue.pop_front() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);
                self.nodes[current].process();
                for &neighbor in &self.nodes[current].neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
}

use graph::Graph;

fn main() {
    
    //     0    
    //   / | \
    //  3  |  1
    //   \ |
    //     2
    //   /
    //  4
    let adjacency_list = [
        vec![1, 2, 3],
        vec![0],
        vec![0, 3, 4],
        vec![0, 2],
        vec![2],
    ];

    let my_graph = Graph::from_adjacency_list(&adjacency_list);
    let start_node = 0;

    my_graph.print(start_node);         // Node 0 has neighbors [1, 2, 3]
                                        // ...

    println!("\nDFS from node 0:");     // DFS from node 0:
    my_graph.dfs(start_node);           // Processing node 0
                                        // ...

    println!("\nBFS from node 0:");     // BFS from node 0:
    my_graph.bfs(start_node);           // Processing node 0
                                        // ...
}
```

## Second attempt

* To stick to the content of the book
    * `dfs()`, `bfs()` and `process()` are not methods but regular functions
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::{HashSet, VecDeque};

struct GraphNode {
    val: usize,
    neighbors: Vec<usize>,
}

impl GraphNode {
    fn new(val: usize, neighbors: Vec<usize>) -> Self {
        Self { val, neighbors }
    }
}

struct Graph {
    nodes: Vec<GraphNode>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn from_adjacency_list(adj_list: &[Vec<usize>]) -> Self {
        let mut graph = Graph::new();
        for (i, neighbors) in adj_list.iter().enumerate() {
            graph.nodes.push(GraphNode::new(i, neighbors.clone()));
        }
        graph
    }
}

fn print_graph(graph: &Graph, start: usize) {
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    
    // This a way to hide the recursive part to the caller who can only call `print_graph()`
    fn deep_copy_recursive_print(graph: &Graph, node: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        println!("Node {} has neighbors {:?}", node, graph.nodes[node].neighbors);
        for &neighbor in &graph.nodes[node].neighbors {
            deep_copy_recursive_print(graph, neighbor, visited);
        }
    }

    deep_copy_recursive_print(graph, start, &mut visited);
}

fn process(node : &GraphNode) {
    println!("Processing node {}", node.val);
}

fn dfs(graph: &Graph, start: usize) {
    let mut visited = HashSet::new();

    fn dfs_recursive(graph: &Graph, start: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&start) {
            return;
        }
        visited.insert(start);
        process(&graph.nodes[start]);
        for &neighbor in &graph.nodes[start].neighbors {
            dfs_recursive(graph, neighbor, visited);
        }
    }

    dfs_recursive(graph, start, &mut visited);
}


fn bfs(graph: &Graph, start: usize) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        process(&graph.nodes[current]);
        for &neighbor in &graph.nodes[current].neighbors {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
}

fn main() {
    //     0    
    //   / | \
    //  3  |  1
    //   \ |
    //     2
    //   /
    //  4
    let adjacency_list = [
        vec![1, 2, 3],
        vec![0],
        vec![0, 3, 4],
        vec![0, 2],
        vec![2],
    ];

    let my_graph = Graph::from_adjacency_list(&adjacency_list);
    let start_node = 0;

    print_graph(&my_graph, start_node); // Node 0 has neighbors [1, 2, 3]
                                        // ...

    println!("\nDFS from node 0:");     // DFS from node 0:
    dfs(&my_graph, start_node);         // Processing node 0
                                        // ...

    println!("\nBFS from node 0:");     // BFS from node 0:
    bfs(&my_graph, start_node);         // Processing node 0
                                        // ...
}
```

## Third attempt

* In the book, `dfs()` and `bfs()` only have a GraphNode as parameter 
* Here, `my_graph` in encapsulated into a `GRAPH` object
    * `GRAPH` is a global static variable whose access is transparently synchronized 
    * In `dfs()` and `bfs()` we need to `let graph = GRAPH.get().expect("blablabla...");` before we can act on it
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


### Advantages
* Simplicity: very easy to read, no need for Rc, RefCell, etc.
* No costly dynamic allocation: nodes are contained in a vector, enabling rapid indexing.
* Good read performance: no borrow() overhead or dynamic memory management.
* Ideal for static graphs: if the graph never changes after its creation.

### Disadvantages
* Global access (GRAPH): OnceLock imposes a single graph, so :
    * no parallel tests,
    * no simultaneous multi-graphs,
    * no dynamic reloading.
* Rigid index: less flexible for more complex algos (e.g. node deletion, transformation).
* Not usable in multithreaded environments without explicit locks: everything is fixed at the OnceLock level, but internal accesses are not protected.
* No granularity on nodes: impossible to access a node independently of the graph.





```rust
use std::collections::{HashSet, VecDeque};
use std::sync::OnceLock;

#[derive(Debug)]
struct GraphNode {
    val: usize,
    neighbors: Vec<usize>,
}

impl GraphNode {
    fn new(val: usize, neighbors: Vec<usize>) -> Self {
        Self { val, neighbors }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<GraphNode>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn from_adjacency_list(adj_list: &[Vec<usize>]) -> Self {
        let mut graph = Graph::new();
        for (i, neighbors) in adj_list.iter().enumerate() {
            graph.nodes.push(GraphNode::new(i, neighbors.clone()));
        }
        graph
    }
}

// Global lazy static for the graph
static GRAPH: OnceLock<Graph> = OnceLock::new();


fn print_graph(start: usize) {
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    let graph = GRAPH.get().expect("Graph not initialized");

    // This a way to hide the recursive part to the caller who can only call `print_graph()`
    fn deep_copy_recursive_print(graph: &Graph, node: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        println!("Node {} has neighbors {:?}", node, graph.nodes[node].neighbors);
        for &neighbor in &graph.nodes[node].neighbors {
            deep_copy_recursive_print(graph, neighbor, visited);
        }
    }

    deep_copy_recursive_print(graph, start, &mut visited);
}


// Function to process a node
fn process(node: &GraphNode) {
    println!("Processing node {}", node.val);
}

// DFS wrapper using global graph
fn dfs(start: usize) {
    let mut visited = HashSet::new();
    let graph = GRAPH.get().expect("Graph not initialized");
    dfs_recursive(graph, start, &mut visited);
}

// Recursive DFS helper
fn dfs_recursive(graph: &Graph, start: usize, visited: &mut HashSet<usize>) {
    if visited.contains(&start) {
        return;
    }
    visited.insert(start);
    process(&graph.nodes[start]);
    for &neighbor in &graph.nodes[start].neighbors {
        dfs_recursive(graph, neighbor, visited);
    }
}

// BFS using global graph
fn bfs(start: usize) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let graph = GRAPH.get().expect("Graph not initialized");

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        process(&graph.nodes[current]);
        for &neighbor in &graph.nodes[current].neighbors {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
            }
        }
    }
}

fn main() {
    let adjacency_list = [
        vec![1, 2, 3],
        vec![0],
        vec![0, 3, 4],
        vec![0, 2],
        vec![2],
    ];

    let my_graph = Graph::from_adjacency_list(&adjacency_list);
    let start_node = 0;
    
    GRAPH.set(my_graph).expect("GRAPH already initialized");

    print_graph(start_node);            // Node 0 has neighbors [1, 2, 3]
                                        // ...

    println!("\nDFS from node 0:");     // DFS from node 0:
    dfs(start_node);                    // Processing node 0
                                        // ...

    println!("\nBFS from node 0:");     // BFS from node 0:
    bfs(start_node);                    // Processing node 0
                                        // ...
}
```

## Fourth Attempt

* Mimic the book as much as possible
* There are no more `Graph` classes at all, just a few `GraphNodes`.
* A `GraphNode` is a value and a list of pointers to other node (`GraphNodeRef`)
* `Rc` and `RefCell` are mandatory
    * We need pointers pointing to the same nodes => `Rc`
    * `RefCell` supports interior mutability. This is a way of bypassing Rust's borrowing rules **at runtime** rather than at **compile-time**.
    * Without `RefCell`, an `Rc<Node>` cannot be modified. Rust forbids to modify a structure contained in an Rc. An Rc only gives (immutable) read access to the underlying data. See : `.borrow()`and `.borrow_mut()`
* There is a cost at runtime.
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


### Advantages
* Modularity per node: each node is an independent entity (strong pointer + internal mutability).
* Easy to modify dynamically: neighbors can be added/removed without rebuilding everything.
* No need for central structure: more flexible for building multiple graphs, sub-graphs, etc.
* Allows cycles. Useful for undirected or cyclic graphs.

### Disadvantages
* Management complexity (`Rc`, `RefCell`): need to `borrow()` and `clone()` at each access, potential source of panic (borrow_mut() in conflict).
* Not thread-safe: `Rc` and `RefCell` are not Send or Sync. Unusable as is in multithreaded mode without conversion to `Arc<Mutex<...>>`.
* Memory overhead: each node has its own reference counter and dynamic allocation.



```rust
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

// Type alias for readability
type GraphNodeRef = Rc<RefCell<GraphNode>>;

// Definition of a graph node
#[derive(Debug)]
struct GraphNode {
    val: usize,
    neighbors: Vec<GraphNodeRef>,
}

// Helper to build graph from adjacency list
fn from_adjacency_list(adj_list: &[Vec<usize>]) -> Vec<GraphNodeRef> {
    let nodes: Vec<GraphNodeRef> = (0..adj_list.len()).map(|i| Rc::new(RefCell::new(GraphNode { val: i, neighbors: vec![] }))).collect();

    for (i, neighbors) in adj_list.iter().enumerate() {
        let mut node_mut = nodes[i].borrow_mut();
        node_mut.neighbors = neighbors.iter().map(|&j| Rc::clone(&nodes[j])).collect();
    }
    nodes
}

fn print_graph(node: &GraphNodeRef) {
    use std::collections::HashSet;
    let mut visited = HashSet::new();

    // This a way to "hide" the recursive part to the caller who can only call `print_graph()`
    fn deep_copy_recursive_print(node: &GraphNodeRef, visited: &mut HashSet<usize>) {
        let val = node.borrow().val;
        if visited.contains(&val) {
            return;
        }
        visited.insert(val);
        let neighbors: Vec<_> = node.borrow().neighbors.iter().map(|n| n.borrow().val).collect();
        println!("Node {} has neighbors {:?}", val, neighbors);
        for neighbor in &node.borrow().neighbors {
            deep_copy_recursive_print(neighbor, visited);
        }
    }

    deep_copy_recursive_print(node, &mut visited);
}

fn process(node: &GraphNodeRef) {
    println!("Processing node {}", node.borrow().val);
}

fn dfs(node: &GraphNodeRef) {
    let mut visited = HashSet::new();

    // Hide the recursive part
    fn dfs_recursive(node: &GraphNodeRef, visited: &mut HashSet<usize>) {
        match visited.insert(node.borrow().val) {
            false => (), // Already visited
            true => {
                process(node);
                for neighbor in &node.borrow().neighbors {
                    dfs_recursive(neighbor, visited);
                }
            }
        }
    }

    dfs_recursive(node, &mut visited);
}


fn bfs(node: &GraphNodeRef) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(Rc::clone(node));

    while let Some(current) = queue.pop_front() {
        let val = current.borrow().val;

        if visited.contains(&val) {
            continue;
        }

        visited.insert(val);
        process(&current); // Visit current node

        for neighbor in &current.borrow().neighbors {
            if !visited.contains(&neighbor.borrow().val) {
                queue.push_back(Rc::clone(neighbor)); // Push unvisited neighbor
            }
        }
    }
}

fn main() {
    //     0    
    //   / | \
    //  3  |  1
    //   \ |
    //     2
    //   /
    //  4
    let adjacency_list = [
        vec![1, 2, 3],
        vec![0],
        vec![0, 3, 4],
        vec![0, 2],
        vec![2],
    ];

    let my_graph = from_adjacency_list(&adjacency_list);
    let start_node = &my_graph[0];
    
    print_graph(start_node);            // Node 0 has neighbors [1, 2, 3]
                                        // ...        
    
    println!("\nDFS from node 0:");     // DFS from node 0:
    dfs(start_node);                    // Processing node 0
                                        // ...
    
    println!("\nBFS from node 0:");     // BFS from node 0:
    bfs(start_node);                    // Processing node 0
                                        // ...
}

```

Same as above but only demonstrate the call to `dfs()` 

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

// Type alias for readability
type GraphNodeRef = Rc<RefCell<GraphNode>>;

#[derive(Debug)]
struct GraphNode {
    val: usize,
    neighbors: Vec<GraphNodeRef>,
}

fn from_adjacency_list(adj_list: &[Vec<usize>]) -> Vec<GraphNodeRef> {
    let nodes: Vec<GraphNodeRef> = (0..adj_list.len()).map(|i| Rc::new(RefCell::new(GraphNode { val: i, neighbors: vec![] }))).collect();

    for (i, neighbors) in adj_list.iter().enumerate() {
        let mut node_mut = nodes[i].borrow_mut();
        node_mut.neighbors = neighbors.iter().map(|&j| Rc::clone(&nodes[j])).collect();
    }
    nodes
}

fn process(node: &GraphNodeRef) {
    println!("Processing node {}", node.borrow().val);
}

fn dfs(node: &GraphNodeRef) {
    let mut visited = HashSet::new();
    
    fn dfs_recursive(node: &GraphNodeRef, visited: &mut HashSet<usize>) {
        match visited.insert(node.borrow().val) {
            false => (), // Already visited
            true => {
                process(node);
                for neighbor in &node.borrow().neighbors {
                    dfs_recursive(neighbor, visited);
                }
            }
        }
    }

    dfs_recursive(node, &mut visited);
}

fn main() {
    let adjacency_list = [
        vec![1, 2, 3],
        vec![0],
        vec![0, 3, 4],
        vec![0, 2],
        vec![2],
    ];

    let my_graph = from_adjacency_list(&adjacency_list);
    let start_node = &my_graph[0];
    
    println!("\nDFS from node 0:");     // DFS from node 0:
    dfs(start_node);                    // Processing node 0
                                        // ...
}
```
