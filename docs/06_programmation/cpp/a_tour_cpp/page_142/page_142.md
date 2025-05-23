---
layout: default
title: "A tour of C++ – Exemple page 142"
parent: "A tour of C++"
nav_order: 142
#math: mathjax
date: 2015-08-04 15:48:57
last_modified_date: 2020-05-02 23:07:10
---

# A Tour of C++ – Exemple page 142

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


Rien de bien compliqué encore à ce stade mais je sens que ça va venir. Bien comprendre la différence entre une tâche (qui peut potentiellement s'exécuter en parallèle à d'autres tâches) et un thread qui est la représentation au niveau système d'une tâche dans un programme. Garder en tête que les thread s'exécutent dans le même espace mémoire (ce n'est pas le cas des process qui s'exécutent dans des espaces mémoire différents.



## Le code source

```
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER

#include <iostream>
#include <thread>

using namespace std;

// ----------------------------------------------------------------------------
void f(){                                                                       // A task can be a function
  cout << "Hello ";                                                             // Error! Access to cout object is not synchronized!
}

// ----------------------------------------------------------------------------
struct F {                                                                      // A task can be a function object
  void operator()(){ cout << "Parallel World!\n"; }                             // F’s call operator (see §5.5 p64). Please note access to cout object is not synchronized
};

// ----------------------------------------------------------------------------
void Test(void) {

  thread t1{ f };                                                               // The task f() is launched by constructing a thread with the task as argument
  thread t2{ F() };                                                             // The task f() executes in one thread and the task F() in an other thread
  t1.join();                                                                    // wait for t1
  t2.join();                                                                    // wait for t2
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

