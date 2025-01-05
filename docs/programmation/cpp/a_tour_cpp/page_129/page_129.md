---
layout: default
title: "A tour of C++ – Exemple page 129"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 15:40:51
last_modified_date: 2020-05-02 23:09:51
---

# A Tour of C++ – Exemple page 129

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


En ce qui me concerne, à ce jour, j'avoue que si je comprend ce qui est écrit dans le bouquin, je ne me sens pas capable, loin de là, d'écrire ce type de code demain matin.

Afin de débrouiller les choses, il faut garder en tête que le but du jeu c'est ce qu'on a aux lignes 54 et 55. Un code simple qui applique le `sort()` uniformément à une liste ou un vecteur. Autrement dit, il faut faire en sorte que selon que l'on passe un vecteur ou une liste soit on appelle une version de la fonction `sort()` soit on en appelle une autre.

C'est pour cette raison que l'on voit aux lignes 16 et 26 une fonction `sort()` qui utilise un `random_access_iterator_tag` et une autre qui utilise un `forward_iterator_tag`.

Quoiqu'il en soit, l'orientation vers une fonction ou une autre, se fait via le template de la fonction `sort()` (ligne 44). C'est évident mais il faut bien remarquer que ce que l'on passe au template sort() c'est le type : vector ou forward_list. Le truc un peu magique, c'est comment à partir du type de donnée passé, on va être capable de retrouver le type d'itérateur utiliser, et donc être capable de faire la distinction entre les fonctions à appeler.

Là, il y a un jeu de poupées gigognes qui s'opère avec tout un ensemble de `using`. Il est à remarquer que les using en question se retrouvent au sein de templates car la signification du using dépend du type passé. C'est ce que l'on peut observer à la ligne 35 par exemple :

```cpp
// ----------------------------------------------------------------------------
template<typename C>
using Iterator_type = typename C::iterator;                                     // C’s iterator type
```

Dans le fragment de code ci-dessus il faut bien voir que C peut être de type `vector` ou de type `forward_list`.

Quand on arrive a dépiler les poupées (les templates et les using) on arrive à retomber sur nos pattes mais bon c'est pas simple à relire.

En fait je pense que le truc les plus important c'est d'écrire ce que l'on souhaite voir dans le source (la fonction `RealTest()` et les 2 appels à `sort()` par exemple) puis ensuite cacher la misère dans des templates tout en sachant que la technique du tag dispatch existe, qu'on a à notre disposition les moyen de retrouver le type d'itérateur concerné etc.

Ne pas louper non plus les lignes 23 et 30 qui montrent comment, dans un template, à partir d'un iterateur, on peut retrouver le type des données sur lesquelles il pointe avec `Value_type`.




## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER

#include <iostream>
#include <vector>
#include <forward_list>
#include <string>
#include <algorithm>
#include <iterator>

using namespace std;

// ----------------------------------------------------------------------------
template<typename Ran>                                                          // for random-access iterators
void sort_helper(Ran beg, Ran end, random_access_iterator_tag){                 // we can subscript into [beg:end)

  sort(beg, end);                                                               // just sort it
}

// ----------------------------------------------------------------------------
template<typename C>
using Value_type = typename C::value_type;                                      // C’s value type

// ----------------------------------------------------------------------------
template<typename For>                                                          // for forward iterators
void sort_helper(For beg, For end, forward_iterator_tag){                       // we can traverse [beg:end)

  vector<Value_type<For>> v{ beg, end };                                        // initialize a vector from [beg:end)
  sort(v.begin(), v.end());
  copy(v.begin(), v.end(), beg);                                                // copy the elements back
}

// ----------------------------------------------------------------------------
template<typename C>
using Iterator_type = typename C::iterator;                                     // C’s iterator type

// ----------------------------------------------------------------------------
template<typename Iter>
using Iterator_category = typename std::iterator_traits<Iter>::iterator_category; // Iter’s category

// ----------------------------------------------------------------------------
template<typename C>
void sort(C& c){

  using Iter = Iterator_type<C>;
  sort_helper(c.begin(), c.end(), Iterator_category<Iter>{});                   // Iterator_category<Iter>{} constructs a "tag" value inidicating the kind of iterator provided
                                                                                // std::random_access_iterator_tag or std::forward_iterator_tag
}

// ----------------------------------------------------------------------------
void RealTest(vector<string>& v, forward_list<int>& lst){

  sort(v);                                                                      // sort the vector
  sort(lst);                                                                    // sort the singly-linked list
}

// ----------------------------------------------------------------------------
void Test(void) {

  vector<string>    myvector  { "riri", "fifi", "loulou" };
  forward_list<int> mylist    { 3, 2, 1 };

  cout << "Before RealTest() :" << endl;
  for (const auto& s : myvector)
    cout << s << endl;

  for (const auto& n : mylist)
    cout << n << endl;

  RealTest(myvector, mylist);

  cout << "\nAfter RealTest() :" << endl;
  for (const auto& s : myvector)
    cout << s << endl;

  for (const auto& n : mylist)
    cout << n << endl;
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

