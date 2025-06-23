---
# published: false
layout: default
title: "p151 - Combine Sorted Linked Lists"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Combine Sorted Linked Lists

* Given ``k`` linked lists, each sorted ascending, combine them in one sorted linked list


<span style="color:orange"><b>The point:</b></span>

* Use a min-heap
* If the comparaison operator of the Node is redefined, this impacts the comparaisons in the min-heap (<span style="color:green"><b>good</b></span>) but also the rest of the application (<span style="color:orange"><b>may be bad</b></span>)


**Complexity :**

| Time               | Space |
|--------------------|-------|
| O(n + log(k))     | O(k)  |

* Time : O(n + log(k)). O(k log(k)) to create le heap (k nodes inserted one by one). Then for all n nodes we push and pop in O(log(k)). O(k log(k)) + n O(log(k)) = O(n log(k))
* Space : O(k) because the heap stores up to one node of each `k` lists 

**About Rust :**
* BinaryHeap is a max-heap by default. It sorts elements so that the largest is at the top. 
* Here we want to extract the smallest element first, which corresponds to a min-heap. 
* So, for BinaryHeap to function as a min-heap, the order of comparison must be reversed. 
  

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## First version

* Behaves like the code in the book
* **ATTENTION** comparaisons between nodes are reversed 


```rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Link = Option<Box<Node>>; // type alias. Use Option and Box to allow an optional pointer to the next node 

#[derive (Debug)]
struct Node {
    val: i32,
    next: Link, // use the alias here for clarity
}

impl Node {
    fn new(val: i32, next: Link) -> Self {
        Self { val, next }
    }
}

// Implement comparison traits for Node so it can be used in a BinaryHeap
impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap behavior
        other.val.cmp(&self.val)
    }
}

// Ord implies PartialOrd, so we must implement PartialOrd if we implement Ord
// Implement PartialOrd to allow comparison using <, >, etc.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn combined_sorted_linked_lists(mut lists: Vec<Link>) -> Link {
    let mut heap = BinaryHeap::new();

    // Initialize heap with head of each list
    for list in lists.iter_mut() {
        if let Some(node) = list.take() {
            heap.push(node);
        }
    }

    // Dummy node to start building the result
    let mut dummy = Box::new(Node::new(-1, None));
    let mut tail = &mut dummy;

    // While the heap is not empty, extract the smallest node
    while let Some(mut node) = heap.pop() {
        // Append node to result list
        tail.next = Some(Box::new(Node::new(node.val, None)));
        tail = tail.next.as_mut().unwrap();

        // Push next node into heap if it exists
        if let Some(next_node) = node.next.take() {
            heap.push(next_node);
        }
    }

    dummy.next
}

fn main(){   // no main() if this code runs in a Jupyter cell 

    fn create_list(vals : &[i32])->Link{
        let mut head = None;
        // for v in vals.into_iter().rev() {    // with .into_iter() vals would be lost
        for v in vals.iter().rev() {            // .iter() returns &T
                head = Some(Box::new(Node::new(*v, head)));
            }
        head
    }

    let lists = vec![
        create_list(&[1, 6]),
        create_list(&[1, 4, 6]),
        create_list(&[3, 7])
    ];

    let head_combined = combined_sorted_linked_lists(lists);

    let mut current = head_combined.as_ref();
    while let Some(node) = current {
        print!("{}->", node.val);
        current = node.next.as_ref();
    }
    println!("EOL"); // End of List 1->1->3->4->6->6->7->EOL
    
    
    // Understand the comparaison between nodes is reversed
    let n1 = Node::new(1, None);
    let n2 = Node::new(2, None);

    println!("\nn1 = {:?}, n2 = {:?}", n1, n2); // n1 = Node { val: 1, next: None }, n2 = Node { val: 2, next: None }
    
    // Because of reversed comparison, n1 > n2
    println!("n1 < n2: {}", n1 < n2);           // false
    println!("n1 > n2: {}", n1 > n2);           // true

} // end of local scope OR end of main()       
```

## Version with an HeapNode

* We may not want to apply the reversed comparaison to the Node because the need is only within the heap. 
* In a larger application if we compare Node we may want to do the comparaison the right way, not the reversed way
* We can create a ``HeapNode`` with a specific reversed compare which will be used only in the heap
* <span style="color:lime"><b>Preferred solution?</b></span> 



```rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

// HeapNode wraps a node and inverts the order for use in a min-heap
// #[derive(Eq, PartialEq)] // BinaryHeap => Eq, PartialEq, Ord, PartialOrd must be implemented
                            // Here, we CANNOT leverage Eq and PartialEq which compare all fields (link included)
struct HeapNode {
    val: i32,
    node: Link, // Using the alias (Option<Box<Node>>)
}

impl Eq for HeapNode {}

// Eq implies PartialEq, so we must implement PartialEq if we implement Eq
impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap behavior
        other.val.cmp(&self.val)
    }
}

// Ord implies PartialOrd, so we must implement PartialOrd if we implement Ord
// Implement PartialOrd to allow comparison using <, >, etc.
impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse order for min-heap behavior
        Some(other.val.cmp(&self.val))
    }
}

fn combined_sorted_linked_lists(mut lists: Vec<Link>) -> Link {
    let mut heap = BinaryHeap::new();

    // Initialize heap with head of each list
    for list in lists.iter_mut() {
        if let Some(node) = list.take() {
            heap.push(HeapNode {
                val: node.val,
                node: Some(node),
            });
        }
    }

    // Dummy node to start building the result
    let mut dummy = Box::new(Node::new(-1, None));
    let mut tail = &mut dummy;

    // While the heap is not empty, extract the smallest node
    while let Some(mut heap_node) = heap.pop() {
        if let Some(mut node) = heap_node.node.take() {
            // Append node to result list
            tail.next = Some(Box::new(Node::new(node.val, None)));
            tail = tail.next.as_mut().unwrap();

            // Push next node into heap if it exists
            if let Some(next_node) = node.next.take() {
                heap.push(HeapNode {
                    val: next_node.val,
                    node: Some(next_node),
                });
            }
        }
    }
    dummy.next
}

fn main(){     // no main() if this code runs in a Jupyter cell 

    fn create_list(vals : &[i32])->Link{
        let mut head = None;
        // for v in vals.into_iter().rev() {    // with .into_iter() vals would be lost
        for v in vals.iter().rev() {            // .iter() returns &T
                head = Some(Box::new(Node::new(*v, head)));
            }
        head
    }

    let lists = vec![
        create_list(&[1, 6]),
        create_list(&[1, 4, 6]),
        create_list(&[3, 7])
    ];

    let head_combined = combined_sorted_linked_lists(lists);

    let mut current = head_combined.as_ref();
    while let Some(node) = current {
        print!("{}->", node.val);
        current = node.next.as_ref();
    }
    print!("EOL"); // End of List   1->1->3->4->6->6->7->EOL
} // end of local scope OR end of main()       
```
