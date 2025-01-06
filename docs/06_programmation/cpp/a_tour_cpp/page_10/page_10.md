---
layout: default
title: "A Tour of C++ – Exemple page 010"
parent: "A tour of C++"
nav_order: 10
#math: mathjax
date: 2014-11-23 01:35:46
last_modified_date: 2020-05-02 23:27:15
---

# A Tour of C++ – Exemple page 10

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


Rien de particulier. Faut juste penser à lire les commentaires ainsi que les paragraphes qui sont autour du source dans le livre.

## Le code source

```cpp
#include <iostream>
#include <limits>
using namespace std;
// ----------------------------------------------------------------------------
void copy_fct(){
    int v1[10] {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};                                    // v1 is an array of 10 integers intialized with values from 0 to 9
    int v2[10];                                                                   // to become a copy of v1
    for (auto x : v1)                                                             // for each x in v
        cout << x << '\n';
    cout << endl;

    for (auto i = 0; i != 10; ++i)                                                // copy elements
        v2[i] = v1[i];

    for (auto x : v2)
        cout << x << '\n';
    cout << endl;
}
// ----------------------------------------------------------------------------
void print(){
    int v[] {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

    for (auto x : v)
        cout << x << '\n';
    cout <<endl;
    for (auto x : {10, 21, 32, 43, 54, 65})
        cout << x << '\n';
    cout << endl;
}
// ----------------------------------------------------------------------------
void increment(){
    int v[] {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};

    for (auto x : v)
        cout << x << '\n';
    cout << endl;
    for (auto& x : v)                                                             // x is a reference to an object whose type is automatically deduced
        ++x;

    for (const auto& x : v)                                                       // x is a read only reference to an object whose type is automatically deduced
        cout << x << '\n';
    cout << endl;
}
// ----------------------------------------------------------------------------
int main(){
    copy_fct();
    print();
    increment();
    cout << "Press ENTER to quit : ";
    cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
