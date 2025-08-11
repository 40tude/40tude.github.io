---
# published: false
layout: default
lang: en-US
title: "p221 - Maximum Sum of a Continuous Path in a Binary Tree"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Maximum Sum of a Continuous Path in a Binary Tree

<div align="center">
<img src="../assets/chap_11.webp" alt="" width="300" loading="lazy"/>
</div>

* Return the maximum sum of a continuous path in a binary tree (with at least one node). A path is :
* Sequence of nodes that can begin and end at any node
* Each consecutive pair of node is connected by an edge
* The path is a single continuous sequence of nodes
* Does'nt split into multiple path



<span style="color:orange"><b>The point:</b></span>

* ...


**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we process each node once during the level-order traversal
* O(n) in space because of the size of the recursive stack. Will grow as large as the heigh (max val = n) 

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* ``.max(...)`` is a method of ``i32``, NOT of ``&mut i32``.
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





<!-- <span style="color:red"><b>TODO : </b></span> 
* Rust : come back on `.as_mut()`, `.as_deref()`         -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn max_path_sum_helper(link : &Link, max_sum : &mut i32) -> i32{
    if let Some(node) = link {
        // collect the gains we can get from left/right subtrees
        // set them to 0 if negative
        let left_sum = max_path_sum_helper(&node.left, max_sum).max(0);
        let right_sum = max_path_sum_helper(&node.right, max_sum).max(0);
        // update the overall maximum path sum if current path sum is larger
        // .max(...) is a method of i32, NOT of &mut i32.
        *max_sum = (*max_sum).max(node.value + left_sum + right_sum);
        node.value + left_sum.max(right_sum) // Return max path sum rooted at this node
    }else{
        0 // If no node the path sum is 0
    }

}

fn max_path_sum(link : &Link) -> i32 {
    let mut max_sum = i32::MIN;
    max_path_sum_helper(link, &mut max_sum);
    max_sum    
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //    -10    8
    //    / \   / \
    //   1  -7 9   7
    //  /   /     / \
    // 11  -1    6   -3
    
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(-10)
                .left(
                    TreeNode::new(1)
                        .left(
                            TreeNode::new(11)
                        )
                )
                .right(
                    TreeNode::new(-7)
                        .left(
                            TreeNode::new(-1)
                        )
                )
        )
        .right(
            TreeNode::new(8)
                .left(
                    TreeNode::new(9)
                )
                .right(
                    TreeNode::new(7)
                        .left(
                            TreeNode::new(6)
                        )
                        .right(
                            TreeNode::new(-3)
                        )
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 -10 1 11 -7 -1 8 9 7 6 -3 
    println!();
    println!("Max sum  = {}", max_path_sum(&root_link)); // Max sum  = 30
    
} // end of local scope OR end of main()
```
