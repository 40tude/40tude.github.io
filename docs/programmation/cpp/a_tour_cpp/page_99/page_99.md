---
layout: default
title: "A tour of C++ – Exemple page 99"
parent: "A tour of C++"
nav_order: 99
#math: mathjax
date: 2014-12-24 11:01:32
last_modified_date: 2020-05-02 23:16:30
---

# A Tour of C++ – Exemple page 99

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


Comme indiqué dans le code, je ne comprend pas l'expression : `using vector<T>::vector;`
La bonne nouvelle c'est que dorénavant, avec MSVC 2015 ce code passe à la compile. Ce n'était pas le cas avec MSVC 2013.



## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <vector>
#include <iostream>
#include <stdexcept>                                                            // out_of_range
#include <limits>                                                               // numeric_limits
using namespace std;
// ----------------------------------------------------------------------------
struct Entry {
  string name;
  int number;
};
// ----------------------------------------------------------------------------
template<typename T>
class Vec : public std::vector<T> {                                             // the class Vec publicly inherit from std::vector
                                                                                // TODO : I d not understand the next statement (using vector<T>::vector;)
                                                                                // See http://www.stroustrup.com/what-is-2009.pdf p10-11
                                                                                // This means "import" vector() (constructors) from std::vector
                                                                                // Inheriting constructors
                                                                                // see http://ideone.com/NpAXgJ
public:
  using vector<T>::vector;                                                      // was not working in MSVC 2103. Now work in MSVC 2015
                                                                                // use the constructors from vector (under the name Vec)

  T& operator[](int i){                                                         // range check
    return vector<T>::at(i);
  }
  const T& operator[](int i) const {                                            // range check const objects; §4.2.1
    return vector<T>::at(i);
  }
};
// ----------------------------------------------------------------------------
void checked(Vec<Entry>& book){
  try {
    book[book.size()] = { "Joe", 999999 };                                      // will throw an exception
    // ...
  } catch (out_of_range) {
    cout << "Range error\n";
  }
}
// ----------------------------------------------------------------------------
void Test(void) {
  Vec<Entry> MyBook;
  checked(MyBook);
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                         // Begins the dump from the start of program execution
    _CrtDumpMemoryLeaks();
#endif // _MSC_VER
    cout << "Press ENTER to quit : ";
    cin.ignore((numeric_limits<streamsize>::max)(), '\n');
  }
  catch (out_of_range){
    cout << "Range Error somewhere..." << endl;
  }
  catch(...){
    cout << "Unknowns Exception Thrown" << endl;
  }
}
```

