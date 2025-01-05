---
layout: default
title: "A tour of C++ – Exemple page 100"
parent: "A tour of C++"
#math: mathjax
date: 2015-01-18 23:04:01
last_modified_date: 2020-05-02 23:16:01
---

# A Tour of C++ – Exemple page 100

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


Par rapport aux code de la page 100 j'ai juste rajouté quelques lignes qui insert un enregistrement de le Phone Book (ligne 74 et suivantes)


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <list>
#include <string>
using namespace std;
struct Entry{
  string  name;
  int     number;
};
// ----------------------------------------------------------------------------
list<Entry> phone_book = {
    { "David Hume", 123456 },
    { "Karl Popper", 234567 },
    { "Bertrand Arthur William Russell", 345678 }
};
// ----------------------------------------------------------------------------
int get_number(const string& s){
  for (const auto& x : phone_book)
    if (x.name == s)
      return x.number;

  return 0;                                                                     // use 0 to represent "number not found"
}
// ----------------------------------------------------------------------------
int get_numberV2(const string& s){
  for (auto p = phone_book.begin(); p != phone_book.end(); ++p)
    if (p->name == s)
      return p->number;

  return 0;                                                                     // use 0 to represent "number not found"
}
// ----------------------------------------------------------------------------
list<Entry>::iterator ReturnIeratorTo(const Entry& ee){

  for (auto p = phone_book.begin(); p != phone_book.end(); ++p)
    if (p->name == ee.name && p->number == ee.number)
      return p;

  return phone_book.end();
}
// ----------------------------------------------------------------------------
void f(const Entry& ee, list<Entry>::iterator p, list<Entry>::iterator q){
  phone_book.insert(p, ee);                                                     // add ee before the element referred to by p
  phone_book.erase(q);                                                          // remove the element referred to by q
}
// ----------------------------------------------------------------------------
void Test(void) {

  if (auto Num = get_number("Karl Popper"))
    cout << "Karl Popper phone number is : " << Num << endl;
  else
    cout << "Karl Popper is NOT in the directory" << endl;
  if (auto Num = get_numberV2("Karl Popper"))
    cout << "Karl Popper phone number is : " << Num << endl;
  else
    cout << "Karl Popper is NOT in the directory" << endl;
  list<Entry>::iterator p = ReturnIeratorTo({ "Karl Popper", 234567 });
  list<Entry>::iterator q=p;
  f({ "Me", 148944 }, p, q);
  cout << "\nListing of the Phone Book" << endl;
  for (const auto & x : phone_book)
    cout << x.name << " : " << x.number << endl;
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
  }  catch (...) {
    cout << "Unknowns Exception Thrown" << endl;
  }
}
```

