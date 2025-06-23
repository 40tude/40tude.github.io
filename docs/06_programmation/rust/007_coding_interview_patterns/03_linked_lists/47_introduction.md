---
# published: false
layout: default
title: "p047 - Introduction"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Introduction

## First implementation
* A node holds a value and a pointer to the next node
* Here, a node holds a value and an Option which is either None or Some() pointer to the next node
* If the Option is None this means we reach the end of the list
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>  -->




```rust
struct Node {
    val: i32,
    next: Option<Box<Node>>, // Use Option and Box to allow an optional pointer to the next node
}

impl Node {
    fn new(val: i32, next: Option<Box<Node>>) -> Node {
        Node { val, next }
    }
}

fn main(){   
    let mut head:Option<Box<Node>> = None; // Start with an empty list (head is None)
    
    let node = Node::new(100, head); // Creates a node linked to the head 
    head = Some(Box::new(node)); // Boxes the node and updates the head 

    let node = Node::new(200, head); 
    head = Some(Box::new(node)); 



    // Using head directly would move the value and make head unusable
    // Using &head gives a reference to the Option itself, NOT to the value inside
    // I need a reference to the Box<Node> stored in the Option, without taking ownership
    // That's what head.as_ref() does: it gives an Option<&Box<Node>>
    // 
    // The binding `current` is mutable (not the data itself)
    // So that it starts at the head and can be updated to point to the next node
    let mut current = head.as_ref(); 

    while let Some(node) = current {
        print!("{}->", node.val); // Access the value
        current = node.next.as_ref(); // Move to the next node
    }
    println!("EOL") // End of List
}   

```

## Second attempt

* `for v in vals.into_iter().rev() {`    
    * `into_iter()` gives ownership of the elements to the iterator
    * no copying, no additional allocations, no unnecessary clones (`.cloned()`) 
    * **but** `vals` is lost

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
struct Node {
    val: i32,
    next: Option<Box<Node>>, // Use Option and Box to allow an optional pointer to the next node
}

impl Node {
    fn new(val: i32, next: Option<Box<Node>>) -> Node {
        Self { val, next } // Now returns Self rather than Node
    }
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow
    
    let mut head:Option<Box<Node>> = None; // Start with an empty list (head is None)
    
    let vals = vec![1, 2, 3, 4];
    // for v in vals.into_iter().rev() {
    //     let node = Node::new(v, head); // Creates a node linked to the head 
    //     head = Some(Box::new(node)); // Boxes the node and updates the head 
    // }
    for v in vals.into_iter().rev() {
        head = Some(Box::new(Node::new(v, head)));
    }

    // Using head directly would move the value and make head unusable
    // Using &head gives a reference to the Option itself, not to the value inside
    // I need a reference to the Box<Node> stored in the Option, without taking ownership
    // That's what head.as_ref() does: it gives an Option<&Box<Node>>
    // 
    // The binding `current` is mutable (not the data itself)
    // So that it starts at the head and can be updated to point to the next node

    let mut current = head.as_ref(); 
    while let Some(node) = current {
        print!("{}->", node.val); // Access the value
        current = node.next.as_ref(); // Move to the next node
    }
    println!("EOL") // End of List
} // end of local scope OR end of main()
```

    1
    2
    3
    4





    ()



## Third attempt
* Use type alias
* Double check how the printing is done
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
type Link = Option<Box<Node>>; // type alias. Use Option and Box to allow an optional pointer to the next node 

struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Self { // now returns Self
        Self { val, next }
    }
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head = None; // Start with an empty list (head is None)
    
    let vals = vec![1, 2, 3, 4];
    for v in vals.into_iter().rev() {
        head = Some(Box::new(Node::new(v, head)));
    }

    // & avoids explicit call to as_ref(), as & on an Option<Box<Node>> obviously gives an &Option<Box<Node>>
    // In addition, Rust automatically applies implicit dereferencing when matching with while let.
    let mut current = &head; 
    while let Some(node) = current {
        print!("{}->", node.val); // Access the value
        current = &node.next; // Move to the next node
    }
    println!("EOL") // End of List
} // end of local scope OR end of main()       
```

    1
    2
    3
    4





    ()


