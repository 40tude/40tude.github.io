---
# published: false
layout: default
title: "p200 - Rightmost Nodes of a Binary Tree"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Rightmost Nodes of a Binary Tree

* Return an array containing the values of the rightmost nodes at each level of a binary tree.




<span style="color:orange"><b>The point:</b></span>

* Level order traversal
* BFS



**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we process each node once during the level-order traversal
* O(n) in space because of the size of the queue. Will grow as large as the level with most nodes. Worst case happens at final level. (``n/2``). Result array does not count in Space complexity 

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* `let left_most_index = queue.front().map(|(_, idx)| *idx).unwrap();`
    * ``front()`` returns a reference on the first element `Option<&(&TreeNode, usize)>`
    * `.map(|(_, idx)| *idx)` : ignore the node, keep the index of type ``&usize`` and  provide `usize`
    * `.unwrap()` panic on None. Should not happens here since we know `queue` is not empty 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





<!-- <span style="color:red"><b>TODO : </b></span> 
* Rust : come back on `.as_mut()`, `.as_deref()`         -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn widest_binary_tree_level(link: &Link) -> usize {

    let mut max_width=0;

    if let Some(node) = link.as_deref() {                               // link is an &Option<Box<TreeNode>> and link.as_deref() returns an Option<&TreeNode>
        let mut queue: VecDeque<(&TreeNode, usize)> = VecDeque::new();  // stores pairs: (reference to node, position index in a full binary tree layout)
        queue.push_back((node, 0));                                     // node is a &TreeNode which can be pushed

        while !queue.is_empty() {
            let level_size = queue.len();
            // Set ``left_most_index`` to the index of the first node in this level 
            let left_most_index = queue.front().map(|(_, idx)| *idx).unwrap(); // we know queue in NOT empty 
            // Set ``right_most_index`` at the same point as ``left_most_index`` and update is as we traverse the level
            let mut right_most_index = left_most_index;
            for _ in 0..level_size {
                if let Some((current, i)) = queue.pop_front() {
                    if let Some(left_node) = current.left.as_deref() {
                        queue.push_back((left_node, 2*i+1));
                    }
                    // Add right child if exists
                    if let Some(right_node) = current.right.as_deref() {
                        queue.push_back((right_node, 2*i+2));
                    }
                    right_most_index = i;
                }
            }
            max_width = max_width.max(right_most_index-left_most_index+1)
        }
    }
    max_width
}


fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //         1
    //      /     \
    //     2       3
    //    / \       \ 
    //   4   5       7 
    //  /\    \     /
    // 8  9   11   14
   
    let tree = TreeNode::new(1)
    .left(
        TreeNode::new(2)
            .left(
                TreeNode::new(4)
                    .left(
                        TreeNode::new(8)
                    )   
                    .right(
                        TreeNode::new(9)
                    )  
            )
            .right(
                TreeNode::new(5)
                    .right(
                        TreeNode::new(11)
                    ) 
            )
    )
    .right(
        TreeNode::new(3)
            .right(
                TreeNode::new(7)
                    .left(
                        TreeNode::new(14)
                    )
        )           
    );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 1 2 4 8 9 5 11 3 7 14   
    println!("\n{:?}", widest_binary_tree_level(& root_link)); // 7 
} // end of local scope OR end of main()
```
