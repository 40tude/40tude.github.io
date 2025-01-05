---
layout: default
title: "A tour of C++ – Exemple page 127"
parent: "A tour of C++"
nav_order: 127
#math: mathjax
date: 2015-08-04 15:36:16
last_modified_date: 2020-05-02 23:10:28
---

# A Tour of C++ – Exemple page 127

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


Rien de particulier. Bien voir le commentaire à propos de `round()` qui passait pas à la compile. Bien voir aussi que Round est un function object qui s'appuie sur une structure.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <vector>
#include <functional>
#include <algorithm>
using namespace std;
// ----------------------------------------------------------------------------
int Myround(double x) {                                                         // conventional 4/5 rounding
  return static_cast<int>(floor(x + 0.5));
}
// ----------------------------------------------------------------------------
function<int(double)> f;                                                        // f can hold anything that can be called with a double and return an int
enum class Round_style { truncate, round };
struct Round {                                                                  // function object carrying a state
  Round_style s;
  Round(Round_style ss) :s(ss) { }
  int operator()(double x) const { return static_cast<int>((s == Round_style::round) ? (x + 0.5) : x); };
};
// ----------------------------------------------------------------------------
void Test(void) {
  function<int(double)> f;                                                      // f is a function object. See §5.5 p64

  f = Myround;                                                                  // Initially Myround was named round. Did not compile under MSVC. Now I use Myround
  cout << f(7.6) << '\n';                                                       // call through f to the function Myround

  f = Round(Round_style::truncate);
  cout << f(7.6) << '\n';                                                       // call the function object
  Round_style style = Round_style::round;

  f = [style](double x){ return static_cast<int>((style == Round_style::round) ? x + 0.5 : x); };
  cout << f(7.6) << '\n';                                                       // call the lambda

  vector<double> v{ 7.6 };
  f = Round(Round_style::round);
  transform(v.begin(), v.end(), v.begin(), f);                                  // pass to algorithm
  cout << v[0] << '\n';                                                         // transformed by the lambda
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

