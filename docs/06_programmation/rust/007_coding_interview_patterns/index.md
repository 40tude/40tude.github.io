---
layout: default
title: "Coding Interview Patterns in Rust"
parent: "Rust"
# math: mathjax
date:               2025-06-22 15:00:00 +0000
last_modified_date: 1964-06-22 15:00:00 +0000   # Ne tient pas compte de cette page dans les pages récemment mises à jour
#last_modified_date: 2025-01-05 15:00:00 +0000
nav_order: 0
# permalink: /programmation/
---

# Coding Interview Patterns in Rust

My notes in **Rust** from the book ISBN-10: [**1736049135**](https://amzn.eu/d/e3MFYEf) 

<div align="center">
<img src="./assets/book_cover.png" alt="" width="450" loading="lazy"/>
</div>



## How to
* Everything is on [Github](https://github.com/40tude/rust_coding_interview)
* One directory per chapter (01_two_pointers ... 19_math_and_geometry)
* In the directories, the number at the beginning of the filename (e.g., ``10_xxx.ipynb``, ``14_yyy.ipynb``) corresponds to the page number in the book.
* When the filename starts with `bonus_` the exercice is one of the ``bonus.pdf`` 
* A copy of ``bonus.pdf`` is available in the ``/assets`` directory
 




## In most notebooks
* I do **NOT** provide any explanation about the algorithm. Read the book.
* There is a recap of the challenge at the top of the page
* Then you will find the following sections
    * **The point** : The tip or the trick to keep in mind about the puzzle
    * **Complexity** : Recap of the time & space complexities
    * <span style="color:red"><b>TODO : </b></span>  This is mostly for me
    * **About Rust** :
        * You will find here a list of function calls, idioms, constructs...
        * There is no order, no priority
        * This is mostly for me since I'm learning Rust
        * I can repeat some items from one solution to another if I have trouble to keep them in mind
        * I usually confirm if the code of the cell has been executed in the [**Rust Playground**](https://play.rust-lang.org/)
    * There may be cells with code that doesn't work and I explain why
    * If there are multiple versions of the same solution 
        * I explain the changes from one version to the next.
        * If it make sense, I highlight if one of the versions is my <span style="color:lime"><b>Preferred solution</b></span>
        * When we come back to a notebook with several versions, the lime color makes it easy to find the one I prefer (this is purely personal)

This how the beginning of a notebook may look like :

<div align="center">
<img src="./assets/exercice_header.png" alt="" width="450" loading="lazy"/>
</div>

## About the sample code
* All code examples are self-contained. By that, I mean that, unlike the code presented in the book or in the associated [**GitHub repository**](https://github.com/ByteByteGoHq/coding-interview-patterns), they are not just isolated functions; instead, each example includes a ``main()`` function where I demonstrate how to invoke the function and how to initialize any required data structures. 
* For example, in [**Chapter 11**](https://github.com/40tude/rust_coding_interview/tree/main/11_trees), which covers binary trees, all the solutions I provide create the trees used in the chapter and display the results. The idea is really that each piece of code should be self-sufficient and easy to run.
* Yes, it results in more code, but I find it much more practical.
* In most of the code examples, I try to keep the names of functions, variables... identical to those in the book.

<div align="center">
<img src="./assets/cell.png" alt="" width="900" loading="lazy"/>
<p><i>Click on the image above to enlarge it.</i></p>
</div>


## What if you don't run Rust code in Jupyter Notebook
* Shame on you 😁. Read this [**README**](https://github.com/40tude/py_coding_interview/blob/main/README.md).
* You can copy and paste the code of the cell of interest either :
    1. In a local project on your PC. Read this page where I share [**my setup on Windows 11**](https://www.40tude.fr/docs/06_programmation/rust/005_my_rust_setup_win11/my_rust_setup_win11.html).
    1. A better solution is to start by using the [**Rust Playground**](https://play.rust-lang.org/). 
    1. In both cases, you may have to **uncomment** the line : ``fn main(){`` 


## This may help
* The page where I explain how to [**Use Rust in Jupyter Notebooks on Windows**](https://www.40tude.fr/docs/06_programmation/rust/001_rust_jupyter/rust_jupyter.html)
* [**My repo with code in Python**](https://github.com/40tude/py_coding_interview). 
    * In the [**README**](https://github.com/40tude/py_coding_interview/blob/main/README.md) I explain why I believe it is better to use Jupyter Notebooks in this learning context.
* The [**repo of the book**](https://github.com/ByteByteGoHq/coding-interview-patterns)

<!-- ## About contributions
This project is developed for personal and educational purposes. Feel free to explore and use it to enhance your own learning in Rust and algorithm.

Given the nature of the project, external contributions are not actively sought nor encouraged. However, constructive feedback aimed at improving the project (in terms of speed, accuracy, comprehensiveness, etc.) is welcome. Please note that this project is being created as a hobby and is unlikely to be maintained once my initial goal has been achieved. -->
