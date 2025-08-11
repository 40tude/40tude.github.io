---
# published: false
layout: default
lang: en-US
title: "p193 - Invert Binary Tree"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Invert Binary Tree

<div align="center">
<img src="../assets/chap_11.webp" alt="" width="300" loading="lazy"/>
</div>

* Invert a binary tree and returns its root
    * invert => mirror image 





<span style="color:red"><b>TODO : </b></span> 
* Rust : come back on `.as_mut()`, `.as_deref()`        

## Recursive

<span style="color:orange"><b>The point:</b></span>

* Child are swapped relative to their parent node
* The swap of the left and right child happens to each node
    * all nodes under the moved child are moved as well
* DFS



**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we traverse each node exactly once
* O(n) in space because of the size of the recursive stack which is the height of the tree (``n`` at its max.) 

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* `&mut` everywhere
* Here `invert_binary_tree_recursive` does not return `root` but it acts in place
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


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

// since there is a swap, the link must be mutable then &mut must be propagated in the body of the function
fn invert_binary_tree_recursive(link: &mut Link){
    if let Some(node) = link {
        std::mem::swap(&mut node.left, &mut node.right);    // process current node
        invert_binary_tree_recursive(&mut node.left);       // left
        invert_binary_tree_recursive(&mut node.right);      // right
    }
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
    //         5
    //      /     \
    //     1       8
    //    / \       \
    //   7   6       4
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(1)
                .left(
                    TreeNode::new(7)
                )
                .right(
                    TreeNode::new(6)
                )
        )
        .right(
            TreeNode::new(8)
                .left(
                    TreeNode::new(4)
                ) // no child on the right
        );

    let mut root_link = Some(Box::new(tree));
    invert_binary_tree_recursive(&mut root_link);
    preorder_print(&root_link); // 5 8 4 1 6 7 
} // end of local scope OR end of main()       

```

## Iterative

<span style="color:orange"><b>The point:</b></span>

* The recursive call stack is a stack
* Here we use our own stack
* DFS



**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we traverse each node exactly once
* O(n) in space because of the size of the stack which is the height of the tree (``n`` at its max.) 

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* `Some(node) = link.as_mut() `
* `.as_mut()` everywhere
* Contrary to what is done in the book, `invert_binary_tree_iterative()` does not return `root` but it acts in place
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


**First idea that does not work :**

```rust
fn invert_binary_tree_iterative(link: &mut Link) { 
    let mut my_stack = Vec::new(); 

    if let Some(node) = link{
    // if let Some(node) = link.as_mut() {
        my_stack.push(node);

        while let Some(current) = my_stack.pop() {
            std::mem::swap(&mut current.left, &mut current.right);

            if let Some(left_node) = current.left{
                my_stack.push(left_node);
            }

            if let Some(right_node) = current.right{
                my_stack.push(right_node);
            }
        }
    }
}
```
* On the line `std::mem::swap(&mut node.left, &mut node.right);` we must be able to mutate `node.left` and `node.right`.
* If we write `if let Some(node) = link {`
    * Since ``link`` is of type ``&mut Link``
    * This mean ``link`` is, in fact, of type ``&mut Option<Box<TreeNode>>`` 
    * When we write `if let Some(node) = link {`
        * Rust is going through a code similar to :

```rust
match link {
    Some(node) => {
        // node is of type & Box<TreeNode>
        // it is read only, NOT mutable
    }
    None => {
        // Do nothing or handle the None case
    }
}
```
What we need is something like :

```rust
match link.as_mut() {
    Some(node) => {
        // node is now of type &mut Box<TreeNode>
        // it is mutable
        let tree_node: &mut TreeNode = node.as_mut(); 
    }
    None => {
        // Nothing to do
    }
}
```

This is why `.as_mut()` is used in ``invert_binary_tree_iterative()`` since it provides a mutable access to content of the reference

```rust
fn invert_binary_tree_iterative(link: &mut Link) { 
    let mut my_stack = Vec::new(); 

    if let Some(node) = link.as_mut() {
        my_stack.push(node);

        while let Some(current) = my_stack.pop() {
            std::mem::swap(&mut current.left, &mut current.right);

            if let Some(left_node) = current.left.as_mut() {
                my_stack.push(left_node);
            }

            if let Some(right_node) = current.right.as_mut() {
                my_stack.push(right_node);
            }
        }
    }
}


```


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

fn invert_binary_tree_iterative(link: &mut Link) { //link is of type &mut Link => &mut Option<Box<TreeNode>>
    let mut my_stack = Vec::new(); // our own stack

    // With `std::mem::swap(&mut node.left, &mut node.right);` we must be able to mutate `node.left` and `node.right`.
    // If we write `if let Some(node) = link {`
    //      * `node` is an immutable reference to the contents of the Option (`&Box<TreeNode>`).
    // We need to mutably dereference `link`, which leads us to use `if let Some(node) = link.as_mut()`
    //      * now `node` is a mutable reference to the contents of the Option (`&mut Box<TreeNode>`).
    // We must continue using `.as_mut()` to propagate mutability throughout the loop.

    if let Some(node) = link.as_mut() {
        my_stack.push(node);

        while let Some(current) = my_stack.pop() {
            // process current node => Swap left and right children
            std::mem::swap(&mut current.left, &mut current.right);

            // If left child exists, push it onto the stack
            // Using `.as_mut()` returns `Option<&mut Box<TreeNode>>`, allowing mutable access to the inner `Box<TreeNode>` so we can modify its children.
            if let Some(left_node) = current.left.as_mut() {
                my_stack.push(left_node);
            }

            // If right child exists, push it onto the stack
            if let Some(right_node) = current.right.as_mut() {
                my_stack.push(right_node);
            }
        }
    }
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
    //         5
    //      /     \
    //     1       8
    //    / \       \
    //   7   6       4
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(1)
                .left(
                    TreeNode::new(7)
                )
                .right(
                    TreeNode::new(6)
                )
        )
        .right(
            TreeNode::new(8)
                .left(
                    TreeNode::new(4)
                ) // no child on the right
        );

    let mut root_link = Some(Box::new(tree));
    invert_binary_tree_iterative(&mut root_link);
    preorder_print(&root_link); // 5 8 4 1 6 7 
} // end of local scope OR end of main()       
```

## Iterative bis

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* Better `invert_binary_tree_iterative` ?
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

fn invert_binary_tree_iterative(link: &mut Link) {
    let mut my_stack = Vec::new();

    if let Some(root) = link.as_mut() {
        my_stack.push(root);

        while let Some(node) = my_stack.pop() {
            // Swap left and right children
            std::mem::swap(&mut node.left, &mut node.right);

            // Push non-null children onto the stack
            for child in [&mut node.left, &mut node.right] {
                if let Some(inner) = child.as_mut() {
                    my_stack.push(inner);
                }
            }
        }
    }
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
    //         5
    //      /     \
    //     1       8
    //    / \       \
    //   7   6       4
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(1)
                .left(
                    TreeNode::new(7)
                )
                .right(
                    TreeNode::new(6)
                )
        )
        .right(
            TreeNode::new(8)
                .left(
                    TreeNode::new(4)
                ) // no child on the right
        );

    let mut root_link = Some(Box::new(tree));
    invert_binary_tree_iterative(&mut root_link);
    preorder_print(&root_link); // 5 8 4 1 6 7 
} // end of local scope OR end of main()       
```
