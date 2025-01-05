---
layout: default
title: "A tour of C++ – Exemple page 126"
parent: "A tour of C++"
nav_order: 126
#math: mathjax
date: 2015-08-04 15:34:15
last_modified_date: 2020-05-02 23:11:11
---

# A Tour of C++ – Exemple page 126

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


Je comprend bind() même si je n'ai pas de cas d'usage en tête ou eu déjà le besoin de l'utiliser. En ce qui concerne mem_fn() je suis très dubitatif. Manque d'expérience sans doute. On peut aller faire un tour sur <http://en.cppreference.com/w/cpp/utility/functional/mem_fn> où il y a 2 exemples supplémentaires.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <functional>
#include <string>
#include <vector>
#include <algorithm> // for_each
using namespace std;
// ----------------------------------------------------------------------------
double cube(double x){
  cout << "x is equal to : " << x << endl;
  return x;
}
// ----------------------------------------------------------------------------
void f(int n, const string& str){
  for (auto i=0; i != n; ++i)
    cout << str << endl;
}
// ----------------------------------------------------------------------------
void Test1(void) {

  auto cube2 = bind(cube, 2);                                                   // a call to cube2() call cube() with argument 2
  double bob = cube2();
  using namespace placeholders;
  auto g = bind(f, 2, _1);                                                      // bind f()’s first argument to 2
  cout << "\nUsing f():" << endl;
  f(2, "hello");

  cout << "\nUsing g():" << endl;
  g("hello");                                                                   // also calls f(2,"hello");
}
// ----------------------------------------------------------------------------
struct Shape{
  int x;
  int y;
  Shape(int x_in, int y_in) : x{ x_in }, y{ y_in } {};
  void draw() {
    cout << "x : " << x << " y : " << y << endl;
  }
};
// ----------------------------------------------------------------------------
void user(Shape* p){
  p->draw();
  auto draw = mem_fn(&Shape::draw);                                             // produce a function object that can be called as a nonmember function
  draw(p);
}
// ----------------------------------------------------------------------------
void draw_all1(vector<Shape*>& v){
  cout << "\nUsing mem_fn() : " << endl;
  for_each(v.begin(), v.end(), mem_fn(&Shape::draw));
}
// ----------------------------------------------------------------------------
void draw_all2(vector<Shape*>& v){
  cout << "\nUsing lambda : " << endl;
  for_each(v.begin(), v.end(), [](Shape* p) { p->draw(); });                   // using lambda as an alternative
}
// ----------------------------------------------------------------------------
void Test2(void) {

  vector<Shape*> v{ new Shape{ 1, 2 }, new Shape{ 3, 4 } };
  draw_all1(v);
  draw_all2(v);

  for (auto &s:v)                                                               // avoid memory leak
    delete(s);
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test1();
    Test2();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                         // Begins the dump FileNameIn the start of program execution
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

