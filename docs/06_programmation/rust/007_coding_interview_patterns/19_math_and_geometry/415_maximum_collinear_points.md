---
# published: false
layout: default
title: "p415 - Maximum Collinear Points"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Maximum Collinear Points

* Given a set of 2D points, determine the max number of aligned points 
* No duplicate points

<span style="color:orange"><b>The point:</b></span>

* Edge cases : vertical line, slope infinite
* Precision issues solve with slope = rise/run = (rise, run)
* Use GCD to make sure both slopes (1,2) = (13,26) are equal 


**Complexity :**

| Time         | Space      |
|--------------|------------|
| O(n² log(m)) | O(n)       |

* n² log(m) in time because 
    * GCD is in O(log(min(rise,sun))) approx. O(log(m)) in worst case
    * ``max_points_from_focal_point()`` calls ``gcd()`` n-1 times => O(nlog(m))
    * ``max_points_from_focal_point()`` is called n times => O(n² log(m))
* O(n) in space because of the hash map stores n-1 key-value pairs (focal point is excluded)  





**About Rust :**

* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->




```rust
use std::collections::HashMap;

fn gcd(mut a:i32, mut b:i32) -> i32{
    while b !=0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn get_slope(p1 : (i32, i32), p2 : (i32, i32))->(i32, i32){
    let rise = p2.1 - p1.1;
    let run = p2.0 - p1.0;
    if run == 0 {
        return (1, 0);
    }
    // simplify the slope to its reduced form
    let gcd_val = gcd(rise, run);
    (rise/gcd_val, run/gcd_val)
}

fn max_points_from_focal_point(focal_point_index : i32, points: &Vec<(i32, i32)>) -> i32{
    let mut slopes_map: HashMap<(i32, i32), i32> = HashMap::new(); // explicit type annotation
    let mut max_points = 0;
    // for current focal point, calculate the slope between it and the others point
    // We group points that share the same slope
    for j in 0..points.len() as i32{
        if j != focal_point_index{
            let curr_slope = get_slope(points[focal_point_index as usize], points[j as usize]);
            let curr_count = slopes_map.get(&curr_slope).unwrap_or(&0) + 1;
            slopes_map.insert(curr_slope, curr_count);
            // update the max count of colinear points for current focal point
            let curr_max = slopes_map.get(&curr_slope).unwrap_or(&0);
            max_points = max_points.max(*curr_max);
        }
    }
    // Add 1 to max count to include the focal point
    max_points+1
}

fn maximum_colinear_points(points : &Vec<(i32, i32)>) -> i32{
    let mut res = 0;
    // treat each point as a focal point
    // determine the max number of point colinear with each focal point
    // the largest is the answer
    for i in 0..points.len() as i32{
        res = res.max(max_points_from_focal_point(i, points));
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let points = vec![
        (1, 1),
        (1, 3),
        (2, 2),
        (3, 1),
        (3, 3),
        (4, 4),
    ];
    println!("{}", maximum_colinear_points(&points)); // 4 
} // end of local scope OR end of main()
```

## V2

**About Rust :**

* ``usize`` where it makes sense
*  `points.iter()...` rather than for loop in `maximum_colinear_points()`
* prefer ``.entry()`` to ``.get()`` + `.insert()`. ``.entry()`` provides a mutable reference for in place modification
    * ``let curr_count = slopes_map.entry(curr_slope).or_insert(0);``
    * ``*curr_count += 1;`` 
* Use ``&[(i32, i32)]`` as parameter (allow Vect, []...)
* Update of ``gcd()``
* <span style="color:lime"><b>Preferred solution?</b></span>
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
use std::collections::HashMap;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn get_slope(p1: (i32, i32), p2: (i32, i32)) -> (i32, i32) {
    let rise = p2.1 - p1.1;
    let run = p2.0 - p1.0;
    if run == 0 {
        return (1, 0); // Vertical line
    }
    // simplify the slope to its reduced form
    let gcd_val = gcd(rise, run);
    (rise / gcd_val, run / gcd_val)
}

// Max points colinear with focal point
fn max_points_from_focal_point(focal_point_index: usize, points: &[(i32, i32)]) -> i32 {
    let mut slopes_map: HashMap<(i32, i32), i32> = HashMap::new(); // explicit type annotation
    let mut max_points = 0;
    // for current focal point, calculate the slope between it and the others point
    // We group points that share the same slope
    for (j, &point) in points.iter().enumerate() {
        if j != focal_point_index {
            let curr_slope = get_slope(points[focal_point_index], point);
            let curr_count = slopes_map.entry(curr_slope).or_insert(0);
            *curr_count += 1; // curr_count is a mutable reference for in place modification
            // update the max count of colinear points for current focal point
            max_points = max_points.max(*curr_count);
        }
    }
    max_points + 1 // Include focal point
}

// Global maximum over all focal points
fn maximum_colinear_points(points: &[(i32, i32)]) -> i32 {
    // treat each point as a focal point
    // determine the max number of point colinear with each focal point
    // the largest is the answer
    points.iter()
        .enumerate()
        .map(|(i, _)| max_points_from_focal_point(i, points))
        .max()
        .unwrap_or(0)
}

fn main() { // no main() if this code runs in a Jupyter cell 
    let points = vec![
        (1, 1),
        (1, 3),
        (2, 2),
        (3, 1),
        (3, 3),
        (4, 4),
    ];
    println!("{}", maximum_colinear_points(&points)); // 4
} // end of local scope OR end of main()

```
