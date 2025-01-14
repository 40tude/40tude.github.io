---
layout: default
title: "My Computer Science Vocabulary Page"
parent: "Programmation"
#math: mathjax
date: 2015-09-27 21:42:50
last_modified_date: 2023-02-16 17:19:55
---

# My Computer Science Vocabulary Page
{: .no_toc }

## Introduction
{: .no_toc }

Too often I have "problems" with computer science vocabulary. In addition, there is always something new I learn along my readings/viewings on the Web. So, I decided to start this page.

Usually Googling helps a lot to get an "answer" at the speed of light. On the other hand, working on this page, adding words, taking the time to read, copy/paste, write, summarize what I understood help me a lot.

### How does it works?
{: .no_toc }

* This is mostly for me
* This is done mostly in a [C++]({%link docs/06_programmation/cpp/index.md%}) context
* There is no goal of completeness (how could it be?)
* Entries are in alphabetical order
* Whenever possible I add the sources, links and references
* Some entries are and remains empty. This act as a reminder. Feel free to help, sharing a source of information in the comment

## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}




## Algorithm

[Knuth,1968] A finite set of rules which gives a sequence of operations for solving a specific set of problems [and] has five important features:

1. Finiteness
1. Definiteness
1. Input
1. Output
1. Effectiveness

In the context of the C++ standard library, an algorithm is a function template operating on sequences of elements.


<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Arguments and Parameters

```
void MyFunction(int i, double d){
    // ...
}

void main(){
    int MyInt = 1;
    MyFunction(MyInt, 42.0);
}
```

* `MyInt` and `42` are arguments. Let's keep in mind that : "The arguments are given"
* `i` and `d` are parameters. They are part of the definition of the function. They are part of the function signature.



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Amdahl's law

The speedup of a program using multiple processors in parallel computing is limited by the time needed for the sequential fraction of the program.

For example, if a program needs 20 hours using a single processor core, and a particular portion of the program which takes one hour to execute cannot be parallelized, while the remaining 19 hours (95%) of execution time can be parallelized, then regardless of how many processors are devoted to a parallelized execution of this program, the minimum execution time cannot be less than that critical one hour. Hence the speedup is limited to at most 20×.

[http://en.m.wikipedia.org/wiki/Amdahl's_law](http://en.m.wikipedia.org/wiki/Amdahl%27s_law)





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Concurrency

When tasks start, run and complete in overlapping time periods.

Task can be seen as a function. I can have concurrency even if I have a sigle thread of excution (this can be achieve with time slicing the thread, cooperative threading model or breaking up tasks and requeuing the tasks)

Concurrency is the basic building block that allow to get to parallelism. Also improve interactivity.

See : Parallelism, Tasks

See : https://www.youtube.com/watch?v=QIHy8pXbneI at 6:30





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Dependency Injection Framework

See : https://stackoverflow.com/questions/130794/what-is-dependency-injection






<!-- ###################################################################### -->
<!-- ###################################################################### -->
## DRY

Don't repeat yourself






<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Expressions

* Are made up of Operators and Operands.
* Are made of Operators and Values (literals or variable)
* If an instruction or a set of instructions evaluate to a single value this is an expression otherwise it is a statement.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Fiber




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Framework

Frameworks are distinct from libs :

* Inversion of control : With lib the application use the lib functions. The flow goes from the app code to the lib. With framework this is the opposite. The overall program's flow of control is dictated by the framework (not the app).
* Users can extend frameworks not libs
* Users can extend frameworks but should not modify their core.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Heuristic

*Example* : I used to use `reserve()`  to try to improve performance, but that turned out to be a waste of effort: The **heuristic** used by vector is better than my guesses, so now I only use `reserve()` to avoid reallocation of elements when I want to use pointers to elements.

*Wikipedia* : refers to experience-based techniques for problem solving, learning, and discovery that find a solution which is not guaranteed to be optimal, but good enough for a given set of goals. More precisely, heuristics are strategies using readily accessible, though loosely applicable, information to control [problem solving](http://en.wikipedia.org/wiki/Problem_solving) in human beings and machines.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Initialization vs copy vs assignments




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Introspection

Usage?




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Invariant



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## KISS

Keep it simple stupid. Ne complique pas les choses.





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Literals

* A string literal is a sequence of characters surrounded by double quotes.
* A literal is a constant: string literals, numeric literals etc.
* A literal is a notation for representing a fixed value in source code ([https://en.wikipedia.org/wiki/Literal_(computer_programming)](https://en.wikipedia.org/wiki/Literal_%28computer_programming%29))




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Parallelism

* When two or more tasks execute simultaneously
* See: Concurrency <https://youtu.be/QIHy8pXbneI?t=385>




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Parameters and Arguments

* See [Arguments and Parameters](#arguments-and-parameters)







<!-- ###################################################################### -->
<!-- ###################################################################### -->
## POD (Plain Old Data)

POD type is a scalar or a POD class with

* no user defined constructors or destructor,
* no user defined copy assignment operator,
* no base classes or virtual functions,
* no private or protected non-static data
* no static data that is non-POD types)




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Pointer dereferencing

* A variable (such as number) directly references a value,
* whereas a pointer indirectly references a value through the memory address it stores
* Referencing a value indirectly via a pointer (*ptr) is called indirection or dereferencing.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Predicates

A function that returns true or false
Read this [page](https://stackoverflow.com/questions/3230944/what-does-predicate-mean-in-the-context-of-computer-science).




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Programming idioms

* Simple Factory
* RAII
* PIMPL





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Reflexion

The ability of some programming languages to inspect type and code information at runtime and modify it. Could be another type of metaprogramming.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Statement

* *Example* :  A declaration is a statement that introduces a name into the program.  C++ provides a conventional set of statements for expressing selection and looping
* Statements are made up of expressions.
* If an instruction or a set of instructions evaluate to a single value this is an expression otherwise it is a statement.

### In French
Une expression est une portion de code que le compilateur peut évaluer pour obtenir une valeur. Les expressions peuvent être simples ou complexes. Elles sont formées d'une combinaison de litéraux, d'identifiants et d’opérateurs.





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Semantic

* *Example* : What is common for all iterators is their **semantics** and the naming of their operations.  However, when you really need the **semantics** of pointers, unique_ptr  is a very lightweight mechanism with no space or time overhead compared to correct use of a built-in pointer.
* <http://web.archive.org/> : It's the **meaning** of the language elements in terms of what they formally mean in terms of computation. This means that it expresses what a term of your language effectively does assuming an underlying kind of model that depends on which semantic we are talking about.
* Semantics is concerned with the interpretation or understanding of programs and how to predict the outcome of program execution. The semantics of a programming language describe the relation between the syntax and the model of computation. Semantics can be thought of as a function which maps syntactical constructs to the computational model. (<http://web.archive.org/web/20040410154109/cs.wwc.edu/~aabyan/PLBook/HTML/Semantics.html>)






<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Object Serialisation

See Python Pickle




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## SFINAE

* Substitution failure is not an error
* <https://en.wikipedia.org/wiki/Substitution_failure_is_not_an_error>
* <http://en.cppreference.com/w/cpp/language/sfinae>




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## SOLID

[https://en.wikipedia.org/wiki/SOLID_(object-oriented_design)](https://en.wikipedia.org/wiki/SOLID_%28object-oriented_design%29)





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Syntax (difference between syntax and semantic)

Syntax is about the **structure** or the grammar of the language. It answers the question: how do I construct a valid sentence? All languages, even English and other human (aka "natural") languages have grammars, that is, rules that define whether or not the sentence is properly constructed.

Here are some C language syntax rules:

* separate statements with a semi-colon
* enclose the conditional expression of an IF statement inside parentheses
* group multiple statements into a single statement by enclosing in curly braces
* data types and variables must be declared before the first executable statement (this feature has been dropped in C99. C99 and latter allow mixed type declarations.)

Semantics is about the **meaning** of the sentence. It answers the questions: is this sentence valid? If so, what does the sentence mean? For example:

```c
x++;                  // increment
foo(xyz, --b, &qrs);  // call foo
```

Are syntactically valid C statements. But what do they mean? Is it even valid to attempt to transform these statements into an executable sequence of instructions? These questions are at the heart of semantics.

Consider the ++ operator in the first statement. First of all, is it even valid to attempt this?

* If x is a float data type, this statement has no meaning (according to the C language rules) and thus it is an error ***even though the statement is syntactically correct.***
* If x is a pointer to **some data type**, the meaning of the statement is to "add sizeof(**some data type**) to the value at address x and store the result into the location at address x".
* If x is a scalar, the meaning of the statement is "add one to the value at address x and store the result into the location at address x".

Finally, note that some semantics cannot be determined at compile-time and must therefore must be evaluated at run-time. In the ++ operator example, if x is already at the maximum value for its data type, what happens when you try to add 1 to it? Another example: what happens if your program attempts to dereference a pointer whose value is NULL?

In summary, syntax is the concept that concerns itself only whether or not the sentence is valid for the grammar of the language . Semantics is about whether or not the sentence has a valid meaning.

See : <http://stackoverflow.com/questions/17930267/what-is-the-difference-between-syntax-and-semantics-of-programming-languages>





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Tasks

A unit of work, often a function, to be executed on a thread.

See : https://www.youtube.com/watch?v=QIHy8pXbneI at 18:15

We call a computation that can potentially be executed concurrently with other computations a task.





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Threads

Execution environment consisting of a stack and processor state running in parallel to other threads.

See : https://www.youtube.com/watch?v=QIHy8pXbneI at 18:15

A thread is the system-level representation of a task in a program.

Threads of a program share a single address space. In this, threads differ from processes, which generally do not directly share data. Since threads share an address space, they can communicate through shared objects (§13.5). Such communication is typically controlled by locks or other mechanisms to prevent data races (uncontrolled concurrent access to a variable).

We call a computation that can potentially be executed concurrently with other computations a task. A thread is the system-level representation of a task in a program. A task to be executed concurrently with other tasks is launched by constructing a std::thread (found in <thread>) with the task as its argument. A task is a function or a function object.

