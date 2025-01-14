---
layout: default
title: "Algorithmie en première S"
parent: "Programmation"
date: 2013-09-25 01:21:17
last_modified_date: 2020-05-04 15:23:18
---

{: .warning } 
Janvier 2025. Je viens de relire en diagonale ce que j'avais écrit en 2013. Je reste globalement d'accord avec tout ce que je disais sauf qu'aujourd'hui je propose [ideone.com + Python](https://ideone.com/l/python-3) comme ticket gagnant. Je dis ideone.com car si je dis [Google Colab](https://colab.google/) j'ai tout les profs qui vont venir manifester devant chez moi, crier au loup, parler de perte de souveraineté, j'en passe et des meilleures. Heu... Sinon, vous savez si y a encore des maths en 2025 en première? Je sais plus trop, j'ai un peu lâché l'affaire.


# Algorithmie en première S

# Introduction

On est en 2013 et je prends le temps d'écrire un billet car je suis un peu énervé...

En gros, j'ai eu l'occasion de lire les bouquins de maths utilisés actuellement en première et en terminale. Sympa, y a de l'algorithmie et de l'initiation à la programmation. Cela dit cela se fait avec des outils qui sont, ne soit pas pratique (programmer une TI 89 c'est un peu la misère) soit complètement propriétaires (AlgoBox). De plus, je ne comprends pas pourquoi on fait travailler les gamins avec des outils qu'ils ne reverront pas plus tard (qui a utilisé sa calculatrice au bureau durant les 6 derniers mois ?). De même, si on suppose que le problème posé est résolu du point de vue de l'algorithme, pourquoi passer par un truc complètement propriétaire, franco-français et qui, encore une fois, n'a rien à voir avec la vraie vie (qui a utilisé AlgoBox récemment).

## L'algorithmie

Je ne reviens pas dessus. Ce n'est pas fun, c'est formateur blablabla...

## Les calculatrices

Je passe sur les calculatrices car je pense que tout le monde comprend ce que je veux dire. Si vous ne voyez pas, prenez une TI ou une Casio. Mettez-vous devant la télé dans une ambiance lumineuse "normale" et essayez de faire un programme de plus de 20 lignes de code. Au bout de cinq minutes vous ne voyez plus rien (écran trop petit, pas rétro éclairé) et vous finissez sur la table de la salle à manger sous le lustre offert par tante Germaine.

Je passe sur l'ergonomie du clavier qui n'est surtout pas fait pour éditer du code. Je passe sous silence les affectations de variables qui ne se font pas dans le bon sens (2->Bob au lieu de Bob=2). Oui, oui, c'est un point de détail mais quitte à apprendre la programmation d'un algorithme à un gamin autant en profiter pour lui apprendre des choses et des méthodes qu'il retrouvera ailleurs (la majorité des langages de programmation que je connais réalisent les affectations de droite à gauche). Tiens en parlant de syntaxe du langage de programmation... Celle de la TI est complètement spécifique et ne se rapproche en rien de ce qui existe ailleurs. Ce n'est pas grave en soit, ce qui me gêne c'est que le gamin ne va pas retrouver une syntaxe similaire ailleurs (attention je ne parle pas de 12 000 fonctions mathématiques disponibles mais bien de la syntaxe).

Heu... J'allais oublier... Le nombre et les noms de variables sont limités. Autrement dit, il est possible qu'après avoir cherché sur Google vous découvriez que telle ou telle variable est déjà utilisée par un autre programme et doit être affectée au votre. Bon j'ergote... Pour finir, je vais mentionner l'absence d'un mode Debug. Pour ceux qui arriveraient de la planète Mars et/ou qui ne savent pas ce qu'est un mode Debug, notez que son absence signifie qu'il n'y a pas moyen de tester son code en progressant d'une ligne à l'autre afin d'observer ce qui se passe (inspecter les valeurs prises par les variables en cours de route par exemple) et essayer de trouver la raison pour laquelle le programme ne se comporte pas comme prévu. Bref... Une misère.

## AlgoBox

Je suis allé sur le site. J'ai téléchargé l'application et j'ai "joué" avec. Franchement c'est très bien fait, on sent qu'il y a du vécu, du retour d'utilisateurs... Bien, très bien, vraiment très belle réalisation, très bon packaging marketing avec des tutos, des fiches des vidéos... Tout est fait pour qu'on débute rapidement.

Cela dit cela n'a rien à voir avec la vraie vie. Je veux dire que si cela prend le gamin par la main, l'incite à déclarer ses variables, à utiliser telle ou telle fonction etc... Demain dans Excel, dans javascript ou autre il ne pourra pas réutiliser son savoir-faire AlgoBox. Bref, de mon point de vue, tout ça c'est du temps perdu... Sauf... sauf pour les profs car AlgoBox propose un environnement gratuit et une méthode de programmation dans laquelle, pour finir, l'enseignant n'a pas besoin de connaître tel ou tel langage puisque dans la méthode par défaut (hors de l'éditeur de code d'AlgoBox) il faut juste cliquer sur des boutons. Mouai... On demande quand même aux profs de connaître les syntaxes TI et Casio (j'ai aussi vu des livres de maths avec du Python à l'intérieur) alors je ne vois pas pourquoi on ne pourrait pas oublier la programmation sur calculette et n'utiliser qu'un seul vrai langage de programmation.

## Et ?

Sous-entendu : "Ok mais tu proposes quoi ? Quel langage ? Quelle méthode et pour quel coût ?"

Reprenons depuis le départ. Je suppose que les gamins ont étudié le problème posé (résolution des équations du second degré dans R dans la suite du billet) et qu'ils ont un algorithme entre les mains.

Bon maintenant, afin d'aller plus loin, de se faire plaisir et d'apprendre en expérimentant, nous devons coder cet algorithme dans un langage de programmation. Idéalement l'environnement de développement (là où les élèves vont taper leur code) doit :

* Être simple (AlgoBox est un bon exemple). De mon point de vue il n'est pas nécessaire qu'il soit en français et de toute façon, en première, on s'adresse à des gamins qui passent leur vie sur le Web et un peu d'anglais ne peut pas faire de mal.
* Pas cher (voir gratuit)
* Pas trop consommateur de ressources (les PC de l'école sont peut-être des dinosaures et/ou le PC de l'élève peut être en train de nous jouer "à bout de souffle").
* Disposer d'un moyen de déboguer son code afin de trouver et de corriger ses erreurs

De mon point de vue, le langage de programmation qu'on va utiliser pour traduire l'algorithme doit :

* Être un standard. C'est **LE** truc le plus important et s'il n'y avait qu'une chose à retenir de ce billet ce serait : "Faites travailler les enfants sur des outils standards, largement utilisés et arrêtez de les confiner à des outils trop spécialisés et qu'ils n'utiliseront plus".
* D'une syntaxe pas trop difficile à retenir (apprendre la syntaxe doit être fait de toute façon que le langage soit standard ou propriétaire). On peut quand même remarquer que compte tenu des exercices, les élèves vont avoir besoin de faire 2 ou 3 entrées/sorties à l'écran, un peu de calcul et des boucles. Je n'ai pas tout lu avec attention mais je ne crois pas qu'on utilise des fichiers, qu'on fasse des appels de fonctions et/ou qu'on mette en œuvre la récursivité. Afin de faire l'inventaire, il suffit d'aller voir sur la page d'AlgoBox pour faire rapidement la liste des choses nécessaires.
* Permettre de traduire facilement l'algorithme en code. Idéalement s'il y a 20 lignes dans l'algorithme il faudrait avoir 20 lignes dans le code source.
* Éviter toutes les limitations mentionnées ci-dessus à propos des calculatrices. Pas de limites dans le nombre des variables ni dans la longueur des noms de variables par exemple.

Ok, ok... Tu nous as fait une liste de Noël. Très bien tu proposes quoi ?

Commençons par les langages et je vais utiliser une approche pragmatique. Quand on fait une recherche rapide sur le Web à propos des langages les plus utilisés on trouve rapidement : Java, C, C++, C#, Objective C, PHP, Python, Ruby...

Bon allez, on taille :

* C# essentiellement Windows et Microsoft. Le langage est très très bien. L'environnement est vraiment super mais un peu lourd pour pas mal de PC "lite". Pas un standard malgré ECMA-334 et Mono sous Linux. Microsoft a laissé tomber du monde quand on est passé de VB à VB .NET. Y a un risque que ça recommence un jour et de toute façon c'est la course aux technologies. J'aime bien Microsoft mais non, on ne prend pas
* Objective C : Faut un Mac, c'est uniquement du Apple. L'environnement XCode est très bien. Pas un standard. On ne prend pas.
* Python : Pas mal d'incompatibilités entre 2.7 et 3.0 par exemple. Problèmes liés à un langage non standard. Rien ne garantit qu'il n'en sera pas de même dans l'avenir. Côté IDE, on peut faire des développements dans XCode, Visual Studio, Eclipse... mais c'est plutôt du lourd. On ne prend pas.
* PHP : Faut un bon environnement de développement puis tout mettre dans une page HTML. Enfin bref pas mal de tuyauterie qui vont poser des problèmes de logistique à tout le monde. On ne prend pas.

C'est dur, c'est sans doute un peu sectaire et cela demanderait un peu plus de développement au niveau des arguments mais bon... Allez il nous reste : Java, C, C++, Ruby

## Le C ANSI

De loin le langage avec lequel je suis le plus à l'aise alors vérifiez tout ce que je vais dire. Quoiqu'il en soit, c'est du standard de chez standard mais le langage est "vieux". Attention je dis ça avec beaucoup de respect mais bon du côté des fonctions à utiliser ça sent vraiment le sapin... Un exemple ?

```c
#include <stdio.h>

int main(){

    char name[10] ;

    scanf("%s",name);
    printf("Hello %s\n", name) ;
    return 0;
}
```

Le code ci-dessus attend que l'utilisateur tape son nom (Bob). Ensuite il affiche "Hello Bob". Question a 2 millions de dollars. Qu'est-ce qui se passe si l'utilisateur est un polonais qui a un nom avec 243 lettres, plein de 'k', de 'i' et de 'y' ?  Je dis 243 pour rire mais plus sérieusement que se passe-t-il si le nom possède plus de lettres que le tableau de caractères de la variable "name" qui ne peut en contenir que 9 plus un caractère null de fin de chaîne). Ben... On ne sait pas. Ça peut tourner, planter... Ce qui est certains c'est qu'on ne peut pas demander à des gamins qui débutent de prendre tout ça en compte. Bon, allez, j'adore le [C]({% link docs/06_programmation/c/index.md%}) mais non, on ne prend pas.

Ah ah. Il ne reste plus que Java, C++ et Ruby.

## Java

J'ai un pote qui est développeur Java. S'il tombe sur ce qui suit il va me tuer... De mon point de vue, Java est complètement piloté par Oracle (langage propriétaire, très largement diffusé et utilisé mais dans les mains d'une seule société) avec une philosophie qui va vers un Java à 2 vitesses entre ceux qui paieront et ceux qui ne pourront pas payer. Cela dit il y a des tonnes de bibliothèques qui peuvent faciliter les développements pros mais ici cela ne compte pas puisqu'on va faire des choses très simples.

De plus j'ai quand même l'impression qu'en dehors de ideone (voir en bas de page, c'est un compilateur en ligne), les environnements de développement à installer sous Windows sont assez lourds (très/trop complets et très lourds). Bref, moi, je prends pas.

## Ruby

Pour le coup c'est de loin le langage que je connais le moins. Sur le Web en 2 minutes on trouve des tutoriaux, sur Mac on peut invoquer "irb" (interactive ruby) et faire quelques tests.

Côté environnement de développement je ne sais pas trop mais Aptana semble être un truc sympa qui évolue dans le bon sens et Eclipse doit pouvoir venir à la rescousse mais c'est du lourd. Et...

Cerise sur le gâteau le langage est normalisé : ISO 30170:2012. Bon ben alors on a trouvé... Allez, on part avec Ruby ?

Du calme... Moi y a une chose qui me gêne : le tout objet. Franchement je trouve que cela amène des lignes de code un peu cabalistique mais bon j'imagine que pour faire des calculs basiques Ruby sera capable de faire l'affaire.

A titre d'exemple voilà un bout de code pour trouver les racines d'un polynôme de degré 2 (vous noterez au passage que les variables ne sont pas déclarées et n'ont pas, à priori de type particulier, ce qui va à l'encontre de ce qui est fait dans AlgoBox) :

```ruby
def cherche(a, b, c)
  d = b*b - 4*a*c
  if (d>=0)
    x1 = -b + Math.sqrt(d)/(2*a)
    x2 = -b - Math.sqrt(d)/(2*a)
    printf("x=%.2f, x=%.2f\n", x1, x2)
  end
end

cherche(1, 4, 4)
cherche(1, 2, 4)
```

## Le C++

C'est standard de chez standard. Celui qui sait programmer en C++ est capable de s'adapter à Objective C, au C et à C#. Attention ne me faites pas dire ce que je n'ai pas dit. Un étudiant de première qui a fait 3 lignes de C++ pour afficher une racine carrée **N'EST PAS** un développeur Objective C.

Cependant, à contrario de AlgoBox, s'il le souhaite, l'élève peut aller plus loin, apprendre... Il ne rencontrera aucune limite (la communauté est immense) et il sera alors en mesure de lire un code C ou Objective C sur le Web par exemple. De plus celui qui connaît le C++ couvrira les langages qui sont en tête du [top 50 des langages de programmation](https://www.tiobe.com/tiobe-index/) que je citais plus haut. Il fait donc d'une pierre deux coups.

Pour le reste, à l'instar de Java et de Ruby, pour faire ce que l'on a à faire en classe de première la syntaxe est très simple (ce qui est vrai pour l'élève l'est aussi pour le prof aussi et ce dernier n'aura que 2 ou trois trucs à connaître).

Le langage est moderne. Par exemple le souci qu'on avait en C précédemment n'existe pas (taille du tableau) et contrairement à Ruby il oblige (et c'est une très bonne chose) l'élève à déclarer les variables utilisées mais aussi à leur donner un type (un entier, un nombre à virgule, une chaîne de caractères).

En termes d'environnement de développement, sous Windows le plus simple et le plus économique serait d'installer Visual Studio Express Edition. Malheureusement cet environnement de développement est trop riche, trop complet et de mon point de vue, alors qu'en 2 min dans AlgoBox on peut faire "Run", dans Visual Studio il faut créer un projet, savoir de quoi on parle, éditer, compiler...

Bref, ce n'est pas cool pour un gamin de 15-16 ans. Après quelques recherches et quelques tests, je propose donc d'utiliser [Ideone](https://ideone.com/). Il s'agit d'un environnement de développement sur le web (rien à installer sur la machine de l'élève ou de l'école) qui supporte une quarantaine de langages dont C++, Ruby, Java etc.

Autre avantage, la compilation se fait sur le serveur. Autrement dit il n'y a pas besoin de mettre dans les mains de l'élève une machine de course. Il suffit de pouvoir accéder à la page Web. Par exemple, avec un ipad je peux atteindre le site web, écrire un code en C++, le compiler et voir le résultat à l'écran. Bien sûr, je peux tout sauvegarder sur le serveur, retrouver mes codes, les partager (ou non) etc.

Le code C++ ci-dessous fait la même chose que le code Ruby précédent. 

```cpp
#include <iostream>
#include <math.h>

using namespace std;

int main() {

    double a=1.0;
    double b=4.0;
    double c=4.0;

    double d = b*b - 4*a*c;
    
    if (d>=0){
        double x1 = (-b - sqrt(d))/(2*a);
        double x2 = (-b + sqrt(d))/(2*a);
        cout << "Les solutions sont   : " << x1 << " et " << x2 << endl;
    }
}
```
Le code ci-dessous fait la même chose que le code Ruby mais il y a plus d'E/S :

```cpp
#include <iostream>
#include <math.h>

using namespace std;

int main() {

    cout << "Entrez les 3 coefficients du trinôme : ";

    double a, b, c;
    cin >> a >> b >> c;

    double d = b*b - 4*a*c;
    cout << "Le trinôme a pour discriminant  : " << d << endl;

    if (d<0){
        cout << "Il n'y a pas de solution dans R" << endl;
    }else if(d==0){
        cout << "Il y a une unique solution dans R" << endl;
        double x = -b/(2*a);
        cout << "La solution est      : " << x << endl;
    }else{
        cout << "Il y a deux solutions dans R" << endl;
        double x1 = (-b - sqrt(d))/(2*a);
        double x2 = (-b + sqrt(d))/(2*a);
        cout << "Les solutions sont   : " << x1 << " et " << x2 << endl;
    }
    return 0;
}
```

## Conclusion

Je propose d'arrêter la "programmation" sur calculette. Ensuite, à ce jour, mon triplet gagnant est :

* Ideone
* C++
* Ruby

L'idée est de les utiliser dans l'environnement de développement Web Ideone. Je place ensuite C++ en premier choix de langage car il oblige à déclarer les variables (alors qu'on peut faire sans dans Ruby) mais surtout il est, de mon point de vue, plus universel et ne demande pas une approche objet dès le départ. Cette solution ne nécessite qu'un accès Web, elle est gratuite et évite toute une logistique (compilateur etc.) qui est au-delà des compétences des élèves et des enseignants.

Un tout dernier mot... **Je ne suis pas sûr qu'il soit nécessaire ou utile de faire de l'algorithmie et de la programmation en première**. Je crois qu'on aborde ces sujets pour noyer le poisson et, qu'à l'instar des livres de physique dans lesquels il n'y a plus de formules, que des belles images et des rappels historiques, on fait du marketing, on met un beau ruban rose autour du colis en priant pour que la pilule passe...

Bref, de mon point de vue on fait de la programmation pour dire qu'on en fait mais finalement les profs y vont à reculons, les élèves aussi et comme il n'y a pas plus d'heures de cours si cela se fait cela se fait au détriment des autres chapitres de maths.

Franchement je crois qu'on perd le focus (faire des maths) et qu'on noie certains élèves avec des trucs en plus à faire et à apprendre. À la limite, qu'on fasse un programme de maths "normal" et qu'en fin d'année on consacre un peu de temps en maths (tiens au fait, pourquoi on voit la programmation en maths et pas en histoire ou en gym ?) pour revenir sur certains sujets de l'année... Mouai... Je ne suis toujours pas convaincu mais ça peut s'entendre.

Je ne demande pas qu'on revienne au programme d'après-guerre. Je dis simplement qu'à vouloir tout faire, tout couvrir on obtiendra non pas des bons à rien mais **des mauvais à tout**.

 
{: .warning }
2025: J'ai dit que mon avais avait changé. Voilà la version Python du code Ruby/C++ précédent

```python
import math
 
a = 1.0
b = 4.0
c = 4.0
 
d = b**2 - 4*a*c
 
if d >= 0:
    x1 = (-b - math.sqrt(d)) / (2 * a)
    x2 = (-b + math.sqrt(d)) / (2 * a)
    print(f"Les solutions sont : {x1} et {x2}")
```


## Liens utiles

* [AlgoBox](http://www.xm1math.net/algobox/)
* [Learning C++](http://www.stroustrup.com/new_learning.pdf) (Bjarne Stroustup)
* [ideone](http://ideone.com/)
* [LangPop](http://langpop.com/) (date de 2011, indice de popularité des langages de programmation)
* [TIOBE](http://www.tiobe.com/index.php/content/paperinfo/tpci/index.html) (Indice de popularité des langages de programmation)
* [Apprendre à programmer en Ruby](http://pine.fm/LearnToProgram/)
* [Second degré](http://gatechgrad.wordpress.com/2011/09/25/quadratic-formula/) dans différents langages
* [C++ for kids](http://www.amazon.com/For-Kids-Those-Heart-ebook/dp/B007Y2PNKE) (date un peu mais prouve qu'on peut faire des choses)

