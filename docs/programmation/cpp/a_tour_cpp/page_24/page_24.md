---
layout: default
title: "A tour of C++ - Exemple page 024"
parent: "A tour of C++"
nav_order: 20
#math: mathjax
date: 2014-12-14 19:23:12
last_modified_date: 2020-05-02 23:24:38
---

# A Tour of C++ – Exemple page 24

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


Bon, là, bien sûr, il y a plusieurs sources car il y a plusieurs fichiers utilisés. Je ne sais pas trop comment tester ce projet dans ideone.


## Fichier p024.cpp

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <limits>
#include "Vector.h"
double sqrt_sum(Vector& v);
using namespace std;
// ----------------------------------------------------------------------------
int main() {
#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
  Vector v(5);                                                                 // make a vector of 5 elements
  for (auto i = 0; i<5; ++i)
    v[i] = i;
  cout<<"Sum of the vector's square rooted components is : "<<sqrt_sum(v)<<endl;
#ifdef _MSC_VER
  _CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER
  cout<<"Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
### Fichier user.cpp

```cpp
// user.cpp
#include "Vector.h"                                                             // get Vector’s interface
#include <cmath>                                                                // get the standard-library math function interface including sqrt()
using namespace std;                                                            // make std members visible (§3.3)
// ----------------------------------------------------------------------------
double sqrt_sum(Vector& v) {

  double sum = 0.0;
  for (int i = 0; i!=v.size(); ++i)
    sum += sqrt(v[i]);                                                          // sum of square roots

  return sum;
}
```
### Fichier Vector.cpp

```cpp
// Vector.cpp
#include "Vector.h" // get the interface
// ----------------------------------------------------------------------------
Vector::Vector(int s) : elem {new double[s]}, sz {s} // initialize members
{
}
// ----------------------------------------------------------------------------
double& Vector::operator[](int i) {
  return elem[i];
}
// ----------------------------------------------------------------------------
int Vector::size() {
  return sz;
}
```
### Fichier Vector.h

```cpp
// Vector.h
class Vector {
public:
  Vector(int s);
  double& operator[](int i);
  int size();
private:
  double ∗elem;                                                                 // elem points to an array of sz doubles
  int sz;
};
```
