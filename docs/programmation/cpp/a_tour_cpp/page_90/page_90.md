---
layout: default
title: "A tour of C++ – Exemple page 90"
parent: "A tour of C++"
nav_order: 90
#math: mathjax
date: 2014-12-24 10:47:29
last_modified_date: 2020-05-02 23:17:49
---

# A Tour of C++ – Exemple page 90

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


Couvre les pages 90-93. Dans le cas de la fonction Test2(), le mieux, c'est de poser un point d'arrêt et d'y aller gentiment pas à pas. En ayant recompilé le code avec Visual Studio Community 2015, j'ai eu le plaisir de pouvoir (enfin) utiliser les constexpr (voir ligne 56 par exemple).


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <string>
#include <vector>
#include <iostream>
#include <sstream>
using namespace std;
struct Entry {
  string  name;
  int     number;
};
// ----------------------------------------------------------------------------
ostream& operator<<(ostream& os, const Entry& e){
  return os << "{\"" << e.name << "\", " << e.number << "}";
}
// ----------------------------------------------------------------------------
istream& operator>>(istream& is, Entry& e){                                     // read { "name" , number } pair. Note: for matted with { " " , and }
  char c, c2;
  if (is >> c && c == '{' && is >> c2 && c2 == '"') {                           // start with a { "
    string name;                                                                // the default value of a string is the empty string: ""
    while (is.get(c) && c != '"')                                               // anything before a " is part of the name
      name += c;
    if (is >> c && c == ',') {
      int number = 0;
      if (is >> number >> c && c == '}') {                                      // read the number and a }
        e = { name, number };                                                   // assign to the entry
        return is;
      }
    }
  }
  is.setstate(ios_base::failbit);                                               // register the failure in the stream
  return is;
}
// ----------------------------------------------------------------------------
void Test(void) {
  vector<Entry> MyInput{
      { "John Marwood Cleese", 123456          },
      { "Michael Edward Palin", 987654 }
  };
  for (auto i:MyInput)                                                          // read from cin into ee
    cout << i << '\n';                                                          // write ee to cout
                                                                                // See 8.6 p 91 about Formating
  cout << 1234 << ',' << hex << 1234 << ',' << oct << 1234 << '\n';             // print 1234,4d2,2322

  constexpr double d = 123.456;                                                 // now supported in MSVC 2015
  cout << d << "; "                                                             // use the default for mat for d
    << scientific << d << "; "                                                  // use 1.123e2 style for mat for d
    << hexfloat << d << "; "                                                    // use hexadecimal notation for d
    << fixed << d << "; "                                                       // use 123.456 style for mat for f
    << defaultfloat << d << '\n';                                               // use the default for mat for d
  cout.precision(8);
  cout << 1234.56789 << ' ' << 1234.56789 << ' ' << 123456 << '\n';

  cout.precision(4);
  cout << 1234.56789 << ' ' << 1234.56789 << ' ' << 123456 << '\n';
}
// ----------------------------------------------------------------------------
template<typename Target = string, typename Source = string>                    // see p 93
Target to(Source arg){                                                          // convert Source to Target
  stringstream  interpreter;
  Target        result;

  if (!(interpreter << arg)                                                     // write arg into stream
    || !(interpreter >> result)                                                 // read result from stream
    || !(interpreter >> std::ws).eof())                                         // stuff left in stream?
    throw runtime_error{ "to<>() failed" };
  return result;
}
// ----------------------------------------------------------------------------
void Test2(void) {
  auto x1 = to<string, double>(1.2);                                            // very explicit (and verbose)
  auto x2 = to<string>(1.2);                                                    // Source is deduced to double
  auto x3 = to<>(1.2);                                                          // Target is defaulted to string; Source is deduced to double
  auto x4 = to(1.2);                                                            // the <> is redundant;
}
// ----------------------------------------------------------------------------
int main() {
#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
  Test();
  Test2();
#ifdef _MSC_VER
  //_CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER
  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
