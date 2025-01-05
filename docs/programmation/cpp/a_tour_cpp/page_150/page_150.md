---
layout: default
title: "A tour of C++ – Exemple page 150"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 16:08:48
last_modified_date: 2020-05-02 23:00:59
---

# A Tour of C++ – Exemple page 150

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


A vrai bonheur. Simple, facile à comprendre...Mais bon cela n'a de sens que si et seulement si les tâches n'ont pas de ressources à partager.



## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <vector>
#include <future>
#include <numeric>
using namespace std;
// ----------------------------------------------------------------------------
double accum(double* beg, double* end, double init){
  return accumulate(beg, end, init);                                            // compute the sum of [beg:end) starting with the initial value init
}
// ----------------------------------------------------------------------------
double comp4(vector<double>& v){                                                // spawn many tasks if v is large enough

  if (v.size()<10000)                                                           // is it worth using concurrency?
    return accumulate(v.begin(), v.end(), 0.0);

  auto v0 = &v[0];
  auto sz = v.size();
  auto f0 = async(accum, v0, v0 + sz / 4, 0.0);                                 // first quarter
  auto f1 = async(accum, v0 + sz / 4, v0 + sz / 2, 0.0);                        // second quarter
  auto f2 = async(accum, v0 + sz / 2, v0 + sz * 3 / 4, 0.0);                    // third quarter
  auto f3 = async(accum, v0 + sz * 3 / 4, v0 + sz, 0.0);                        // fourth quarter

  return f0.get() + f1.get() + f2.get() + f3.get();                             // collect and combine the results
}
// ----------------------------------------------------------------------------
void Test(void) {
  vector<double> v(20000);
  iota(v.begin(), v.end(), 0.0);
  auto result = comp4(v);

  cout << "Sum of " << v.size() << " elements equals : " << fixed << result << endl;
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                         // Begins the dump since the start of program execution
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

