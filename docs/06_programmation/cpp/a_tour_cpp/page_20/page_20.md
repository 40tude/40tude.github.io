---
layout: default
title: "A Tour of C++ – Exemple page 020"
parent: "A tour of C++"
nav_order: 20
#math: mathjax
date: 2014-12-01 23:52:26
last_modified_date: 2020-05-02 23:25:10
---

# A Tour of C++ – Exemple page 20

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


J'ai gardé les instructions qui permettent de surveiller les fuites mémoires. Ici ce n'est pas vraiment indispensable (en plus le code est un peu lourd et c'est pas très beau). Ceci dit, maintenant qu'on sait comment lire les lignes _CrtDumpMemoryLeaks et autres et autant les utiliser. En cas de problème d'allocation mémoire on sera prévenu. Si vous ne voyez pas de quoi je parle allez faire un tour sur [l'exemple de la page 17](https://www.40tude.fr/a-tour-of-c-exemple-de-la-page-17/ "A Tour of C++ – Exemple de la page 17").
Ici je pense que le mieux c'est de **poser un point d'arrêt sur la ligne 31** et de tracer pas à pas pour bien voir ce qui se passe.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <limits>
using namespace std;
enum class Traffic_light { green, yellow, red };
// ----------------------------------------------------------------------------
Traffic_light& operator++(Traffic_light& t){                                    // prefix increment: ++
  switch (t) {
    case Traffic_light::green:  return t = Traffic_light::yellow;
    case Traffic_light::yellow: return t = Traffic_light::red;
    case Traffic_light::red:    return t = Traffic_light::green;
  }
}
// ----------------------------------------------------------------------------
void Test(void){

  enum class Color { red, blue, green };
  Color col = Color::red;
  Traffic_light light = Traffic_light::red;
  Traffic_light next = ++light;                                                 // next becomes Traffic_light::green
}
// ----------------------------------------------------------------------------
int main(){
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
