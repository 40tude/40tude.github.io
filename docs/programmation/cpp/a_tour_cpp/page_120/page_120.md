---
layout: default
title: "A tour of C++ – Exemple page 120"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 15:26:50
last_modified_date: 2020-05-02 23:12:23
---

# A Tour of C++ – Exemple page 120

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


Il semble que `make_unique()` existe en C++14 (<http://en.cppreference.com/w/cpp/memory/unique_ptr/make_unique>) en tout cas on retrouve sa définition avec ALT F12 dans Visual Studio Community 2013 ou 2015.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <string>
#include <memory>
using namespace std;
// ----------------------------------------------------------------------------
template<typename T, typename ... Args>                                         // variadic template see p 66
unique_ptr<T> Make_Unique(Args&&... args) {
  return std::unique_ptr<T>{new T{ std::forward<Args>(args)... }};
}
// ----------------------------------------------------------------------------
void Test(void) {
  struct S {
    int     i;
    string  s;
    double  d;
    // ...
    S(int ii, string ss, double dd) : i{ ii }, s(ss), d{ dd } {};
  };
  shared_ptr<S> p1{ new S{ 1, "Ankh Morpork", 4.65 } };

  auto p2 = make_shared<S>(2, "Oz", 7.62);
  auto p3 = make_unique<S>(3, "Atlantis", 11.3);                                // make_unique already exist in C++14.
                                                                                // Right click on the name and select Go To Definition (F12) or Peek Defintion (ALT F12)
  auto p4 = Make_Unique<S>(4, "Austin", 42.0);
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

