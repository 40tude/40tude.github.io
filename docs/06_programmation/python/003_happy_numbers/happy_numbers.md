---
layout: default
title: "Happy Numbers"
parent: "Python"
#math: mathjax
date:               2025-01-31 01:00:00
last_modified_date: 2025-01-31 01:00:00
---

# Happy Numbers

<div align="center">
<img src="./assets/img_01.webp" alt="" width="560" loading="lazy"/>
</div>

Image from [Wikipedia](https://en.wikipedia.org/wiki/Happy_number)





## Intro

An integer is **happy** if, when calculating the sum of the squares of its digits, then the sum of the squares of the digits of the resulting number, and so on, it eventually reaches 1. Otherwise, the number is **unhappy**.


<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/ZbZSe6N_BXs?si=w2W87H7oAvvdJJCM" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>



## First Python Code

Let's check if 24 is happy. Here is the code : 

```python
def sum_of_squared_digits2(n1:int)->int:
    n2 = 0
    while n1:
        digit=n1%10
        # n2 += pow(digit, 2)
        # n2+=digit**2
        n2+=digit*digit
        n1=n1//10
    n1=n2
    return n2


n_set = set()
n = 24

while (n!=1 and n not in n_set):
    n_set.add(n)
    n = sum_of_squared_digits2(n)
    print(n)
print("Happy")   if n==1 else print("Unhappy") 
```

This output looks like :

```powershell
20
4
16
37
58
89
145
42
20
Unhappy
```

I did some test with `pow()` and `**2` but finally using the cached value `digit` was more efficient.


## Second implementation
Here I try to simplify ``sum_of_squared_digits()``
* The one liner goes like that :
    * Convert ``n`` in a string 
    * Then read each "char" (digit) of the string
    * Convert each cahr as an ``int`` 
    * Elevate the ``int`` to power of 2 



```python
def sum_of_squared_digits(n:int)->int:
    return sum([int(i)**2 for i in str(n)])

n_set = set()
n = 24
while (n!=1 and n not in n_set):
    n_set.add(n)
    n = sum_of_squared_digits(n)
    print(n)

print("Happy") if n==1 else print("Unhappy") 

```

No surprise, we get the same output. 

However we may want to know which approach is faster. One thing I learned with optimized C++ compiler: 
* **When it comes to benchmarks, never assume. Measure!**













## Benchmarking 1 in Python 

```python
import time

k_MAX=100_000

def sum_of_squared_digits(n:int)->int:
    return sum([int(i)**2 for i in str(n)])

start_time = time.time()  # Start timer

for n in range(1, k_MAX+1):
    n_set = set()
    n_init = n
    while (n!=1 and n not in n_set):
        n_set.add(n)
        n = sum_of_squared_digits(n)

end_time = time.time()  
print(f"Execution time: {end_time - start_time:.6f} seconds")

```

``Execution time: 0.797350 seconds``











## Benchmarking 2 in Python 


```python
import time

k_MAX = 100_000

def sum_of_squared_digits2(n_in:int)->int:
    n_out = 0
    while n_in:
        digit=n_in%10
        n_out += digit*digit
        n_in=n_in//10
    n_in=n_out
    return n_out

start_time = time.time()  # Start timer

for n in range(1, k_MAX + 1):
    n_set = set()
    n_init = n
    while n != 1 and n not in n_set:
        n_set.add(n)
        n = sum_of_squared_digits2(n)

end_time = time.time()  
print(f"Execution time: {end_time - start_time:.6f} seconds")

```

``Execution time: 0.482066 seconds``. The code of `sum_of_squared_digits2()` is longer **but** there is no string conversion etc. And we use ``digit`` which is in the local cache. This explains the 1.6 speed ratio.

I did some testings calling the script with ``-O`` within a console but I did'nt get any significant improvement. No, I did'nt transpile Python to C.  











## Benchmarking in C++

* If you don't hace a C++ compiler on your host, copy-past and run the code below on this online compiler : https://cpp.sh/
* I selected C++23 and 02 optimization but it works wtih C++20 and no optimization.

```cpp
#include <iostream>
#include <unordered_set>
#include <chrono>

constexpr int k_MAX = 100'000;

int sum_of_squared_digits3(int n_in) {
    int n_out = 0;
    while (n_in != 0) {
        int digit = n_in % 10;
        n_out += digit * digit;
        n_in /= 10;
    }
    return n_out;
}

int main() {
    auto start_time = std::chrono::high_resolution_clock::now();

    for (int n = 1; n <= k_MAX; ++n) {
        std::unordered_set<int> n_set;
        int n_current = n;

        while (n_current != 1 && !n_set.contains(n_current)) {
            n_set.insert(n_current);
            n_current = sum_of_squared_digits3(n_current);
        }
    }

    auto end_time = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed_seconds = end_time - start_time;
    std::cout << "Execution time: " << elapsed_seconds.count() << " seconds\n";
    return 0;
}

```
``Execution time: 0.083385 seconds``

* 34 lines in C++ vs 24 in Python
* 0.08 sec vs 0.48 sec
* 6 times faster






## Benchmarking C++ V2

```cpp

#include <iostream>
#include <unordered_set>
#include <array>
#include <chrono>

constexpr int k_MAX = 100'000;

constexpr std::array<int, 10> k_squares = [] {
    std::array<int, 10> squares{};
    for (int i = 0; i < 10; ++i) {
        squares[i] = i * i;
    }
    return squares;
}();

int sum_of_squared_digits2(int n_in) {
    int n_out = 0;
    while (n_in != 0) {
        int digit = n_in % 10;
        n_out += k_squares[digit];
        n_in /= 10;
    }
    return n_out;
}


int main() {
    auto start_time = std::chrono::high_resolution_clock::now();

    for (int n = 1; n <= k_MAX; ++n) {
        std::unordered_set<int> n_set;
        int n_current = n;

        while (n_current != 1 && !n_set.contains(n_current)) {
            n_set.insert(n_current);
            n_current = sum_of_squared_digits2(n_current);
        }
    }

    auto end_time = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed_seconds = end_time - start_time;
    std::cout << "Execution time: " << elapsed_seconds.count() << " seconds\n";
    return 0;
}

```
Similar timing


## Conclusion
* Yes could have used numpy, threads etc.
* Yes we could do a musch better job with some multithreading etc.
* But... It is always a tradeoff between the time spent vs speed improvement
* Feel free to read this post about [Sieve of Eratosthenes]({%link docs/06_programmation/c/000_crible_eratosthene/crible_eratosthene.md%}) 



