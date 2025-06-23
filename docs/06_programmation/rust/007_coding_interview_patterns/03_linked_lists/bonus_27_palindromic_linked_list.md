---
# published: false
layout: default
title: "bonus027 - Palindromic Linked List"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Palindromic Linked List

* Given the head of a singly linked list, determine if it is a palindrome.
    * 1 -> 2 -> 2 -> 1          True
    * 1 -> 2 -> 3 -> 2 -> 1     True
    * 1 -> 2 -> 3 -> 4          False


<span style="color:orange"><b>The point:</b></span>    
* We only need to compare the first half of the original linked list with the reverse of the second half
* If there are an odd number of elements, we can include the middle node in both halves

So : 
* Find the middle of the linked list to get the head of the second half
* Reverse the second half of the linked list from this middle node
* We assume it is acceptable to modify the linked list (here second half is reversed)

**Complexity Analysis :**

| Time | Space |
|------|-------|
| O(n) | O(1)  | 

* n is the capacity of the cache
* O(n) because iterate 3 times along the list
* O(1) in space complexity






**About Rust :**
* We only need to read => ``Box<T>``
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




<!-- <span style="color:red"><b>TO DO : </b></span> 
* More comments in the source code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
type Link = Option<Box<Node>>; // type alias. Use Option and Box to allow an optional pointer to the next node 

#[derive(Clone)]
struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Self {
        Self { val, next }
    }
}

// It borrows the list instead of taking ownership
// Returns a Link (i.e. a cloned Box<Node>). Indeed we want to perform a destructive reverse_list on the second half only. 
// This cloning avoids altering the original list, which is may be important
fn find_middle(head: &Link) -> Link {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while let Some(f_node) = fast {
        if let Some(f_next) = f_node.next.as_ref() {
            fast = f_next.next.as_ref();
            slow = match slow {
                Some(n) => n.next.as_ref(),
                None => None,
            };
            // The previous match can be replaced by the line below
            //      .and_then(...) apply an operation that returns an Option, if and only if the Option is Some(...), otherwise, it returns None.
            //      if slow is Some(val), then the closure |n| ... is executed 
            //      if slow is None, then nothing is executed, and .and_then(...) returns None
            // slow = slow.and_then(|n| n.next.as_ref());
        } else {
            break;
        }
    }
    slow.cloned() // Return a new Box<Node> if any
}


// see 50_linked_list_reversal.ipynb
// It takes ownership and return the new inverted head.
// The returned version replaces the old list, 
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

// Use a reference to the list
fn palindromic_linked_list(head: &Link) -> bool {
    let mid = find_middle(head);
    let second_head = reverse_list(mid);
    let mut ptr1 = head.as_ref();
    let mut ptr2 = second_head.as_ref();

    while let Some(node2) = ptr2 {
        if let Some(node1) = ptr1 {
            if node1.val != node2.val {
                return false;
            }
            ptr1 = node1.next.as_ref();
            ptr2 = node2.next.as_ref();
        } else {
            return false;
        }
    }
    true
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut head_a = None; // Start with an empty list (head is None)
    let vals_a = vec![1, 2, 3, 2, 1];
    for v in vals_a.into_iter().rev() {
        head_a = Some(Box::new(Node::new(v, head_a)));
    }
    println!("{}", palindromic_linked_list(&head_a));  // true
    
    let mut head_b = None; // Start with an empty list (head is None)
    let vals_b = [1, 2, 1, 2];
    for v in vals_b.into_iter().rev() {
        head_b = Some(Box::new(Node::new(v, head_b)));
    }    
    println!("{}", palindromic_linked_list(&head_b));  // false

} // end of local scope OR end of main()    
```

    true
    false





    ()


