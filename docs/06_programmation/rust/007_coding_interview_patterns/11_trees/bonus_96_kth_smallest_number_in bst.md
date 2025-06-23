---
# published: false
layout: default
title: "bonus096 - Kth Smallest Number in a Binary Search Tree"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Kth Smallest Number in a Binary Search Tree

* Given the root of a binary search tree (BST) and an integer k, find the kth smallest node value.
* n>=1
* 1<=k<=n

<span style="color:orange"><b>The point:</b></span>

* Take advantage of the fact that we're dealing with a BST
* Start with the leftmost node since this is always the smallest node in a BST.
* **Inorder traversal**, where for each node, the left subtree is processed first, followed by the current node, and then the right subtree
* Build a sorted list of values using inorder traversal with a recursive function then return k-1 







<!-- <span style="color:red"><b>TODO : </b></span> 
* Rust : come back on `.as_mut()`, `.as_deref()`         -->



<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## V1 - Recursive

**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we need to traverse through all n nodes of the tree to attain the sorted list.
* O(n) in space because of the size of recursive call stack. Can grow as large as the height of the binary tree (height max. = n)


**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn inorder(link: &Link) -> Vec<i32> {
    match link {
        Some(node) => {
            let mut left = inorder(&node.left);
            left.push(node.value);
            left.append(&mut inorder(&node.right));
            left
        },
        None => vec![],
    }
}

fn kth_smallest_number_in_bst_recursice(root : &Link, k : usize) -> i32{
    let sorted_list = inorder(root);
    sorted_list[k-1]
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     2     7
    //    / \   / \
    //   1   4 6   9
    
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
                .left(
                    TreeNode::new(6)
                )
                .right(
                    TreeNode::new(9)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 4 7 6 9  
    println!();
    println!("{:?}", kth_smallest_number_in_bst_recursice(&root_link, 5)); // 6
    
} // end of local scope OR end of main()
```

## V2 - Recursive

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* Use an **accumulator**
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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn inorder_acc(link: &Link, acc: &mut Vec<i32>) {
    if let Some(node) = link {
        inorder_acc(&node.left, acc);
        acc.push(node.value);
        inorder_acc(&node.right, acc);
    }
}

fn inorder(link: &Link) -> Vec<i32> {
    let mut acc = Vec::new();
    inorder_acc(link, &mut acc);
    acc
}

fn kth_smallest_number_in_bst_recursice(root : &Link, k : usize) -> i32{
    let sorted_list = inorder(root);
    sorted_list[k-1]
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     2     7
    //    / \   / \
    //   1   4 6   9
    
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
                .left(
                    TreeNode::new(6)
                )
                .right(
                    TreeNode::new(9)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 4 7 6 9  
    println!();
    println!("{:?}", kth_smallest_number_in_bst_recursice(&root_link, 5)); // 6
    
} // end of local scope OR end of main()
```

## V3 - Iterative

**Complexity :**

| Time        | Space |
|-------------|-------|
| O(k+h)      | O(h)  |

* O(k+h) in time because 
    1. Iterative inorder traversal is in O(k)
    1. Traversing to the leftmost node is in O(h) (must be considered separately since we can have h>k, worst case h=n) 
* O(h) in space because of the size of the stack. Can grow as large as the height of the binary tree (height max. = n)


**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* `let mut current = root_link.as_deref()` and then `while current.is_some() ...`
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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn kth_smallest_number_in_bst_iterative(root_link: &Link, mut k: usize) -> i32 {
    
    let mut stack = Vec::new();             // empty stack to simulate in-order traversal
    let mut current = root_link.as_deref(); // current is of type Link = Option<Box<TreeNode>>

    while current.is_some() || !stack.is_empty() {
        // Go to the leftmost node and push all nodes onto the stack
        while let Some(node) = current {
            stack.push(node);
            current = node.left.as_deref();
        }

        // stack is NOT empty so ``.unwrap()`` is safe
        let node = stack.pop().unwrap();
        k -= 1;

        // If we've processed k nodes, return the current value
        if k == 0 {
            return node.value;
        }

        // Move to the right subtree
        current = node.right.as_deref();
    }

    // Should never happen if k is valid
    // assert!(false); // does not work here
    panic!("Tree does not contain k elements"); // panic!() never returns so the compiler is happy
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     2     7
    //    / \   / \
    //   1   4 6   9
    
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
                .left(
                    TreeNode::new(6)
                )
                .right(
                    TreeNode::new(9)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 4 7 6 9  
    println!();
    println!("{:?}", kth_smallest_number_in_bst_iterative(&root_link, 5)); // 6
    
} // end of local scope OR end of main()
```

## V4 - Iterative bis

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* `let mut stack: Vec<&TreeNode> = Vec::new();`. 
* We now use a stack of `&TreeNode` rather than `&Box<TreeNode>`
* Use a ``loop`` to shorten the code
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

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn kth_smallest_number_in_bst_iterative(root: &Link, mut k: usize) -> i32 {
    let mut stack: Vec<&TreeNode> = Vec::new(); // empty stack to simulate in-order traversal
    let mut current = root.as_deref(); // current is of type &TreeNode

    loop {
        while let Some(node) = current {
            stack.push(node);
            current = node.left.as_deref();
        }
        let node = stack.pop().expect("Tree does not contain k elements");
        k -= 1;
        if k == 0 {
            return node.value;
        }
        current = node.right.as_deref();
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     2     7
    //    / \   / \
    //   1   4 6   9
    
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
                .left(
                    TreeNode::new(6)
                )
                .right(
                    TreeNode::new(9)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 2 1 4 7 6 9  
    println!();
    println!("{:?}", kth_smallest_number_in_bst_iterative(&root_link, 5)); // 6
    
} // end of local scope OR end of main()
```
