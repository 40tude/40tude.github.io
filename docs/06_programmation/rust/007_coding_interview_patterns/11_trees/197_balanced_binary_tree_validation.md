---
# published: false
layout: default
title: "p197 - Balanced Binary Tree Validation"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Balanced Binary Tree Validation

* Determine if a binary tree is height-balanced
    * left and right subtrees don't have a height difference higher than 1

<span style="color:orange"><b>The point:</b></span>

* All the subtree need to be balanced too
* The height of a tree is equal to the deepest subtree+1
* Recursive
* DFS



**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we traverse each node exactly once
* O(n) in space because of the size of the recursive stack which is the height of the tree (``n`` at its max.) 

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

fn get_height_imbalance(link: &Link) -> i32 {
    if let Some(node) = link {
        // Recursively get the height of left and right subtrees
        let left_height = get_height_imbalance(&node.left);
        let right_height = get_height_imbalance(&node.right);

        // Propagate imbalance if detected in subtrees
        // if left_height == -1 || right_height == -1 {
        //     -1
        // } else if (left_height - right_height).abs() > 1 {
        //     -1
        // Return -1 if either subtree is imbalanced or height difference > 1
        if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1 {
            -1
        } else {
            1 + left_height.max(right_height)
        }
    } else {
        0
    }
}

fn balanced_binary_tree_validation(link: & Link)-> bool{
    get_height_imbalance(link) != -1
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
    //    / \     \
    //   1   4     9
    //      /     /
    //     3      6 
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(2)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(4)
                        .left(
                            TreeNode::new(3)
                        ) 
                )
        )
        .right(
            TreeNode::new(7)
                .right(
                    TreeNode::new(9)
                        .left(
                            TreeNode::new(6)
                        )
                ), 
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 4 3 7 9 6  
    println!("\n{:?}", balanced_binary_tree_validation(& root_link)); // false
} // end of local scope OR end of main()
```

## V2

**About Rust :**
* Does not use magic numbers like ``-1``
* It returns a `Result<i32, ()>` instead
    * Check the `?` at the end of the line ``let left_height = get_height_imbalance(&node.left)?;``
    * `Err(())` and ``Ok(i32)``
* `get_height_imbalance(link).is_ok()` because `get_height_imbalance` returns a ``Result<T, E>``
* <span style="color:lime"><b>Preferred solution?</b></span> 
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

fn get_height_imbalance(link: &Link) -> Result<i32, ()> {
    if let Some(node) = link {
        let left_height = get_height_imbalance(&node.left)?;
        let right_height = get_height_imbalance(&node.right)?;

        if (left_height - right_height).abs() > 1 {
            Err(())
        } else {
            Ok(1 + left_height.max(right_height))
        }
    } else {
        Ok(0)
    }
}

fn balanced_binary_tree_validation(link: &Link) -> bool {
    get_height_imbalance(link).is_ok()
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
    //    / \     \
    //   1   4     9
    //      /     /
    //     3      6 
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(2)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(4)
                        .left(
                            TreeNode::new(3)
                        ) 
                )
        )
        .right(
            TreeNode::new(7)
                .right(
                    TreeNode::new(9)
                        .left(
                            TreeNode::new(6)
                        )
                ), 
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 4 3 7 9 6 
    println!("\n{:?}", balanced_binary_tree_validation(& root_link)); // false
} // end of local scope OR end of main()
```

Copy-past the code below to return ``true`` (balanced tree)


```rust
fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     2     7
    //    / \     \
    //   1   4     9
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(2)
                .left(
                    TreeNode::new(1)
                )
                .right(
                    TreeNode::new(4)
                )
        )
        .right(
            TreeNode::new(7)
                .right(
                    TreeNode::new(9)
                ), 
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 4 7 9  
    println!("\n{:?}", balanced_binary_tree_validation(& root_link)); // true
} // end of local scope OR end of main()
```
