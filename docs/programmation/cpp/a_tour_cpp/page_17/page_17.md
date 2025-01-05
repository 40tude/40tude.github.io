---
layout: default
title: "A Tour of C++ – Exemple page 017"
parent: "A tour of C++"
nav_order: 17
#math: mathjax
date: 2014-11-23 02:09:50
last_modified_date: 2020-05-02 23:25:41
---

# A Tour of C++ – Exemple page 17

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


J'ai rajouté du code car en fait il y a des fuites mémoire dans le code. Afin de les faire ressortir j'utilise crtdbg.h et les fonctions qui vont bien (_CrtDumpMemoryLeaks par exemple). Ceci dit, comme je souhaite que le code soit portable (sous [ideone.com](http://ideone.com/) par exemple) il faut que j'encadre tout ça par des #ifdef _MSC_VER. Rien de très complexe mais bon, à la lecture, c'est moins cool.
Ceci dit, à la fin de l’exécution, dans Visual Studio, voilà ce que je vois dans la fenêtre Output :
```bash
Detected memory leaks!
Dumping objects ->
{169} normal block at 0x0067C538, 24 bytes long.
 Data: <       ?       @> 00 00 00 00 00 00 F0 3F 00 00 00 00 00 00 14 40
Object dump complete.
Detected memory leaks!
Dumping objects ->
{169} normal block at 0x0067C538, 24 bytes long.
 Data: <       ?       @> 00 00 00 00 00 00 F0 3F 00 00 00 00 00 00 14 40
Object dump complete.
The program '[10516] p017.exe' has exited with code 0 (0x0).
```

Si jamais vous copiez/collez le code ci-dessous dans Ideone.com pensez bien à faire la saisie des entiers dans l'onglet "stdin" **AVANT** de lancer le code.
Par exemple si vous saisissez "1 5 9" voilà ce que cela donne de mon côté :
[![p017](https://www.40tude.fr/wp-content/uploads/2014/11/p017.png)](https://www.40tude.fr/wp-content/uploads/2014/11/p017.png)
A l'exécution, Ideone ne détecte pas la fuite mémoire mais bon c'est pas très grave.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <limits>
#include <vector>
using namespace std;
class Vector {
private:
  double  *elem;                                                                // pointer to the elements
  int     sz;                                                                   // the number of elements
public:
  Vector(int s) : elem {new double[s]}, sz {s} {}                               // construct a Vector
  double& operator[](int i) { return elem[i]; }                                 // element access: subscripting
  int size() { return sz; }
};
// ----------------------------------------------------------------------------
double read_and_sum(int s) {
  Vector v(s);                                                                  // make a vector of s elements
  cout << "Enter " << s << " integers separated with spaces : ";
  for (int i = 0; i!=v.size(); ++i)
    cin>>v[i];                                                                  // read into elements
  double sum = 0;
  for (int i = 0; i!=v.size(); ++i)
    sum += v[i];                                                                // take the sum of the elements
  return sum;
}
// ----------------------------------------------------------------------------
int main() {
#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF|_CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
  cout<<"Sum of 3 is : "<<read_and_sum(3)<<endl;
  cin.sync();
#ifdef _MSC_VER
  //_CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER
  cout<<"Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
