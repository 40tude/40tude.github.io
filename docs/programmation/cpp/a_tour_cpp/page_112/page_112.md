---
layout: default
title: "A tour of C++ – Exemple page 112"
parent: "A tour of C++"
#math: mathjax
date: 2015-08-04 15:18:34
last_modified_date: 2020-05-02 23:13:32
---

# A Tour of C++ – Exemple page 112

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


Afin de jouer avec les istream_iterator et ostream_iterator dans `DoSomeFileIO1()` et `DoSomeFileIO2()` on commence par créer des fichiers input1.txt et inpout2.txt. Dernier détail j'ai renommé les variables `from` et `to` par `FileNameIn` et `FileNameOut`.


## Le code source

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <string>
#include <set>
#include <vector>
#include <algorithm>
#include <fstream>
#include <iterator>
using namespace std;
// ----------------------------------------------------------------------------
int DoSomeFileIO(){

  string FileNameIn{ "input.txt" };                                             // fill input.txt file with some words
  ofstream MyFile(FileNameIn);
  string DefaultContent { "Lorem ipsum dolor sit amet, iriure disputationi ut vel, ullum velit congue cu his.\nCu nam utinam appellantur, tibique incorrupte constituam qui cu." };
  MyFile << DefaultContent << endl;
  MyFile.close();
                                                                                // see bottom of p112
  ifstream InputStream { FileNameIn };                                          // input stream for file "FileNameIn"
  istream_iterator<string> InputIterator { InputStream };                       // input iterator for stream
  istream_iterator<string> eos {};                                              // input sentinel

  string FileNameOut { "output.txt" };
  ofstream OutputStream { FileNameOut };                                        // output stream for file "output.txt"
  ostream_iterator<string> OutputIterator { OutputStream, "\n" };               // output iterator for stream. '\n' will delimit outpu values
  vector<string> Buffer { InputIterator, eos };                                 // Construct Buffer using 2 iterators
  sort(Buffer.begin(), Buffer.end());                                           // sort the Buffer
  unique_copy(Buffer.begin(), Buffer.end(), OutputIterator);                    // copy Buffer in FileNameOut discarding replicated values
  return !InputStream.eof() || !OutputStream;                                   // return error state (§1.3, §8.4)
}
// ----------------------------------------------------------------------------
int DoSomeFileIO2(){                                                            // see p113

  string FileNameIn{ "input2.txt" };                                            // fill input.txt file with some words
  ofstream MyFile(FileNameIn);
  string DefaultContent { "Lorem ipsum dolor sit amet, iriure disputationi ut vel, ullum velit congue cu his.\nCu nam utinam appellantur, tibique incorrupte constituam qui cu." };
  MyFile << DefaultContent << endl;
  MyFile.close();
                                                                                // TODO : Add more comment and/or rename variables
  ifstream InputStream { FileNameIn };                                          // input stream for file "FileNameIn"
  istream_iterator<string> InputIterator { InputStream };                       // input iterator for stream
  istream_iterator<string> eos {};                                              // input sentinel
  string FileNameOut{ "output2.txt" };
  ofstream OutputStream{ FileNameOut };                                         // output stream for file "output2.txt"
  set<string> Buffer { istream_iterator<string>{InputStream}, istream_iterator<string>{} }; // read input
  copy(Buffer.begin(), Buffer.end(), ostream_iterator<string>{OutputStream, "\n"});         // copy FileNameOut output
  return !InputStream.eof() || !OutputStream;                                   // return error state (§1.3, §8.4)
}
// ----------------------------------------------------------------------------
void Test(void) {
  ostream_iterator<string> OutputIterator { cout };                             // write strings to cout
  *OutputIterator = "Hello, ";                                                  // meaning cout<<"Hello, "
  //++OutputIterator;                                                           // TODO : need to understand. Seems useless
  *OutputIterator = "world!\n";                                                 // meaning cout<<"world!\n"
  DoSomeFileIO();
  DoSomeFileIO2();
}
// ----------------------------------------------------------------------------
int main() {
  try{
#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
    Test();
#ifdef _MSC_VER
    //_CrtMemDumpAllObjectsSince(NULL);                                         // Begins the dump FileNameIn the start of program execution
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

