---
layout: default
title: "C++ Design Patterns - Observer"
parent: "C++"
#math: mathjax
date: 2014-10-04 21:28:48
last_modified_date: 2020-05-03 23:30:21
---


# C++ Design Patterns - Observer

## Introduction
Je lis toujours [Head First Design Patterns](http://www.amazon.fr/First-Design-Patterns-Elisabeth-Freeman/dp/0596007124). J'en suis au Capitre 2 à la page 37 où l'on parle du Observer. Ce coup-ci j'ai ajouté un peu plus de commentaires dans le code source de mon Observer en C++... Ça ne fait jamais de mal. Cela dit, avec le bouquin ça devrait aller.





## Ce que j'ai retenu

Plus tard, je vais mettre ici mes notes à propos du Design Patterns en question. Là il n'y a rien car j'ai collé le code et zou, je suis passé à autre chose.








## Le code

```cpp
/*
See Chapter II, page 37

                Subject - Interface  ----------------->   Observer - Interface
                -------------------                                   ---------------------
                RegisterObserver                                        Update
                RemoveObserver
                NotifyObservers                                   ^
                                                                      |
                            ^                                         |
                            |                                         |
                            |                                         |

                Concrete Subject       <-----------------      Concrete Observer
                ----------------                                    ------------------
                RegisterObserver                                      Update
                RemoveObserver                                  ...
                NotifyObservers
                ...

*/

#ifdef _MSC_VER
  #define _CRTDBG_MAP_ALLOC
  #include <crtdbg.h>
#endif

#include <iostream>
#include <memory>
#include <list>
#include <vector>
#include <numeric> // accumulate

using namespace std;

// ----------------------------------------------------------------------------
class ObserverInterface{
public:
    virtual void Update(double humidity, double temp, double pressure) = 0;
  // See Strategy source code for comments
  virtual ~ObserverInterface() = 0 {};
};

class SubjectInterface{
public:
    virtual void RegisterObserver(ObserverInterface *o) = 0;
    virtual void RemoveObserver(ObserverInterface *o) = 0;
  virtual ~SubjectInterface() = 0 {};

private:
    virtual void NotifyObservers(void) const = 0;
};

class DisplayElementInterface{
public:
    virtual void Display(void) const = 0;
  virtual ~DisplayElementInterface() = 0 {}
};

// ----------------------------------------------------------------------------
// Tracks the data coming from the weather station and updates the displays
// WeatherData is the concrete subject
class WeatherData : public SubjectInterface {
private:
    double mHumidity;
    double mTemperature;
    double mPressure;
    list<ObserverInterface*> mObservers;

public:
    void RegisterObserver(ObserverInterface *o) {
        mObservers.push_back(o);
    }

    void RemoveObserver(ObserverInterface *o) {
        mObservers.remove(o);
    }

    void NotifyObservers(void) const {
        for (auto o : mObservers){
            o->Update(mHumidity, mTemperature, mPressure);
        }
    }

  // In real life, it is supposed to be called by the weather station directly
    void MeasurementChanged(void){
        NotifyObservers();
    }

    // Helper-Fake function.
    // Everything looks like, data are updated by the weather station and then MeasurementChanged() gets called by the weather station
    void SetMeasurements(double Humidity, double Temperature, double Pressure){
        mHumidity           = Humidity;
        mTemperature    = Temperature;
        mPressure           = Pressure;
        MeasurementChanged();
    }
};

// ----------------------------------------------------------------------------
// Shows to the users the current weather conditions
// A concrete Observer and a concrete Display
class CurrentConditionsDisplay : public ObserverInterface, public DisplayElementInterface{
private:
    double  mHumidity;
    double  mTemperature;
    double  mPressure;
    SubjectInterface& mWeatherData; // Keep a ref to the Subject just in case. Not used here however

public:
    CurrentConditionsDisplay(SubjectInterface& weatherData) : mWeatherData(weatherData) {
        mWeatherData.RegisterObserver(this);
    }

    // Implementation from the ObserverInterface
    void Update(double Humidity, double Temperature, double Pressure) {
        mHumidity         = Humidity;
        mTemperature  = Temperature;
        mPressure         = Pressure;
        Display();
    }

    // Implementation from the DisplayElementInterface
    void Display(void)  const {
        cout << "Current Conditions Display :" << endl;
        cout << "\tHumidity            : " << mHumidity << endl;
        cout << "\tTemperature         : " << mTemperature << endl;
        cout << "\tPressure            : " << mPressure << endl;
        cout << endl;
    }
};

// ----------------------------------------------------------------------------
class AveragedConditionsDisplay : public ObserverInterface, public DisplayElementInterface{
private:
    vector<double>        mHumidity;
    vector<double>        mTemperature;
    vector<double>        mPressure;
    SubjectInterface& mWeatherData;

public:
    AveragedConditionsDisplay(SubjectInterface& weatherData) : mWeatherData(weatherData){
        mWeatherData.RegisterObserver(this);
    }

    void Update(double Humidity, double Temperature, double Pressure) {
        mHumidity.push_back(Humidity);
        mTemperature.push_back(Temperature);
        mPressure.push_back(Pressure);
        Display();
    }

    void Display(void)  const {
        cout << "Averaged Display :" << endl;
        cout << "\tAverage humidity    : " << accumulate(mHumidity.begin(), mHumidity.end(),0.0)/mHumidity.size() << endl;
        cout << "\tAverage temperature : " << accumulate(mTemperature.begin(), mTemperature.end(), 0.0) / mTemperature.size() << endl;
        cout << "\tAverage pressure    : " << accumulate(mPressure.begin(), mPressure.end(), 0.0) / mPressure.size() << endl;
        cout << endl;
    }
};

// ----------------------------------------------------------------------------
static void Test(void){

    WeatherData                             MyWeatherDataObject;
    CurrentConditionsDisplay    CurrentDisplay(MyWeatherDataObject);
    AveragedConditionsDisplay   StatisticDisplay(MyWeatherDataObject);

    // Simulate new weather measurements
    MyWeatherDataObject.SetMeasurements(80, 22, 1013);
    MyWeatherDataObject.SetMeasurements(85, 25, 1023);
    MyWeatherDataObject.SetMeasurements(90, 26, 1020);
    MyWeatherDataObject.SetMeasurements(85, 25, 1025);

  MyWeatherDataObject.RemoveObserver(&StatisticDisplay);
  cout << "Now we have one display." << endl << endl;

  MyWeatherDataObject.SetMeasurements(100, 40, 1000);
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
