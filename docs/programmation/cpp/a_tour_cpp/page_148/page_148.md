---
layout: default
title: "A tour of C++ – Exemple page 148"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 16:03:25
last_modified_date: 2020-05-02 23:02:01
---

# A Tour of C++ – Exemple page 148

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


A retenir : "Put in Promise" et "Get from Future". Pour le reste j'aime bien le fait qu'on puisse récupérer et/ou passer les exceptions. Cerise sur le gâteau la tuyauterie à l'air plus simple qu'avec les `condition_variable`.
Dans le code j'ai juste rajouté des commentaires et renommé quelques variables. Bine lire le début de la fonction `Test()` pour bien comprendre comment les choses se mettent en place et la façon dont on met en route les threads.



## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <future>
using namespace std;
// ----------------------------------------------------------------------------
void Producer(promise<int>& px){                                                // a task: place the result in px
  try{                                                                          // ... compute a value for res ...
    int MyValue { 42 };
    px.set_value(MyValue);                                                      // put the value MyValue into the promise
  } catch (...) {                                                               // oops: couldn’t compute res
    px.set_exception(current_exception());                                      // pass the exception to the future’s thread. The current_exception() refers to the caught exception.
  }
}
// ----------------------------------------------------------------------------
void Consumer(future<int>& fx){                                                 // a task: get the result from fx
  try{
    int v = fx.get();                                                           // If the value isn’t there yet, the thread is blocked until it arrives
                                                                                // If the value couldn’t be computed, get() might throw an exception
                                                                                // From the system or transmitted from the task from which we were trying to get() the value
                                                                                // ... use v ...
    cout << "In Consumer(), the value is : " << v << endl;
  } catch (...) {                                                               // oops: someone couldn’t compute v
    cout << "Exception in Consumer()" << endl;
  }
}
// ----------------------------------------------------------------------------
void Test(void) {
  promise<int>  myPromise;
  future<int>   myFuture = myPromise.get_future();
  thread t1{Producer, ref(myPromise)};
  thread t2{Consumer, ref(myFuture)};
  t1.join();
  t2.join();
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

