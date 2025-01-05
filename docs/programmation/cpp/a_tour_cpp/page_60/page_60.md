---
layout: default
title: "A tour of C++ – Exemple page 60"
parent: "A tour of C++"
nav_order: 60
#math: mathjax
date: 2014-12-22 00:37:13
last_modified_date: 2020-05-02 23:20:45
---

# A Tour of C++ – Exemple page 60

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


Rien de bien particulier. Bien lire les commentaires et les paragraphes du livre. Ne pas oublier de faire du pas à pas avec le débogueur.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <string>
#include <list>
#include <iostream>
using namespace std;
// ----------------------------------------------------------------------------
template<typename T>
class Vector {
private:
  T*  elem;                                                                     // elem points to an array of sz elements of type T
  int sz;
public:
  explicit Vector(int s);                                                       // constructor: establish invariant, acquire resources
  ~Vector() { delete[] elem; }                                                  // destructor: release resources
  // ... copy and move operations ...
  T& operator[](int i) { return elem[i]; }
  const T& operator[](int i) const;                                             // read only operator since retruned value is const
  int size() const { return sz; }
};
// ----------------------------------------------------------------------------
template<typename T>
Vector<T>::Vector(int s){
  if (s<0)
    throw out_of_range{ "Vector constructor" };
  elem = new T[s];
  sz = s;
}
// ----------------------------------------------------------------------------
template<typename T>
const T& Vector<T>::operator[](int i) const
{
  if (i<0 || size() <= i)
    throw out_of_range{ "Vector::operator[]" };
  return elem[i];
}
// ----------------------------------------------------------------------------
template<typename T>
T* begin(Vector<T>& x){
  return x.size() ? &x[0] : nullptr;                                            // pointer to first element or nullptr
}
// ----------------------------------------------------------------------------
template<typename T>
T* end(Vector<T>& x){
  return begin(x) + x.size();                                                   // pointer to one-past-last element
}
// ----------------------------------------------------------------------------
void func(Vector<string>& vs){                                                  // Vector of some strings
  auto i = 0;
  for (auto& s : vs){
    cout << "String #" << i << " :" << s << '\n';
    i++;
  }
}
// ----------------------------------------------------------------------------
void Test(void) {
  Vector<char>      vc(200);                                                    // vector of 200 characters
  Vector<string>    vs(17);                                                     // vector of 17 strings
  Vector<list<int>> vli(45);                                                    // vector of 45 lists of integers
  Vector<string>    MyVec(3);
  func(MyVec);
  try{
    string bob = MyVec[100];
  } catch(...){
    cout << "Oh oh I just catched an exception." << endl;
  }
}
// ----------------------------------------------------------------------------
int main() {
#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
  Test();
#ifdef _MSC_VER
  //_CrtMemDumpAllObjectsSince(NULL);                                           // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER
  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
