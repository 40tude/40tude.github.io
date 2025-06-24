---
# published: false
layout: default
title: "p060 - Linked List Intersection"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Linked List Intersection

<div align="center">
<img src="../assets/chap_03.webp" alt="" width="300" loading="lazy"/>
</div>

* Return the node where 2 singly linked lists intersect (none otherwise)
* The intersection has nothing to do with the values of the nodes
   
**Complexity Analysis :**

One pass


| Time | Space |
|------|-------|
| O(n+m) | O(1)|

- Time, because we traverse both lists A (of length n) and B (of length m) 
- Space, because in place





**About Rust :**
* Intersection means pointers pointing to the same nodes => Rc
* No need to modify anything, just find the intersection => No RefCell
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





## Build & print lists
* Just to make sure I can "play" with such lists



```rust
use std::rc::Rc;
type Link = Option<Rc<Node>>; // type alias. Use Option and Rc to allow an optional reference count to the next node 

struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Self {
        Self { val, next }
    }
}


// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head_common = None; // Start with an empty list (head is None)
    let vals_common = vec![4, 8, 7, 2];
    for v in vals_common.into_iter().rev() {
        head_common = Some(Rc::new(Node::new(v, head_common)));
    }

    // This is NOT a copy. Here .clone() add a new pointer to head_common
    let mut head_a = head_common.clone(); 
    let vals_a = vec![1, 3];
    for v in vals_a.into_iter().rev() {
        head_a = Some(Rc::new(Node::new(v, head_a)));
    }

    let mut current = &head_a;
    while let Some(node) = current {
        print!("{}->", node.val);
        current = &node.next;
    }
    println!("EOL"); // End of List


    let mut head_b = head_common.clone(); 
    let vals_b = vec![6];
    for v in vals_b.into_iter().rev() {
        head_b = Some(Rc::new(Node::new(v, head_b)));
    }

    let mut current = &head_b;
    while let Some(node) = current {
        print!("{}->", node.val);
        current = &node.next;
    }
    println!("EOL"); // End of List

} // end of local scope OR end of main()   


```

    1 -> 3 -> 4 -> 8 -> 7 -> 2 -> None
    6 -> 4 -> 8 -> 7 -> 2 -> None





    ()



## Code 

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::rc::Rc;
type Link = Option<Rc<Node>>;

struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Self {
        Self { val, next }
    }
}

fn print_linked_list(head: &Link, name: &str) {
    print!("{} : ", name);
    let mut current = head;
    while let Some(node) = current {
        print!("{}->", node.val);
        current = &node.next;
    }
    println!("EOL") // End of List
}

fn linked_list_intersection(head_a: &Link, head_b: &Link) -> Link {
    let mut ptr_a = head_a;
    let mut ptr_b = head_b;

    loop {
        match (&ptr_a, &ptr_b) {
            (Some(a), Some(b)) => {
                if Rc::ptr_eq(a, b) {
                    // .clone() is mandatory here
                    // It is called on the referenced value (Option<Rc<Node>>) NOT on the the reference itself (&Option<Rc<Node>>)
                    // None or Some(Rc<Node>) is cloned (cheap operation since the reference counter is incremented)
                    return ptr_a.clone();
                }
            }
            (None, None) => return None,
            _ => {}
        }
        // If we reach the end of A, we start traversing B
        ptr_a = match ptr_a {
            Some(node) => &node.next,
            None => head_b,
        };
        // Traverse B then A
        ptr_b = match ptr_b {
            Some(node) => &node.next,
            None => head_a,
        };
    }
}


fn main(){     // no main() if this code runs in a Jupyter cell 
// {                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head_common = None; // Start with an empty list (head is None)
    let vals_common = vec![8, 7, 2];
    for v in vals_common.into_iter().rev() {
        head_common = Some(Rc::new(Node::new(v, head_common)));
    }

    // This is NOT a copy. Here .clone() add a new pointer to head_common
    let mut head_a = head_common.clone(); 
    let vals_a = vec![1, 3, 4];
    for v in vals_a.into_iter().rev() {
        head_a = Some(Rc::new(Node::new(v, head_a)));
    }                          
    print_linked_list(&head_a, "List       A");


    let mut head_b = head_common.clone(); 
    let vals_b = vec![6, 5];
    for v in vals_b.into_iter().rev() {
        head_b = Some(Rc::new(Node::new(v, head_b)));
    }                          
    print_linked_list(&head_b, "List       B");

    let intersection = linked_list_intersection(&head_a, &head_b);
    print_linked_list(&intersection, "Intersection");

} // end of local scope OR end of main()       
```

    List A : 1 -> 3 -> 4 -> 8 -> 7 -> 2 -> None
    List B : 6 -> 5 -> 8 -> 7 -> 2 -> None
    Intersection : 8 -> 7 -> 2 -> None





    ()


