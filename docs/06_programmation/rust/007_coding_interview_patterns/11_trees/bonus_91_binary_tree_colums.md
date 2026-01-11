---
# published: false
layout: default
lang: en-US
title: "bonus091 - Binary Tree Columns"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Binary Tree Columns

<div align="center">
<img src="../assets/chap_11.webp" alt="" width="300" loading="lazy"/>
</div>

* Given the root of a binary tree
* Return a list of arrays
* Where each array represents a vertical column of the tree
* Nodes in the same column should be ordered from top to bottom
* Nodes in the same row and column should be ordered from left to right

<span style="color:orange"><b>The point:</b></span>

* root col id = 0. <0 on the left, >0 on the right
* for any node, the column ids of node.left and node.right are column - 1 and column + 1, respectively
* hash map with col ids as keys
* BFS


**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we process each node once during the level-order traversal
* O(n) in space because of the size of the queue. Will grow as large as the level with the most nodes (max val around n/2). The output vector is not taken into account in space complexity






<!-- <span style="color:red"><b>TODO : </b></span>
* Rust : come back on `.as_mut()`, `.as_deref()`         -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## V1

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::VecDeque;
use std::collections::HashMap;

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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn binary_tree_columns(link : &Link) -> Vec<Vec<i32>> {
    let mut column_map : HashMap<i32, Vec<i32>> = HashMap::new();
    let (mut leftmost_column, mut rightmost_column) = (0, 0);

    // if let Some(_) = link {
    if link.is_some(){
        let mut queue: VecDeque<(&Link, i32)> = VecDeque::new(); // link to node, column index
        queue.push_back((link, 0));
        while !queue.is_empty(){
            if let Some((Some(node), column)) = queue.pop_front() {
                // Add the current node's value to its corresponding list in the hash map
                //column_map[&column].push(node.value);
                if let Some(val) = column_map.get_mut(&column) { val.push(node.value); };
                leftmost_column = leftmost_column.min(column);
                rightmost_column = rightmost_column.max(column);
                // Add the current node's children to the queue with their respective column ids.
                queue.push_back((&node.left, column - 1));
                queue.push_back((&node.right, column + 1));
            }
        }
    }
    // Construct the output list by collecting values from each column in the hash
    // map in the correct order.
    let mut res: Vec<Vec<i32>> = Vec::new();
    for i in leftmost_column..= rightmost_column {
            res.push(column_map[&i].clone());
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell
    // Build the tree:
    //        5
    //      /   \
    //     9     3
    //    / \   / \
    //   2   1 4   7

    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(9)
                .left(
                    TreeNode::new(2)
                )
                .right(
                    TreeNode::new(1)
                )
        )
        .right(
            TreeNode::new(3)
                .left(
                    TreeNode::new(4)
                )
                .right(
                    TreeNode::new(7)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 9 2 1 3 4 7
    println!();
    println!("{:?}", binary_tree_columns(&root_link)); // 5 9 2 1 3 4 7

} // end of local scope OR end of main()
```

## V2

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* I wanted to lower the number of `if let Some(blah_blah_blah)...`
* pay attention on how the return collection is built
```rust
(leftmost_column..=rightmost_column)
        .filter_map(|i| column_map.get(&i).cloned())
        .collect()
```
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::VecDeque;
use std::collections::HashMap;

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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn binary_tree_columns(link: &Link) -> Vec<Vec<i32>> {

    let mut column_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let (mut leftmost_column, mut rightmost_column) = (0, 0);

    let mut queue: VecDeque<(&TreeNode, i32)> = VecDeque::new(); // link to node, column index

    if let Some(node) = link.as_deref() {
        queue.push_back((node, 0));
    }

    while let Some((node, column)) = queue.pop_front() {
        // Add the current node's value to its corresponding list in the hash map
        // column_map.entry(column).or_insert_with(Vec::new).push(node.value);
        column_map.entry(column).or_default().push(node.value);
        leftmost_column = leftmost_column.min(column);
        rightmost_column = rightmost_column.max(column);

        // Add the current node's children to the queue with their respective column ids.
        if let Some(left) = node.left.as_deref() {
            queue.push_back((left, column - 1));
        }
        if let Some(right) = node.right.as_deref() {
            queue.push_back((right, column + 1));
        }
    }

    // Construct the output list by collecting values from each column in the hash map in the correct order.
    (leftmost_column..=rightmost_column)
        .filter_map(|i| column_map.get(&i).cloned())
        .collect()
}

fn main() { // no main() if this code runs in a Jupyter cell
    // Build the tree:
    //        5
    //      /   \
    //     9     3
    //    / \   / \
    //   2   1 4   7

    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(9)
                .left(
                    TreeNode::new(2)
                )
                .right(
                    TreeNode::new(1)
                )
        )
        .right(
            TreeNode::new(3)
                .left(
                    TreeNode::new(4)
                )
                .right(
                    TreeNode::new(7)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 9 2 1 3 4 7
    println!();
    println!("{:?}", binary_tree_columns(&root_link)); // [[2], [9], [5, 1, 4], [3], [7]]

} // end of local scope OR end of main()
```
