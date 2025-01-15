---
layout: default
title: "C++ Design Patterns - Decorator"
parent: "C++"
#math: mathjax
date: 2014-10-18 19:34:49
last_modified_date: 2020-05-03 23:27:51
---

# C++ Design Patterns - Decorator




## Introduction
Je lis [Head First Design Patterns](http://www.amazon.fr/First-Design-Patterns-Elisabeth-Freeman/dp/0596007124). J'en suis au chapitre 3, page 79. Je m'attaque donc au Decorator en C++.




## Ce que j'ai retenu

Plus tard, je vais mettre ici mes notes à propos du Design Patterns en question. Là il n'y a rien car j'ai collé le code et zou, je suis passé à autre chose.









## Le code

```cpp
#ifdef _MSC_VER
  #define _CRTDBG_MAP_ALLOC
  #include <crtdbg.h>
#endif

#include <iostream>
#include <string>
#include <memory>

using namespace std;

// ----------------------------------------------------------------------------
class Beverage {
protected:
  string mDescription;

public:
  Beverage() : mDescription("Unknown Beverage") {
  }

  // Don't forget the "virtual" here
  virtual string getDescription() const {
    return mDescription;
  }

  virtual double cost() const = 0;

  // See Strategy source code for comments
  virtual ~Beverage() = 0 {}
};

// ----------------------------------------------------------------------------
class CondimentDecorator : public Beverage {
public:
  virtual string getDescription() const = 0;
  virtual ~CondimentDecorator() = 0 {}
};

// ----------------------------------------------------------------------------
class Espresso : public Beverage {
public :
  Espresso() {
    // TODO : Cannot be done in initialization list since mDescription is in the base.
    mDescription = "Espresso";
  }

  double cost() const {
    return 1.99;
  }
};

// ----------------------------------------------------------------------------
class HouseBlend : public Beverage {
public:
  HouseBlend() {
    mDescription = "House Blend Coffee";
  }

  double cost() const {
    return .89;
  }
};

// ----------------------------------------------------------------------------
class Mocha : public CondimentDecorator {

private:
  const Beverage *mBeverage;

public :
  explicit Mocha(Beverage *beverage) : mBeverage(beverage) {
  }

  ~Mocha() {
    delete mBeverage;
  }

  string getDescription() const {
    return mBeverage->getDescription() + ", Mocha";
  }

  double cost() const {
    return .20 + mBeverage->cost();
  }
};

// ----------------------------------------------------------------------------
class Soy : public CondimentDecorator {

private:
  const Beverage *mBeverage;

public:
  explicit Soy(Beverage *beverage) : mBeverage(beverage) {
  }

  ~Soy() {
    delete mBeverage;
  }

  string getDescription() const {
    return mBeverage->getDescription() + ", Soy";
  }

  double cost() const {
    return .50 + mBeverage->cost();
  }
};

// ----------------------------------------------------------------------------
static void Test(void){

  Espresso MyExpresso;
  cout << MyExpresso.getDescription() << " $" << MyExpresso.cost() << endl;

  Beverage *beverage = new Espresso;
  cout << beverage->getDescription() << " $" << beverage->cost() << endl;
  delete beverage;

  Beverage *OtherBeverage = new HouseBlend;
  OtherBeverage = new Soy(OtherBeverage);
  OtherBeverage = new Mocha(OtherBeverage);
  cout << OtherBeverage->getDescription() << " $" << OtherBeverage->cost() << endl;
  delete OtherBeverage;
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

*La suite au prochain épisode...*

