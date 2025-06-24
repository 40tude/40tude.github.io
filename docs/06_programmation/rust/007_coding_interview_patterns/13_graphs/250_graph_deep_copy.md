---
# published: false
layout: default
title: "p250 - Graph Deep Copy"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Graph Deep Copy

<div align="center">
<img src="../assets/chap_13.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a reference to a node in an undirected graph
* Create a deep copy of the graph
* The copy must be independant from the original
* New nodes must be created


<span style="color:orange"><b>The point:</b></span>

* Duplicate while traversing
* deep_copy_recursive



**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n + e)    | O(n)         |

* O(n + e) in time because we create n nodes and traverse e edges
* O(n) in space because the size of the recursive stack (can grow up to n) + clone_map hash map stores n key-val pairs









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

**About Rust :**
* Based on the version using `Rc` and `RefCell` 
    * check `247_introduction.ipynb`
    * `Rc` and `RefCell` are mandatory
    * We need pointers pointing to the same nodes => `Rc`
    * `RefCell` supports interior mutability. This is a way of bypassing Rust's borrowing rules **at runtime** rather than at **compile-time**.
    * Without `RefCell`, an `Rc<Node>` cannot be modified. Rust forbids to modify a structure contained in an Rc. An Rc only gives (immutable) read access to the underlying data. See : `.borrow()`and `.borrow_mut()`
    * There is a cost at runtime.
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

fn print_graph(node: &GraphNodeRef) {
    use std::collections::HashSet;
    let mut visited = HashSet::new();

    // This a way to hide the recursive part to the caller who can only call `print_graph()`
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

// I’m not nesting deep_copy_recursive() inside deep_copy() to stay consistent with the book’s style and to improve code readability.
fn deep_copy_recursive(node: &GraphNodeRef, clone_map: &mut HashMap<usize, GraphNodeRef>) -> GraphNodeRef {
    //  If this node was already cloned, then return this previously cloned node
    let node_val = node.borrow().val;
    if let Some(clone) = clone_map.get(&node_val) {
        return Rc::clone(clone);
    }

    // Clone the current node
    let cloned_node = Rc::new(RefCell::new(GraphNode {
        val: node_val,
        neighbors: vec![],
    }));

    //  Store the current clone to ensure it doesn't need to be created again in future DFS calls
    clone_map.insert(node_val, Rc::clone(&cloned_node));

    // Iterate through the neighbors of the current node to connect their clones to the current cloned node
    for neighbor in &node.borrow().neighbors {
        let cloned_neighbor = deep_copy_recursive(neighbor, clone_map);
        cloned_node.borrow_mut().neighbors.push(cloned_neighbor);
    }
    cloned_node
}

// Public deep copy function
fn deep_copy(start_node: &GraphNodeRef) -> GraphNodeRef {
    let mut clone_map = HashMap::new();
    deep_copy_recursive(start_node, &mut clone_map)
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
    let my_start_node = &my_graph[0];

    let my_cloned_start = deep_copy(my_start_node);
    
    print_graph(&my_cloned_start);
}
```

## V2

**About Rust :**
* Changes are in `deep_copy_recursive`
    * Reduce the number of `node.borrow()`
    * Use `Vec::with_capacity` for the vector of `neighbors`
* <span style="color:lime"><b>Preferred solution?</b></span>     
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

fn print_graph(node: &GraphNodeRef) {
    use std::collections::HashSet;
    let mut visited = HashSet::new();

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

// I’m not nesting deep_copy_recursive() inside deep_copy() to stay consistent with the book’s style and to improve code readability.
fn deep_copy_recursive(node: &GraphNodeRef, clone_map: &mut HashMap<usize, GraphNodeRef>) -> GraphNodeRef {
    //  If this node was already cloned, then return this previously cloned node
    let node_borrowed = node.borrow();
    if let Some(clone) = clone_map.get(&node_borrowed.val) {
        return Rc::clone(clone);
    }

    // Clone the current node
    let cloned_node = Rc::new(RefCell::new(GraphNode {
        val: node_borrowed.val,
        neighbors: Vec::with_capacity(node_borrowed.neighbors.len()), /
    }));

    //  Store the current clone to ensure it doesn't need to be created again in future DFS calls
    clone_map.insert(node_borrowed.val, Rc::clone(&cloned_node));

    // Iterate through the neighbors of the current node to connect their clones to the current cloned node
    for neighbor in &node_borrowed.neighbors {
        let cloned_neighbor = deep_copy_recursive(neighbor, clone_map);
        cloned_node.borrow_mut().neighbors.push(cloned_neighbor);
    }
    cloned_node
}

// Public deep copy function
fn deep_copy(start_node: &GraphNodeRef) -> GraphNodeRef {
    let mut clone_map = HashMap::new();
    deep_copy_recursive(start_node, &mut clone_map)
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
    let my_start_node = &my_graph[0];

    let my_cloned_start = deep_copy(my_start_node);
    
    print_graph(&my_cloned_start);
}
```

## V3

**About Rust :**
* Changes are in `deep_copy_recursive`
    * Use `.entry()` from `HashMap` to replace `.get()` and `.insert()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

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

fn print_graph(node: &GraphNodeRef) {
    use std::collections::HashSet;
    let mut visited = HashSet::new();

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

// I’m not nesting deep_copy_recursive() inside deep_copy() to stay consistent with the book’s style and to improve code readability.
fn deep_copy_recursive(node: &GraphNodeRef, clones: &mut HashMap<usize, GraphNodeRef>) -> GraphNodeRef {
    let node_borrowed = node.borrow();
    let val = node_borrowed.val;

    match clones.entry(val) {
        //  If this node was already cloned, then return this previously cloned node
        std::collections::hash_map::Entry::Occupied(entry) => {
            Rc::clone(entry.get())
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            // Clone the current node
            let cloned_node = Rc::new(RefCell::new(GraphNode {
                val,
                neighbors: Vec::with_capacity(node_borrowed.neighbors.len()), // Optional optimization
            }));

            //  Store the current clone to ensure it doesn't need to be created again in future DFS calls
            let cloned_node_ref = Rc::clone(&cloned_node);
            entry.insert(cloned_node);

            // Iterate through the neighbors of the current node to connect their clones to the current cloned node
            for neighbor in &node_borrowed.neighbors {
                let cloned_neighbor = deep_copy_recursive(neighbor, clones);
                cloned_node_ref.borrow_mut().neighbors.push(cloned_neighbor);
            }

            cloned_node_ref
        }
    }
}

// Public deep copy function
fn deep_copy(start_node: &GraphNodeRef) -> GraphNodeRef {
    let mut clone_map = HashMap::new();
    deep_copy_recursive(start_node, &mut clone_map)
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
    let my_start_node = &my_graph[0];

    let my_cloned_start = deep_copy(my_start_node);
    
    print_graph(&my_cloned_start);
}
```

## V4

**About Rust :**
* Use classes. 
* Do not use an "inner" module (see `First attempt` in `247_introduction.ipynb`)
* Traverse the graph (`deep_copy_recursive()`)
* Duplicate nodes while traversing
* Use a hashmap where original nodes are the key and the corresponding cloned node is the value
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
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

    // Public print function
    fn print(&self, start: usize) {
        let mut visited = HashSet::new();
        self.print_recursive(start, &mut visited);
    }

    fn print_recursive(&self, start: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&start) {
            return;
        }
        visited.insert(start);
        self.nodes[start].process();
        for &neighbor in &self.nodes[start].neighbors {
            self.print_recursive(neighbor, visited);
        }
    }

    // Public deep copy function
    fn deep_copy(&self) -> Graph {
        // Return an empty graph if the original graph has no nodes
        if self.nodes.is_empty() {
            return Graph::new();
        }

        // Create a map to track already cloned nodes by their index
        let mut clone_map = HashMap::new();

        // Start the deep copy from node index 0
        self.deep_copy_recursive(0, &mut clone_map);

        // Convert HashMap to Vec sorted by index
        let mut nodes = vec![GraphNode::new(0, vec![]); self.nodes.len()];
        for (index, node) in clone_map {
            nodes[index] = node;
        }

        // Return the new graph with the copied nodes
        Graph { nodes }
    }

    fn deep_copy_recursive(&self, index: usize, clone_map: &mut HashMap<usize, GraphNode>) -> GraphNode {
        // If the node at this index has already been cloned, return the existing clone
        if let Some(existing) = clone_map.get(&index) {
            return existing.clone(); // Clone the previously cloned node
        }

        // Clone the current node (without neighbors for now)
        let node = &self.nodes[index];
        let mut cloned_node = GraphNode::new(node.val, Vec::with_capacity(node.neighbors.len()));

        // Store a copy of the node in the map before recursion
        // This avoids infinite recursion in case of cycles
        clone_map.insert(index, cloned_node.clone());

        // Recursively clone all neighbors and update the neighbor list of the cloned node
        for &neighbor_index in &node.neighbors {
            let cloned_neighbor = self.deep_copy_recursive(neighbor_index, clone_map);
            cloned_node.neighbors.push(cloned_neighbor.val); // Store only the neighbor's value
        }

        // Update the map with the fully constructed node
        clone_map.insert(index, cloned_node.clone());

        // Return the cloned node
        cloned_node
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
        vec![2]
    ];

    let graph = Graph::from_adjacency_list(&adjacency_list);

    println!("Original :");     // Original :
    graph.print(0);             // Processing node 0
                                // ...
    
    let graph_copy = graph.deep_copy();

    println!("\nCopy :");       // Copy :
    graph_copy.print(0)         // Processing node 0
                                // ...                                      
}
```

## V5

**About Rust :**
* There are no pointer relationships, so deep copying can be simplified
* But... We don't play by the rules
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::collections::{HashSet};

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

    fn print(&self, start: usize) {
        let mut visited = HashSet::new();
        self.print_recursive(start, &mut visited);
    }

    fn print_recursive(&self, start: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&start) {
            return;
        }
        visited.insert(start);
        self.nodes[start].process();
        for &neighbor in &self.nodes[start].neighbors {
            self.print_recursive(neighbor, visited);
        }
    }

    // There are no pointer relationships, so deep copy can be very simple.
    fn deep_copy(&self) -> Graph {
        let copied_nodes = self.nodes
            .iter()
            .map(|node| GraphNode::new(node.val, node.neighbors.clone()))
            .collect();

        Graph { nodes: copied_nodes }
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

    let graph = Graph::from_adjacency_list(&adjacency_list);

    println!("Original :");     // Original :
    graph.print(0);             // Processing node 0
                                // ...
    let graph_copy = graph.deep_copy();

    println!("\nCopy :");       // Copy :
    graph_copy.print(0)         // Processing node 0
                                // ...                                      
}
```
