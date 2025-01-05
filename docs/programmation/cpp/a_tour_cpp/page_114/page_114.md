---
layout: default
title: "A tour of C++ – Exemple page 114"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 15:21:05
last_modified_date: 2020-05-02 23:12:55
---

# A Tour of C++ – Exemple page 114

## Introduction
Il s'agit d'un des codes d'exemple du livre **A Tour of C++** écrit par Bjarne Stroustrup et qui est disponible [ici](http://www.amazon.fr/Tour-C-Bjarne-Stroustrup/dp/0321958314/ref%3Dsr_1_1?ie=UTF8&qid=1416699327&sr=8-1&keywords=a+tour+of+c%2B%2B). 

Le contenu de ce livre correspond au début du livre **The C++ Programming Language** qu'on trouve [ici](http://www.amazon.fr/The-Programming-Language-Bjarne-Stroustrup/dp/0321563840/ref%3Dpd_sim_eb_3?ie=UTF8&refRID=0CR047TTJV1HA6CVA9XA).

Pour chaque exemple, j'ai essayé de faire en sorte que le code se compile, soit complètement autonome et tienne dans un seul source.

J'ai aussi tenté d'avoir des sorties à l'écran afin qu'on se rende compte un peu mieux de se qui se passe vraiment.

Oui, oui bien sûr, y a des cas où il faut aller vérifier avec un débogueur.
En tout cas l'objectif est de pouvoir avoir un code fonctionnel qu'on puisse compiler et dans lequel on puisse naviguer.

J'ai tenté de coller au maximum à l'exemple du bouquin. À part peut être un ou deux cas, j'ai dû renommer certains variables.

J'ai surtout travaillé avec [Visual Studio Community 2013](http://www.visualstudio.com/products/visual-studio-community-vs).

Ceci dit les exemples tournent dans [Ideone.com](http://ideone.com/) mais faut pas hésiter à les tester avec d'autres compilateurs en ligne. [Compiler Explorer](https://godbolt.org/) est un très bon exemple.

* Dans certains, j'ai été amené à aller y faire des tests.  
* Dans certains cas j'ai aussi fait aussi des tests avec le compilateur clang de mon Mac.  
* Pour utiliser le code, il suffit de faire un copier coller et normalement, zou c'est parti.  
* Pour les explications, bien sûr il vaut mieux se référer aux livres de l'auteur.  


## Notes spécifiques à cet exemple


Garder en tête que `find_if()` retourne in iterator sur ***premier*** élément qui correspond au prédicat.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <map>
#include <string>
#include <algorithm>
using namespace std;
// ----------------------------------------------------------------------------
struct Greater_than {

  int val;

  Greater_than(int v) : val{ v } { }
  bool operator()(const pair<string, int>& r) { return r.second>val; }
};
// ----------------------------------------------------------------------------
void f(map<string, int>& m){
  auto p = find_if(m.begin(), m.end(), Greater_than{ 40 });                     // Returns an iterator to the first element in the range [first,last) for which pred returns true
  cout << p->first << " : " << p->second << endl;
  // ...
}
// ----------------------------------------------------------------------------
void g(map<string, int>& m){                                                    // use a lambda

  auto p = find_if(m.begin(), m.end(), [](const pair<string, int>& r) { return r.second>20; });
  cout << p->first << " : " << p->second << endl;
  // ...
}
// ----------------------------------------------------------------------------
void Test(void) {

  map<string, int> MyValues{ { "riri", 10 }, { "fifi", 30 }, { "loulou", 50 }, };

  f(MyValues);
  g(MyValues);
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                         // Begins the dump FileNameIn the start of program execution
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

