---
layout: default
title: "A tour of C++ – Exemple page 107"
parent: "A tour of C++"
#math: mathjax
date: 2015-01-18 23:12:33
last_modified_date: 2020-05-02 23:14:49
---

# A Tour of C++ – Exemple page 107

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


Rien de spécial dans le code. Bien lire les paragraphes du livre cependant.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <list>
#include <string>
#include <vector>
#include <algorithm>
#include <iterator>
using namespace std;
struct Entry{
  string  name;
  int     number;
};
// ----------------------------------------------------------------------------
bool operator<(const Entry& x, const Entry& y){                                 // less than operator. Returns true when satisfied and false otherwise
  return x.name < y.name;                                                       // order Entrys by their names
}
// ----------------------------------------------------------------------------
bool operator==(const Entry& x, const Entry& y){                                // equality
  return x.name == y.name;                                                      // test Entrys by their names
}
// ----------------------------------------------------------------------------
void f_p107(vector<Entry>& vec, list<Entry>& lst){
  sort(vec.begin(), vec.end());                                                  // use < for order
  unique_copy(vec.begin(), vec.end(), lst.begin());                              // don’t copy adjacent equal elements
}
// ----------------------------------------------------------------------------
list<Entry> f_p108(vector<Entry>& vec){

  list<Entry> res;
  sort(vec.begin(), vec.end());
  unique_copy(vec.begin(), vec.end(), back_inserter(res));                      // append to res
  return res;                                                                   // list have a move constructor so this is OK
}
// ----------------------------------------------------------------------------
void Test(void) {
  vector<Entry> phone_book = {
      { "David Hume", 123456 },
      { "Karl Popper", 234567 },
      { "Bertrand Arthur William Russell", 345678 },
      { "Karl Popper", 234567 },
      { "David Hume", 123456 },
      { "Karl Popper", 234567 },
      { "David Hume", 123456 },
      { "Bertrand Arthur William Russell", 345678 },
      { "Karl Popper", 234567 },
      { "Bertrand Arthur William Russell", 345678 },
      { "David Hume", 123456 },
      { "Karl Popper", 234567 }
  };
  list<Entry> MyList{ 10 };
  f_p107(phone_book, MyList);                                                       // MyList must exist and be large enough
  for (const auto x : MyList)
    cout << x.name << endl;
  list<Entry> MyOtherList = f_p108(phone_book);                                     // MyOtherList is built
  for (const auto x : MyOtherList)
    cout << x.name << endl;
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
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

