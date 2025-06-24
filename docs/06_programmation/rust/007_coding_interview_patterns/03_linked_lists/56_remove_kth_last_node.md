---
# published: false
layout: default
title: "p056 - Remove the kth Last Node From a Linked List"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Remove the kth Last Node From a Linked List

<div align="center">
<img src="../assets/chap_03.webp" alt="" width="300" loading="lazy"/>
</div>

**Complexity Analysis :**

| Time | Space |
|------|-------|
| O(n) | O(1)  |

- Time, because the algorithm first traverse at most n nodes then the 2 pointers traverse the linked list at most once each
- Space, because in place





**About Rust :**
* Comments on Rust implementation


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





## 2 pointers with safe code 
* This code <span style="color:red"><b>does not work</b></span>
* We have 2 mutable pointers on the same list
* We can read, but none of them can write (modifying the node pointed by next for example)
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->



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

fn remove_kth_last_node(head: Link, k: usize) -> Link {

    // Move the leader k times
    let mut leader = head.as_ref();
    for _ in 0..k {
        if let Some(node) = leader {
            leader = node.next.as_ref();
        } else {
            return head; // k trop grand
        }
    }

    // Move leader and trailer forward until leader reaches the end
    let mut trailer = head.as_ref();
    while let Some(node) = leader {
        leader = node.next.as_ref();
        trailer = trailer.unwrap().next.as_ref();
    }

    // Suppress the node
    // Ideally I would like to get a pointer to the node and remove it
    // if let Some(node) = trailer {
    //     node.next = node.next.take();
    // }

    head // just because I need to return something
}


// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head = None; // Start with an empty list (head is None)
    
    let vals = vec![1, 2, 3, 4];
    for v in vals.into_iter().rev() {
        head = Some(Box::new(Node::new(v, head)));
    }
    

    // head = reverse_list(head);
    head = remove_kth_last_node(head, 2);

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



## Safe but...
* Safe but the code does not use 2 pointers
    * Phase 1 : determine the len of the list (first pass)
    * Phase 2 : remove the node (second pass)
* This is exactly what we **DON'T** want
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



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

fn remove_kth_last_node(head: Link, k: usize) -> Link {
    // Phase 1 : measure len
    let mut length = 0;
    let mut curr = head.as_ref();
    while let Some(node) = curr {
        length += 1;
        curr = node.next.as_ref();
    }

    if k == 0 || k > length {
        return head;
    }

    // Phase 2 : remove the node at index len-k
    let mut dummy = Box::new(Node::new(-1, head));
    let mut current = &mut dummy.next;
    for _ in 0..(length - k) {
        current = &mut current.as_mut().unwrap().next;
    }

    if let Some(node) = current {
        *current = node.next.take();
    }

    dummy.next // the head may have been removed
}


// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head = None; // Start with an empty list (head is None)
    
    let vals = vec![1, 2, 3, 4];
    for v in vals.into_iter().rev() {
        head = Some(Box::new(Node::new(v, head)));
    }
    

    // head = reverse_list(head);
    head = remove_kth_last_node(head, 2);

    let mut current = head.as_ref(); 
    while let Some(node) = current {
        print!("{}->", node.val); // Access the value
        current = node.next.as_ref(); // Move to the next node
    }
    println!("EOL") // End of List   1->3->4->EOL

} // end of local scope OR end of main()    
```

    1
    2
    4





    ()



## Use some unsafe code
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



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

fn remove_kth_last_node(head: Link, k: usize) -> Link {
    let mut dummy = Box::new(Node::new(-1, head));

    // Create raw pointers to navigate
    // Use raw pointers to bypass the borrow checker (safe here because we control the logic)
    let mut leader: *mut Link = &mut dummy.next;
    let mut trailer: *mut Link = &mut dummy.next;

    // Advance leader k times
    for _ in 0..k {
        unsafe {
            match &mut *leader {
                Some(node) => {
                    leader = &mut node.next;
                }
                None => return dummy.next,
            }
        }
    }

    // Advance both pointers until leader reaches the end of the list
    unsafe {
        while let Some(node) = &mut *leader {
            leader = &mut node.next;

            if let Some(t_node) = &mut *trailer {
                trailer = &mut t_node.next;
            }
        }

        // Remove the kth-last node
        if let Some(node) = &mut *trailer {
            *trailer = node.next.take();
        }
    }

    dummy.next // the head may have been removed
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head = None; // Start with an empty list (head is None)
    
    let vals = vec![1, 2, 3, 4];
    for v in vals.into_iter().rev() {
        head = Some(Box::new(Node::new(v, head)));
    }

    head = remove_kth_last_node(head, 2);

    let mut current = head.as_ref(); 
    while let Some(node) = current {
        print!("{}->", node.val); // Access the value
        current = node.next.as_ref(); // Move to the next node
    }
    println!("EOL") // End of List   1->3->4->EOL

} // end of local scope OR end of main()    


```

    1
    2
    4





    ()



## Safe
* Use `Rc` & `RefCell`
* We need pointers pointing to the same nodes => `Rc`
* `RefCell` supports interior mutability. This is a way of bypassing Rust's borrowing rules at **runtime** rather than at **compile-time**.
* Without `RefCell`, an `Rc<Node>` cannot be modified. Rust forbids to modify a structure contained in an `Rc`. An `Rc` only gives (immutable) read access to the underlying data. See :
```rust
trailer_borrow.next = to_remove.borrow().next.clone(); 
```
* There is a cost at runtime because... There is no free lunch.
* <span style="color:lime"><b>Preferred solution?</b></span> 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self {val, next: None }))
    }
}

fn remove_kth_last_node(head: Link, k: usize) -> Link {
    let dummy = Rc::new(RefCell::new(Node {
        val: 0,
        next: head.clone(),
    }));

    let mut leader = dummy.clone();
    let mut trailer = dummy.clone();

    // Move leader k+1 steps ahead
    for _ in 0..=k {
        let next = leader.borrow().next.clone();
        match next {
            Some(n) => leader = n,
            None => return dummy.borrow().next.clone(), // k is too large
        }
    }

    // Move both pointers until leader reaches the end
    while let Some(next_rc) = {
        let next = leader.borrow().next.clone(); // scoped borrow
        next
    } {
        leader = next_rc; // borrow finished, leader now reassignable
        let t_next = trailer.borrow().next.clone().unwrap();
        trailer = t_next;
    }

    // Remove the k-th last node
    let mut trailer_borrow = trailer.borrow_mut();
    if let Some(to_remove) = trailer_borrow.next.take() {
        trailer_borrow.next = to_remove.borrow().next.clone();
    }

    dummy.borrow().next.clone()
}


fn main(){     // no main() if this code runs in a Jupyter cell 
// {                 // local scope to avoid issue with the lifetime of head during borrow
    let mut head: Link = None;
    
    let vals = vec![1, 2, 3, 4];
    
    for v in vals.into_iter().rev() {
        let node = Node::new(v);
        node.borrow_mut().next = head;
        head = Some(node);
    }

    head = remove_kth_last_node(head, 2);
    
    let mut current = head.clone(); // clone the smart pointer (this is cheap, Rc++)
    while let Some(node) = current {
        print!("{}->", node.borrow().val);
        current = node.borrow().next.clone();
    }
    println!("EOL") // End of List    1->3->4->EOL
} // end of local scope OR end of main()    
```

    1 
    3 
    4 





    ()


