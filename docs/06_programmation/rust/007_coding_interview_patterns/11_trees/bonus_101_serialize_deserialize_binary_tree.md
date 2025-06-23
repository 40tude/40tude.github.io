---
# published: false
layout: default
title: "bonus101 - Serialize and Deserialize a Binary Tree"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Serialize and Deserialize a Binary Tree

* Write a function to serialize a binary tree into a string
* Write another function to deserialize that string into initial binary tree

<span style="color:orange"><b>The point:</b></span>

* Serialization
* Make sure to we can identify the value of the root
* DFS preorder (root, left, right)
* In the string use '#' to represent empty nodes

Deserialization
* use preorder traversal to reconstruct the tree
* the first value is root

**Complexity :**

| Op          | Time        | Space |
| ------------|-------------|-------|
| Serialize   | O(n)        | O(n)  |
| Deserialize | O(n)        | O(n)  |

* O(n) in time in both cases because we need to visit n nodes exactly once. The serialize function converts to a string, which also takes
time.
* O(n) in space in both cases because of the size of recursive call stack. Can grow as large as the height of the binary tree (height max. = n). The serialize function uses a list to store the values of the string.



<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* Explicit index in ``deserialize()``

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



<span style="color:red"><b>TODO : </b></span> 
* Add more notes in ``About Rust``




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

// Helper function to perform serialization through preorder traversal.
fn preorder_serialize (node_link : &Link, serialized_list : &mut Vec<String>){
    match node_link {
        None => {
            // Base case: mark missing nodes as '#'.
            serialized_list.push("#".to_string());
            // return
        },
        Some(node) => {
            // Preorder traversal processes the current node first, then the left and right children
            serialized_list.push(node.value.to_string());
            preorder_serialize(&node.left, serialized_list); // no need for &mut serialized_list in recursive calls
            preorder_serialize(&node.right, serialized_list);
        },
    }
}

fn serialize(root_link : &Link) -> String {
    // Perform a preorder traversal to add node values to a list, then convert the list to a string.
    let mut serialized_list : Vec<String> = Vec::new(); // list of strings (node's values as string)
    preorder_serialize(root_link, &mut serialized_list);
    serialized_list.join(",")
}

fn build_tree(values: &[&str], index: &mut usize) -> Link {
    if *index >= values.len() {
        return None;
    }

    let val = values[*index];
    *index += 1;

    // Base case: '#' indicates a missing node
    if val == "#" {
        return None;
    }

    // Use preorder traversal processes the current node first, then the left and right children
    let parsed_val = val.parse().expect("Invalid integer");
    let mut node = TreeNode::new(parsed_val);
    node.left = build_tree(values, index);
    node.right = build_tree(values, index);
    Some(Box::new(node))
}

fn deserialize(data: &str) -> Link {
    // Obtain the node values by splitting the string using the comma delimiter
    let values: Vec<&str> = data.split(',').map(str::trim).collect();
    let mut index = 0;
    build_tree(&values, &mut index)
}


fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     9     3
    //    /     / \
    //   2     4   7
    //  /       \
    // 11        6
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(9)
                .left(
                    TreeNode::new(2)
                        .left(
                            TreeNode::new(11)
                        )
                )
                
        )
        .right(
            TreeNode::new(3)
                .left(
                    TreeNode::new(4)
                        .right(
                            TreeNode::new(6)
                        )
                )
                .right(
                    TreeNode::new(7)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 9 2 11 3 4 6 7   
    println!();
    
    let my_serial = serialize(&root_link);
    println!("{:?}", my_serial); // "5,9,2,11,#,#,#,#,3,4,#,6,#,#,7,#,#"
    
    let root_link2 = deserialize (&my_serial);
    preorder_print(&root_link2); // 5 9 2 11 3 4 6 7     

} // end of local scope OR end of main()
```

## V2 - With iterator

**About Rust :**
* Based on V2 (see ``189_intro.ipynb``) for easy tree building
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)





<span style="color:red"><b>TODO : </b></span> 
* Add comments in code
* Add more notes in ``About Rust``


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

// Helper function to perform serialization through preorder traversal.
fn preorder_serialize (node_link : &Link, serialized_list : &mut Vec<String>){
    match node_link {
        None => {
            serialized_list.push("#".to_string());
            return
        },
        Some(node) => {
            // Preorder traversal processes the current node first, then the left and right children
            serialized_list.push(node.value.to_string());
            preorder_serialize(&node.left, serialized_list); // no need for &mut serialized_list in recursive calls
            preorder_serialize(&node.right, serialized_list);
        },
    }
}

fn serialize(root_link : &Link) -> String{
    // Perform a preorder traversal to add node values to a list, then convert the list to a string.
    let mut serialized_list : Vec<String> = Vec::new(); // list of strings (node's values as string)
    preorder_serialize(&root_link, &mut serialized_list);
    serialized_list.join(",")
}

fn build_tree<'a, I>(values: &mut I) -> Link
where
    I: Iterator<Item = &'a str>,
{
    let val = values.next()?;
    // Base case: '#' indicates a missing node
    if val == "#" {
        return None;
    }
    let val = val.parse().expect("Invalid integer");
    let mut node = TreeNode::new(val);
    node.left = build_tree(values);
    node.right = build_tree(values);
    Some(Box::new(node))
}

fn deserialize(data: &str) -> Link {
    // Obtain the node values by splitting the string using the comma delimiter
    let mut values_iter = data.split(',').map(str::trim);
    build_tree(&mut values_iter)
}


fn main() { // no main() if this code runs in a Jupyter cell 
    // Build the tree:
    //        5
    //      /   \
    //     9     3
    //    /     / \
    //   2     4   7
    //  /       \
    // 11        6
    let tree = TreeNode::new(5)
        .left(
            TreeNode::new(9)
                .left(
                    TreeNode::new(2)
                        .left(
                            TreeNode::new(11)
                        )
                )
                
        )
        .right(
            TreeNode::new(3)
                .left(
                    TreeNode::new(4)
                        .right(
                            TreeNode::new(6)
                        )
                )
                .right(
                    TreeNode::new(7)
                )
        );

    let root_link = Some(Box::new(tree));
    preorder_print(&root_link); // 5 9 2 11 3 4 6 7   
    println!();
    
    let my_serial = serialize(&root_link);
    println!("{:?}", my_serial); // "5,9,2,11,#,#,#,#,3,4,#,6,#,#,7,#,#"
    
    let root_link2 = deserialize (&my_serial);
    preorder_print(&root_link2); // 5 9 2 11 3 4 6 7     

} // end of local scope OR end of main()
```
