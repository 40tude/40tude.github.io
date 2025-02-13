---
layout: default
title: "CodinGame, puzzles en C++"
parent: "C++"
#math: mathjax
date: 2015-01-14 23:35:53
last_modified_date: 2020-05-03 22:24:37
---

# CodinGame, puzzles en C++

<!-- {:.warning}
Cet article commence à dater (2015). La plupart des informations à propos des algorithmes doivent être toujours correctes mais il y a sans doute eu des évolutions dans le mauvais sens du côté de Codingame. Par mauvais sens je veux dire : vous empêcher de développer chez vous à votre rythme puis coller votre solution quand VOUS le souhaitez. Vous obliger à ne coder que sur leur plateforme, ne plus vous permettre de récupérer les jeux d'entrainement et les solutions pour travailler en autonome... J'espère me tromper mais il sera intéressant d'y retrouner de temps à autre et de vérifier si ce que je dis se réalise (ou pas) 🤞  -->

{:.warning}
Les dernières versions de mes solutions, qu'elles soient documentées ou non sur cette page sont soit dans un [repo Git C++](https://github.com/40tude/codingame_cpp) ou un [repo Git Python](https://github.com/40tude/codingame_py). 





## Introduction
Je ne connaissais pas [Codingame](http://www.codingame.com/home). Suite discussion avec un pote j'ai eu l'occasion d'y faire un tour ce soir. C'est vraiment super. Que vous soyez développeur C, C++, Python, Java... vous y trouverez des problèmes à résoudre. Y a un tutorial qui explique l'environnement, ce que l'on peut faire, ce que l'on ne peut pas faire etc. Je suis bluffé par ce que j'ai vu. Faudra que je m'y intéresse vraiment.

Une fois que vous avez passé le problème du tutoriel il faut juste aller dans Games/Single Player par exemple.

Pour info, le code que j'ai envoyé pour "résoudre" le premier "challenge" en Janvier 2015

```cpp
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

struct enemy{
    int     dist;
    string  name;

    enemy(int d, const std::string& n) : dist(d), name(n) {}
};

int main(){

    vector<enemy> MyEnemies;

    while (1) {
        int count;                          // The number of current enemy ships within range
        cin >> count;
        cin.ignore();

        for (int i = 0; i < count; i++) {
            string  name;                   // The name of this enemy
            int     dist;                   // The distance to your cannon of this enemy
            cin >> name >> dist;
            cin.ignore();

            MyEnemies.push_back(enemy(dist, name));
        }

        sort(MyEnemies.begin(), MyEnemies.end(),
            [](const enemy &a, const enemy &b) {
                return a.dist > b.dist;
            });

        // Write an action using cout. DON'T FORGET THE "<< endl"
        // To debug: cerr << "Debug messages..." << endl;
        enemy e = MyEnemies.back();
        cout << e.name << endl;
        MyEnemies.pop_back();
    }
}
```

Je viens de retourner faire un tour sur le code en question. Il semble que depuis Janvier 2015 ils aient décidé de nous faire débuter avec un challenge encore plus simple. Voilà ce que j'ai proposé ce soir :

```cpp
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

struct enemy {
  int     dist;
  string  name;
  enemy(const int d, const std::string& n) : dist(d), name(n) {}
};

int main() {

  vector<enemy> MyEnemies;
  while (true) {
    for (auto i = 0; i != 2; ++i) {
      string  name;                                                             // The name of the enemy
      int     dist;                                                             // The distance to our cannon
      cin >> name >> dist; cin.ignore();

      MyEnemies.push_back({ dist, name });
    }

    sort(MyEnemies.begin(), MyEnemies.end(), [](const enemy &a, const enemy &b) {
      return a.dist > b.dist;
    });

    enemy e = MyEnemies.back();
    cout << e.name << endl;
    MyEnemies.pop_back();
  }
}
```

