---
layout: default
title: "A tour of C++ – Exemple page 144"
parent: "A tour of C++"
nav_order: 144
#math: mathjax
date: 2015-08-04 15:54:11
last_modified_date: 2020-05-02 23:04:15
---

# A Tour of C++ – Exemple page 144

## Introduction
Il s'agit d'un des codes d'exemple du livre **A Tour of C++** écrit par Bjarne Stroustrup et qui est disponible [ici](http://www.amazon.fr/Tour-C-Bjarne-Stroustrup/dp/0321958314/ref%3Dsr_1_1?ie=UTF8&qid=1416699327&sr=8-1&keywords=a+tour+of+c%2B%2B). 

Le contenu de ce livre correspond au début du livre **The C++ Programming Language** qu'on trouve [ici](http://www.amazon.fr/The-Programming-Language-Bjarne-Stroustrup/dp/0321563840/ref%3Dpd_sim_eb_3?ie=UTF8&refRID=0CR047TTJV1HA6CVA9XA).

Pour chaque exemple, j'ai essayé de faire en sorte que le code se compile, soit complètement autonome et tienne dans un seul source.

J'ai aussi tenté d'avoir des sorties à l'écran afin qu'on se rende compte un peu mieux de se qui se passe vraiment.

Oui, oui bien sûr, y a des cas où il faut aller vérifier avec un débogueur.
En tout cas l'objectif est de pouvoir avoir un code fonctionnel qu'on puisse compiler et dans lequel on puisse naviguer.

J'ai tenté de coller au maximum à l'exemple du bouquin. À part peut être un ou deux cas, j'ai dû renommer certains variables.

J'ai surtout travaillé avec [Visual Studio Community 2013](http://www.visualstudio.com/products/visual-studio-community-vs).

Ceci dit les exemples tournent dans [Ideone.com](http://ideone.com/) mais faut pas hésiter à les tester avec d'autres compilateurs en ligne. [Compiler Explorer](https://godbolt.org/) est un très bon exemple.

* Dans certains, j'ai été amené à aller y faire des tests.  
* Dans certains cas j'ai aussi fait aussi des tests avec le compilateur clang de mon Mac.  
* Pour utiliser le code, il suffit de faire un copier coller et normalement, zou c'est parti.  
* Pour les explications, bien sûr il vaut mieux se référer aux livres de l'auteur.  


## Notes spécifiques à cet exemple


Noter que le paramètre est passé en tant que const reference et que la valeur retournée est le second argument. Bien voir le `cref()` utilisé lors de la création de t1.



## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <thread>
#include <vector>
#include <functional>                                                           // cref()
#include <numeric>                                                              // accumulate
using namespace std;
// ----------------------------------------------------------------------------
void f(const vector<double>& v, double* res){                                   // take input from a const ref and place result in *res
  *res = accumulate(v.begin(), v.end(), 0);
}
// ----------------------------------------------------------------------------
class F {                                                                       // function object
public:
  F(const vector<double>& vv, double* p) :v{ vv }, res{ p } { }                 // take input from a const ref and place result in *res
  void operator()(){
    *res = accumulate(v.begin(), v.end(), 0);
  }
private:
  const vector<double>& v;                                                      // source of input
  double*res;                                                                   // target for output
};
// ----------------------------------------------------------------------------
void Test(void) {
  vector<double> some_vec{ 1, 2, 3, 4, 5, 6, 7, 8, 9 };
  vector<double> vec2{ 10, 20, 30, 40, 50, 60, 70, 80, 90 };
  // ...
  double res1;
  double res2;
  thread t1{ f, cref(some_vec), &res1 };                                        // cref onstructs a const reference_wrapper from an argument.
  thread t2{ F{ vec2, &res2 } };
  t1.join();
  t2.join();
  cout << res1 << ' ' << res2 << '\n';
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                         // Begins the dump since the start of program execution
    _CrtDumpMemoryLeaks();
#endif // _MSC_VER
    cout << "Press ENTER to quit : ";
    cin.ignore((numeric_limits<streamsize>::max)(), '\n');
  }
  catch (...) {
    cout << "Unknowns Exception Thrown" << endl;
  }
}
```

