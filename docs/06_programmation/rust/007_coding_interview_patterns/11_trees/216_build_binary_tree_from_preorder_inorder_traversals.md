---
# published: false
layout: default
lang: en-US
title: "p216 - Build Binary Tree from Preorder and Inorder Traversals"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Build Binary Tree from Preorder and Inorder Traversals

<div align="center">
<img src="../assets/chap_11.webp" alt="" width="300" loading="lazy"/>
</div>

* Construct a binary tree using arrays of values obtained after a preorder traversal and an inorder traversal of the tree

<span style="color:orange"><b>The point:</b></span>

* We need preorder and inorder arrays
* First value of the preorder array is the root 
* In the inorder array, all values to the left of val are in the left subtree of val (resp right)
* Build the tree in preorder
* The next node to be created is the next value in the preorder array
* Recursive
* DFS



**Complexity :**

| Time        | Space |
|-------------|-------|
| O(n)        | O(n)  |

* O(n) in time because we traverse each elements in the preorder and inorder arrays once
* O(n) in space because of the size of the recursive stack which is the height of the tree (``n`` at its max.) 

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





<!-- <span style="color:red"><b>TODO : </b></span> 
* Rust : come back on `.as_mut()`, `.as_deref()`         -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
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
}

fn build_subtree(left : usize, right : usize, preorder : &[i32], inorder : &[i32], preorder_index: &mut usize, inorder_indexes_map : &HashMap<i32, usize>) -> Link{
    if left>right{
        return None;
    }
    
    let val = preorder[*preorder_index];
    *preorder_index +=1;

    if let Some(&inorder_index) = inorder_indexes_map.get(&val){
        // inorder_index > left <=> inorder_index > 0 && left <= inorder_index - 1
        // checking inorder_index > 0 avoid underflow when calling build_subtree() with inorder_index = 0 and so  inorder_index - 1 < 0
        let left_subtree = if inorder_index > left {    
            build_subtree(left, inorder_index - 1, preorder, inorder, preorder_index, inorder_indexes_map)
        } else {
            None
        };
        let right_subtree = build_subtree(inorder_index + 1, right, preorder, inorder, preorder_index, inorder_indexes_map);

        let mut node = TreeNode::new(val);
        node.left = left_subtree;
        node.right = right_subtree;
        Some(Box::new(node))
    }else{
        None
    }
}

fn build_binary_tree(preorder:&[i32], inorder:&[i32])->Link{
    let mut preorder_index = 0;
    let mut inorder_indexes_map = HashMap::new();

    for (i, val) in inorder.iter().enumerate(){
        inorder_indexes_map.insert(*val, i);
    }
    build_subtree(0, inorder.len()-1, preorder, inorder, &mut preorder_index, &inorder_indexes_map)
}

fn preorder_print(link: &Link) {
    if let Some(node) = link {
        print!("{} ", node.value);      // Process current node
        preorder_print(&node.left);     // Traverse left child
        preorder_print(&node.right);    // Traverse right child
    }
}

fn main(){
    let preorder = vec![5, 9, 2, 3, 4, 7];
    let inorder  = vec![2, 9, 5, 4, 3, 7];
    let tree = build_binary_tree( &preorder, &inorder);
    preorder_print(&tree); // 5 9 2 3 4 7 
}
```
