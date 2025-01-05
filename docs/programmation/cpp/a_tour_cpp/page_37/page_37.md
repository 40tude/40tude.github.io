---
layout: default
title: "A tour of C++ – Exemple page 37"
parent: "A tour of C++"
#math: mathjax
date: 2014-12-14 19:41:29
last_modified_date: 2020-05-02 23:23:13
---

# A Tour of C++ – Exemple page 37

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


La fonction cin.clear() à la fin de la fonction read(...) est là pour que cela se passe bien suite à la lecture du ^Z à la fin de la saisie des doubles.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#define _SCL_SECURE_NO_WARNINGS                                                 // Avoid error with copy(...) in the initializer list ctor
#endif // _MSC_VER
#include <algorithm>
#include <iostream>
#include <limits>
using namespace std;
// ----------------------------------------------------------------------------
class Vector {
private:
  double  *elem;                                                                // elem points to an array of sz doubles
  int     sz;
public:
  Vector() : elem {nullptr}, sz {0} {}                                          // default constructor
  Vector(int s) :elem {new double[s]}, sz {s} {                                 // constructor: acquire resources
    for (int i = 0; i!=s; ++i) {                                                // initialize elements
      elem[i] = 0;
    }
  }
  Vector(Vector& v) :elem {new double[v.sz]}, sz {v.sz} {                       // copy constructor. Used in function read()
    for (int i = 0; i!=sz; ++i) {                                               // initialize elements
      elem[i] = v.elem[i];
    }
  }
  Vector(std::initializer_list<double>);                                        // initialize with a list of doubles
  void push_back(double);                                                       // add element at end, increasing the size by one
  ~Vector() { delete[] elem; sz = 0; }                                          // destructor: release resources

  double& operator[](int i) const { return elem[i]; };

  int size() const {return sz;};
};
// ----------------------------------------------------------------------------
                                                                                // initialize with a list
Vector::Vector(std::initializer_list<double> lst) :elem {new double[lst.size()]}, sz {static_cast<int>(lst.size())} {
  copy(lst.begin(), lst.end(), elem);                                           // copy from lst into elem (§10.6)
}
// ----------------------------------------------------------------------------
void Vector::push_back(double a) {

  if (elem==nullptr) {
    elem = new double[1];
    elem[0] = a;
    sz=1;
  } else {
    double *tmp = new double[sz+1];
    for (int i = 0; i!=sz; ++i) {                                                // initialize elements
      tmp[i] = elem[i];
    }
    tmp[sz] = a;
    delete[] elem;
    elem = tmp;
    sz++;
  }
}
// ----------------------------------------------------------------------------
void fct(int n) {
  Vector v(n);

  // ... use v ...

  {
    Vector v2(2*n);
    // ... use v and v2 ...
  }                                                                             // v2 is destroyed here

  // ... use v ..
}                                                                               // v is destroyed here
// ----------------------------------------------------------------------------
Vector read(istream& is) {

  Vector v;

  for (double d; is>>d;)                                                        // read floating-point values into d
    v.push_back(d);                                                             // add d to v

  cin.clear();
  return v;                                                                     // Make a copy using copy constructor, delete v then return
}
// ----------------------------------------------------------------------------
void Test(void) {

  Vector v1 = {1, 2, 3, 4, 5};                                                  // v1 has 5 elements
  Vector v2 = {1.23, 3.45, 6.7, 8};                                             // v2 has 4 elements
  fct(1024);

  cout << "Enter double values separated with SPACE. Finish the line with CTRL+Z : ";
  Vector v = read(cin);
  for (int i=0; i != v.size(); ++i)
    cout << v[i] << endl;
}
// ----------------------------------------------------------------------------
int main() {
#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
  Test();
#ifdef _MSC_VER
  //_CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER

  cout<<"Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
