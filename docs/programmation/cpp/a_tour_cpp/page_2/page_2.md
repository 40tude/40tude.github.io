---
layout: default
title: "A Tour of C++ - Exemple page 2"
parent: "A tour of C++"
#math: mathjax
date: 2014-11-23 01:26:50
last_modified_date: 2020-05-02 23:28:23
---

# A Tour of C++ - Exemple page 2

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

* Afin de faire ressortir le code et éviter d'avoir des sources trop long, les commentaires commencent à la colonne 81 de chaque ligne.  
* Il ne faut donc pas hésiter à mettre cette page web en plein écran et/ou à aller voir sur la droite du code source.  
* La toute première ligne est un exemple qui permet de compiler le code en ligne de commande avec le compilateur C++ de Visual Studio.  Bien sûr il faut être dans une console de type Developer Command Prompt for VS2013 (Démarrer/Visual Studio 2013/Visual Studio Tools/Developer Command Prompt for VS2013)
* Le "#include <limits>" est nécessaire à cause des 2 dernières lignes de code.
* Les lignes commentées à la toute fin du code obligent l'application à attendre que l'on tape sur ENTER avant de se terminer.
* Cela peut être utile si vous lancez l'application en double cliquant sur le nom de l'exécutable ou si vous lancez l'application depuis Visual Studio (F5). En mode Debug, au sein de l'environnement de développement ces 2 lignes ne sont pas nécessaires.
* On note qu'il n'y a pas de paramètre à la fonction main(). C'est possible mais pas obligatoire. Il n'y a pas non plus de valeur retournée par la fonction main(). Typiquement y a pas le fameux "return 0;". Si nécessaire, le compilateur le rajoute pour vous. Pour plus de détail voir : <http://en.cppreference.com/w/cpp/language/main_function>
* Hello world semble être une figure imposée et généralement on y prête pas beaucoup d'attention. N'hésitez pas à lire (relire) cet article : <http://www.stroustrup.com/new_learning.pdf>


## Le code source

```cpp
                                                                                // On Windows : cl /EHsc /nologo /W4 /MT /O2 /GL p002.cpp
                                                                                // On Mac     : clang++ -Wall -std=c++0x -stdlib=libc++ exemple149bis.cpp -o exemple149bis
#include <iostream>                                                             // includes ("import") the declarations for the I/O stream library
#include <limits>
// ----------------------------------------------------------------------------
int main(){
  std::cout << "Hello, World!\n";

  // Uncomment the 2 lines below if needed
  // std::cout << "Press ENTER to quit : ";
  // std::cin.ignore((std::numeric_limits<std::streamsize>::max)(), '\n');
}
```
