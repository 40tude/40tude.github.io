---
# published: false
layout: default
lang: en-US
title: "bonus067 - Implement a Queue Using Stacks"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Implement a Queue Using Stacks

<div align="center">
<img src="../assets/chap_07.webp" alt="" width="300" loading="lazy"/>
</div>

* Implement a queue using the stack data structure. Include the following functions:
    * ``enqueue(x: i32)  : ``adds the value of ``x`` to the **end** of the queue.
    * ``dequeue() -> i32 : ``removes and returns the element from the **front** of the queue.
    * ``peek() -> i32    : ``returns the value (without removing the element) at the **front** of the queue.


<span style="color:orange"><b>The point:</b></span>

* A stack to push values onto during each enqueue call (enqueue_stack).
* A stack to pop values from during each dequeue call (dequeue_stack).

**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(n)  |

* O(n) because we travers the string and join up to n chars at the end. push/pop contributes 0(1) time
* O(n) because the stack can store up to ``n`` chars 

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



```rust
#[derive(Debug)]
struct Queue{
    enqueue_stack : Vec<i32>,
    dequeue_stack : Vec<i32>,
}

impl Queue{
    fn new() -> Self {
        Self {
            enqueue_stack : Vec::<i32>::new(),
            dequeue_stack : Vec::<i32>::new(),
        }
    }

    fn enqueue(&mut self, x : i32){
        self.enqueue_stack.push(x);
    }

    fn transfer_enqueue_to_dequeue(&mut self) {
        // If the dequeue stack is empty, push all elements from the enqueue stack
        // onto the dequeue stack. This ensures the top of the dequeue stack
        // contains the least recently added value.
        if self.dequeue_stack.is_empty(){
            // while !self.enqueue_stack.is_empty(){
            //     self.dequeue_stack.push(self.enqueue_stack.pop().unwrap());
            // }
            
            // Same as above. Avoid unwrap() prefer while let. 
            // Make a pop and we check if the result is Some value
            while let Some(element) = self.enqueue_stack.pop(){
                self.dequeue_stack.push(element);
            }
        }
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.transfer_enqueue_to_dequeue();
        // Pop and return the value at the top of the dequeue stack.
        // self.dequeue_stack.pop() if self.dequeue_stack else None
        self.dequeue_stack.pop()
    }

    fn peek(&mut self) -> Option<&i32> { // returns an option to a reference
        self.transfer_enqueue_to_dequeue();
        // self.dequeue_stack[-1] if self.dequeue_stack else None
        self.dequeue_stack.last()             // .last() returns Option<&T>
    }
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    let mut my_queue = Queue::new();
    
    my_queue.enqueue(1);
    my_queue.enqueue(2);
    my_queue.dequeue();
    my_queue.enqueue(3);
    println!("{:?}", my_queue.peek()); // Some(2)
    println!("{:?}", my_queue);        // Queue { enqueue_stack: [3], dequeue_stack: [2] }
} // end of local scope OR end of main()       

```

## More generic

* <span style="color:lime"><b>Preferred solution?</b></span> 


```rust
#[derive(Debug)]
struct Queue<T> {
    enqueue_stack: Vec<T>,
    dequeue_stack: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            enqueue_stack: Vec::new(),
            dequeue_stack: Vec::new(),
        }
    }

    fn enqueue(&mut self, x: T) {
        self.enqueue_stack.push(x);
    }

    fn transfer_enqueue_to_dequeue(&mut self) {
        if self.dequeue_stack.is_empty() {
            while let Some(val) = self.enqueue_stack.pop() {
                self.dequeue_stack.push(val);
            }
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        self.transfer_enqueue_to_dequeue();
        self.dequeue_stack.pop()
    }

    fn peek(&mut self) -> Option<&T> {
        self.transfer_enqueue_to_dequeue();
        self.dequeue_stack.last()
    }
}

fn main(){     // no main() if this code runs in a Jupyter cell 
    let mut my_queue = Queue::new();
    
    my_queue.enqueue(1.0);
    my_queue.enqueue(2.0);
    my_queue.dequeue();
    my_queue.enqueue(3.0);
    println!("{:?}", my_queue.peek()); // Some(2.0)
    println!("{:?}", my_queue); // Queue { enqueue_stack: [3.0], dequeue_stack: [2.0] }
} // end of local scope OR end of main()       


```
