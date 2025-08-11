---
# published: false
layout: default
lang: en-US
title: "p050 - Linked List Reversal"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Linked List Reversal

<div align="center">
<img src="../assets/chap_03.webp" alt="" width="300" loading="lazy"/>
</div>

* Reverse a singly linked list
   
**Complexity Analysis :**

One pass


| Time | Space |
|------|-------|
| O(n) | O(1)  |

- Time, because we perform constant time pointer manipulation at each node
- Space, because in place




**About Rust :**
* <span style="color:lime"><b>Preferred solution?</b></span> 
* Use type alias to lighten code writing
    * This impact the signature of ``reverse_list()``
* `for v in vals.into_iter().rev()`
    * `into_iter()` gives ownership of the elements to the iterator
    * no copying, no additional allocations, no unnecessary clones (`.cloned()`) 
    * **but** `vals` is lost
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





```rust
type Link = Option<Box<Node>>; // type alias. Use Option and Box to allow an optional pointer to the next node 

struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Self {
        Self { val, next }
    }
}

fn reverse_list(mut head: Link) -> Link {
    let mut prev_node = None; // Start with no previous node

    while let Some(mut current_node) = head {
        // Save the next node before changing pointers
        let next_node = current_node.next.take(); // take() replaces current_node.next with None and gives its original value

        // Reverse the pointer
        current_node.next = prev_node;

        // Move prev_node and head forward
        prev_node = Some(current_node);
        head = next_node;
    }
    prev_node
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head = None; // Start with an empty list (head is None)
    
    let vals = vec![1, 2, 3, 4];
    for v in vals.into_iter().rev() {
        head = Some(Box::new(Node::new(v, head)));
    }
    
    head = reverse_list(head);

    let mut current = head.as_ref(); 
    while let Some(node) = current {
        print!("{}->", node.val); // Access the value
        current = node.next.as_ref(); // Move to the next node
    }
    println!("EOL") // End of List

} // end of local scope OR end of main()       
```

    4
    3
    2
    1





    ()



## Too much ?
* I prefer the code in the previous cell because it is more in the "spirit" of the book
* Here the implementation of ``main()`` looks good but then the algorithm get lost in the ``impl`` part of the ``List``

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
type Link = Option<Box<Node>>; // type alias. Use Option and Box to allow an optional pointer to the next node 

// #[derive(Debug)]
struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

// #[derive(Debug)]
struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(), // take() replaces self.head.next with None and gives its original value
        });
        self.head = Some(new_node);
    }

    fn reverse(&mut self) {
        let mut head = self.head.take(); // Take ownership of the head
        let mut prev_node = None; // Initialize previous node to None
    
        while let Some(mut current_node) = head {
            let next_node = current_node.next.take(); // Save next node
            current_node.next = prev_node; // Reverse the pointer
            prev_node = Some(current_node); // Move prev_node forward
            head = next_node; // Advance head to next node
        }
        self.head = prev_node; // Update head to the new front
    }

    fn print(&self) {
        let mut current = self.head.as_deref(); // Shortcut: as_deref() turns Option<Box<T>> into Option<&T>
        while let Some(Node { val, next }) = current {
            print!("{}->", val);
            current = next.as_deref();
        }
        println!("EOL") // End of List
    }   
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut list = List::new();
    for v in [1, 2, 3, 4] {
        list.push(v);
    }
    list.print(); //println!("{:?}", list);
    list.reverse();
    list.print(); //println!("{:?}", list);
    
} // end of local scope OR end of main()       
```

    4
    3
    2
    1
    
    1
    2
    3
    4
    





    ()



## Test using the Rust collections::LinkedList

**About Rust :**
* `for val in list.iter() `
    * `.iter()` provides immutable references to the elements ``&T`` 
    * `list` remains available afterwards (not consumed)
    * since ``list`` is no longer used we could have used ``.into_iter()`` instead

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::LinkedList;

fn reverse_list(mut list: LinkedList<i32>) -> LinkedList<i32> {
    let mut reversed = LinkedList::new();

    // Pop elements from the front of the original list and push them to the front of the new one
    while let Some(val) = list.pop_front() {
        reversed.push_front(val);
    }

    reversed
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow
    let mut list = LinkedList::new();
    let vals = vec![1, 2, 3, 4];
    for v in vals {
        list.push_back(v); // Insert elements in order
    }

    list = reverse_list(list);

    for val in list.iter() {
    // for val in list.into_iter() {
        print!("{}->", val); // Print each value in the reversed list
    }
    println!("EOL") // End of List
} // end of local scope OR end of main()       

```
