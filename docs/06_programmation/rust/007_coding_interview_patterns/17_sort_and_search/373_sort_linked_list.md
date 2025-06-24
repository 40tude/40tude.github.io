---
# published: false
layout: default
title: "p373 - Sort Linked List"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Sort Linked List

* Given the head of singly linked list of i32
* Sort the list in ascending order


<span style="color:orange"><b>The point:</b></span>

* Sort a list NOT an array 
    * no support for random access
    * Quicksort cannot be used
* Merge sort
* Merge sort is stable sort 
    * Original order of equal element is preserved


**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n log(n))    | O(log(n)) |

* O(n log(n)) in time because it use merge sort. The split happens log2(n). Merging takes n operation => O(n log(n))
* O(log(n)) in space because of depth of the recursive call stack (can grow up to log2(n))









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* First implementation

**About Rust :**
* In ``split_list()`` function, both ``slow`` and ``fast`` pointers need to point to head => ``Rc``
* In ``split_list()`` `slow` pointers needs to be modified => `RefCell` 
    * see `second = slow.as_ref().unwrap().borrow_mut().next.take();`
* See ``73_linked_list_loop.ipynb`` or `56_remove_kth_last_node.ipynb`
    * Where `Rc` and `RefCell` are used
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>; // type alias NOT a new type

struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self { val, next }))
    }
}

fn merge(l1: Link, l2: Link) -> Link {
    let dummy = Node::new(0, None);
    // This pointer will be used to append nodes to the tail of the merged linked list
    let mut tail = dummy.clone();

    let mut l1 = l1;
    let mut l2 = l2;

    // Continually append the node with the smaller value from each list to the merged list until one of the lists has no more node
    while l1.is_some() && l2.is_some() {
        let v1 = l1.as_ref().unwrap().borrow().val;
        let v2 = l2.as_ref().unwrap().borrow().val;

        if v1 < v2 {
            let next = l1.as_ref().unwrap().borrow_mut().next.take();
            tail.borrow_mut().next = l1.clone();
            l1 = next;
        } else {
            let next = l2.as_ref().unwrap().borrow_mut().next.take();
            tail.borrow_mut().next = l2.clone();
            l2 = next;
        }
        let next_tail = tail.borrow().next.as_ref().unwrap().clone();
        tail = next_tail;
    }
    // One of the 2 lists could still have nodes remaining
    tail.borrow_mut().next = if l1.is_some() { l1 } else { l2 };
    dummy.borrow().next.clone()
}

fn split_list(head: Link) -> (Link, Link) {
    let mut slow = head.clone();
    let mut fast = head.clone();

    while let Some(f_node) = fast {
        let f_next = f_node.borrow().next.clone();
        if let Some(f_next_next) = f_next.and_then(|n| n.borrow().next.clone()) {
            fast = Some(f_next_next);
            slow = slow.unwrap().borrow().next.clone();
        } else {
            break;
        }
    }
    let second = slow.as_ref().unwrap().borrow_mut().next.take();
    (head, second) // Return the original head without consuming it
}

fn sort_linked_list(head: Link) -> Link {
    // If the list is empty or has only one element, it is sorted
    if head.is_none() || head.as_ref().unwrap().borrow().next.is_none() {
        return head;
    }

    // Split the linked list into halves using the fast and slow pointer
    let (first, second) = split_list(head);

    // Recursively sort both halves
    let first_sorted = sort_linked_list(first);
    let second_sorted = sort_linked_list(second);

    merge(first_sorted, second_sorted)
}

fn main() { // no main() if this code runs in a Jupyter cell
// {        // local scope to avoid issue with the lifetime of head during borrow

    let mut head: Link = None; // Start with an empty list (head is None)
    let vals = vec![3, 2, 4, 5, 1];

    for v in vals.into_iter().rev() {
        head = Some(Node::new(v, head));
    }

    head = sort_linked_list(head);

    let mut current = head.clone();         // clone the smart pointer (this is cheap, Rc++)
    while let Some(node) = current {
        print!("{}->", node.borrow().val);
        current = node.borrow().next.clone();
    }
    println!("EOL") // EOL=End of List | expected output: 1->2->3->4->5->EOL
    
} // end of local scope OR end of main()
```

## V2 

* In place by updating `.next` Link of the nodes

**About Rust :**
* In additions to changes in the signatures, main modification are in `merge_in_place()` and `split_list()`
    * Read the comments
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>; // type alias NOT a new type

struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self { val, next }))
    }
}

fn merge_in_place(l1: Link, l2: Link) -> Link {
    let dummy = Node::new(0, None);
    // This pointer will be used to append nodes to the tail of the merged linked list
    let mut tail = dummy.clone();

    let mut l1 = l1;
    let mut l2 = l2;

    // Continually append the node with the smaller value from each list to the merged list until one of the lists has no more node
    while l1.is_some() && l2.is_some() {
        let v1 = l1.as_ref().unwrap().borrow().val;
        let v2 = l2.as_ref().unwrap().borrow().val;

        if v1 < v2 {
            let next = l1.as_ref().unwrap().borrow_mut().next.take();
            tail.borrow_mut().next = l1; // not l1.clone();
            l1 = next;
        } else {
            let next = l2.as_ref().unwrap().borrow_mut().next.take();
            tail.borrow_mut().next = l2; // not l2.clone();
            l2 = next;
        }
        let next_tail = tail.borrow().next.as_ref().unwrap().clone();
        tail = next_tail;
    }
    // One of the 2 lists could still have nodes remaining
    tail.borrow_mut().next = if l1.is_some() { l1 } else { l2 };
    dummy.borrow().next.clone()
}

fn split_list(head: &mut Link) -> (Link, Link) {
    let mut slow = head.clone();
    let mut fast = head.clone();

    while let Some(f_node) = fast {
        let f_next = f_node.borrow().next.clone();
        if let Some(f_next_next) = f_next.and_then(|n| n.borrow().next.clone()) {
            fast = Some(f_next_next);
            slow = slow.unwrap().borrow().next.clone();
        } else {
            break;
        }
    }
    // Now slow is at the midpoint
    let second = slow.as_ref().unwrap().borrow_mut().next.take();
    (head.take(), second)   // not (head, second)
                            // head is now a &mut Link
                            // take() replaces head with None and gives the contents of head as a property (Option<Rc<RefCell<Node>>>)
                            //      The original head in the caller is now None (empty)
                            //      The list is now splitted correctly and head points to the first half while and second points to the second half
}

fn sort_linked_list_in_place(head: &mut Link) {
    // If the list is empty or has only one element, it is sorted
    if head.is_none() || head.as_ref().unwrap().borrow().next.is_none() {
        return;
    }

    // Split the linked list into halves using the fast and slow pointer
    let (mut first, mut second) = split_list(head);

    // Recursively sort both halves
    sort_linked_list_in_place(&mut first);
    sort_linked_list_in_place(&mut second);

    // Merge sorted halves and update the original head
    *head = merge_in_place(first, second);
}

fn main() { // no main() if this code runs in a Jupyter cell
// {        // local scope to avoid issue with the lifetime of head during borrow

    let mut head: Link = None; // Start with an empty list (head is None)
    let vals = vec![3, 2, 4, 5, 1];

    for v in vals.into_iter().rev() {
        head = Some(Node::new(v, head));
    }

    sort_linked_list_in_place(&mut head);

    let mut current = head.clone(); // clone the smart pointer (this is cheap, Rc++)
    while let Some(node) = current {
        print!("{}->", node.borrow().val);
        current = node.borrow().next.clone();
    }
    println!("EOL"); // EOL=End of List | expected output: 1->2->3->4->5->EOL

} // end of local scope OR end of main()

```
