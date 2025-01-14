---
layout: default
title: "rvalue references et move semantics"
parent: "C++"
#math: mathjax
date: 2012-10-17 15:43:54
last_modified_date: 2020-04-26 16:35:29
---

# rvalue references et move semantics

## Introduction
Je suis toujours en train de "ré-apprendre" le C++ et je fais des tests soit avec XCode (clang) sous Mac OS soit avec MSVC 2012 Express for Desktop lorsque je suis sous Windows 7. J'ai aussi installé récemment Code::Blocks 10.05 (gcc 4.4.1) mais bon, je l'utilise pas trop celui là. Ici je vais surtout faire des tests sous Windows et je vais essayer de transcrire ce que je crois avoir compris du move semantics et des RValues References qui sont deux éléments nouveaux des spécifications de C++ 11.

Ceci dit, attention... En effet, si vous connaissez déjà le sujet passez votre chemin car je ne crois pas que vous apprendrez grand chose. Par contre, si après avoir lu et cherché sur le web vous vous posez encore des questions j'espère que ces quelques notes vous permettront de mieux comprendre ce qui se passe et/ou de lire un article en français avec un point de vue qui vous permettra d'avancer dans vos réflexions.




## Le code source de départ
On va partir d'un exemple hyper classique : une classe qui gère des tableaux d'entiers. Cette classe comporte le strict minimum pour survivre. Pour les besoins de la cause elle est déclarée et définie dans le code source principal (et unique) qui comprend aussi la fonction main(). Je sais c'est peut être pas top mais bon cela devrait vous permettre de copier/coller/tester rapidement.

Quoiqu'il en soit, la classe MyArray comporte un Default Constructor, un Copy Constructor, un Destructor, un Subscripting Operator et finalement un Assignment Operator. Question membres on retrouve un pointeur sur des entiers et un entier qui contient le nombre d'éléments du tableau.

Avant d'aller plus loin étudions le source de la fonction main(). Oublions pour l'instant la macro DUMMY et notons que c'est l'invocation de la fonction OutputDebugString qui nous permet d'avoir des sorties dans la fenêtre Output de MSVC 2012. En gros, après avoir "tracé "le démarrage de la fonction main(), on déclare une variable objet de 20 entiers (Tableau2), on appelle la fonction CreateArray(), on affiche le contenu du tableau dans la console, on attend que l'utilisateur appuie sur ENTER et on "trace" la fin du code.

A propos de la "tuyauterie" il faut remarquer de la macro DUMMY est nécessaire car sinon, lors de la compilation, on a une collision entre le "max" du SDK Windows et celui des bibliothèques C++. Si vous n'utilisez pas std::cin.ignore(...) pour capturer l'appui sur la touche ENTER vous pouvez oublier la macro DUMMY.

C'est bon? Allez je vous laisse jeter un oeil sur le reste du code source de départ et je vous retrouve pour parler de la fonction CreateArray().

```cpp
#include "stdafx.h"
#include <iostream>
#include <cstddef>        // std::size_t
#include <windows.h>      // OutputDebugString

// ----------------------------------------------------------------------------
class MyArray{

  public:
    MyArray(const std::size_t size = 0);                    // Default ctor
    MyArray(const MyArray& other);                          // Copy ctor
    int& operator[](const int i);                           // Subscripting operator
    ~MyArray();                                             // Destructor
        MyArray& MyArray::operator=(const MyArray &rhs);    // Assignment operator

private:
    std::size_t mSize;
    int         *mArray;
};

// ----------------------------------------------------------------------------
// Default ctor
MyArray::MyArray(const std::size_t size) : mSize(size), mArray(mSize ? new int[mSize]() : nullptr){

  OutputDebugString(L"Default Constructor\n");
}

// ----------------------------------------------------------------------------
// Copy constructor
MyArray::MyArray(const MyArray& other) : mSize(other.mSize), mArray(mSize ? new int[mSize] : nullptr){

  OutputDebugString(L"Copy Constructor\n");
  std::copy(other.mArray, other.mArray+mSize, mArray);
}

// --------------------------------------------------------------------
// Assignment operator by reference
MyArray& MyArray::operator=(const MyArray &rhs) {

  OutputDebugString(L"Assignment operator by reference\n");
    if(this!=&rhs){
        delete [] mArray;
        mSize=rhs.mSize;
        mArray = new int[mSize];
        std::copy(rhs.mArray, rhs.mArray+mSize, mArray);
    }
  return *this;
}

// ----------------------------------------------------------------------------
// Subscripting operator
int& MyArray::operator[](const int i){

  return mArray[i];
}

// ----------------------------------------------------------------------------
// Destructor
MyArray::~MyArray() {

    OutputDebugString(L"Destructor\n");

  delete [] mArray;
    mArray = nullptr;
    mSize = 0;
}

// ----------------------------------------------------------------------------
static MyArray CreateArray(void){

    OutputDebugString(L"CreateOneArray begins\n");

    MyArray tmp(20);
  for(int i=0; i<20; i++)
    tmp[i]=2*i;

    OutputDebugString(L"CreateOneArray ends\n");
  return tmp;
}

// ----------------------------------------------------------------------------
int main(){

    // Really weird. Avoid the issue with max and the collision with Windows.h
    #define DUMMY

    OutputDebugString(L"Main begins\n");

    MyArray Tableau2(20);

    Tableau2 = CreateArray();
    for(int i=0; i<20; ++i)
        std::cout << Tableau2[i] << " ";
    std::cout << std::endl;

    std::cout << std::endl << std::endl << "Strike ENTER to exit :";
    std::cin.ignore( std::numeric_limits <std::streamsize>::max DUMMY (), '\n' );

    OutputDebugString(L"Main ends\n");
}
```

La fonction ``CreateArray()`` est simple. Elle déclare localement un objet de type ``MyArray`` qui contient toujours 20 entiers, lui assigne des valeurs puis retourne cet objet à la fin. Attention, normalement c'est vraiment PAS une bonne idée (c'est même interdit de faire des trucs comme ça en C par exemple) car l'objet tmp étant alloué localement sur la pile ce dernier n'est plus valide à la fin de la fonction ``CreateArray()``. Ceci dit, la fonction ``CreateArray()`` est surtout là à des fins didactiques et on en reparlera un peu plus loin.

Ok, ok c'est peut être pas une bonne idée mais je ne rêve pas, moi dans le ``main()`` je vois bien une ligne du style : ``Tableau2 = CreateArray();``

C'est vrai. Pour l'instant on va juste admettre que si tout se passe bien cette ligne va assigner à l'objet ``Tabeau2`` les valeurs de l'objet ``MyArray`` retourné par la fonction  ``CreateArray()``.

C'est toujours bon pour toi? 

Ok, mais qu'est-ce qui se passe si, en mode ``Debug``, on compile et on lance? 

La première fois, le compilateur de MSVC pleurniche un peu et émet un ``warning C4996``. Pour le calmer, dans le code ci-dessus, juste après la ligne ``#include "stdafx.h"`` il suffit de définir la macro ``_SCL_SECURE_NO_WARNINGS``. Le tout début du source devient alors :

```cpp
#include "stdafx.h"
#define _SCL_SECURE_NO_WARNINGS
#include <iostream>
#include <cstddef>            // std::size_t
#include <windows.h>      // OutputDebugString
```

Objectivement si vous n'êtes pas sous MSVC 2012 vous n'avez pas besoin des deux premières lignes. Par contre si vous travaillez avec MSVC vous mettez simplement le ``#define`` dans le fichier ``stdafx.h`` et vous en profitez pour y mettre aussi les lignes qui concernent ``iostream``, ``cstddef`` et ``windows``.

Quoiqu'il en soit, lorsque le code démarre la console apparaît, on clique sur ENTER et dans la fenêtre Output de MSVC 2012 on peut lire les messages suivants :

```bash
Main begins
Default Constructor
CreateOneArray begins
Default Constructor
CreateOneArray ends
Copy Constructor
Destructor
Assignment operator by reference
Destructor
Main ends
Destructor
```




## Explications détaillées

Décortiquons tout cela ligne à ligne quitte à prendre du temps et à être un peu lourd...

1. La première ligne "Main begins" n'appelle pas de commentaire particulier c'est simplement la trace du début de l'application.
2. "Default constructor" s'affiche quand à la ligne 89 on déclare ``Tableau2`` comme un objet de type ``MyArray``.
3. Ensuite on passe dans la fonction ``CreateArray()`` où l'on commence par tracer l'exécution avec "CreateOneArray begins"
4. On continue en émettant le message "Default constructor" au moment de la déclaration de l'objet local ``tmp``.
5. La fonction ``CreateArray()`` semble se terminer lorsque le message "CreateOneArray ends" est émis. Ceci dit c'est vraiment là que **les choses intéressantes commencent...**
6. Si on résume : à ce stade, dans la fonction ``CreateArray()``, on a un tableau de 20 entiers (variable ``tmp``) qu'il faut retourner au code appelant (la fonction ``main()``). Comme on a dit qu'on ne pouvait pas compter sur la variable ``tmp`` qui est allouée sur sur la pile et qui va disparaître à la "vraie" fin de la fonction ``CreateArray()`` il va falloir trouver un truc... Avec les moyens dont il dispose le compilateur commence par invoquer la fonction membre Copy Constructor sur l'objet ``tmp`` (message "Copy Constructor").  Cette fonction construit de toute pièce un objet supplémentaire de type ``MyArray`` et qui est une copie de l'objet sur lequel elle s'applique (``tmp`` ici).
7. Ceci fait, le compilateur  peut ensuite invoquer la destruction de l'objet ``tmp`` (message "Destructor") et la "vraie" fin de la fonction ``CreateArray()`` peut avoir lieux.
8. À ce stade, on a donc une copie "saine" de l'objet ``tmp`` sur laquelle on peut compter. Cette copie est "quelque part" (on va y revenir) et le compilateur invoque alors l'opérateur d'assignation de la variable ``Tableau2`` en lui passant comme paramètre la variable "saine" que l'on vient de créer. Le message "Assignment operator by reference" est alors émis. Comme on peut le voir dans le source de la fonction membre "operator=" une recopie de l'objet préalablement créé (celui dont on ne sait pas exactement où il est) a lieu.
9. Ensuite l'objet sain et temporaire peut être détruit (message "Destructor")  et finalement le tout nouveau tout beau contenu de la variable objet ``Tableau2`` de la fonction ``main()`` peut être affiché dans la console.
10. La fonction ``main()`` se termine alors (message "Main ends")
11. Tout à la fin (sur l'accolade fermante en fait) la variable objet ``Tableau2`` est détruite (message "Destructor").

Ouf, ça y est c'est finit ! J'espère que l'intérêt des traces dans la fenêtre Output de MSVC 2012 apparaît plus clairement. En effet, même si l'utilisation de la fonction ``OutputDebugString`` est un peu lourde (n'oubliez as le 'L' devant les chaînes de caractères), elle offre un bon moyen de "tracer" dans le bon ordre ce qui se passe. Pensez à la dernière destruction d'objet ``MyArray``. Si de manière classique on faisait afficher un message dans la console à partir du destructeur de la classe ``MyArray`` on ne verrait rien car la destruction effective de l'objet se fait au niveau de la dernière accolade de la fonction ``main()``. À cet instant il y a bien longtemps que la console n'est plus à l'écran.







## Encore plus de détails

Histoire d'être vraiment lourd, je vais revenir sur les points 6 à 9 qui concernent la ligne de code ``Tableau2 = CreateArray();`` et qui sont ceux qu'ils faut vraiment comprendre si on veut avancer.

À **l'étape 6**, le point qui peut paraître litigieux est le suivant : localement on a une variable objet nommée ``tmp`` sur laquelle on invoque le Copy Constructor. Très bien mais si on regarde dans le détail, on remarque que le Copy Constructor attend une référence sur un objet et pas un objet (qui serait passé par valeur). Comment est-ce possible?

En fait, une référence est un pointeur :

* qui est automatiquement déréférencé (ce qui allège généralement l'écriture du code source)
* qui doit obligatoirement être "attaché" à une autre variable
* qui doit obligatoirement être initialisé lors de sa création
* qui une fois attaché à variable ne peut pas être attaché à une autre

Ce comportement est beaucoup plus restrictif que celui des pointeurs C classiques qui peuvent être créés quand on veut, pointer sur ce que l'on veut et qui n'ont pas l'obligation d'être initialisés à leur création. On a coutume de dire qu'une référence n'est rien d'autre qu'un synonyme de la variable qu'elle référence (tout ce que l'on fait subir à une référence impacte la variable référencée).

Ceci ayant été rappelé le bout de code suivant devrait nous aider à comprendre ce qui est licite :

```cpp
int& MaFonction(int& x) {
  x++;
  return x;
}

int main() {
  int a = 0;
  MaFonction(a);
}
```

Il est donc important de comprendre que l'on peut légalement passer le nom d'une variable à une fonction qui attend comme paramètre une référence. Ce faisant on fait d'une pierre deux coups. 
1. On a plus de passage de paramètre par valeur (ce qui peut être très lourd et très lent quand les objets contiennent de grande quantités de données) 
1. En plus, dans la fonction appelée tout se passe comme si on avait reçu un paramètre en valeur (la syntaxe est simple).

Autrement dit, au **point 6**, au lieu d'appeler ``MaFonction`` tout se passe comme si on appelait la fonction Copy Constructor en lui passant, non pas la variable "a" mais la variable "tmp". À la fin du Copy Constructor un nouvel objet "sans nom" a été créé.

Une dernière remarque : il ne faut pas se laisser impressionner par le "const" qui apparaît dans la définition du Copy Constructor (``MyArray::MyArray(const MyArray& other)``). En fait c'est une sécurité. Écrire ``const`` indique au compilateur que le développeur n'a pas l'intention de modifier le contenu de "other". Si par excès d'enthousiasme le développeur écrivait dans le corps du Copy Constructor une ligne de code qui mettrait en cause l'intégrité de la référence other, le code ne compilerait plus.

On peut se permettre de passer rapidement sur le **point 7**. En effet on est au niveau de la dernière accolade de la définition de la fonction CreateArray et l'objet tmp est tout simplement détruit car on en a plus l'usage.

Concernant le **point 8** le rappel précédent sur les références va nous permettre d'aller beaucoup plus vite. La seule chose qu'il faut bien garder en tête c'est que le signe "=" qui apparaît dans le code source est un leurre, une facilité syntaxique. En fait on devrait avoir une ligne de code qui ressemble à : ``Tableau2.operator=(ObjetSansNom);``

Pour le reste, la définition de l'opérateur d'assignement montre qu'on attend, là aussi, une référence (``const`` elle aussi). Compte tenu de ce que l'on a dit précédemment on peut donc en toute légalité lui passer en paramètre le nom de l'objet sans nom (oui, oui je sais ça fait bizarre de lire ça) qui vient d'être créé avec le Copy Constructor.

Au **point 9**, il n'y a rien de vraiment tordu. L'objet sans nom a fait son office, on va quitter l'opérateur d'assignement, il est grand temps de le détruire.




## rvalue references et move semantics

Bon ben voilà... J'espère que c'est plus clair maintenant et j'espère que vous comprenez mieux tout se qui se passe parfois derrière une simple ligne (constructions, destructions, copies multiples) On va donc pouvoir passer à la suite et pour cela on va commencer par prendre un peu de recul.

En gros, à la ligne 89 (``Tableau2 = CreateArray();`` ) on souhaite, entre autres, assigner au tableau de l'objet ``Tableau2`` le contenu du tableau retourné par ``CreateArray()``. Ceci dit, dans le détail, on se rend compte qu'en dehors des choses essentielles (création de l'objet tmp par exemple) on a été amené à créer une copie de la variable ``tmp`` puis à refaire encore une copie afin de remplir le tableau de la variable objet ``Tableau2``. Ça manque un peu d'efficacité et on peut légitimement se demander pourquoi on ne peut pas directement copier le contenu de la variable ``tmp`` dans la variable ``Tableau2``. Il faut bien garder en tête que la création par copie d'une variable non nommée puis la copie du contenu de cette dernière dans la variable Tableau2 nous coûte deux opérations de copie (``std::copy``). Là ça va à peu près car on a un tableau de 20 entiers. Quel serait l'impact si on avait des millions d'affectations et/ou des objets contenant non pas 20 entiers mais 2Mo de caractères?

C'est entre autres pour répondre à cette question que les **RValues References** et le **move semantics** ont été incorporés dans les spécifications de C++11. Avant de rentrer dans les détails, je vous propose de jeter un oeil sur le code modifié. À ce stade la seule chose vraiment importante à remarquer c'est que la fonction ``main()`` reste identique. Autrement dit, je vous demande de garder en tête que l'implémentation du move semantics et l'utilisation des RValues References n'impose aucun changement dans les codes sources qui utilisent les classes. Pour peu que le move semantics et le RValues References apportent leurs lots d'améliorations en termes de performances, on gagne sur tous les tableaux. Champagne !

```cpp
#include "stdafx.h"
#define _SCL_SECURE_NO_WARNINGS
#include <iostream>
#include <cstddef>        // std::size_t
#include <windows.h>      // OutputDebugString

// ----------------------------------------------------------------------------
class MyArray{

  public:
    MyArray(const std::size_t size = 0);                // Default ctor
    MyArray(const MyArray& other);                      // Copy ctor
        MyArray(MyArray&& other);                       // Move constructor
    MyArray& MyArray::operator=(const MyArray &rhs);    // Assignment operator
        MyArray& MyArray::operator=(MyArray&& rhs);     // Move assignment operator
        int& operator[](const int i);                   // Subscripting operator
    ~MyArray();                                         // Destructor

    private:
    std::size_t mSize;
    int         *mArray;
};

// ----------------------------------------------------------------------------
// Default ctor
MyArray::MyArray(const std::size_t size) : mSize(size), mArray(mSize ? new int[mSize]() : nullptr){

  OutputDebugString(L"Default Constructor\n");
}

// ----------------------------------------------------------------------------
// Copy ctor
MyArray::MyArray(const MyArray& other) : mSize(other.mSize), mArray(mSize ? new int[mSize] : nullptr){

  OutputDebugString(L"Copy Constructor\n");
  std::copy(other.mArray, other.mArray+mSize, mArray);
}

// ----------------------------------------------------------------------------
// Move ctor
MyArray::MyArray(MyArray&& other):mSize(other.mSize), mArray(other.mArray){

    OutputDebugString(L"Move Constructor\n");
    other.mSize=0;
    other.mArray=nullptr;
}

// --------------------------------------------------------------------
// Assignment operator by reference
MyArray& MyArray::operator=(const MyArray &rhs) {

  OutputDebugString(L"Assignment operator by reference\n");
    if(this!=&rhs){
        delete [] mArray;
        mSize = rhs.mSize;
        mArray = new int[mSize];
        std::copy(rhs.mArray, rhs.mArray+mSize, mArray);
    }
  return *this;
}

// ----------------------------------------------------------------------------
// Move assignement operator
MyArray& MyArray::operator=(MyArray&& rhs){

    OutputDebugString(L"Move assignment operator\n");
    if(this!=&rhs){
        delete [] mArray;
        mSize = rhs.mSize;
        mArray = rhs.mArray;
        rhs.mSize=0;
        rhs.mArray=nullptr;
    }
    return *this;
}

// ----------------------------------------------------------------------------
// Subscripting operator
int& MyArray::operator[](const int i){

  return mArray[i];
}

// ----------------------------------------------------------------------------
// Destructor
MyArray::~MyArray() {

    OutputDebugString(L"Destructor\n");

  delete [] mArray;
    mArray = nullptr;
    mSize = 0;
}

// ----------------------------------------------------------------------------
static MyArray CreateArray(void){

    OutputDebugString(L"CreateOneArray begins\n");

    MyArray tmp(20);
  for(int i=0; i<20; i++)
    tmp[i]=2*i;

    OutputDebugString(L"CreateOneArray ends\n");
  return tmp;
}

// ----------------------------------------------------------------------------
int main(){

    // Really weird. Avoid the issue with max and the collision with Windows.h
    #define DUMMY

    OutputDebugString(L"Main begins\n");

    MyArray Tableau2(20);

    Tableau2 = CreateArray();
    for(int i=0; i<20; ++i)
        std::cout << Tableau2[i] << " ";
    std::cout << std::endl;

    std::cout << std::endl << std::endl << "Strike ENTER to exit :";
    std::cin.ignore( std::numeric_limits <std::streamsize>::max DUMMY (), '\n' );

    OutputDebugString(L"Main ends\n");
}
```

Pour faire court **une RValue Reference est une référence sur une RValue**. Super ! Et alors?

En C++11 une RValue est une variable non nommée. Typiquement la variable non nommée dont on parlait précédemment était une RValue. D'un autre coté, si j'appelle une fonction (qui attend une RValue Reference comme paramètre) en lui passant une expression (``MaFonction (x+y)`` par exemple), le résultat de l'expression ``x+y`` est une RValue.

Une autre façon de se faire une idée de ce qu'est une RValue consiste à dire que c'est **tout ce qui n'est pas une LValue** sachant qu'**une LValue est un truc qui possède une adresse** (une variable, un objet...).

Mouai... Et alors? En fait les rvalue references vont permettre, entre autres mais pas uniquement, d'implémenter deux fonctions spéciales supplémentaires dans notre classe :

1. Move Constructor
2. Move Assignment

L'intérêt de ces deux nouvelles fonctions c'est que le compilateur pourra, au moment de la compilation, choisir de les utiliser si le paramètre qu'elles vont utiliser est un objet temporaire sans nom. Dans ce cas, dans la définition de ces deux fonctions particulières au lieu de faire des copies (rappelez-vous que dans le Copy Constructor et dans le Assignment Operator précédents on avait des std::copy) on pourra se permettre de "voler" la propriété de l'objet temporaire et la passer à l'objet sur lequel on les applique.

Bon... Je me rend compte que tout cela à l'air un peu fumeux et qu'un exemple pratique serait le bien venu.

Imaginons que nous soyons le compilateur et que nous soyons rendu au point 6 précédent (on a une variable objet ``tmp`` initialisée et on s'apprête à appeler le Copy Constructor). Halte au feu! Je suis le compilateur. Je sais que ``CreateArray()`` retourne l'objet ``tmp`` et je sais aussi que ``tmp`` est une variable locale qui a une durée de vie limitée. Bref, je sais que j'ai un objet déjà initialisé qui va être jeté à la poubelle et que d'un autre côté je dois retourner une copie de cet objet. Soyons malin, je vois que dans la classe MyArray il y a une fonction Move Constructor et donc je prend la décision de l'invoquer lui plutôt que le Copy Constructor classique.

Comparons les définitions du Copy Constructor et du Move Constructor :

```cpp
// ----------------------------------------------------------------------------
// Copy ctor
MyArray::MyArray(const MyArray& other) : mSize(other.mSize), mArray(mSize ? new int[mSize] : nullptr){

  OutputDebugString(L"Copy Constructor\n");
  std::copy(other.mArray, other.mArray+mSize, mArray);
}

// ----------------------------------------------------------------------------
// Move ctor
MyArray::MyArray(MyArray&& other):mSize(other.mSize), mArray(other.mArray){

    OutputDebugString(L"Move Constructor\n");
    other.mSize=0;
    other.mArray=nullptr;
}
```

Dans la Initializer List du Copy Constructor classique, on construit et on initialise un objet. Si la taille du tableau qu'on va copier (``other.mSize``) est non nulle alors on créé un tableau d'entiers (``new int[mSize]``). Ensuite, dans le corps de la fonction, on effectue la copie proprement dite avec un ``std::copy``.

Dans le cas du Move Constructor la première chose à remarquer c'est le type du paramètre : ``MyArray&& other``. C'est la syntaxe à utiliser pour **définir une RValue Reference**. Notez aussi que **le paramètre n'est pas "const"**. C'est très important car, comme on va le voir, ce dernier va être modifié.

Pour le reste dans la Initializer List il faut noter que si on construit un objet, on ne perd pas de temps à créer un tableau. En effet et c'est là la ruse de sioux, on va voler la propriété de l'objet passé en paramètre et on va la transférer à un autre. Dans le code ci-dessus ``other.mArray`` est un pointeur valide qui pointe sur une zone mémoire déjà allouée et remplie. Sous réserve que l'on s'engage à ne plus l'utiliser, plutôt que de jeter cette zone mémoire à la poubelle autant la donner à l'objet qu'on est en train de construire. C'est pour cela que dans la Initializer List il n'y a pas d'allocation mémoire : il n'y en a tout simplement pas besoin.

Histoire de renforcer le fait qu'on ne va plus utiliser l'objet (other) sur lequel on invoque le Move Constructor, à la fin de la définition de la fonction, on met à zéro la taille du tableau (``other.mSize=0;``) et on annule le pointeur mArray (``other.mArray=nullptr;``)

Si on se raccroche aux différents points évoqués précédemment au point 7 le compilateur ne change rien et il détruit la variable objet ``tmp`` qui est dorénavant l'hôte d'un tableau de taille nulle et d'un pointeur nul aussi.

Au point 8 le compilateur invoque ``Tableau2.operator=(...)`` sur l'objet non nommé et prêt à l'emploi retourné par ``CreateArray()``. Encore une fois, je suis le compilateur. Je sais que l'objet passé en paramètre à la fonction ``operator=`` ne va pas survivre longtemps et je sais aussi que cet objet est initialisé, propre sur lui etc. Au lieu d'invoquer la fonction ``operator=`` classique qui m'oblige à faire un ``std::copy``, si j'ai le choix, autant invoquer la fonction ``operator=`` qui prend comme paramètre une RValue Reference dont je vais pouvoir "voler" la propriété.

Comparons maintenant les définitions des deux fonctions operator= :

```cpp
// --------------------------------------------------------------------
// Assignment operator by reference
MyArray& MyArray::operator=(const MyArray &rhs) {

  OutputDebugString(L"Assignment operator by reference\n");
    if(this!=&rhs){
        delete [] mArray;
        mSize = rhs.mSize;
        mArray = new int[mSize];
        std::copy(rhs.mArray, rhs.mArray+mSize, mArray);
    }
  return *this;
}

// ----------------------------------------------------------------------------
// Move assignement operator
MyArray& MyArray::operator=(MyArray&& rhs){

    OutputDebugString(L"Move assignment operator\n");
    if(this!=&rhs){
        delete [] mArray;
        mSize = rhs.mSize;
        mArray = rhs.mArray;
        rhs.mSize=0;
        rhs.mArray=nullptr;
    }
    return *this;
}
```

Afin qu'il n'y ait pas d'embrouille entre nous j'imagine que l'on étudie le code suivant : ``Tableau2.operator=(ObjetNonNommé)``.

Dans la première implémentation (la classique) on reçoit une référence constante sur laquelle le développeur c'est engagé à ne rien toucher. Après avoir vérifier que je ne perd pas mon temps sur une auto assignation (``if(this!=&rhs){...``) je commence par détruire la zone mémoire qui contient les données actuelles de Tableau2 (``delete [] mArray;``). Ensuite j'initialise le paramètre ``mSize`` avec la nouvelle taille (``mSize = rhs.mSize;``), j'alloue une nouvelle zone mémoire (``mArray = new int[mSize];``) et je termine en beauté par un superbe ``std::copy``.

Dans le cas du Move Assignment, là aussi on commence par recevoir un paramètre de type RValue Reference. Ici aussi le paramètre n'est pas ``const``.

Ensuite après avoir vérifié que je ne suis pas dans un cas d'auto assignement (``if(this!=&rhs){...``) je commence par détruire la zone mémoire qui contient les données actuelles de Tableau2 (``delete [] mArray;``). Attention, il s'agit d'une opération d'assignement et donc l'objet cible existe déjà et est initialisé. Il contient donc des données qu'il faut relâcher proprement avant de faire le Move.

Ensuite j'initialise le paramètre ``mSize`` avec la nouvelle taille (``mSize = rhs.mSize;``). À partir de là, comme je sais que la RValue Reference passée en paramètre est un objet "prêt à l'emploi", recyclable et qui ne sera plus utilisé par la suite, je vais me permettre de faire ce que l'on a fait précédemment dans le Move Constructor : on va voler le droit de propriété et le transférer à l'objet Tableau2.

Pour cela, on affecte directement à mArray la valeur du pointeur rhs.mArray et surtout on alloue rien dynamiquement (avec new int[...] par exemple) et on évite d'invoquer ``std::copy``.

À l'instar de ce que l'on a fait dans le move constructor, histoire de renforcer le fait qu'on ne va plus utiliser l'objet (rhs) à la fin de la définition de la fonction on met à zéro la taille du tableau (``rhs.mSize=0;``) et on annule le pointeur mArray (``rhs.mArray=nullptr;``)

Je peux comprendre que ce ne soit pas si facile à lire et à retenir. Ceci dit, à ce stade, la chose à retenir c'est qu'en enrichissant notre classe avec le **Move Constructor** et le **Move Assignment**, sous réserve que cela fonctionne, lors de l'exécution de la ligne ``Tableau2 = CreateArray();`` **on évite les deux std::copy** et on réalise vraiment ce que l'on voulait : le transfert quasiment direct de l'objet ``tmp`` vers l'objet ``Tableau2``.

Au fait... Est-ce que ça marche? Lançons le code et regardons le contenu de la fenêtre Output :

```bash
Main begins
Default Constructor
CreateOneArray begins
Default Constructor
CreateOneArray ends
Move Constructor
Destructor
Move assignment operator
Destructor
Main ends
Destructor
```

Bingo! Tout est identique à l'exception des lignes Move Constructor et Move assignment operator.

