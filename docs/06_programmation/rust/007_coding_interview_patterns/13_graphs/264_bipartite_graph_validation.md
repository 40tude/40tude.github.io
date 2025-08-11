---
# published: false
layout: default
lang: en-US
title: "p264 - Bipartite Graph Validation"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Bipartite Graph Validation

<div align="center">
<img src="../assets/chap_13.webp" alt="" width="300" loading="lazy"/>
</div>

* Given an undirected graph
* Determine if it is bipartite
    * nodes can be colored in one of 2 colors
    * no adjacent node are the same color


<span style="color:orange"><b>The point:</b></span>

* For each node we color blue, color all of its neighbors orange et vice versa
* DFS
* The graph may not be fully connected 



**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(n + e)    | O(n)         |

* O(n + e) in time because we create n nodes and traverse e edges
* O(n) in space because the size of the recursive call stack (can grow up to n). Colors array contributes in O(n)









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First implementation

**About Rust :**
* Based on the version using `GraphNode` and `Graph` classes  
* Pay some attention to `for &neighbor in &self.nodes[node].neighbors {...`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
mod graph {
    use std::collections::HashSet;
    
    struct GraphNode {
        val: usize,
        neighbors: Vec<usize>,
    }

    impl GraphNode {
        fn new(val: usize, neighbors: Vec<usize>) -> Self {
            Self { val, neighbors }
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

        pub fn bipartite_graph_validation(&self) -> bool{
            let mut colors = vec![0; self.nodes.len()]; // unvisited
            // Determine if each graph component is bipartite
            for i in 0..self.nodes.len(){
                if colors[i] == 0 && !self.dfs(i, 1, &mut colors){
                    return false;
                }
            } 
            true
        }

        fn dfs(&self, node : usize, color : i32, colors: &mut Vec<i32>) -> bool{
            colors[node] = color;
            for &neighbor in &self.nodes[node].neighbors{
                // If current neighbor has same color the graph is NOT bipartite
                if colors[neighbor]==color{
                    return false;
                }
                // If current neighbor is not colored, color it with the other color and continue the DFS
                if (colors[neighbor]==0) && !self.dfs(neighbor, -color, colors){
                    return false;
                }
            }
            true
        }
    }
}

use graph::Graph;

fn main() {
    //     0 - 1   
    //   /      \
    //  4        2
    //   \ 
    //     3
    let adjacency_list = [
        vec![1, 4],
        vec![0, 2],
        vec![1],                // try `vec![1, 3],` to get false
        vec![4],
        vec![0, 3],
    ];

    let my_graph = Graph::from_adjacency_list(&adjacency_list);
    let start_node = 0;
    my_graph.print(start_node); // Node 0 has neighbors [1, 4]
                                // ...
    println!("{}", my_graph.bipartite_graph_validation()); // true
}
```

## V2

**About Rust :**
* Remove the `.print()` method
* Use an enum for colors
* `val` is renamed as `id` then `id` is deleted because never used
    * I let the previous line in comments 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
mod graph {
    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Color {
        Unvisited,
        Orange,
        Blue,
    }

    struct GraphNode {
        // id: usize,
        neighbors: Vec<usize>,
    }

    impl GraphNode {
        // fn new(id: usize, neighbors: Vec<usize>) -> Self {
        fn new(neighbors: Vec<usize>) -> Self {
            // Self { id, neighbors }
            Self {neighbors }
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
            // for (i, neighbors) in adj_list.iter().enumerate() {
            for neighbors in adj_list.iter() {
                graph
                    .nodes
                    //.push(GraphNode::new(i, neighbors.clone()));
                    .push(GraphNode::new(neighbors.clone()));
            }
            graph
        }

        pub fn bipartite_graph_validation(&self) -> bool {
            let mut colors = vec![Color::Unvisited; self.nodes.len()];

            for i in 0..self.nodes.len() {
                if colors[i] == Color::Unvisited
                    && !self.dfs(i, Color::Orange, &mut colors)
                {
                    return false;
                }
            }
            true
        }

        fn dfs(&self, node: usize, color: Color, colors: &mut Vec<Color>) -> bool {
            colors[node] = color;
            let opposite = match color {
                Color::Orange => Color::Blue,
                Color::Blue => Color::Orange,
                Color::Unvisited => unreachable!(),
            };

            for &neighbor in &self.nodes[node].neighbors {
                match colors[neighbor] {
                    c if c == color => return false, // same color → not bipartite
                    Color::Unvisited => {
                        if !self.dfs(neighbor, opposite, colors) {
                            return false;
                        }
                    }
                    _ => (), // already visited with the right color
                }
            }
            true
        }
    }
}

use graph::Graph;

fn main() {
    //     0 - 1   
    //   /      \
    //  4        2
    //   \ 
    //     3
    let adjacency_list = [
        vec![1, 4],
        vec![0, 2],
        vec![1], // try vec![1, 3] for a non-bipartite example
        vec![4],
        vec![0, 3],
    ];

    let my_graph = Graph::from_adjacency_list(&adjacency_list);
    println!("{}", my_graph.bipartite_graph_validation()); // true
}

```


**About Rust :**
* Same as above but commented lines are removed
* <span style="color:lime"><b>Preferred solution?</b></span>     
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
mod graph {
    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Color {
        Unvisited,
        Orange,
        Blue,
    }

    struct GraphNode {
        neighbors: Vec<usize>,
    }

    impl GraphNode {
        fn new(neighbors: Vec<usize>) -> Self {
            Self { neighbors }
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
            for neighbors in adj_list.iter() {
                graph.nodes.push(GraphNode::new(neighbors.clone()));
            }
            graph
        }

        pub fn bipartite_graph_validation(&self) -> bool {
            let mut colors = vec![Color::Unvisited; self.nodes.len()];

            for i in 0..self.nodes.len() {
                if colors[i] == Color::Unvisited && !self.dfs(i, Color::Orange, &mut colors) {
                    return false;
                }
            }
            true
        }

        fn dfs(&self, node: usize, color: Color, colors: &mut Vec<Color>) -> bool {
            colors[node] = color;
            let opposite = match color {
                Color::Orange => Color::Blue,
                Color::Blue => Color::Orange,
                Color::Unvisited => unreachable!(),
            };

            for &neighbor in &self.nodes[node].neighbors {
                match colors[neighbor] {
                    c if c == color => return false, // same color → not bipartite
                    Color::Unvisited => {
                        if !self.dfs(neighbor, opposite, colors) {
                            return false;
                        }
                    }
                    _ => (), // already visited with the right color
                }
            }
            true
        }
    }
}

use graph::Graph;

fn main() {
    //     0 - 1
    //   /      \
    //  4        2
    //   \
    //     3
    let adjacency_list = [
        vec![1, 4],
        vec![0, 2],
        vec![1], // try vec![1, 3] for a non-bipartite example
        vec![4],
        vec![0, 3],
    ];

    let my_graph = Graph::from_adjacency_list(&adjacency_list);
    println!("{}", my_graph.bipartite_graph_validation()); // true
}
```
