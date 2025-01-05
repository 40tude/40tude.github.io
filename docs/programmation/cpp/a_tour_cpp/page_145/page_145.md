---
layout: default
title: "A tour of C++ – Exemple page 145"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 15:57:24
last_modified_date: 2020-05-02 23:03:42
---

# A Tour of C++ – Exemple page 145

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


Forcément, puisque les variables doivent être modifiées par plusieurs threads, ces dernières sont globales. Il en va de même pour les mutex. Bref y a pas mal de variables globales au début du code. Bien voir le `defer_lock` dans la fonction g() à la ligne 30. J'ai rajouté le bout de code de la page 146 qui met en oeuvre `this_thread::sleep_for(milliseconds{ 20 })`.



## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <thread>
#include <mutex>
#include <chrono>
using namespace std;
double  gMyDble = 0.0;
mutex   gMyMutex;                                                               // manage access to gMyDble
int     gMyInt1 = 0;
int     gMyInt2 = 100;
mutex   gMutex1;
mutex   gMutex2;
// ----------------------------------------------------------------------------
void f(){                                                                       // take input from v; place result in *res
  unique_lock<mutex> lck{ gMyMutex };                                           // acquire mutex
  gMyDble+=3;                                                                   // manipulate shared data
}                                                                               //release mutex implicitly
// ----------------------------------------------------------------------------
void g(){

  unique_lock<mutex> lck1{ gMutex1, defer_lock };                               // defer_lock: don’t yet try to acquire the mutex
  unique_lock<mutex> lck2{ gMutex2, defer_lock };
  lock(lck1, lck2);                                                             //acquire both locks
  gMyInt1++;
  gMyInt2--;
}                                                                               // implicitly release all mutexes
// ----------------------------------------------------------------------------
void Test(void) {
  thread t1{ f };
  thread t2{ f };
  t1.join();
  t2.join();

  cout << "1 value has been manipulated (+3) by 2 threads. Its value is now : " << gMyDble << endl;
  thread t3{ g };
  thread t4{ g };
  t3.join();
  t4.join();
  cout << "2 values have been manipulated (+1 and -1) by 2 threads. Their values are : " << gMyInt1 << " " << gMyInt2 << endl;
  using namespace std::chrono;                                                  // see §11.4 p125
  auto time0 = high_resolution_clock::now();
  this_thread::sleep_for(milliseconds{ 20 });
  auto time1 = high_resolution_clock::now();
  cout << "Current thread was supposed to sleep for 20 ms" << endl;
  cout << duration_cast<nanoseconds>(time1 - time0).count() << " nanoseconds passed\n";
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

