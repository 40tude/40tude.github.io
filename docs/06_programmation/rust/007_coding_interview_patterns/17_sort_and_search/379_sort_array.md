---
# published: false
layout: default
lang: en-US
title: "p379 - Sort Array"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Sort Array

<div align="center">
<img src="../assets/chap_17.webp" alt="" width="300" loading="lazy"/>
</div>

* Given an array of i32
* Sort it in ascending order


<span style="color:orange"><b>The point:</b></span>

* Quicksort
* Place each number in its sorted position one at a time
    * Move all number smaller to the left (resp. greater, right). Partitioning
    * Number on the left (right) may not be ordered
    * The number is called pivot. Must be in correct position
    * Recursively sort left and right part
* Partition  
    * Pivot selection : rightmost number
    * 2 ptrs : low and i set on left. If `i<pivot` swap i and low then move low to the right  

**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n log(n))    | O(log(n)) |

* O(n log(n)) in time because it divides the array in 2 (more or less). So the tree is in log2(n). For each level, perform partition in O(n) => O(n log(n))
    * Worst case O(n²)
* O(log(n)) in space because of the depth of the recursive call stack (can grow up to log2(n)). 
    * Worst case O(n)









<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in code -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->



## V1

* First implementation

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)




```rust
fn partition( nums : &mut [i32], left : usize, right : usize) -> usize {
    let pivot = nums[right];
    let mut low = left;
    // Move all numbers less than pivot to the left (=> numbers greater than pivot are on the right)
    for i in left..right{
        if nums[i] < pivot{
            nums.swap(low, i);
            low +=1;
        }
    }
    // After partition low is where the pivot should be => swap the pivot number with the number at low pointer
    nums.swap(low, right);
    low
}

fn quicksort(nums : &mut [i32], left : usize, right : usize) {
    // Base case : if subarray has 0 or 1 element it is sorted
    if left>=right{
        return;
    }

    // Partition the array and retrieve the pivot index
    let pivot_index = partition(nums, left, right);
    
    // Call quicksort on left and right parts
    quicksort(nums, left, pivot_index.saturating_sub(1)); // prevent underflow
    quicksort(nums, pivot_index+1, right);
}

fn sort_array(nums : &mut [i32]) -> Vec<i32> {
    quicksort(nums, 0, nums.len()-1);
    nums.to_vec()
}

fn main() { // no main() if this code runs in a Jupyter cell
    let mut vals = vec![6, 8, 4, 2, 7, 3, 1, 5]; 
    println!("{:?}", sort_array(&mut vals)); // [1, 2, 3, 4, 5, 6, 7, 8]
} // end of local scope OR end of main()
```

## Optimized

* Try to avoid to select extreme pivot : one larger or smaller than most other element
* Random pivot
* First implementation

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
// # Cargo.toml
// [dependencies]
// rand = "0.9.1"

// If in a Jupyter cell
// :dep rand = "0.9.1"

// No reference to rand needed in Rust playground

use rand::Rng;

fn partition( nums : &mut [i32], left : usize, right : usize)->usize{
    let pivot = nums[right];
    let mut low = left;
    // Move all numbers less than pivot to the left (=> numbers greater than pivot are on the right)
    for i in left..right{
        if nums[i] < pivot{
            nums.swap(low, i);
            low +=1;
        }
    }
    // After partition low is where the pivot should be => swap the pivot number with the number at low pointer
    nums.swap(low, right);
    low
}

fn quicksort_optimized(nums : &mut [i32], left : usize, right : usize){
    // Base case : if subarray has 0 or 1 element it is sorted
    if left>=right{
        return;
    }

    // Choose a pivot at a random index
    let mut rng = rand::rng();
    let random_index = rng.random_range(left..=right);
    // Swap the random pivot with rightmost element to position pivot at rightmost index
    nums.swap(random_index, right);

    // Partition the array and retrieve the pivot index
    let pivot_index = partition(nums, left, right);
    
    // Call quicksort on left and right parts
    quicksort_optimized(nums, left, pivot_index.saturating_sub(1)); // prevent underflow
    quicksort_optimized(nums, pivot_index+1, right);
}

fn sort_array(nums : &mut [i32]) -> Vec<i32>{
    quicksort_optimized(nums, 0, nums.len()-1);
    nums.to_vec()
}

fn main() { // no main() if this code runs in a Jupyter cell

    let mut vals = vec![6, 8, 4, 2, 7, 3, 1, 5]; 
    println!("{:?}", sort_array(&mut vals)); // [1, 2, 3, 4, 5, 6, 7, 8]
    
} // end of local scope OR end of main()
```

## Optimized V2

* `sort_array()` and `quicksort_optimized()` work in place on the array and do not return a new vector


**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
// # Cargo.toml
// [dependencies]
// rand = "0.9.1"

// If in a Jupyter cell
// :dep rand = "0.9.1"

// No reference to rand needed in Rust playground

use rand::Rng;

fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = nums[right]; // The pivot is the value at the 'right' index
    let mut low = left; // 'low' starts at 'left'

    // Move all numbers less than pivot to the left
    for i in left..right {
        if nums[i] < pivot {
            nums.swap(low, i);
            low += 1;
        }
    }

    // After partition low is where the pivot should be => swap the pivot number with the number at low pointer
    nums.swap(low, right);
    low
}

fn quicksort_optimized(nums: &mut [i32], left: usize, right: usize) {
    // Base case : if subarray has 0 or 1 element it is sorted
    if left >= right {
        return;
    }

    // Choose a pivot at a random index
    let mut rng = rand::rng(); 
    let random_index = rng.random_range(left..=right); 
    // Swap the random pivot with rightmost element to position pivot on rightmost index
    nums.swap(random_index, right);

    // Partition the array and retrieve the pivot index
    let pivot_index = partition(nums, left, right);

    // Call quicksort on left and right parts
    // To prevent underflow both options are possible
    // if pivot_index > 0 {
    //     quicksort_optimized(nums, left, pivot_index - 1);
    // }
    quicksort_optimized(nums, left, pivot_index.saturating_sub(1)); 
    quicksort_optimized(nums, pivot_index+1, right);
}

fn sort_array(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }
    quicksort_optimized(nums, 0, nums.len() - 1);
}

fn main() { // no main() if this code runs in a Jupyter cell
    let mut vals = vec![6, 8, 4, 2, 7, 3, 1, 5];
    sort_array(&mut vals);
    println!("{:?}", vals); // [1, 2, 3, 4, 5, 6, 7, 8]
} // end of local scope OR end of main()

```

## Optimized V3

* Remove `left` and `right` parameters from `quicksort_optimized()` and `partition()`
* Random generator `rng` created once in `sort_array()` and passed as a parameter

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)
* <span style="color:lime"><b>Preferred solution?</b></span>     

### Note about `impl` in `quicksort()` signature
* The compiler requires `impl` 
* ``Rng`` is a trait NOT a datatype
* ``impl Rng`` means the function accept anything that implements `Rng`, without worrying about the exact type


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add more comments in the code -->



```rust
// # Cargo.toml
// [dependencies]
// rand = "0.9.1"

// If in a Jupyter cell
// :dep rand = "0.9.1"

// No reference to rand needed in Rust playground

use rand::Rng;

fn partition(nums: &mut [i32]) -> usize {
    let pivot = nums[nums.len() - 1];
    let mut low = 0;
    
    // Move all numbers less than pivot to the left (=> numbers greater than pivot are on the right)
    for i in 0..nums.len() - 1 {
        if nums[i] < pivot {
            nums.swap(low, i);
            low += 1;
        }
    }

    // After partition low is where the pivot should be => swap the pivot number with the number at low pointer
    nums.swap(low, nums.len() - 1);
    low
}

fn quicksort(nums: &mut [i32], rng: &mut impl Rng) { 
    // Base case : if subarray has 0 or 1 element it is sorted
    if nums.len() <= 1 {
        return;
    }

    // Choose a pivot at a random index
    let random_index = rng.random_range(0..nums.len());
    // Swap the random pivot with rightmost element to position pivot on rightmost index
    nums.swap(random_index, nums.len() - 1);

    let mid = partition(nums);
    let (left, right) = nums.split_at_mut(mid);

    // Call quicksort on left and right parts
    quicksort(left, rng);
    if right.len() > 1 {
        quicksort(&mut right[1..], rng);
    }
}

fn sort_array(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }
    let mut rng = rand::rng();
    quicksort(nums, &mut rng);
}

fn main() { // no main() if this code runs in a Jupyter cell
    let mut vals = vec![6, 8, 4, 2, 7, 3, 1, 5];
    sort_array(&mut vals);
    println!("{:?}", vals); // [1, 2, 3, 4, 5, 6, 7, 8]
} // end of local scope OR end of main()

```

## Counting sort V1

* The array only have u32
* All values are less or equal to 1_000


<span style="color:orange"><b>The point:</b></span>

* We know the largest  possible number and is not that large
* Non comparaison approach
* Count number of occurrences
* Create an n size array `counts`
* Increment the value at each index based on how many occurrences of the val we have
* Build sorted array `result` 
    * Iterate `counts` and add `i` according to `counts[i]`


**Complexity :**

| Time           | Space     |
|----------------|-----------|
| O(n+k)         | O(n+k)    |

* O(n+k) in time because if `k` is the max val of `nums`, it take O(n) to count occurrences of each element and O(k) to build the sorted array
* O(n+k) in space because `res` id O(n) and the `counts` is up to O(k)



**About Rust :**
* First implementation
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn sort_array_counting_sort(nums: &[i32])-> Vec<i32> { 
    if nums.is_empty() {
        return vec![];
    }
    
    // Count occurrences of each element in nums
    let max_value = *nums.iter().max().unwrap();        // Get the max value (unwrap because max() returns an Option<&T>)
    let mut counts = vec![0; (max_value as usize) + 1]; // Create a vector of zeros
    for num in nums{
        counts[*num as usize]+=1;
    }

    // For each value `i`, repeat `i` exactly `counts[i]` times and add all to the `res` vector
    let mut res:Vec<i32> = Vec::new();
    for (i, count) in counts.iter().enumerate(){
        res.extend(&vec![i as i32; *count as usize]);
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell
    let vals = vec![6, 8, 4, 2, 7, 3, 1, 5];
    println!("{:?}", sort_array_counting_sort(&vals)); // [1, 2, 3, 4, 5, 6, 7, 8]
} // end of local scope OR end of main()

```

## Counting sort V2

* Replace `for num in nums{` with `for &num in nums {` to avoid `*num` in the loop
* Create `res` with `Vec::with_capacity()` 
* Avoid to use `vec![]` in the loop 
    * it allocates a new vector each time
    * Use `std::iter::repeat(...).take(...)` instead 
* `for (i, count) in...` replaced with `for (i, &count) in...` 

**About Rust :**
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



```rust
fn sort_array_counting_sort(nums: &[i32]) -> Vec<i32> {
    if nums.is_empty() {
        return Vec::new();
    }

    // Count occurrences of each element in nums
    let max_value = *nums.iter().max().unwrap();        // Get the max value (unwrap because it returns an Option<&T>)
    let mut counts = vec![0; (max_value as usize) + 1]; // Create a vector of zeros
    for &num in nums {
        counts[num as usize] += 1;                      // Increment count for each number
    }

    // Pre-allocate result vector with capacity equal to input length
    let mut res = Vec::with_capacity(nums.len());

    // For each value `i`, repeat `i` exactly `counts[i]` times and add all to the `res` vector
    for (i, &count) in counts.iter().enumerate() {
        // res.extend(std::iter::repeat(i as i32).take(count as usize));
        res.extend(std::iter::repeat_n(i as i32, count as usize));
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell
    let vals = vec![6, 8, 4, 2, 7, 3, 1, 5];
    println!("{:?}", sort_array_counting_sort(&vals)); // [1, 2, 3, 4, 5, 6, 7, 8]
} // end of local scope OR end of main()
```

## Counting sort V3

* Avoid `.unwrap()`

**About Rust :**
* `let Some(&max_value) = nums.iter().max() else {...`. No `if`. This is direct destructuring.

```rust

let PATTERN = EXPR else {
    ... // code if pattern matching fails
};
```
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)



### Note about Some(&max_value)
* `iter()`: produces &i32 references.
* `max()`: returns a reference to the maximum element: Option<&i32>.
* At the end we get an `Option<&i32>`, **NOT** an `Option<i32>`.


```rust
fn sort_array_counting_sort(nums: &[i32]) -> Vec<i32> {
    // If the array is empty, return an empty vector
    let Some(&max_value) = nums.iter().max() else {
        return Vec::new();
    };

    // Count occurrences of each element in nums
    let mut counts = vec![0; (max_value as usize) + 1];  // Create a vector of zeros
    for &num in nums {
        counts[num as usize] += 1;                       // Increment count for each number
    }

    // Pre-allocate result vector with capacity equal to input length
    let mut res = Vec::with_capacity(nums.len());

    // For each value `i`, repeat `i` exactly `counts[i]` times and add all to the `res` vector
    for (i, &count) in counts.iter().enumerate() {
        res.extend(std::iter::repeat_n(i as i32, count as usize));
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell
    let vals = vec![6, 8, 4, 2, 7, 3, 1, 5];
    println!("{:?}", sort_array_counting_sort(&vals)); // [1, 2, 3, 4, 5, 6, 7, 8]
} // end of local scope OR end of main()

```

## Counting sort V4

**About Rust :**
* No `.repeat()` but `.push()`. Should be faster.
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)
* <span style="color:lime"><b>Preferred solution?</b></span>     




```rust
fn sort_array_counting_sort(nums: &[i32]) -> Vec<i32> {
    // If the array is empty, return an empty vector
    let Some(&max_value) = nums.iter().max() else {
        return Vec::new();
    };

    // Count occurrences of each element in nums
    let mut counts = vec![0; (max_value as usize) + 1];  // Create a vector of zeros
    for &num in nums {
        counts[num as usize] += 1;                       // Increment count for each number
    }

    // Pre-allocate result vector with capacity equal to input length
    let mut res = Vec::with_capacity(nums.len());

    // For each value `i`, repeat `i` exactly `counts[i]` times and add all to the `res` vector
    for (i, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            res.push(i as i32);  // Push `i` `count` times into `res`
        }
    }
    res
}

fn main() { // no main() if this code runs in a Jupyter cell
    let vals = vec![6, 8, 4, 2, 7, 3, 1, 5];
    println!("{:?}", sort_array_counting_sort(&vals)); // [1, 2, 3, 4, 5, 6, 7, 8]
} // end of local scope OR end of main()

```
