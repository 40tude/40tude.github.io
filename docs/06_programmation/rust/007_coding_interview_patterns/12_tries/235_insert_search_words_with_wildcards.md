---
# published: false
layout: default
title: "p235 - Insert and Search Words with Wildcards"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Insert and Search Words with Wildcards

* Design and  implement a trie data structure
    * `insert(word: &str) -> None`
    * `search(word: &str) -> bool` : ``word`` may contain wildcard `.` (any letter)
    * Only English char

<span style="color:orange"><b>The point:</b></span>

* Insert : All words inserted are child of the root. Find if there is a prefix, create a branch if needed
* Search : returns ``true`` if the final node is tagged `is_word`

**Complexity :**

| Method      | Time         | Space      |
| ------------|--------------|------------|
| `insert()`  | O(k)         | O(k)       |
| `search()`  | O(k) O(26^k) | O(1) O(k)  |

**Time:**
* O(k) for ``insert()`` because we need to visit up to ``k`` nodes
* O(k) for ``search()`` with no wildcard because search through up to ``k`` chars in the trie
* O(26^k) for ``search()`` with only wildcards. Need to check 26 chars for each.

**Space:**
* O(k) for ``insert()`` because in worst case the word does'nt share a prefix (k nodes created)
* O(1) for ``search()`` with no wildcard because no additional space required
* O(k) for ``search()`` with only wildcard because the `k` entries in the recursive call stack








<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

**About Rust :**
* Check `node = node.children.entry(c).or_insert_with(TrieNode::new);` in `.insert()`
* ``fn search_helper(&self, word_index : usize, word : &str, mut node : &TrieNode) -> bool``
    * The ``&TrieNode`` reference is immutable. We can't modify the TrieNode.
    * ``node`` is mutable so that we can reassign ``node`` to point to another reference (see node = next_node;)
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word : bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode{
            children : HashMap::new(),
            is_word : false,
        }
    }
}

struct InsertAndSearchWordsWithWildcards {
    root : TrieNode
}

impl InsertAndSearchWordsWithWildcards {
    fn new() -> Self {
        InsertAndSearchWordsWithWildcards{
            root : TrieNode::new()
        }
    }

    fn insert(&mut self, word : &str) {
        let mut node = &mut self.root;
        // for each char, if not a child of current node, create a new TrieNode for the char
        for c in word.chars(){
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_word=true;
    }

    fn search(&self, word : &str) -> bool{
        // No need for a &mut self.root
        // Indeed we don't want to see its content to be modified
        self.search_helper(0, word, &self.root)
    }

    // The &TrieNode reference is immutable. We can't modify the TrieNode.
    // The variable node is mutable so that we can reassign node to another reference 
    // See node = next_node;
    fn search_helper(&self, word_index : usize, word : &str, mut node : &TrieNode) -> bool{
        for i in word_index.. word.len(){
            match word.chars().nth(i){
                Some('.') => {
                    for child in node.children.values(){
                        if self.search_helper(i+1, word, child){
                            return true;
                        }
                    }
                    return false;
                }
                None => return false,
                Some(c) => {
                    // node = node.children.get(&c).unwrap(); // .unwrap() should never happen here
                    match node.children.get(&c) {
                        Some(next_node) => {
                            node = next_node; // rebind node to next_node
                        }
                        None => return false, // if not found, word doesn't exist
                    }
                }
            }
        } 
        node.is_word
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let mut my_trie = InsertAndSearchWordsWithWildcards::new();
    my_trie.insert("band");
    my_trie.insert("rat");
    println!("Search 'ra.': {}", my_trie.search("ra."));    // Search 'ra.': true
    println!("Search 'b..': {}", my_trie.search("b.."));    // Search 'b..': false
    my_trie.insert("ran");
    println!("Search '.an': {}", my_trie.search(".an"));    // Search '.an' true
} // end of local scope OR end of main()
```

## V2

**About Rust :**
* Avoid the ``.nth(i)`` which inspect ``i`` chars each time (=> O(n²) )
* Better 
    * to create a vector of chars in `search()`, 
    * modify search_helper() signature 
        * Now ``fn search_helper(&self, word_index: usize, chars: &[char], mut node: &TrieNode) -> bool``
        * Before ``fn search_helper(&self, word_index : usize, word : &str, mut node : &TrieNode) -> bool``
    * Access the char in O(1) in `search_helper()`
    * Simplify the ``match`` expression in `search_helper()`
* In Rust
```rust
if condition {
    return true;
}
return false;
```    
Can be replaced by
```rust
condition
```
So 
```rust
for child in node.children.values() {
    if self.search_helper(i + 1, chars, child) {
        return true;
    }
}
false

```
Can be replaced by :
```rust
node.children.values().any(|child| self.search_helper(i + 1, chars, child))
```
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word : bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode{
            children : HashMap::new(),
            is_word : false,
        }
    }
}

struct InsertAndSearchWordsWithWildcards {
    root : TrieNode
}

impl InsertAndSearchWordsWithWildcards {
    fn new() -> Self {
        InsertAndSearchWordsWithWildcards{
            root : TrieNode::new()
        }
    }

    fn insert(&mut self, word : &str) {
        let mut node = &mut self.root;
        // for each char, if not a child of current node, create a new TrieNode for the char
        for c in word.chars(){
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_word=true;
    }

    // fn search(&self, word : &str) -> bool{
    //     self.search_helper(0, word, &self.root)
    // }

    fn search(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        // No need for a &mut self.root
        // Indeed we don't want to see its content to be modified
        self.search_helper(0, &chars, &self.root)
    }

    // The &TrieNode reference is immutable. We can't modify the TrieNode.
    // The variable node is mutable so that we can reassign node to another reference 
    // See node = next_node;
    fn search_helper(&self, word_index: usize, chars: &[char], mut node: &TrieNode) -> bool{
        // for i in word_index.. word.len(){
            // match word.chars().nth(i){
        for i in word_index..chars.len() {
            match chars[i] {
                // Some('.') => {
                '.' => {
                    // for child in node.children.values(){
                    //     if self.search_helper(i+1, word, child){
                    //         return true;
                    //     }
                    // }
                    // return false;
                    return node.children.values().any(|child| self.search_helper(i + 1, chars, child))
                }
                // None => return false,
                //Some(c) => {
                c => {
                    // node = node.children.get(&c).unwrap(); // .unwrap() should never happen here
                    match node.children.get(&c) {
                        Some(next_node) => {
                            node = next_node; // rebind node to next_node
                        }
                        None => return false, // if not found, word doesn't exist
                    }
                }
            }
        } 
        node.is_word
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let mut my_trie = InsertAndSearchWordsWithWildcards::new();
    my_trie.insert("band");
    my_trie.insert("rat");
    println!("Search 'ra.': {}", my_trie.search("ra."));    // Search 'ra.': true
    println!("Search 'b..': {}", my_trie.search("b.."));    // Search 'b..': false
    my_trie.insert("ran");
    println!("Search '.an': {}", my_trie.search(".an"));    // Search '.an' true
} // end of local scope OR end of main()
```

## V3

**About Rust :**
* `search_helper` with no explicit return
* <span style="color:lime"><b>Preferred solution?</b></span> 
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word : bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode{
            children : HashMap::new(),
            is_word : false,
        }
    }
}

struct InsertAndSearchWordsWithWildcards {
    root : TrieNode
}

impl InsertAndSearchWordsWithWildcards {
    fn new() -> Self {
        InsertAndSearchWordsWithWildcards{
            root : TrieNode::new()
        }
    }

    fn insert(&mut self, word : &str) {
        let mut node = &mut self.root;
        for c in word.chars(){
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_word=true;
    }

    fn search(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        self.search_helper(0, &chars, &self.root)
    }

    fn search_helper(&self, word_index: usize, chars: &[char], mut node: &TrieNode) -> bool {
        for i in word_index..chars.len() {
            match chars[i] {
                '.' => return node.children.values().any(|child| self.search_helper(i + 1, chars, child)),
                c => match node.children.get(&c) {
                    Some(next_node) => node = next_node,
                    None => return false,
                },
            }
        }
        node.is_word
    }
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let mut my_trie = InsertAndSearchWordsWithWildcards::new();
    my_trie.insert("band");
    my_trie.insert("rat");
    println!("Search 'ra.': {}", my_trie.search("ra."));    // Search 'ra.': true
    println!("Search 'b..': {}", my_trie.search("b.."));    // Search 'b..': false
    my_trie.insert("ran");
    println!("Search '.an': {}", my_trie.search(".an"));    // Search '.an' true
} // end of local scope OR end of main()
```
