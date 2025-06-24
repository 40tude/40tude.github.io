---
# published: false
layout: default
title: "p280 - Merging Communities"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Merging Communities

* `n` people from 0 to `n-1`
* Initially each person belong to a separated community
* When two people connect their communities merge
* Write
    * `connect(x: usize, y: usize)` to connect person `x` to person `y` and merges their communities
    * `get_communitiy_size(x: usize) -> usize` returns `x`'s community size

<span style="color:orange"><b>The point:</b></span>

* Union-Find data structure
* Aka Disjoint Set Union (DSU)
* Communities are graphs where each member points to its representative (parent)
* Parents in a parent array. `parent[i]` is the parent of person `i`
* Union : connect x to y => connect rep of y to rep of x
    * representative (root) of x can be different of parent of x
    * size optimization : the rep with the larger community becomes the new rep
    * path compression optimization : we make sure each person is directly connected to the new rep (p 284)
* Find : need to traverse x's parent chain until representative (person whose parent is themselves)


**Complexity :**

| Time        | Space        |
|-------------|--------------|
| O(1)        | O(n)         |

* O(1) in time because 
    * `find()` with size and path compression optimization branches are very short. It is constant-time in most cases 
    * `get_size()` uses `find()` so it is amortized O(1)
* O(n) in space because the Union-Find data structure has 2 array of size  `n`. The space for the recursive call stack is amortized O(1) since the branches are very short 









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## First implementation

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


```rust
struct UnionFind{
    size : Vec<usize> ,
    parent : Vec<usize> 
}

impl UnionFind{
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    fn union(&mut self, x: usize, y: usize){
        let (rep_x, rep_y) = (self.find(x), self.find(y));
        if rep_x != rep_y{
            if self.size[rep_x] > self.size[rep_y]{
                // If rep_x represent a larger community, connect rep_y community to it
                self.parent[rep_y] = rep_x;
                self.size[rep_x] += self.size[rep_y] 
            }else{
                // connect rep_x to rep_y otherwise
                self.parent[rep_x] = rep_y;
                self.size[rep_y] += self.size[rep_x] 
            }
        }
    }

    fn find(&mut self, x: usize)->usize{ // returns the representative of x
        if x==self.parent[x]{
            return x;
        }
        // path compression
        self.parent[x] = self.find(self.parent[x]);
        self.parent[x]
    }

    fn get_size(&mut self, x: usize) -> usize{ // in .find(), self can mutate
        let rep_x = self.find(x);
        self.size[rep_x]
    }
}

struct MergingCommunities{
    uf : UnionFind
}

impl MergingCommunities{
    fn new(size: usize)->Self{
        Self{
            uf: UnionFind::new(size)
        }
    }

    fn connect(&mut self, x: usize, y: usize){
        self.uf.union(x, y);
    }

    fn get_community_size(&mut self, x: usize)->usize{
        self.uf.get_size(x)
    }
}

fn main() {
    
    let n = 5;
    let mut my_communities = MergingCommunities::new(n);

    my_communities.connect(0, 1);
    my_communities.connect(1, 2);
    println!("{}", my_communities.get_community_size(3)); // 1   
    println!("{}", my_communities.get_community_size(0)); // 3    
    my_communities.connect(3, 4);
    println!("{}", my_communities.get_community_size(4)); // 2
}
```
