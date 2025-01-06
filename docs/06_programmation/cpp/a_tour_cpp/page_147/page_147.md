---
layout: default
title: "A tour of C++ – Exemple page 147"
parent: "A tour of C++"
nav_order: 147
#math: mathjax
date: 2015-08-04 16:01:59
last_modified_date: 2020-05-02 23:02:40
---

# A Tour of C++ – Exemple page 147

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


**ATTENTION** : Mi Août 2015 - Il semble qu'il y a des fuites mémoire quand j’exécute se code avec MSVC 2015. Je ne sais plus si il y en avait avec MSVC 2013. Je suis en train de faire des test. J'ai essayé d'autre code, d'autre personnes. Je pense que c'est soit une fuite dans la STL ou bien une erreur de ma part dans la façon de tracker les fuites.
Bien comprendre que le thread principal s'endort pour n secondes et que pendant ce temps les 2 threads `producer()` et `consumer()` travaillent en parallèle. Il y a quelques modifications par rapport au code initial car je souhaitais avoir un truc qui bouge à l'écran.



## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <queue>
#include <condition_variable>
#include <mutex>
#include <thread>
using namespace std;
using namespace std::chrono;                                                    // see §11.4
bool gRun       = true;
struct Message {                                                                // object to be communicated
  int Index;
};
queue<Message>      gQueue;                                                     // the queue of messages
condition_variable  gCond;                                                      // the variable communicating events
mutex               gMutex;                                                     // the locking mechanism
// ----------------------------------------------------------------------------
void producer(){
  static int count = 0;
  while (gRun) {
    //this_thread::sleep_for(seconds{ 1 });                                     // may be used to generate a message every second
    Message m;
    m.Index = ++count;
    cout << "producing index : " << m.Index << endl;

    unique_lock<mutex> lock{ gMutex };                                          // protect operations
    gQueue.push(m);
    gCond.notify_one();                                                         // notify
  }                                                                             // release lock (at end of scope)
}
// ----------------------------------------------------------------------------
void consumer(){
  while (gRun) {
    unique_lock<mutex> lock{ gMutex };                                          // acquire gMutex
    gCond.wait(lock);                                                           // re-acquire lock upon wakeup

    while (!gQueue.empty()){                                                    // get one or more messages
      auto m = gQueue.front();
      cout << "consuming index : " << m.Index << endl;
      gQueue.pop();                                                             // Removes an element from the front of the queue
    }
    lock.unlock();                                                              // release lock
  }
}
// ----------------------------------------------------------------------------
void Test(void) {
  thread t1{ producer };
  thread t2{ consumer };

  this_thread::sleep_for(seconds{ 10 });
  cout << "Test finished to wait" << endl;
  gRun = false;
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

