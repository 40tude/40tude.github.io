---
layout: default
title: "A tour of C++ - Exemple page 064"
parent: "A tour of C++"
nav_order: 64
#math: mathjax
date: 2014-12-22 00:40:49
last_modified_date: 2020-05-02 23:20:09
---

# A Tour of C++ – Exemple page 64

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


Couvre les sources des pages 64-66.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <vector>
#include <list>
#include <string>
#include <iostream>
using namespace std;
// ----------------------------------------------------------------------------
template<typename T>
class Less_than {
private:
  const T val;                                                                  // value to compare against
public:
  Less_than(const T& v) :val(v) { }
  bool operator()(const T& x) const { return x<val; }                           // call operator
};
// ----------------------------------------------------------------------------
template<typename C, typename P>
int count(const C& c, P pred){
  int cnt = 0;
  for (const auto& x : c)
    if (pred(x))
      ++cnt;
  return cnt;
}
// ----------------------------------------------------------------------------
void f(const vector<int>& vec, const list<string>& lst, int x, const string& s){
  cout << "number of values less than " << x
    << ": " << count(vec, Less_than<int>{x})
    << '\n';
  cout << "number of values less than " << s
    << ": " << count(lst, Less_than<string>{s})
    << '\n';
}
// ----------------------------------------------------------------------------
void g(const vector<int>& vec, const list<string>& lst, int x, const string& s){
  cout << "number of values less than " << x
    << ": " << count(vec, [&](int a){ return a<x; })
    << '\n';
  cout << "number of values less than " << s
    << ": " << count(lst, [&](const string& a){ return a<s; })
    << '\n';
}
// ----------------------------------------------------------------------------
template<typename T, typename ... Tail>
void h(T head, Tail... tail){
  i(head);                                                                      // do something to head
  h(tail...);                                                                   // try again with tail
}
// ----------------------------------------------------------------------------
void h() { }                                                                    // Handle cases where h() get called. Do nothing
// ----------------------------------------------------------------------------
template<typename T>
void i(T x){
  cout << x << " ";
}
// ----------------------------------------------------------------------------
void Test(void) {
  Less_than<int> lti{ 42 };                                                     // lti(i) will compare i to 42 using < (i<42)
  Less_than<string> lts{ "Backus" };                                            // lts(s) will compare s to "Backus" using < (s<"Backus")
  auto n=24;
  bool b1 = lti(n);                                                             // true if n<42
  auto s= "Blabla";
  bool b2 = lts(s);                                                             // true if s<"Backus"
  // ...
  vector<int> MyVec { 100, 200, 300, 400 };
  list<string> MyList{ "aaaa", "bbbb", "ccc", "zzzz" };
  cout << "Using f() : " << endl;
  f(MyVec, MyList, 250, "ddd");
  cout << "\nUsing g() : " << endl;
  g(MyVec, MyList, 250, "ddd");
  // Variadic templates
  cout << "\n\nFirst  : ";
  h(1, 2.2, "hello");
  cout << "\nSecond : ";
  h(0.2, 'c', "yuck!", 0, 1, 2);
  cout << "\n";
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
