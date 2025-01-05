---
layout: default
title: "A tour of C++ – Exemple page 109"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 15:08:46
last_modified_date: 2020-05-02 23:14:04
---

# A Tour of C++ – Exemple page 109

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


Rien de bien compliqué. Bien lire les paragraphes du livre cependant. Le code de la fonction Test() est un peu lourd car on fait en sorte d'avoir des sorties consoles qui aident à comprendre ce qui se passe dans le code.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <vector>
#include <string>
#include <list>
using namespace std;
// ----------------------------------------------------------------------------
bool has_c1(const string& s, char c){                                           // does string s contain the character c?
  auto p = find(s.begin(), s.end(), c);
  if (p != s.end())
    return true;
  else
    return false;
}
// ----------------------------------------------------------------------------
bool has_c2(const string& s, char c){                                           // does string s contain the character c?
  return find(s.begin(), s.end(), c) != s.end();
}
// ----------------------------------------------------------------------------
vector<string::iterator> Find_All(string& s, char c){                           // find all occurrences of c in s
  vector<string::iterator> res;
  for (auto p = s.begin(); p != s.end(); ++p)
    if (*p == c)
      res.push_back(p);

  return res;
}
// ----------------------------------------------------------------------------
template<typename T>
using Iterator = typename T::iterator;                                          // T’s iterator
template<typename C, typename V>
vector<Iterator<C>> find_all(C& c, V v){                                        // find all occurrences of v in c
  vector<Iterator<C>> res;
  for (auto p = c.begin(); p != c.end(); ++p)
    if (*p == v)
      res.push_back(p);
  return res;
}
// ----------------------------------------------------------------------------
void Test(void) {

  auto Bob = has_c1("alibaba", 'a');
  if (Bob)
    cout << "There is at least one 'a' in 'Alibaba'" << endl;
  else
    cout << "There is a bug in has_c1()" << endl;
  Bob = has_c2("alibaba", 'a');
  if (Bob)
    cout << "There is at least one 'a' in 'Alibaba'" << endl;
  else
    cout << "There is a bug in has_c2()" << endl;
  cout << endl;
  string m1 { "Mary had a little lamb" };                                       // Use the "genuine" version
  auto count1 = 0;
  for (auto p : Find_All(m1, 'a'))
    if (*p != 'a')
      cerr << "A bug!\n";
    else
      ++count1;
  cout << "There are " << count1 << " 'a' in 'Mary had a little lamb'" << endl;

  string m2 { "Mary had a little lamb" };                                       // Now it use the templated version
  auto count2 = 0;
  for (auto p : find_all(m2, 'a'))                                              // p is a string::iterator
    if (*p != 'a')
      cerr << "string bug!\n";
    else
      ++count2;
  cout << "There are " << count2 << " 'a' in 'Mary had a little lamb'" << endl;
  cout << endl;
  list<double> ld{ 1.1, 2.2, 3.3, 1.1 };
  auto count3 = 0;
  for (auto p : find_all(ld, 1.1))
    if (*p != 1.1)
      cerr << "list bug!\n";
    else
      ++count3;
  cout << "There are " << count3 << " values equal to 1.1 in the list" << endl;
  vector<string> vs{ "red", "blue", "green", "green", "orange", "green" };
  cout << "\nInitial list :" << endl;
  for (const auto &p : vs) cout << p << endl;
  cout << endl;
  auto count4 = 0;
  for (auto p : find_all(vs, "red"))
    if (*p != "red")
      cerr << "vector bug!\n";
    else
      ++count4;
  cout << "There are " << count4 << " red in the vector" << endl;
  cout << endl;

  for (auto p : find_all(vs, "green"))
    *p = "vert";
  cout << "After replacing 'green' by 'vert' in the list :" << endl;
  for (const auto &p : vs) cout << p << endl;
  cout << "\n";
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
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

