---
# published: false
layout: default
lang: en-US
title: "p170 - Largest Overlap of Interval"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Largest Overlap of Interval

<div align="center">
<img src="../assets/chap_09.webp" alt="" width="300" loading="lazy"/>
</div>

* Given an array of intervals
* Determine the **maximum number** of intervals that overlap
* Overlaps are half open : ``[..[``

<span style="color:orange"><b>The point:</b></span>

* If we encounter a start point increment `active_interval`
* If we encounter a end point decrement `active_interval`



**Complexity :**

| Time               | Space |
|--------------------|-------|
| O(n log(n))        | O(n)  |

* O(n+m) because we sort the ``points`` array of size 2n befor iterating over it in O(n)
* O(1) because space taken by ``points`` array 

**About Rust :**
* `points.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));`
* `active_intervals += if point_type == 'S' { 1 } else { -1 };`
* `enum PointType { Start, End }`
```rust
active_intervals += match point_type {
    PointType::Start => 1,
    PointType::End => -1,
};
```
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)

<!-- 
<span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->

<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->


## V1


```rust
struct Interval {
    start : usize,
    end : usize
}

impl Interval {
    fn new(start:usize, end:usize) -> Self {
        Self {start, end}
    }
}

fn largest_overlap_of_intervals(intervals:&[Interval]) -> usize{
    
    let mut points = Vec::new();
    
    for interval in intervals{
        points.push((interval.start, 'S')); // we push tuples into the vector
        points.push((interval.end, 'E'));
    }

    // Sort in positional order & prioritize end point before start point
    points.sort_by(|a, b| {
        let cmp_i32 = a.0.cmp(&b.0); // first compare i32
        if cmp_i32 == std::cmp::Ordering::Equal { // if equal, compare chars
            a.1.cmp(&b.1)
        } else {
            cmp_i32
        }
    });

    let (mut active_intervals, mut max_overlaps) = (0, 0);

    for (_, point_type) in points{
        if point_type=='S' {
            active_intervals +=1;
        }else{
            active_intervals -=1;
        }
        max_overlaps = max_overlaps.max(active_intervals)
    }
    max_overlaps
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    let intervals = vec![
        Interval::new(1, 3),
        Interval::new(2, 6),
        Interval::new(4, 8),
        Interval::new(6, 7),
        Interval::new(5, 7),
    ];

    println!("{:?}", largest_overlap_of_intervals(&intervals)); // 3 
} // end of local scope OR end of main()       
```

## V2
* Simplify how sort is coded with `sort_unstable_by`
* In the `for` loop, use the fact that the ``if`` is an expression 


```rust
struct Interval {
    start : usize,
    end : usize
}

impl Interval {
    fn new(start:usize, end:usize) -> Self {
        Self {start, end}
    }
}

fn largest_overlap_of_intervals(intervals:&[Interval]) -> usize{
    
    let mut points = Vec::with_capacity(intervals.len() * 2);
    
    for interval in intervals{
        points.push((interval.start, 'S')); // we push tuples into the vector
        points.push((interval.end, 'E'));
    }

    // Sort in positional order & prioritize end point before start point
    points.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let (mut active_intervals, mut max_overlaps):(i32, usize) = (0, 0);

    for (_, point_type) in points{
        active_intervals += if point_type == 'S' { 1 } else { -1 };
        max_overlaps = max_overlaps.max(active_intervals as usize)
    }
    max_overlaps
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    let intervals = vec![
        Interval::new(1, 3),
        Interval::new(2, 6),
        Interval::new(4, 8),
        Interval::new(6, 7),
        Interval::new(5, 7),
    ];

    println!("{:?}", largest_overlap_of_intervals(&intervals)); // 3 
} // end of local scope OR end of main()       
```

## V3
* Rewrite the ``for`` loop
* I want to get ride of the `_` case in the `match` expression


```rust
struct Interval {
    start : usize,
    end : usize
}

impl Interval {
    fn new(start:usize, end:usize) -> Self {
        Self {start, end}
    }
}

fn largest_overlap_of_intervals(intervals:&[Interval]) -> usize{
    
    let mut points = Vec::with_capacity(intervals.len() * 2);
    
    for interval in intervals{
        points.push((interval.start, 'S')); // we push tuples into the vector
        points.push((interval.end, 'E'));
    }

    // Sort in positional order & prioritize end point before start point
    points.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let (mut active_intervals, mut max_overlaps) = (0, 0);

    for (_, point_type) in points {
        active_intervals += match point_type {
            'S' => 1,
            'E' => -1,
            _ => 0, // should never happen
        };
        max_overlaps = max_overlaps.max(active_intervals as usize);
    }
    max_overlaps
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    let intervals = vec![
        Interval::new(1, 3),
        Interval::new(2, 6),
        Interval::new(4, 8),
        Interval::new(6, 7),
        Interval::new(5, 7),
    ];

    println!("{:?}", largest_overlap_of_intervals(&intervals)); // 3 
} // end of local scope OR end of main()       
```

## V4.1

* Replace ``S`` and ``E`` with custom type (enum based) 
    * To get ride of the `_` case in the ``match`` expression

This code <span style="color:red"><b>DOES NOT WORK</b></span>
* The compiler complains because it can't find a ``.cmp()`` method for ``PointType``
* For ``PointType`` 
    * We can leverage default ``PartialEq``, ``Eq``
    * We must implement `Ord` and `PartialOrd` 


```rust
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "start must be <= end");
        Self { start, end }
    }
}

enum PointType { Start, End }


fn largest_overlap_of_intervals(intervals: &[Interval]) -> usize {
    let mut points = Vec::with_capacity(intervals.len() * 2);
    
    for interval in intervals {
        points.push((interval.start, PointType::Start));
        points.push((interval.end, PointType::End));
    }

    // Sort in positional order & prioritize end point before start point
    points.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let (mut active_intervals, mut max_overlaps) = (0, 0);
    for (_, point_type) in points {
        active_intervals += match point_type {
            PointType::Start => 1,
            PointType::End => -1,
        };
        max_overlaps = max_overlaps.max(active_intervals as usize);
    }
    max_overlaps
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    let intervals = vec![
        Interval::new(1, 3),
        Interval::new(2, 6),
        Interval::new(4, 8),
        Interval::new(6, 7),
        Interval::new(5, 7),
    ];

    println!("{:?}", largest_overlap_of_intervals(&intervals)); // 3 
} // end of local scope OR end of main()       
```

## V4.2

* <span style="color:lime"><b>Preferred solution?</b></span> 


```rust
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "start must be <= end");
        Self { start, end }
    }
}

#[derive(PartialEq, Eq)]
enum PointType { Start, End }

// Manual ordering such that End < Start
impl Ord for PointType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PointType::End, PointType::Start) => std::cmp::Ordering::Less,
            (PointType::Start, PointType::End) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for PointType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn largest_overlap_of_intervals(intervals: &[Interval]) -> usize {
    let mut points = Vec::with_capacity(intervals.len() * 2);
    
    for interval in intervals {
        points.push((interval.start, PointType::Start));
        points.push((interval.end, PointType::End));
    }

    // Sort in positional order & prioritize end point before start point
    points.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let (mut active_intervals, mut max_overlaps) = (0, 0);
    for (_, point_type) in points {
        active_intervals += match point_type {
            PointType::Start => 1,
            PointType::End => -1,
        };
        max_overlaps = max_overlaps.max(active_intervals as usize);
    }
    max_overlaps
}

fn main(){   // no main() if this code runs in a Jupyter cell 
    let intervals = vec![
        Interval::new(1, 3),
        Interval::new(2, 6),
        Interval::new(4, 8),
        Interval::new(6, 7),
        Interval::new(5, 7),
    ];

    println!("{:?}", largest_overlap_of_intervals(&intervals)); // 3 
} // end of local scope OR end of main()       
```
