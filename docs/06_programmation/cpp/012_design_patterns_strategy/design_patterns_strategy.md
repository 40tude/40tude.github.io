---
layout: default
title: "C++ Design Patterns - Strategy"
parent: "C++"
#math: mathjax
date: 2014-09-30 22:57:00
last_modified_date: 2020-05-03 23:32:42
---

# C++ Design Patterns - Strategy

## Introduction
Je suis en train de relire [Head First Design Patterns](http://www.amazon.fr/First-Design-Patterns-Elisabeth-Freeman/dp/0596007124).

Ci-dessous vous trouverez ma version du Strategy Pattern en C++(Chapitre 1, page 1). Je ne clame pas que ce soit la meilleure implémentation mais au moins elle fonctionne (apparemment) et je la comprends. Ce n'est déjà pas si mal...

Contrairement à d'autres code que j'ai pu trouver sur le Web, j'essaie de tout mettre dans un seul code source afin de faciliter la lecture et la compréhension. Ensuite, bien sûr, libre à vous d'exploser tout ça dans différents fichiers...

Pour information, je suis sous Visual Studio Express 2013. Pour utiliser le code ci-dessous, il faut :

1. Commencer par le copier le code
2. Dans MSVC, créer un projet de type console.
3. Dans l'assistant, bien penser à créer un projet vide.
4. Ajouter un fichier source au projet et y coller ce que vous avez copié
5. Enfin, F5 et ça doit rouler.

Pour voir les sorties du traceur de fuites mémoire faut aller dans View/Output (ALT+2 pour les intimes). Si vous êtes dans Code::Block ou sous Linux avec clang++ le source devrait passer de la même manière.







## Ce que j'ai retenu

Plus tard, je vais mettre ici mes notes à propos du Design Patterns en question. Pour l'instant, il n'y a rien car j'ai collé mon code source et zou, je suis passé à autre chose.




## Le code

```cpp
#ifdef _MSC_VER
  #define _CRTDBG_MAP_ALLOC
  #include <crtdbg.h>
#endif

#include <iostream>
#include <memory>

using namespace std;

// ----------------------------------------------------------------------------
class FlyBehaviorInterface {
public:
    virtual void Fly() const = 0;

  // DONE : Don't fully understand = 0 AND the inline definition on the same line
  // See : http://www.studytonight.com/cpp/virtual-destructors.php
  // Pure Virtual Destructors are legal in C++. Also, pure virtual Destructors must
  // be defined, which is against the pure virtual behaviour. The only difference
  // between Virtual and Pure Virtual Destructor is, that pure virtual destructor
  // will make its Base class Abstract, hence you cannot create object of that class.
  // There is no requirement of implementing pure virtual destructors in the derived classes.

  // See : http://stackoverflow.com/questions/4998344/why-must-a-pure-virtual-destructors-implementation-be-empty-and-should-it-be-i
  // You need implementation of pure virtual destructor because, the way virtual
  // destructor works is that the most derived class's destructor is called first,
  // then the destructor of each base class is called. It means compilers will generate
  // a call to base class pure virtual destructor even though the class is abstract,
  // so you must provide a body for the function. If you don't provide body, the linker
  // will complain about a missing symbol.
  // There might be a case, when you want your base class to be abstract even if your
  // class does not contain any pure virtual function.Here declaring pure virtual destructor
  // will make your class abstract.In this case, your destructor can have empty body.
  // To avoid paying overhead cost of a call to an empty body destructor, declare it as inline.
  virtual ~FlyBehaviorInterface() = 0 {};
};

// Default constructor calls ~FlyBehaviorInterface
class FlyWithWings :public FlyBehaviorInterface {
public:
    void Fly() const {
        cout << "I'm Flying!" << endl;
    }
};

class FlyNoWay :public FlyBehaviorInterface {
public:
    void Fly() const {
        cout << "I can't Fly" << endl;
    }
};

// ----------------------------------------------------------------------------
class QuackBehaviorInterface {
public:
    virtual void Quack() const = 0;
  virtual ~QuackBehaviorInterface() = 0 {};
};

class QuackReal :public QuackBehaviorInterface {
public:
    void Quack() const {
        cout << "Quack" << endl;
    }
};

class QuackSqueak : public QuackBehaviorInterface {
public:
    void Quack() const {
        cout << "Squeak" << endl;
    }
};

// ----------------------------------------------------------------------------
class Duck {
private:
    unique_ptr<FlyBehaviorInterface>    mFlyBehavior;
    unique_ptr<QuackBehaviorInterface>    mQuackBehavior;

public:
    explicit Duck(FlyBehaviorInterface *FlyBehavior, QuackBehaviorInterface *QuackBehavior) : mFlyBehavior{ FlyBehavior }, mQuackBehavior{ QuackBehavior } { }

    virtual void Display() const = 0;

  virtual ~Duck() = 0 {}

    void PerformFly() {
        // Delegate to the fly behavior class
        mFlyBehavior->Fly();
    }

    void PerformQuack() {
        // Delegate to the quack behavior class
        mQuackBehavior->Quack();
    }

    void Swim() {
        cout << "All ducks float, even decoys!" << endl;
    }

    // Change Fly behavior dynamically - on Fly :-)
    void SetFlyBehavior(FlyBehaviorInterface *fb) {
        mFlyBehavior = unique_ptr<FlyBehaviorInterface>(fb);
    }
};

class DuckMallard :public Duck {
public:
    DuckMallard() : Duck(new FlyWithWings, new QuackReal)   {   }

    void Display() const {
        cout << "I'm a real Mallard duck" << endl;
    }
};

class DuckRubber :public Duck {
public:
    DuckRubber() : Duck(new FlyNoWay, new QuackSqueak) { }

    void Display() const {
        cout << "I'm a yellow rubber duck" << endl;
    }
};

// ----------------------------------------------------------------------------
static void Test(void){

    DuckMallard mallard;
    mallard.Display();
    mallard.PerformQuack();
    mallard.PerformFly();
    mallard.Swim();

    cout << endl;

    DuckRubber duckie;
    duckie.Display();
    duckie.PerformQuack();
    duckie.PerformFly();
    duckie.Swim();
    duckie.SetFlyBehavior(new FlyWithWings);
    duckie.PerformFly();
}

// ----------------------------------------------------------------------------
int main(){

#ifdef _MSC_VER
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
#endif

    Test(); // Putting all code in Test that way, allow to check memory leaks

#ifdef _MSC_VER
    _CrtMemDumpAllObjectsSince(NULL); // Begins the dump from the start of program execution
    _CrtDumpMemoryLeaks();
#endif

    cout << endl << "Strike ENTER to exit : ";
    cin.ignore((numeric_limits<streamsize>::max)(), '\n');
}
```

Je ne promets rien mais l'idée est de mettre les autres Design Patterns du livre [Head First Design Patterns](http://www.amazon.fr/First-Design-Patterns-Elisabeth-Freeman/dp/0596007124) au fur et à mesure de ma relecture et de ma compréhension. Cela dit j'ai un boulot le reste du temps et donc pas d'engagement sur le rythme des publications. En plus je prends toujours un temps infini à lire, à relire et à tester un code avant de le copier/coller sur ce site.

*La suite au prochain épisode...*

