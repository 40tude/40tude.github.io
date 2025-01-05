---
layout: default
title: "A Tour of C++ – Exemple page 16"
parent: "A tour of C++"
#math: mathjax
date: 2014-11-23 01:43:34
last_modified_date: 2020-05-02 23:26:12
---

# A Tour of C++ – Exemple page 16

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


Faut juste penser à lire les commentaires ainsi que les paragraphes qui sont autour du source dans le livre. On peut cependant remarquer que comme indiqué dans le livre, si il y a un new (ligne 14) il n'y a pas de delete. Bref, il y a une fuite mémoire dans le code et c'est pas une bonne chose. Ce problème est résolu dans §4.2.2 et dans le code de la page 37.

## Le code source

```cpp
#include <iostream>
#include <limits>
using namespace std;
// ----------------------------------------------------------------------------
struct Vector {
  int     sz;                                                                   // number of elements
  double  *elem;                                                                // pointer to elements
};
// ----------------------------------------------------------------------------
void vector_init(Vector& v, int s){
  v.elem = new double[s];                                                       // allocate an array of s doubles
  v.sz = s;
}
// ----------------------------------------------------------------------------
double read_and_sum(int s){                                                     // reads integers from cin and return their sum; s is assumed to be positive
  Vector v;
  vector_init(v, s);                                                            // allocate s elements for v

  cout << "Enter " << s << " integers separated with one or more spaces : ";
  for (int i = 0; i != s; ++i)
    cin >> v.elem[i];                                                           // read into elements

  double sum = 0;
  for (int i = 0; i != s; ++i)
    sum += v.elem[i];                                                           // take the sum of the elements
  return sum;
}
// ----------------------------------------------------------------------------
int main(){

  cout << "The sum of the 3 integers is : " << read_and_sum(3) << endl;
  cin.sync();
  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
