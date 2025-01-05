---
layout: default
title: "A tour of C++ – Exemple page 137"
parent: "A tour of C++"
nav_order: 137
#math: mathjax
date: 2015-08-04 15:45:34
last_modified_date: 2020-05-02 23:08:28
---

# A Tour of C++ – Exemple page 137

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


Pas de grosse difficulté ici. Faut juste penser à prendre le temps de parcourir le code pas à pas afin de voir comment x (ligne 32) et y (ligne 35) sont initialisées.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <random>
#include <functional>                                                           // bind
using namespace std;
// ----------------------------------------------------------------------------
class Rand_int {
public:
  Rand_int(int low, int high) :dist{ low, high } { }
  int operator()() { return dist(re); }                                         // draw an int
private:
  default_random_engine       re;
  uniform_int_distribution<>  dist;
};
// ----------------------------------------------------------------------------
void Test(void) {
  using my_engine = default_random_engine;                                      // type of engine
  using my_distribution = uniform_int_distribution<>;                           // type of distribution
  my_engine re{};                                                               // the default engine
  my_distribution one_to_six{ 1, 6 };                                           // distribution that maps to the ints 1..6
  auto die = bind(one_to_six, re);                                              // make a generator
  int x = die();                                                                // roll the die: x becomes a value in [1:6]
  auto die2 = bind(uniform_int_distribution<>{1, 60}, default_random_engine{});
  int y = die2();

  constexpr int max = 8;                                                        // constexpr works now in MSVC 2015
  Rand_int rnd{ 0, max };                                                       // make a uniform random number generator
  vector<int> histogram(max + 1);                                               // make a vector of appropriate size

  for (int i = 0; i != 200; ++i)
    ++histogram[rnd()];                                                         // fill histogram with the frequencies of numbers [0:max]

  for (int i = 0; i != histogram.size(); ++i) {                                 // write out a bar graph
    cout << i << '\t';
    for (int j = 0; j != histogram[i]; ++j) cout << '*';
    cout << endl;
  }
  cout << endl;
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

