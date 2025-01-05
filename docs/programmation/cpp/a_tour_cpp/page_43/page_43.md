---
layout: default
title: "A tour of C++ – >Exemple page 43"
parent: "A tour of C++"
nav_order: 43
#math: mathjax
date: 2014-12-15 06:43:37
last_modified_date: 2020-05-02 23:22:08
---

# A Tour of C++ – Exemple page 43

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


J'ai un peu ratatouillé avant que le code compile, link et démarre. En fait la compilation passait mais pas le link. J'ai été mieux supporté sous IdeOne que sous MSVC. En fait j'avais un peu oublié de définir les fonction Smiley wink(), move() et rotate(). Rien de très grave mais bon les messages sous MSVC étaient pas franchement "friendly" alors que sous IdeOne les messages émis par Clang étaient vraiment utiles. Enfin bref, encore une fois c'est toujours une très bonne idée que de faire passer ses sources dans différents compilateurs et éditeur de liens.
Bien voir que j'implémente les sources qui sont dispos sur les pages 43-47. Il y a donc une classe Smiley qui utilise l'explicit overriding, un dynamique_cast dans la fonction Test(). Ceci dit, attention, je n'ai pas implémenté la fonction read_shape(). C'est ce que l'on peut voir dans la task list de visual studio ou en haut du code.



## Le code source

```cpp
                                                                                // TODO : immplement read_shape() using unique_ptr. see p 48
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>

#define _SCL_SECURE_NO_WARNINGS                                                 // Avoid error with copy(...) in the initializer list ctor
#endif // _MSC_VER

#include <iostream>
#include <vector>
#include <limits>

using namespace std;

class Point {
  double x;
  double y;
};

class Shape {
public:
  virtual Point center() const = 0;                                             // pure virtual
  virtual void move(Point to) = 0;
  virtual void draw() const = 0;                                                // draw on current "Canvas"
  virtual void rotate(int angle) = 0;
  virtual ~Shape() {}                                                           // destructor
  // ...
};

class Circle: public Shape {
public:
  Circle(Point p, int rr) : x {p}, r {rr} {}                                    // constructor
  Point center() const { return x; }
  void move(Point to) { x = to; }
  void draw() const {};
  void rotate(int) {}                                                           // nice simple algorithm
private:
  Point x;                                                                      // center
  int r;                                                                        // radius
};

class Smiley: public Circle {                                                   // use the circle as the base for a face
public:
  Smiley(Point p, int r) : Circle {p, r}, mouth {nullptr} {}
  ~Smiley() {
    delete mouth;
    for (auto p:eyes)
      delete p;
  }
  void move(Point to) override;                                                 // Explicit overriding
  void draw() const override;
  void rotate(int) override;

  void add_eye(Shape* s) { eyes.push_back(s); }
  void set_mouth(Shape *s);
  virtual void wink(int i);                                                     // wink eye number i
  // ...
private:
  vector<Shape*> eyes;                                                          // usually two eyes
  Shape* mouth;
};

void Smiley::draw() const {

  Circle::draw();
  for (auto p:eyes) p->draw();
  mouth->draw();
}

void Smiley::move(Point to){
  cout << "Move" << endl;
}

void Smiley::rotate(int i){
  cout << "Rotate" << endl;
}

void Smiley::wink(int i){
  cout << "wink eye N° " << i << endl;
}

void rotate_all(vector<Shape*>& v, int angle){                                  // rotate v's elements by angle degrees
  for (auto p:v) p->rotate(angle);
}

// ----------------------------------------------------------------------------
void Test(void) {

  Smiley MySmiley {Point(), 5};
  Shape* ps = &MySmiley;                                                        // see p 47

  if (Smiley* p = dynamic_cast<Smiley*>(ps)) {
    cout<<"This is a smiley"<<endl;                                             // ... is the Smiley pointer to by p ...
  }  else {
    cout<<"This is NOT a smiley"<<endl;                                         // ... not a Smiley, try something else ...
  }
}

// ----------------------------------------------------------------------------
int main() {

#ifdef _MSC_VER
  _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif // _MSC_VER

  Test();

#ifdef _MSC_VER
  //_CrtMemDumpAllObjectsSince(NULL);                                             // Begins the dump from the start of program execution
  _CrtDumpMemoryLeaks();
#endif // _MSC_VER

  cout << "Press ENTER to quit : ";
  cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```
