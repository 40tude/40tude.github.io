---
layout: default
title: "A tour of C++ – Exemple page 49"
parent: "A tour of C++"
nav_order: 49
#math: mathjax
date: 2014-12-15 12:13:25
last_modified_date: 2020-05-02 23:21:22
---

# A Tour of C++ – Exemple page 49

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


Le mieux ici c'est de mettre un point d'arrêt sur la ligne 116 (première ligne de la fonction Test()) et de voir, pas à pas, comment cela se passe. Attention les codes couvrent les sources qui sont sur les pages 49-52


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
using namespace std;
// ----------------------------------------------------------------------------
class Vector {
private:
  double* elem;                                                                 // elem points to an array of sz doubles
  int sz;
public:
  Vector() : elem{ nullptr }, sz{ 0 } {}
  explicit Vector(int s) :elem{ new double[s] }, sz{ s } {                      // constructor: establish invariant, acquire resources
                                                                                // explicit => no implicit conversion from int to Vector. Avoid Vector v2 = 7; allow Vector v2(7);
                                                                                // advise : use explicit for constructors that take a single argument unless there is a good reason not to
    for (int i = 0; i != s; ++i) {                                                                    // initialize elements (see p 54)
      elem[i] = 0;
    }
  }
  ~Vector() { delete[] elem; }                                                  // destructor: release resources
  Vector(const Vector& a);                                                      // copy constructor
  Vector& operator=(const Vector& a);                                           // copy assignment
  Vector(Vector&& a);                                                           // move constructor
  Vector& operator=(Vector&& a);                                                // move assignment
  //const double& operator[](int i) const { return elem[i]; };                  // read only operator since returned value is const
  double& operator[](int i) const { return elem[i]; };
  int size() const {return sz;};
};
// ----------------------------------------------------------------------------
// copy constructor
Vector::Vector(const Vector& a) : elem{ new double[a.sz] }, sz{ a.sz } {          // allocate space for elements
  for (int i = 0; i != sz; ++i)                                                   // copy elements
    elem[i] = a.elem[i];
}
// ----------------------------------------------------------------------------
// copy assignment
Vector& Vector::operator=(const Vector& a) {
  double* p = new double[a.sz];
  for (int i = 0; i != a.sz; ++i)
    p[i] = a.elem[i];
  delete[] elem;                                                                // delete old elements
  elem = p;
  sz = a.sz;
  return *this;
}
// ----------------------------------------------------------------------------
// A move constructor does not take a const argument
Vector::Vector(Vector&& a) : elem{ a.elem }, sz{ a.sz } {                       // "grab the elements" from a
  a.elem = nullptr;                                                             // now a has no elements
  a.sz = 0;
}
// ----------------------------------------------------------------------------
// move assignement
Vector& Vector::operator=(Vector&& a) {
  if (this != &a) {
    delete[] elem;
    sz = 0;
    elem = a.elem;
    sz = a.sz;
    a.elem = nullptr;                                                           // now a has no elements
    a.sz = 0;
  }
  return *this;
}
// ----------------------------------------------------------------------------
Vector operator+(const Vector& a, const Vector& b) {
  if (a.size() != b.size())
    throw out_of_range("operator+");                                            // throw Vector_size_mismatch {};

  Vector res(a.size());
  for (int i = 0; i != a.size(); ++i)
    res[i] = a[i] + b[i];
  return res;
}
// ----------------------------------------------------------------------------
void g(const Vector& x, const Vector& y, const Vector& z) {
  Vector r;
  // ...
  r = x + y + z;
  // ...
}
// ----------------------------------------------------------------------------
Vector f() {
  Vector x(1000);
  Vector y(1000);
  Vector z(1000);
  z = x;                                                                        // we get a copy
  y = std::move(x);                                                             // we get a move
  return z;                                                                     // we get a move
};
// ----------------------------------------------------------------------------
void Test(void) {
  Vector myV = f();
  Vector x(1000);
  Vector y(1000);
  Vector z(1000);
  g(x, y, z);
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
