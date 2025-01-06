---
layout: default
title: "A tour of C++ – Exemple page 149"
parent: "A tour of C++"
nav_order: 149
#math: mathjax
date: 2015-08-04 16:06:23
last_modified_date: 2020-05-02 22:59:52
---

# A Tour of C++ – Exemple page 149

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


Attention. Je n'arrive pas à faire tourner le code tel qu'il est donné dans le livre. A priori je ne suis pas le seul voir par exemple cet article sur [StackOverflow](http://stackoverflow.com/questions/20306335/c11-packaged-task-running-with-its-own-thread-need-a-join-in-order-to-get-fu). J'ai fait pas mal de tests sur mac (clang), dans Ideone (gcc etc.), dans le compilateur en ligne de Microsoft ainsi que dans Visual Studio Community 2015. Le code du livre ne passe dans aucun environnement. Bref, il faut vraiment mettre les deux join() avant de faire les get().


## Le code source

```cpp
// TODO : generate exception on http://ideone.com/ and http://webcompiler.cloudapp.net/
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <vector>
#include <numeric>
#include <thread>
#include <future>
using namespace std;
// ----------------------------------------------------------------------------
double accum(double* beg, double* end, double init){
  return accumulate(beg, end, init);                                            // compute the sum of [beg:end) starting with the initial value init
}
// ----------------------------------------------------------------------------
double comp2(vector<double>& v){
  using Task_type = double(double*, double*, double);                           // type of task

  packaged_task<Task_type> pt0{ accum };                                        // package the task (i.e., accum)
  packaged_task<Task_type> pt1{ accum };

  future<double> f0{ pt0.get_future() };                                        // get hold of pt0’s future
  future<double> f1{ pt1.get_future() };                                        // get hold of pt1’s future

  double* first = &v[0];
  thread t1{ move(pt0), first, first + v.size() / 2, 0 };                       // start a thread for pt0
  thread t2{ move(pt1), first + v.size() / 2, first + v.size(), 0 };            // start a thread for pt1

  t1.join();                                                                    // does'nt work otherwise
  t2.join();

return f0.get() + f1.get();                                                     // get the results
}
// ----------------------------------------------------------------------------
void Test(void) {
  vector<double> v{ 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0 };
  auto result = comp2(v);
  cout << result << endl;
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                         // Begins the dump since the start of program execution
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

