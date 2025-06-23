---
# published: false
layout: default
title: "p239 - Find All Words on a Board"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Find All Words on a Board

* Given a 2D board of chars and an array of words
* Find all the words
* Adjacent cells = hor or vert
* A cell not more than one time
* Only English char

<span style="color:orange"><b>The point:</b></span>

* Backtracking (see p293) to keep track of visited cells
* Use the ``word`` attribute instead of ``is_word`` and to know which word has ended
* DFS
* Set the `word` attribute to None to so we cannot record the same word



**Complexity :**

| Time                   | Space      |
|------------------------|------------|
| O(N x L + m x n x 3^L) | O(N x L)   |

Where :
* N = # of words
* L = length of longest word
* m x n = size of the board

Reasons :
* O(N x L + m x n x 3^L) in time because
    * Insert N words of max len L in O(NxL)
    * Search over mxn cells with DFS call. Each of them is in O(3^L) because there are 3 recursive calls per cell. One per adjacent cell (excluding the one we come from). This is repeated L times (len of longest word)
    * O(Nxl) + m x n O(3^L) = O(N x L + m x n x 3^L)
* O(N x L) in space because N words of maximum length L plus size of recursive stack L : O(N x l) + O(L) = O(N x L)










<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

A translation of the code available in the book.

**About Rust :**
* In TrieNode, use `String` rather `&str` avoid lifetime issues at compile time
* Check `node = node.children.entry(c).or_insert_with(TrieNode::new);` in `find_all_words_on_a_board()`
* ``.get_mut()`` in `if let Some(node) = root.children.get_mut(&ch) {` because we need a pass a ``node`` as mutable reference to ``dfs()``
* Look for "PYTHON" and double check how the line `if is_within_bounds(next_r, next_c, board) && board[next_r][next_c] in node.children{` is converted
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    word : String, // using String rather &str avoid lifetime issues at compile time
}

impl TrieNode{
    fn new() -> Self {
        TrieNode{
            children : HashMap::new(),
            word : "".to_string(),
        }
    }
}

fn is_within_bounds(r : i32, c : i32, board : &[Vec<char>]) -> bool {
    let r_ok = r >= 0 && r < board.len() as i32;
    let c_ok = c >= 0 && c < board[0].len() as i32;
    r_ok && c_ok
}

fn dfs(board: &mut Vec<Vec<char>>, r : i32, c : i32, node : &mut TrieNode, res : &mut Vec<String>){
    // If the current node is the end of a word, add the word to res
    if !node.word.is_empty(){
        res.push(node.word.to_string());
        node.word = "".to_string();
    }
    let temp = board[r as usize][c as usize];
    // Mark the current cell as visited
    board[r as usize][c as usize] = '#';

    // Explore adjacents cells that correspond with a child of current TrieNode
    let dirs : Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for d in dirs{
        let next_r = r + d.0;
        let next_c = c + d.1;
        // PYTHON : if is_within_bounds(next_r, next_c, board) && board[next_r][next_c] in node.children{
        if is_within_bounds(next_r, next_c, board) {
            let ch = board[next_r as usize][next_c as usize];
            if let Some(next_node) = node.children.get_mut(&ch) {
                dfs(board, next_r, next_c, next_node, res);
            }
        }
    }
    // backtrack
    board[r as usize][c as usize] = temp;
}

fn find_all_words_on_a_board(board: &[Vec<char>], words: &Vec<&str>) -> Vec<String> {
    let mut root = TrieNode::new();
    
    // Insert every word into the trie
    for word in words{
        let mut node = &mut root;
        for c in word.chars(){
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.word = word.to_string();
    }

    let mut res = Vec::new();
    let mut board_mut = board.to_owned();
    // Start de DFS call from each cell of the board that contains a
    // child of the root node, which represents the first letter of a word in the trie
    for r in 0..board.len(){
        for c in 0..board[0].len(){
            // PYTHON if board[r][c] in root.children {
            let ch = board[r][c];
            if let Some(node) = root.children.get_mut(&ch) {
                dfs(&mut board_mut, r as i32, c as i32, node, &mut res);
            }
             
        }
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let board = vec![vec!['b', 'y', 's'], vec!['r', 't', 'e'], vec!['a', 'i', 'n']];
    let words = vec!["byte", "bytes", "rat", "rain", "trait", "train"];
    let words_out = find_all_words_on_a_board(&board, &words);
    for w in words_out{
        println!("{} ", w)
    }
} // end of local scope OR end of main()
```

## V2

**About Rust :**
* In the TrieNode `word` is now `Option<String>` (allow to use None rather than "")
* less call to ``.to_string()``
* `r` and `c` are ``usize`` since we work in the range ``[0, board.len())`` 
* `is_within_bounds()` deleted
* `board` is still cloned
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>, // None if not a word
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            word: None,
        }
    }
}

static DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, node: &mut TrieNode, res: &mut Vec<String>) {
    if let Some(w) = &node.word {
        res.push(w.clone());
        node.word = None; // Avoid duplicates
    }
    let temp = board[r][c];
    board[r][c] = '#'; // mark as visited

    for &(dr, dc) in &DIRS {
        let next_r = r as isize + dr;
        let next_c = c as isize + dc;
        // PYTHON : if is_within_bounds(next_r, next_c, board) && board[next_r][next_c] in node.children{
        if next_r >= 0 && next_c >= 0 && (next_r as usize) < board.len() && (next_c as usize) < board[0].len(){
            let ch = board[next_r as usize][next_c as usize];
            if let Some(next_node) = node.children.get_mut(&ch) {
                dfs(board, next_r as usize, next_c as usize, next_node, res);
            }
        }
    }

    board[r][c] = temp; // backtrack
}

fn find_all_words_on_a_board(board: &[Vec<char>], words: &[&str]) -> Vec<String> {
    let mut root = TrieNode::new();

    // Insert every word into the trie
    for &word in words {
        let mut node = &mut root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.word = Some(word.to_string());
    }

    let mut res = Vec::new();
    let mut board_mut = board.to_vec(); // clone board for mutability

    // Start de DFS call from each cell of the board that contains a
    // child of the root node, which represents the first letter of a word in the trie
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            let ch = board[r][c];
            if let Some(node) = root.children.get_mut(&ch) {
                dfs(&mut board_mut, r, c, node, &mut res);
            }
        }
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let board = vec![
        vec!['b', 'y', 's'],
        vec!['r', 't', 'e'],
        vec!['a', 'i', 'n'],
    ];
    let words = vec!["byte", "bytes", "rat", "rain", "trait", "train"];
    let words_out = find_all_words_on_a_board(&board, &words);
    for w in words_out {
        println!("{}", w);
    }
} // end of local scope OR end of main()

```

## V3

**About Rust :**
* `board` is no longer cloned (and so it is mutable)
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>, // None if not a word
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            word: None,
        }
    }
}

static DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, node: &mut TrieNode, res: &mut Vec<String>) {
    // If the current node is the end of a word, add the word to res
    if let Some(w) = node.word.take() { // take() moves the String out, replaces with None (avoid duplicates)
        res.push(w);
    }
    let temp = board[r][c];
    board[r][c] = '#'; // mark as visited

    // Explore adjacents cells that correspond with a child of current TrieNode
    for &(dr, dc) in &DIRS {
        let next_r = r as isize + dr;
        let next_c = c as isize + dc;
        // PYTHON : if is_within_bounds(next_r, next_c, board) && board[next_r][next_c] in node.children{
        if next_r >= 0 && next_c >= 0 && (next_r as usize) < board.len() && (next_c as usize) < board[0].len() {
            let ch = board[next_r as usize][next_c as usize];
            if let Some(next_node) = node.children.get_mut(&ch) {
                dfs(board, next_r as usize, next_c as usize, next_node, res);
            }
        }
    }

    board[r][c] = temp; // backtrack
}

fn find_all_words_on_a_board(board: &mut Vec<Vec<char>>, words: &[&str]) -> Vec<String> {
    let mut root = TrieNode::new();

    // Insert every word into the trie
    for &word in words {
        let mut node = &mut root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.word = Some(word.to_string());
    }

    let mut res = Vec::new();

    // Start de DFS call from each cell of the board that contains a
    // child of the root node, which represents the first letter of a word in the trie
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            let ch = board[r][c];
            if let Some(node) = root.children.get_mut(&ch) {
                dfs(board, r, c, node, &mut res);
            }
        }
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let mut board = vec![
        vec!['b', 'y', 's'],
        vec!['r', 't', 'e'],
        vec!['a', 'i', 'n'],
    ];
    let words = vec!["byte", "bytes", "rat", "rain", "trait", "train"];
    let words_out = find_all_words_on_a_board(&mut board, &words);
    for w in words_out {
        println!("{}", w);
    }
} // end of local scope OR end of main()

```
