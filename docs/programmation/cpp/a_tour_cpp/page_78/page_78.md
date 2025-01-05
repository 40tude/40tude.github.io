---
layout: default
title: "A tour of C++ – Exemple page 78"
parent: "A tour of C++"
#math: mathjax
date: 2014-12-24 10:33:00
last_modified_date: 2020-05-02 23:19:01
---

# A Tour of C++ – Exemple page 78

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


Couvre les codes des pages 78-83. J'ai eu pas mal de soucis avec le R"((\ew+))" (p83 en bas) qui passe pas dans MSVC ni dans IdeOne. Au final j'ai une expression régulière qui fonctionne dans MSVC mais pas dans IdeOne (bizarre cette histoire). Vérification faite il s'agit d'une erreur de l'édition de Septembre 2013. L'expression que j'utilise se retrouve dans l'édition Février 2014.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <regex>
#include <iostream>
using namespace std;
// ----------------------------------------------------------------------------
void use_p78(){
  regex pat(R"(\w{2}\s*\d{5}(-\d{4})?)");                                       // US postal code pattern: XXddddd-dddd and variants
  vector<string> source{ "US postal code pattern: XXddddd-dddd and variants", "TX 77845", "TX       77845", "TX       77845-12345678", "TX77845-1234", "DC 20500-0001" };
  int lineno = 0;
  //for (string line; getline(cin, line);) {                                    // read into line buffer
  for (auto line : source){
    ++lineno;
    smatch matches;                                                             // matched strings go here
    if (regex_search(line, matches, pat))                                       // search for pat in line
      cout << lineno << ": " << matches[0] << '\n';
  }
}
// ----------------------------------------------------------------------------
void use_p79() {
  vector<string> source{ "US postal code pattern: XXddddd-dddd and variants", "TX 77845", "TX       77845", "TX       77845-12345678", "TX77845-1234", "DC 20500-0001" };
  regex pat{ R"(\w{2}\s*\d{5}(-\d{4})?)" };                                     // U.S. postal code pattern
  int lineno = 0;
  //for (string line; getline(in, line);) {
  for (auto line : source) {
    ++lineno;
    smatch matches;                                                             // matched strings go here
    if (regex_search(line, matches, pat)) {
      cout << lineno << ": The complete match  : " << matches[0] << '\n';
      if (1<matches.size() && matches[1].matched)
        cout << " : Subpattern : " << matches[1] << '\n';
    }
  }
}
// ----------------------------------------------------------------------------
bool is_identifier(const string& s){                                            // see p 82
  regex pat{ R"([_[:alpha:]]\w*)" };                                            // underscore or letter
                                                                                // followed by zero or more underscore, letters or digits
  return regex_match(s, pat);
}
// ----------------------------------------------------------------------------
void Test(void) {
  cout << "Using use_p78()" << endl;
  use_p78();
  cout << "\nUsing use_p79()" << endl;
  use_p79();
  cout << "\nCheck valid identifier" << endl;
  if (is_identifier(string("MyFunctionCall")))
    cout << "MyFunctionCall is a valid identifier" << endl;
  else
    cout << "MyFunctionCall is not a valid identifier" << endl;
  cout << "\nOutput white space separated words   : " << endl;                  // see p83
  string input1 = "aa as; asd ++eˆasdf asdfg";
  regex pat1{ R"(\s+(\w+))" };                                                  // output white space separated words
  for (sregex_iterator p(input1.begin(), input1.end(), pat1); p != sregex_iterator{}; ++p)
    cout << (*p)[1] << '\n';
  cout << "\nOutput white space separated words 2 : " << endl;
  regex pat2{ R"((\w+))" };                                                     // do not forget the very first "aa"
  for (sregex_iterator p(input1.begin(), input1.end(), pat2); p != sregex_iterator{}; ++p)
    cout << (*p)[1] << '\n';
}
// ----------------------------------------------------------------------------
int main() {
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
```
