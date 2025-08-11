---
# published: false
layout: default
lang: en-US
title: "p073 - Linked List Loop"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Linked List Loop

<div align="center">
<img src="../assets/chap_04.webp" alt="" width="300" loading="lazy"/>
</div>

* Given a singly linked list, determine if it contains a cycle
* A cycle occurs if a node's next pointer reference an earlier node in the list (causing a loop)



<span style="color:orange"><b>The point:</b></span>
* Use 2 ptrs 
* One fast and one slow
* fast move 2 steps at a time (``fast = fast.next.next``)
* fast ptr will always catch up slow ptr. worst case : k, length of the cycle

If no cycle, fast leave the linked list

**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(1)  |




**About Rust :**
* Naïve and not naïve implementations are demonstrated 
* If there is a loop in the list => 2 pointers pointing to the same cell => Rc
* RefCell supports interior mutability. This is a way of bypassing Rust's borrowing rules at runtime rather than at compile-time.
* Without RefCell, an Rc<Node> cannot be modified. Rust forbids to modify a structure contained in an Rc. An Rc only gives (immutable) read access to the underlying data. See :

```rust
tail.borrow_mut().next = Some(Rc::clone(target)); 
```
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

type Link = Option<Rc<RefCell<Node>>>; 

struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self { val, next }))
    }
}

fn linked_list_loop_naive(head : &Link)->bool{
    let mut visited = HashSet::new(); // O(n) space
    let mut current = head.as_ref().map(Rc::clone);
    while let Some(node_rc) = current {
        let addr = Rc::as_ptr(&node_rc) as usize;
        if visited.contains(&addr){
            return true;
        }
        visited.insert(addr);
        current = node_rc.borrow().next.as_ref().map(Rc::clone);
    }
    false
}



fn linked_list_loop(head: &Link) -> bool {
    let mut slow = head.as_ref().map(Rc::clone);
    let mut fast = head.as_ref().map(Rc::clone);

    while let (Some(s_node), Some(f_node)) = (&slow, &fast) {
        // Advance slow one step
        let next_slow = s_node.borrow().next.as_ref().map(Rc::clone);

        // Advance fast two steps
        let next_fast_1 = f_node.borrow().next.as_ref().map(Rc::clone);
        let next_fast_2 = match next_fast_1 {
            Some(ref next) => next.borrow().next.as_ref().map(Rc::clone),
            None => None,
        };

        slow = next_slow;
        fast = next_fast_2;

        // Check for pointer equality (cycle detection)
        if let (Some(s_ptr), Some(f_ptr)) = (&slow, &fast) {
            if Rc::ptr_eq(s_ptr, f_ptr) {
                return true;
            }
        }
    }
    false
}

fn main(){     // no main() if this code runs in a Jupyter cell 
// {                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head = None; // Start with an empty list (head is None)
    let vals = vec![0, 1, 2, 3, 4, 5];
    let mut nodes = vec![];  // Keep references to all nodes in a vector for later access
    for v in vals.into_iter().rev() {
        let new_node = Node::new(v, head.take());
        head = Some(Rc::clone(&new_node));
        nodes.push(new_node);
    }
    println!("{}", linked_list_loop_naive(&head));
    println!("{}", linked_list_loop(&head));

    let tail = &nodes[0]; // node 5
    let target = &nodes[2]; // node 3
    tail.borrow_mut().next = Some(Rc::clone(target)); // cycle: 5 → 3
    println!("{}", linked_list_loop_naive(&head));
    println!("{}", linked_list_loop(&head));
} // end of local scope OR end of main()    

```

    false
    false
    true
    true





    ()


