---
layout: default
title: "A tour of C++ – Exemple page 35"
parent: "A tour of C++"
nav_order: 35
#math: mathjax
date: 2014-12-14 19:30:04
last_modified_date: 2020-05-02 23:23:48
---

# A Tour of C++ – Exemple page 35

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


Bon... Là le mieux, c'est de poser un point d'arrêt à la ligne 90 (première ligne de la fonction Test(void)) et de regarder "pas à pas" ce qui se passe vraiment. J'ai rajouté des commentaires et des liens sur des référence car y avait des trucs qui n'étaient pas très clairs pour moi. J'espère que ça en aidera certains.
Pour le reste, à partir de maintenant la fonction main() appellera une fonction Test(). Le but est de simplifier la lecture du code tout en gardant la surveillance des fuites mémoire. Typiquement cela veut dire que dorénavant il suffit de se rendre directement dans la fonction Test() afin d'étudier le code.


## Le code source

```cpp
// Read : http://en.wikibooks.org/wiki/C%2B%2B_Programming/Operators/Operator_Overloading
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER
#include <iostream>
#include <limits>
using namespace std;
// ----------------------------------------------------------------------------
class complex {
private:
  double re;                                                                    // representation: two doubles
  double im;
public:
  complex(double r, double i) :re {r}, im {i} {}                                // construct complex from two scalars
  complex(double r) :re {r}, im {0} {}                                          // construct complex from one scalar
  complex() :re {0}, im {0} {}                                                  // default complex: {0,0}

  double real() const { return re; }
  void real(double d) { re = d; }
  double imag() const { return im; }
  void imag(double d) { im = d; }

                                                                                // Operators are generally overloaded as members when they:
                                                                                //    1 - change the left-hand operand, or
                                                                                //    2 - require direct access to the non-public parts of an object.
                                                                                // When an operator is defined as a member, the number of explicit parameters
                                                                                // is reduced by one, as the calling object is implicitly supplied as an operand

                                                                                // Compound assignment operators
                                                                                // Compound assignment operators should be overloaded as member functions, as
                                                                                // they change the left-hand operand
  complex& operator+= (complex z) { re += z.re, im += z.im; return *this; }     // add to re and im and return the result
  complex& operator-= (complex z) { re -= z.re, im -= z.im; return *this; }
  complex& operator*= (complex rhs);                                            // defined out-of-class somewhere
  complex& operator/= (complex);                                                // defined out-of-class somewhere
};
                                                                                // Don't need access to private data and don't change the lhs so there are defined as non member functions
                                                                                // argument passed by value, so one can modify an argument without affecting the caller’s copy
complex operator+(complex a, complex b) { return a += b; }
complex operator-(complex a, complex b) { return a -= b; }
complex operator-(complex a) { return {-a.real(), -a.imag()}; }                 // unary minus
complex operator*(complex a, complex b) { return a *= b; }
complex operator/(complex a, complex b) { return a /= b; }
// ----------------------------------------------------------------------------
bool operator== (complex a, complex b){                                         // equal
  return a.real()==b.real() && a.imag()==b.imag();
}
// ----------------------------------------------------------------------------
bool operator!= (complex a, complex b){                                         // not equal
  return !(a==b);
}
// ----------------------------------------------------------------------------
complex& complex::operator*=(complex rhs) {
  double tmp_re = this->re * rhs.re - this->im * rhs.im;
  double tmp_im = this->re * rhs.im + this->im * rhs.re;

  this->re = tmp_re;
  this->im = tmp_im;
  return *this;
}
// ----------------------------------------------------------------------------
complex& complex::operator/=(complex rhs) {
  double tmp_re = this->re * rhs.re + this->im * rhs.im;
  double tmp_im = this->im * rhs.re - this->re * rhs.im;
  double tmp = rhs.re * rhs.re + rhs.im * rhs.im;
  this->re = tmp_re/tmp;
  this->im = tmp_im/tmp;
  return *this;
}
// ----------------------------------------------------------------------------
void Test(void) {
  complex a {1.4, 0.4};                                                         // set a breakpoint here and trace inside the code step by step
  complex b {1/a};
  complex c {0.8, 0.8};
  complex d = a/c;
  complex e {a+b*complex {1, 2.3}};

  if (c!=b)
    c = -(b/a)+2*b;
}
// ----------------------------------------------------------------------------
int main() {
#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER
  Test();
#ifdef _MSC_VER
  _CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER
  cout<<"Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
