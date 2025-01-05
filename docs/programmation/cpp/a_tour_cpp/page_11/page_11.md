---
layout: default
title: "A Tour of C++ – Exemple page 11"
parent: "A tour of C++"
#math: mathjax
date: 2014-11-23 01:38:35
last_modified_date: 2020-05-02 23:26:44
---

# A Tour of C++ – Exemple page 11

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


Rien de particulier. Faut juste penser à lire les commentaires ainsi que les paragraphes qui sont autour du source dans le livre.  Attention toutefois. Comme indiqué dans les commentaires il y a dans l'édition de Septembre 2013 des instructions qui ne marchent pas. Les problèmes sont corrigés dans l'édition 2014 (voir <http://www.stroustrup.com/Tour_printing2.html>)


## Le code source

```cpp
#include <iostream>
#include <limits>
using namespace std;
// ----------------------------------------------------------------------------
int count_x1(char* p, char x){                                                  // count the number of occurrences of x in p[]
                                                                                // p is assumed to point to a zero-terminated array of char (or to nothing)
    if (p == nullptr) return 0;

  int count = 0;
  //for (; p != nullptr; ++p)                                                   // In Sept 2013 printing, the for statement did'nt work for me : for (; p != nullptr; ++p)
  for (; *p != 0; ++p)                                                          // Corrected in the Sept 2014 prinitng
    if (*p == x) ++count;

  return count;
}
// ----------------------------------------------------------------------------
int count_x2(char* p, char x){                                                  // count the number of occurrences of x in p[]
                                                                                // p is assumed to point to a zero terminated array of char (or to nothing)
  if (p == nullptr) return 0;

  int count = 0;
  while (*p) {                                                                  // There was a typo in Sept 2013 printing. It was written while (p) instead of while (*p). Correction made in Sept 2014 printing.
    if (*p == x) ++count;
    ++p;
  }
  return count;
}
// ----------------------------------------------------------------------------
int main(){

  auto n1 = count_x1("alibaba", 'a');
  cout << "Number of 'a' : " << n1 << endl;

  auto n2 = count_x2("alibaba", 'a');
  cout << "Number of 'a' : " << n1 << endl;
  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
