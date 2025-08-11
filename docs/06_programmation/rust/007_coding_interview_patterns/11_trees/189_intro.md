---
# published: false
layout: default
lang: en-US
title: "p189 - Introduction"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Introduction

<div align="center">
<img src="../assets/chap_11.webp" alt="" width="300" loading="lazy"/>
</div>

**Complexity :**

| Operation | Time     | Space |
| ----------|----------|-------|
| DFS       | O(n)     | O(h)  |
| BFS       | O(n)     | O(n)  |

**DFS - Depth First Search** :
* O(n) because we visit each node
* O(h) because space is mostly determined by the maximum depth of the recursive call stack (h)

**BFS - Breath First Search** :
* O(n) because we visit each node
* O(n) because space is mostly determined by the maximum number of nodes stored in the queue (n/2)

**About Rust :**
* No binary tree in Rust std lib (one can look for the crates ``id_tree``, ``binary-tree`` or ``im``)


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## V1 & DFS (Depth First Search)

**About Rust :**
* I keep the call to `process()` just to show where it takes place
* I don't like the way the tree is built in `main()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
type Link = Option<Box<TreeNode>>; 

struct TreeNode {
    value: i32,
    left: Link, 
    right: Link, 
}

impl TreeNode {
    fn new(value: i32, left: Link, right: Link) -> Self { 
        Self { value, left, right }
    }
}

// Process a node 
fn process(node: &Link){
    if let Some(current_node) = node{
        print!("{:?} ", current_node.value);
    }
}

fn preorder_print(node : &Link){
    if let Some(current_node) = node{
        process(node);
        preorder_print(&current_node.left);
        preorder_print(&current_node.right)
    }
}

fn main(){
    let node1 = Some(Box::new(TreeNode::new(1, None, None)));
    let node3 = Some(Box::new(TreeNode::new(3, None, None)));
    let node6 = Some(Box::new(TreeNode::new(6, None, None)));
    let node7 = Some(Box::new(TreeNode::new(7, None, None)));
    let node2 = Some(Box::new(TreeNode::new(2, node1, node3)));
    let node5 = Some(Box::new(TreeNode::new(5, node6, node7)));
    let node4 = Some(Box::new(TreeNode::new(4, node2, node5)));

    let root = &node4; 
    preorder_print(root); // 4 2 1 3 5 6 7 
}
```

## V2 & DFS (Depth First Search) preorder

* I don't like the way the tree is built in `main()`
* ``process()`` now get a ``&TreeNode`` as parameter
* ``dfs()`` has a ``&Link`` as parameter

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
type Link = Option<Box<TreeNode>>;

struct TreeNode {
    value: i32,
    left: Link,
    right: Link,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Add child on the left
    fn left(mut self, node: TreeNode) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    // Add child on the right
    fn right(mut self, node: TreeNode) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

// Process a node 
fn process(node: &TreeNode) {
    print!("{} ", node.value);
}

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        process(node);      // Process current node
        preorder_print(&node.left);    // Traverse left child
        preorder_print(&node.right);   // Traverse right child
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //         4
    //      /     \
    //     2       5
    //    / \     / \
    //   1   3   6   7
    let tree = TreeNode::new(4)
        .left(
            TreeNode::new(2)
                .left(TreeNode::new(1))
                .right(TreeNode::new(3)),
        )
        .right(
            TreeNode::new(5)
                .left(TreeNode::new(6))
                .right(TreeNode::new(7)),
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 4 2 1 3 5 6 7
} // end of local scope OR end of main()       
```

## V2 & BFS (Breath First Search)

**About Rust :**
* `use std::collections::VecDeque;`
* `let mut queue: VecDeque<&TreeNode> = VecDeque::new();`
* `.as_deref()`
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::VecDeque;
type Link = Option<Box<TreeNode>>; 

struct TreeNode {
    value: i32,
    left: Link,
    right: Link,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Add child on the left
    fn left(mut self, node: TreeNode) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    // Add child on the right
    fn right(mut self, node: TreeNode) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

// Process a node 
fn process(node: &TreeNode) {
    print!("{} ", node.value);
}

fn bfs(link: &Link) {
    // if let Some(_) = link {
    if link.is_some(){
        let mut queue: VecDeque<&TreeNode> = VecDeque::new(); // store TreeNode, not Link
        if let Some(node) = link.as_deref() {   // link is an &Option<Box<TreeNode>> and link.as_deref() returns an Option<&TreeNode>
            queue.push_back(node);              // node is a &TreeNode which can be pushed
        }

        while let Some(current) = queue.pop_front() {
            process(current);              // Process current node
            // Add left child if exists
            if let Some(left_node) = current.left.as_deref() { // current.left is an Option<Box<TreeNode>> and we get an Option<&TreeNode>
                queue.push_back(left_node);
            }
            // Add right child if exists
            if let Some(right_node) = current.right.as_deref() {
                queue.push_back(right_node);
            }
        }
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //         4
    //      /     \
    //     2       5
    //    / \     / \
    //   1   3   6   7
    let tree = TreeNode::new(4)
        .left(
            TreeNode::new(2)
                .left(TreeNode::new(1))
                .right(TreeNode::new(3))
        )
        .right(
            TreeNode::new(5)
                .left(TreeNode::new(6))
                .right(TreeNode::new(7))
        );

    let root_link = Some(Box::new(tree));
    bfs(&root_link); //4 2 5 1 3 6 7
} // end of local scope OR end of main()       
```

## V3 - DFS preorder - BFS

**About Rust :**
* `build_balanced_tree()` - input vector of i32 **must be** in ascending order
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::VecDeque;
type Link = Option<Box<TreeNode>>;

struct TreeNode {
    value: i32,
    left: Link,
    right: Link,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

// nums is ascending order
fn build_balanced_tree(nums: &[i32]) -> Link {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;
    let mut root = TreeNode::new(nums[mid]);

    root.left = build_balanced_tree(&nums[..mid]);
    root.right = build_balanced_tree(&nums[mid+1..]);

    Some(Box::new(root))
}

// Process a node 
fn process(node: &TreeNode) {
    print!("{} ", node.value);
}

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        process(node);      // Process current node
        preorder_print(&node.left);    // Traverse left child
        preorder_print(&node.right);   // Traverse right child
    }
}

fn bfs(link: &Link) {
    if link.is_some() {
        let mut queue: VecDeque<&TreeNode> = VecDeque::new(); // store TreeNode, not Link
        if let Some(node) = link.as_deref() {   // link is an &Option<Box<TreeNode>> and link.as_deref() returns an Option<&TreeNode>
            queue.push_back(node);              // node is a &TreeNode which can be pushed
        }

        while let Some(current) = queue.pop_front() {
            process(current);              // Process current node
            // Add left child if exists
            if let Some(left_node) = current.left.as_deref() { // current.left is an Option<Box<TreeNode>> and we get an Option<&TreeNode>
                queue.push_back(left_node);
            }
            // Add right child if exists
            if let Some(right_node) = current.right.as_deref() {
                queue.push_back(right_node);
            }
        }
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    //         4
    //       /   \
    //      2     6
    //     / \   / \
    //    1   3 5   7
    let nums: Vec<i32> = (1..=7).collect();
    let root_link = build_balanced_tree(&nums);         // ! nums in ascending order
    preorder_print(&root_link); // 4 2 1 3 6 5 7
    println!();
    bfs(&root_link); // 4 2 6 1 3 5 7
} // end of local scope OR end of main()       
```
