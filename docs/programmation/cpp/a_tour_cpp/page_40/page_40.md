---
layout: default
title: "A tour of C++ – Exemple page 40"
parent: "A tour of C++"
#math: mathjax
date: 2014-12-14 19:47:13
last_modified_date: 2020-05-02 23:22:38
---

# A Tour of C++ – Exemple page 40

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


Faut juste penser à lire les commentaires ainsi que les paragraphes qui sont autour du source dans le livre.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#define _SCL_SECURE_NO_WARNINGS                                                 // Avoid error with copy(...) in the initializer list ctor
#endif // _MSC_VER
#include <algorithm>
#include <vector>
#include <list>
#include <iostream>
using namespace std;
// ----------------------------------------------------------------------------
class Container {
public:
  virtual double& operator[](int) = 0;                                          // pure virtual function
  virtual int size() const = 0;                                                 // const member function (§4.2.1)
  virtual ~Container() {}                                                       // destructor (§4.2.2)
};
// ----------------------------------------------------------------------------
class Vector {
private:
  double  *elem;                                                                // elem points to an array of sz doubles
  int     sz;
public:
  Vector(std::initializer_list<double>);                                        // initialize with a list of doubles
  ~Vector() { delete[] elem; sz = 0; }                                          // destructor: release resources
  double& operator[](int i) const { return elem[i]; };
  int size() const {return sz;};
};
// ----------------------------------------------------------------------------
                                                                                // Initialize Vector with a list
Vector::Vector(std::initializer_list<double> lst) :elem {new double[lst.size()]}, sz {static_cast<int>(lst.size())} {
  copy(lst.begin(), lst.end(), elem);                                           // copy from lst into elem (§10.6)
}
// ----------------------------------------------------------------------------
class Vector_container: public Container {                                      // Vector_container implements Container
private :
  Vector v;
public:
  Vector_container(std::initializer_list<double>);                              // initialize with a list of doubles
  ~Vector_container() {}
  double& operator[](int i) { return v[i]; }
  int size() const { return v.size(); }
};
// ----------------------------------------------------------------------------
Vector_container::Vector_container(initializer_list<double> lst) : v{ lst } {}  // initialize with a list
// ----------------------------------------------------------------------------
class List_container: public Container {                                        // List_container implements Container
private:
  list<double> ld;                                                              // (standard-library) list of doubles (§9.3)
public:
  List_container(initializer_list<double> il)  {
    ld = {il};
  }
  ~List_container() {}
  double& operator[](int i);
  int size() const { return ld.size(); }
};
// ----------------------------------------------------------------------------
double& List_container::operator[](int i) {

  for (auto& x:ld) {
    if (i==0) return x;
    --i;
  }
  throw out_of_range("List container");
}
// ----------------------------------------------------------------------------
void use(Container& c) {
  const int sz = c.size();
  for (int i = 0; i!=sz; ++i)
    cout<<c[i]<<'\n';
}
// ----------------------------------------------------------------------------
void Test(void) {

  Vector_container vc {10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0};
  cout<<"Content of Vector_Container"<<endl;
  use(vc);
  List_container lc = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  cout<<"Content of List_Container"<<endl;
  use(lc);
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
  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
