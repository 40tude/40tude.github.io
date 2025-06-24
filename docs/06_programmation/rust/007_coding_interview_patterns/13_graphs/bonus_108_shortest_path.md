---
# published: false
layout: default
title: "bonus108 - Shortest Path"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Shortest Path

<div align="center">
<img src="../assets/chap_13.webp" alt="" width="300" loading="lazy"/>
</div>

* n represents nodes (0 to n - 1) 
* undirected graph
* array of non-negative weighted edges
* return an array where each index i contains the shortest path length from a specified start node to node i
    * If a node is unreachable, set its distance to -1.
* Each edge is represented by a triplet of positive integers: the start node, the end node, and the weight of the edge.


<span style="color:orange"><b>The point:</b></span>

* BFS algo if same weight on edges (can be 0)
* Dijkstra’s algo if no edges with negative weights
* Bellman-Ford algo if edges have negative weights
* Floyd-Warshall to find the shortest paths between all pairs of nodes 
* Here Dijkstra’s algo
    * Greedy
    * At each step, we move to the unvisited node with the shortest known distance from the start node (local optimum)
    * Use a min heap to access the unvisited node with the shortest known distance at any point in the process


**Complexity :**

| Time               | Space        |
|--------------------|--------------|
| O((n+e) * log(n))  | O(n+e)       |

* O((n+e) * log(n)) in time because 
    * creating adjacency list takes O(e) 
    * Dijkstra's algorithm traverses up to all n nodes and explores each edge of the graph. To access each node, we pop it from the heap, and for each edge, up to one node is pushed to the heap (when we process each node’s neighbors). Since each push and pop operation takes O(lon(n)) time, the time complexity of Dijkstra’s algorithm is O((n+e) * log(n)) => O(e) + O((n+e) * log(n)) = O((n+e) * log(n))
.
* O(n+e) in space because adjacency list is in O(n+e), ``distances`` array and  `min_heap` take up O(n) in space each

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First implementation

**About Rust :**
* BinaryHeap in Rust is a max-heap, so to simulate a min-heap, we need reverse the order with a custom struct 
* In the very first attempt, the heap contained a tuple `(usize, usize)`, but it has been transformed in a struct that implements Ord in reverse (mimic min-heap)
    * See : 
        * `09_intervals\170_largest_overlap_of_intervals.ipynb`
        * `08_heaps\145_k_most_frequent_strings.ipynb` 
        * `08_heaps\151_combine_sorted_linked_lists.ipynb`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    distance: usize,
    node: usize,
}

// Custom ordering for min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the ordering to make a min-heap
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(n: usize, edges: &[(usize, usize, usize)], start: usize) -> Vec<i32> {
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut distances = vec![usize::MAX; n];
    distances[start] = 0;

    // Represent the graph as an adjacency list
    for &(u, v, w) in edges {
        graph.entry(u).or_default().push((v, w));
        graph.entry(v).or_default().push((u, w)); 
    }

    let mut min_heap = BinaryHeap::new();
    min_heap.push(State {
        distance: 0,
        node: start,
    });

    // Use Dijkstra's algorithm to find the shortest path between the start node and all other nodes.
    while let Some(State {distance, node }) = min_heap.pop() {
        if distance > distances[node] {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &(neighbor, weight) in neighbors {
                let neighbor_dist = distance + weight;
                // Only update the distance if we find a shorter path to this neighbor
                if neighbor_dist < distances[neighbor] {
                    distances[neighbor] = neighbor_dist;
                    min_heap.push(State {
                        distance: neighbor_dist,
                        node: neighbor,
                    });
                }
            }
        }
    }

    // convert all infinity values to -1, representing unreachable nodes.
    distances
        .into_iter()
        .map(|d| if d == usize::MAX { -1 } else { d as i32 })
        .collect()
}

fn main() {
    let n = 6;
    let edges = vec![
        (0, 1, 5),
        (0, 2, 3),
        (1, 2, 1),
        (1, 3, 4),
        (2, 3, 4),
        (2, 4, 5),
    ];
    let start = 0;
    let result = shortest_path(n, &edges, start);
    println!("{:?}", result); // [0, 4, 3, 7, 8, -1]
}
```
