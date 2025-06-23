---
# published: false
layout: default
title: "p207 - Binary Search Tree Validation"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Binary Search Tree Validation

* Verify if a binary tree is a valid BST (binary search tree)
* nodes in left subtree are lower than node
* nodes in right subtree are greater than node

<span style="color:orange"><b>The point:</b></span>

* Recursive
* DFS



**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we traverse each node exactly once
* O(n) in space because of the size of the recursive stack which can grow as large as the heigh of binary tree (``n`` at its max.) 

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





<!-- <span style="color:red"><b>TODO : </b></span> 
* Rust : come back on `.as_mut()`, `.as_deref()`         -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## V1


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

fn is_within_bounds(link: &Link, lower_bound:i32, upper_bound:i32) -> bool {
    if let Some(node) = link {
        // if current node value not within the bounds, this is not a valid BST
        if node.value>=upper_bound || node.value<=lower_bound{
            return false
        }
        // If left subtree not a valid BST, this is not a valid BST
        if !is_within_bounds(&node.left, lower_bound, node.value){
            return false
        }
        // Otherwise returns true if the right subtree is a valid BST
        is_within_bounds(&node.right, node.value, upper_bound)
    } else {
        // base case, there is no node. BST condition is satisfied
        true
    }
}

fn binary_search_tree_validation(link: & Link)-> bool{
    is_within_bounds(link, i32::MIN, i32::MAX)
}

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     2     7
    //    / \   / \
    //   1   6 7   9
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(2)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(6)
                )
        )
        .right(
            TreeNode::new(7)
                .left(
                    TreeNode::new(7)
                )
                .right(
                    TreeNode::new(9)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 6 7 7 9    
    println!("\n{:?}", binary_search_tree_validation(& root_link)); // false
} // end of local scope OR end of main()
```

## V2

* <span style="color:lime"><b>Preferred solution?</b></span> 


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

fn is_within_bounds(link: &Link, lower_bound: i32, upper_bound: i32) -> bool {
    match link {
        Some(node) => {
            let value = node.value;
            // If current node's value is out of valid bounds, this is not a valid BST
            if value <= lower_bound || value >= upper_bound {
                return false;
            }
            // Recursively check the left and right subtrees
            is_within_bounds(&node.left, lower_bound, value) && is_within_bounds(&node.right, value, upper_bound)
        }
        None => true, // Empty subtree is valid
    }
}

fn binary_search_tree_validation(link: & Link)-> bool{
    is_within_bounds(link, i32::MIN, i32::MAX)
}

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     2     7
    //    / \   / \
    //   1   6 7   9
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(2)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(6)
                )
        )
        .right(
            TreeNode::new(7)
                .left(
                    TreeNode::new(7)
                )
                .right(
                    TreeNode::new(9)
                )
        );


    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 6 7 7 9    
    println!("\n{:?}", binary_search_tree_validation(& root_link)); // false
} // end of local scope OR end of main()
```

Copy-past the code below to return true (valid BST)


```rust
fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     3     8
    //    / \   / \
    //   1   4 7   9
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(3)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(4)
                )
        )
        .right(
            TreeNode::new(8)
                .left(
                    TreeNode::new(7)
                )
                .right(
                    TreeNode::new(9)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 3 1 4 8 7 9    
    println!("\n{:?}", binary_search_tree_validation(& root_link)); // true
} // end of local scope OR end of main()
```
