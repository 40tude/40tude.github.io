---
layout: default
title: "A tour of C++ – Exemple page 96"
parent: "A tour of C++"
#math: mathjax
date: 2014-12-24 10:54:29
last_modified_date: 2020-05-02 23:17:18
---

# A Tour of C++ – Exemple page 96

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


Faut penser à mettre un point d'arrêt à la ligne 49 (`vector<int> v1 = { 1, 2, 3, 4 };`) pour aller voir ce qui se passe.


## Le code source

```
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER

#include <vector>
#include <iostream>
#include <string>

using namespace std;

// ----------------------------------------------------------------------------
struct Entry {
  string name;
  int number;
};

// ----------------------------------------------------------------------------
struct Shape {
  int x;
  int y;
  int z;
};

// ----------------------------------------------------------------------------
ostream& operator<<(ostream& os, const Entry& e){

  return os << "{\"" << e.name << "\", " << e.number << "}";
}

// ----------------------------------------------------------------------------
void print_book(const vector<Entry>& book){
                                                                                // auto x         : when you want to work with copies.
                                                                                // auto &x        : when you want to work with original items and may modify them.
  for (const auto& x : book) cout << x << '\n';                                 // auto const &x  : when you want to work with original items and will not modify them.
}

// ----------------------------------------------------------------------------
void Test(void) {

  vector<Entry> phone_book = {
      { "David Hume", 123456 },
      { "Karl Popper", 234567 },
      { "Bertrand Arthur William Russell", 345678 }
  };

  print_book(phone_book);

  vector<int> v1 = { 1, 2, 3, 4 };                                              // size is 4
  vector<string> v2;                                                            // size is 0
  vector<Shape*> v3(23);                                                        // size is 23; initial element value: nullptr
  vector<double> v4(32, 9.9);                                                   // size is 32; initial element value: 9.9
}

// ----------------------------------------------------------------------------
int main() {

#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER

  Test();

#ifdef _MSC_VER
  //_CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
