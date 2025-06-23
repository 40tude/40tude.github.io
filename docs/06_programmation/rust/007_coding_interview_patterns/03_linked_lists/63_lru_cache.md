---
# published: false
layout: default
title: "p063 - LRU Cache"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# LRU Cache

* Design and implement a data structure for Least Recently Used (LRU) cache.
* It should support the following operations:
    * LRUCache(capacity:int)
    * get (key:int)-> int
    * put (key:int, value:int) -> None



<span style="color:orange"><b>The point:</b></span>  
* Use a doubly linked list to keep track of the most recent used nodes AND hash map of size capacity to access the cache in O(1)

  
**Complexity Analysis :**

| Time | Space |
|------|-------|
| O(1) | O(n)  | 

* n is the capacity of the cache
* O(1) because put() and get() use _remove_node() and _add_to_tail() which are O(1)
* O(n) in space complexity because we store n nodes in the cache and doubly linked list








<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->







## Works but it uses a ``linked-hash-map``

**About Rust :**
* Work in JupyterLab or a standalone project
* <span style="color:red"><b>NOT YET </b></span> : tested on the [Rust Playground](https://play.rust-lang.org/). 
    * Indeed I don't know yet how to take into account `linked-hash-map`


<span style="color:red"><b>TODO : </b></span> 
* See how to depends on `linked-hash-map` in Rust Playground 




```rust

// # Cargo.toml
// [dependencies]
// linked-hash-map = "0.5"

// If in a Jupyter cell
:dep linked-hash-map = "0.5"


    
use linked_hash_map::LinkedHashMap;

struct LRUCache {
    capacity: usize,
    map: LinkedHashMap<i32, i32>,
}

impl LRUCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: LinkedHashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.map.remove(&key) {
            self.map.insert(key, value); // Re-insert to mark as recently used
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.map.remove(&key);
        } else if self.map.len() == self.capacity {
            self.map.pop_front(); // Remove least recently used
        }
        self.map.insert(key, value);
    }

    fn debug_print(&self) {
        print!("LRUCache: ");
        for (_k, v) in self.map.iter() {
            print!("{}->", v);
        }
        println!("EOC"); // Enf Of Cache
    }
}

// fn main() {
{
    let mut cache = LRUCache::new(3);
    cache.put(1, 100);
    cache.put(2, 250);
    println!("{}", cache.get(2));
    cache.put(4, 300);
    cache.put(3, 200);
    println!("{}", cache.get(4));
    println!("{}", cache.get(1));
    cache.debug_print();
}

```

    250
    300
    -1
    LRUCache: 250->200->300->END





    ()



## Hands made solution

In the code below we're looking to build a modifiable double-chained list, in which :
- Each Node can be moved within the list (linked to a previous and a next Node),
- And each Node can be accessed via the HashMap.

Pointers to Nodes must be shared AND mutable.

So the idea is:
* Store the Nodes elsewhere, and reference them via pointers
    * Store all `Box<Node>` in a `Vec<Box<Node>>`, so they live there and will never be moved.
    * Reference these Nodes from the chained list with a bare pointer: `NonNull<Node>`.


**Complexity Analysis :**

| Time | Space |
|------|-------|
| O(1) | O(n)  | 

* n is the capacity of the cache
* O(1) because put() and get() use _remove_node() and _add_to_tail() which are O(1)
* O(2n) in space complexity because we store n nodes in the cache and doubly linked list => O(n)


**About Rust :**
* `NonNull<T>` is a wrapper on `*mut T`, but it guarantees that the pointer is never null (`Option<NonNull<T>>` is therefore as compact as a simple `*mut T`).
* `NonNull<T>` can be manipulated without `unsafe`, as long as we store or compare it.
* To access the pointed data, we pass through `unsafe`, which is localized and controlled.
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/). 



```rust
use std::fmt; // Required for implementing Display
use std::collections::HashMap;
use std::ptr::NonNull;

type Link = Option<NonNull<DoublyLinkedListNode>>; // type alias. Use Option and NonNull to allow optional pointers to the next and previous nodes 

struct DoublyLinkedListNode {
    key: i32,
    val: i32,
    prev: Link,
    next: Link,
}

impl DoublyLinkedListNode {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

pub struct LRUCache {
    capacity: usize,
    hashmap: HashMap<i32, NonNull<DoublyLinkedListNode>>, // Hash map that maps keys to nodes
    head: NonNull<DoublyLinkedListNode>,
    tail: NonNull<DoublyLinkedListNode>,
    nodes: Vec<Box<DoublyLinkedListNode>>, // Owns all the nodes. Box<Node> are stored here to preserve ownership and prevent pointers from becoming dangling.
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        let mut nodes: Vec<Box<DoublyLinkedListNode>> = Vec::new();

        nodes.push(Box::new(DoublyLinkedListNode::new(-1, -1))); // head. Initialize the head and tail dummy nodes and connect them to each other in a basic two-node doubly linked list
        nodes.push(Box::new(DoublyLinkedListNode::new(-1, -1))); // tail

        let head_ptr = NonNull::new(nodes[0].as_mut() as *mut _).unwrap();
        let tail_ptr = NonNull::new(nodes[1].as_mut() as *mut _).unwrap();

        
        nodes[0].next = Some(tail_ptr); // no need for unsafe code here
        nodes[1].prev = Some(head_ptr);
        
        Self {
            capacity,
            hashmap: HashMap::new(),
            head: head_ptr,
            tail: tail_ptr,
            nodes,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&node_ptr) = self.hashmap.get(&key) {
            // Make this node the most recent used
            // Remove it from its current position and add it to the tail
            self._remove_node(node_ptr);
            self._add_to_tail(node_ptr);
            unsafe { node_ptr.as_ref().val }
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // If a node with this key exists, remove it form linked list (before to add it to tail)
        if let Some(&existing_ptr) = self.hashmap.get(&key) {
            self._remove_node(existing_ptr);
            self.hashmap.remove(&key);
        }

        let mut node = Box::new(DoublyLinkedListNode::new(key, value));
        let node_ptr = NonNull::new(node.as_mut() as *mut _).unwrap();
        self.nodes.push(node);
        self._add_to_tail(node_ptr);
        self.hashmap.insert(key, node_ptr);

        // Remove the least recently used node if we exceed the capacity
        if self.hashmap.len() > self.capacity {
            let lru_ptr = unsafe { self.head.as_ref().next.unwrap() };
            let lru_key = unsafe { lru_ptr.as_ref().key };
            self._remove_node(lru_ptr);
            self.hashmap.remove(&lru_key);
        }
    }

    fn _add_to_tail(&mut self, mut node_ptr: NonNull<DoublyLinkedListNode>) {
        unsafe {
            let mut tail_prev = self.tail.as_ref().prev.unwrap();

            node_ptr.as_mut().prev = Some(tail_prev);
            node_ptr.as_mut().next = Some(self.tail);

            tail_prev.as_mut().next = Some(node_ptr);
            self.tail.as_mut().prev = Some(node_ptr);
        }
    }

    fn _remove_node(&mut self, node_ptr: NonNull<DoublyLinkedListNode>) {
        unsafe {
            let mut prev = node_ptr.as_ref().prev.unwrap();
            let mut next = node_ptr.as_ref().next.unwrap();

            prev.as_mut().next = Some(next);
            next.as_mut().prev = Some(prev);
        }
    }
}

impl fmt::Display for LRUCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cur = unsafe { self.head.as_ref().next };
        // write!(f, "LRUCache: ")?; // Begin output
        while let Some(node_ptr) = cur {
            if node_ptr == self.tail {
                break;
            }
            let node = unsafe { node_ptr.as_ref() };
            write!(f, "{} -> ", node.val)?;
            cur = node.next;
        }
        write!(f, "EOC") // Enf Of Cache
    }
}

fn main(){     // no main() if this code runs in a Jupyter cell 
// {                 // local scope to avoid issue with the lifetime of head during borrow
    let mut cache = LRUCache::new(3);
    cache.put(1, 100);
    cache.put(2, 250);
    println!("{}", cache.get(2)); // 250
    cache.put(4, 300);
    cache.put(3, 200);
    println!("{}", cache.get(4)); // 300
    println!("{}", cache.get(1)); // -1
    println!("LRUCache: {}", cache); // LRUCache: 250 -> 200 -> 300 -> EOC
} // end of local scope OR end of main()       

```

    250
    300
    -1
    LRUCache: 250 -> 200 -> 300 -> EOC





    ()



### Shows why Box cannot be used

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/). 



```rust
type Link = Option<Box<DoublyLinkedListNode>>; 

struct DoublyLinkedListNode {
    val: i32,
    prev: Link,
    next: Link,
}

impl DoublyLinkedListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            prev: None,
            next: None,
        }
    }
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
{                 // local scope to avoid issue with the lifetime of head during borrow

    let mut a = Box::new(DoublyLinkedListNode::new(1));
    let mut b = Box::new(DoublyLinkedListNode::new(2));
    
    a.next = Some(b); // b is moved
    b.prev = Some(a); // <- b is already moved
} // end of local scope OR end of main()       

```
