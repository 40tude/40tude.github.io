---
# published: false
layout: default
title: "bonus005 - Shift zeros to the end"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

# Shift zeros to the end

* Given an array of integers, modify the array in place to move all zeros to the end while maintaining the relative order of non-zero elements.

<span style="color:orange"><b>The point:</b></span>    
* Use two staged pointers:
    * 'left' points to the position where non-zero values should be placed
    * 'right' points to non-zero values
* Both pointers start at the left
* Move 'right' to the first non-zero value
* Swap values
* Increment both pointers by +1
* Move 'right' forward to the next non-zero value and repeat

**Complexity :**

| Time | Space |
|------|-------|
| O(n) | O(1)  |

* Because we traverse the array once and shift values in place






**About Rust :**
* Pay attention to the mutability of the reference vs mutability of the binding. [Read this page](https://www.40tude.fr/docs/06_programmation/rust/004_mutability/mutability_us.html) if unsure.
* **YES** : tested on the [Rust Playground](https://play.rust-lang.org/)


<!-- <span style="color:red"><b>TODO : </b></span> 
* Add comments in the source code        
 -->


<!-- * <span style="color:lime"><b>Preferred solution?</b></span>      -->





## Works with arrays and vectors 


```rust
// Here this is NOT the binding "nums" which is mutable, but the reference which allow us to modify the memory it points to
fn shift_zeros_to_the_end(nums: &mut[i32]){
    let mut left = 0;
    for right in 0..nums.len(){
        if nums[right] != 0 {
            nums.swap(left, right);
            left += 1;
        }
    }
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    let mut bob = [1, 0, 5, 0, 3, 12];
    shift_zeros_to_the_end(&mut bob);
    println!("{:?}", &bob);  // [1, 3, 12, 0, 0]
    
    let mut bob = vec![0, 1, 0, 3, 12];
    shift_zeros_to_the_end(&mut bob);
    println!("{:?}", &bob);  // [1, 3, 12, 0, 0]
// }
```

    [1, 5, 3, 12, 0, 0]
    [1, 3, 12, 0, 0]


## While loop rather than for loop
* The idea is to check the assembly code on [Compiler Explorer](https://godbolt.org/)
* In release mode LLVM generated code is similar in both case
* It seems the for loop is preferred for 
    * ease of reading with ranges
    * lower risk of indexing error
    * more idiomatic (not sure 100% about this one)

In both case, `cargo build --release avec opt-level=3`, for the loops (while of for) generates a code similar to : 

```asm
shift_zeros_to_the_end_for:
    xor     eax, eax            ; left = 0
    xor     ecx, ecx            ; right = 0
.LBB1_1:
    cmp     rcx, rsi             ; right < nums.len()
    jae     .LBB1_5              ; exit
    mov     edx, dword ptr [rdi + 4*rcx]
    test    edx, edx
    je      .LBB1_4
    mov     dword ptr [rdi + 4*rcx], dword ptr [rdi + 4*rax]
    mov     dword ptr [rdi + 4*rax], edx
    inc     rax
.LBB1_4:
    inc     rcx
    jmp     .LBB1_1
.LBB1_5:
    ret
```


```rust
fn shift_zeros_to_the_end(nums: &mut[i32]){    
    let mut left = 0;
    let mut right = 0;
    while right < nums.len(){
        if nums[right] != 0{
            nums.swap(left, right);
            left += 1;
        }
        right += 1;
    }
}

// fn main(){     // no main() if this code runs in a Jupyter cell 
    let mut bob = vec![1, 0, 5, 0, 3, 12];
    shift_zeros_to_the_end(&mut bob);
    println!("{:?}", &bob);  // [1, 5, 3, 12, 0, 0]

    let mut bob = vec![0, 1, 0, 3, 12];
    shift_zeros_to_the_end(&mut bob);
    println!("{:?}", &bob);  // [1, 3, 12, 0, 0]
// }
```

    [1, 5, 3, 12, 0, 0]
    [1, 3, 12, 0, 0]

