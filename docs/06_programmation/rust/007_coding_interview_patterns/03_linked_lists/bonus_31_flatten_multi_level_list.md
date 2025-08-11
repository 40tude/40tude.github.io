---
# published: false
layout: default
lang: en-US
title: "bonus031 - Flatten a Multi-Level Linked List"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Flatten a Multi-Level Linked List

<div align="center">
<img src="../assets/chap_03.webp" alt="" width="300" loading="lazy"/>
</div>

* Flatten the multi-level linked list into a single-level linked list by linking the end of each level to the start of the next one.
* Each node has a next pointer and child pointer.
* The order of the nodes on each level needs to be preserved.
* All the nodes in one level must connect before appending nodes from the next level.
* BFS => Queue => Linear space complexity
* Can we flatten in place ?
    * We can append each child linked list to the end of the current level, which effectively merges these two levels into one.
    * With all the nodes on level ‘L + 1’ appended to level ‘L’, we can continue this process by appending nodes from level ‘L + 2’ to level ‘L + 1’, and so on.



<div align="center">
<img src="./assets/img_01.png" alt="" width="900" loading="lazy"/>
</div>



<span style="color:orange"><b>The point:</b></span>
* use 2 pointers 
* tail points to the end of the level 0 list
* current traverse the list. Each time a child is found, the child list is appended to the end of the level 0 (including the child's children)
* **tail pointer** : we would need a reference to level 1’s tail node so we can easily add nodes to the end of the linked list. 
* **curr pointer** : traverse the linked list. Whenever it encounters a node with a child node that isn’t null, the child linked list is added to the end of the linked list. 
tail pointer must be updated


**Complexity :**
| Time | Space |
|------|-------|
| O(n) | O(1)  | 






**About Rust :**
* We have pointers pointing to the same structure => Rc
* We need to be able to modify what's is pointed => RefCell
    * RefCell supports interior mutability. This is a way of bypassing Rust's borrowing rules at runtime rather than at compile-time.
    * Without RefCell, an Rc<Node> cannot be modified. In fact, Rust forbids us to modify a structure contained in an Rc, as Rc only gives (immutable) read access to the underlying data. 
* Pay attention to :
```rust
let linked_lists: Vec<Link> = descriptor
        .iter()
        .map(|(_, vals)| build_the_list(vals.clone()))
        .collect();
```
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




<span style="color:red"><b>TODO : </b></span> 
* More comments in the source code


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<MultiLevelListNode>>>;    // type alias. Use Option and Rc<RefCell<...>>) to allow optional pointers to the next and child nodes 
                                                        // Rc<RefCell<...>>) to handle shared and mutable references

#[derive(Debug)]
struct MultiLevelListNode {
    val: i32,
    next: Link,
    child: Link,
}

impl MultiLevelListNode {
    fn new(val: i32, next: Link, child: Link) -> Link {
        Some(Rc::new(RefCell::new(Self { val, next, child })))
    }
}

fn print_linked_list(mut head: Link) {
    while let Some(node) = head {
        let node_borrow = node.borrow();
        print!("{}->", node_borrow.val);
        head = node_borrow.next.clone();
    }
    println!("EOL"); // Enf Of List
}

// Build a linked list from a vector 
fn build_the_list(values: Vec<i32>) -> Link {
    let mut head: Link = None;
    for &val in values.iter().rev() {
        head = MultiLevelListNode::new(val, head, None);
    }
    head
}

// Attach a child list to the node with matching value in the parent list
fn attach_list(ll_child: Link, val: i32, ll_parent: &Link) {
    let mut curr = ll_parent.clone();
    while let Some(node) = curr {
        if node.borrow().val == val {
            node.borrow_mut().child = ll_child;
            break;
        }
        curr = node.borrow().next.clone();
    }
}

// Flatten the multilevel list by appending child lists at the end
fn flatten_multi_level_list(head: Link) -> Link {
    if head.is_none() {
        return None;
    }

    let mut tail = head.clone();
    while let Some(t) = tail.clone() {
        let next = t.borrow().next.clone();  // Borrow ends here
        if next.is_some() {
            tail = next;
        } else {
            break;
        }
    }

    let mut curr = head.clone();
    while let Some(node) = curr {
        let mut node_mut = node.borrow_mut();
        if let Some(child) = node_mut.child.take() {
            // Append the child list to the tail
            tail.as_ref().unwrap().borrow_mut().next = Some(child.clone());

            // Move tail to the new end
            let mut new_tail = tail.clone();
            while let Some(t) = new_tail.clone() {
                let next = t.borrow().next.clone();  // Borrow ends here
                if next.is_some() {
                    new_tail = next;
                } else {
                    break;
                }
            }
            tail = new_tail;
        }
        curr = node_mut.next.clone();
    }
    head
}

fn main(){     // no main() if this code runs in a Jupyter cell 
// {                 // local scope to avoid issue with the lifetime of head during borrow
    let descriptor = [
        (None, vec![1, 2, 3, 4, 5]),
        (Some(2), vec![6, 7]),
        (Some(7), vec![10]),
        (Some(4), vec![8, 9]),
        (Some(8), vec![11]),
    ];

    let linked_lists: Vec<Link> = descriptor
        .iter()
        .map(|(_, vals)| build_the_list(vals.clone()))
        .collect();

    attach_list(linked_lists[1].clone(), 2, &linked_lists[0]);
    attach_list(linked_lists[2].clone(), 7, &linked_lists[1]);
    attach_list(linked_lists[3].clone(), 4, &linked_lists[0]);
    attach_list(linked_lists[4].clone(), 8, &linked_lists[3]);

    let flat = flatten_multi_level_list(linked_lists[0].clone());
    print_linked_list(flat); // 1->2->3->4->5->6->7->8->9->10->11->EOL
} // end of local scope OR end of main()    
```

    1->2->3->4->5->6->7->8->9->10->11->EOL





    ()


