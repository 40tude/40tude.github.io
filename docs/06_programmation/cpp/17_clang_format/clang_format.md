---
layout: default
title: "clang-format"
parent: "C++"
#math: mathjax
date: 2017-02-23 23:30:08
last_modified_date: 2020-05-03 14:18:53
---

# clang-format

Je suis sous Windows 10, dans une console classique ou une console Powershell. Pour une première invocation, on peut essayer :

```bash
clang-format --style=llvm -dump-config
```

Cela permet de lister les paramètres du style LLVM. Il y a d'autres styles "de base" possibles (Google, Chromium, Mozilla etc.). Ensuite on peut sauvegarder la configuration dans un fichier ".clang-format" (fichier caché) ou bien "_clang_format". Cela permet de partir d'une base puis de la modifier par exemple. Voilà la ligne de commande à invoquer pour générer un fichier caché :

```bash
clang-format --style=Mozilla -dump-config > .clang-format
```

Enfin, on peut appliquer ce paramétrage sur un code source en particulier

```bash
clang-format -i --style=file MyCode.cpp
```

Il faut noter 2 choses :

1. L'option -i qui dit, "vas-y fais toi plaise, insère directement les modifications dans le fichier
2. L'option --style qui cette fois est réglée sur "file"

Dans mon cas par exemple voilà le début du code source original :

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER

#include <iostream>
#include <vector>
#include <string>
#include <list>

using namespace std;

// ----------------------------------------------------------------------------
bool has_c1(const string& s, char c){                                           // does string s contain the character c?

  auto p = find(s.begin(), s.end(), c);
  if (p != s.end())
    return true;
  else
    return false;
}
```

Voilà ce que cela donne après le formatage "à la Mozilla" :

```cpp
#ifdef _MSC_VER
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif // _MSC_VER

#include <iostream>
#include <list>
#include <string>
#include <vector>

using namespace std;

// ----------------------------------------------------------------------------
bool
has_c1(const string& s, char c)
{ // does string s contain the character c?

  auto p = find(s.begin(), s.end(), c);
  if (p != s.end())
    return true;
  else
    return false;
}
```

J'ai choisi Mozilla car sinon, compte tenu de la façon dont je formate mes sources par défaut, il y a peu ou pas de différence avec le style LLVM. Ici au moins on voit que les variables de retour des fonctions sont sur leur propre ligne, que les commentaires sont rapprochés etc.

Bon après, c'est une guerre de religion qui commence et/ou quelques heures de réglages fins afin de créer son propre style (qui peut bien sûr s'appuyer sur LLVM, Google, Mozilla etc.). Il suffit d'éditer le fichier _clang-format. En tout cas la [page des options de styles](https://clang.llvm.org/docs/ClangFormatStyleOptions.html) explique très bien tout ce qu'il est possible de faire. Je me demande même s'il n'y a pas encore plus d'option que dans l'outil de remise en forme des sources de Visual Studio (je pense par exemple à l'alignement des déclarations consécutives de variables ou des affectations consécutives etc.). Je ne sais pas. Faudrait qu'un spécialiste confirme/infirme ce point.

Un seul regret. Il n'y a pas, comme dans Visual Studio, un moyen de configurer graphiquement son fichier .clang-format. J'ai quand même trouvé cette [page](https://clangformat.com/) ou mieux encore [celle-là](https://zed0.co.uk/clang-format-configurator/). Il ne faut surtout pas hésiter à utiliser l'éditeur à droite, à modifier le code source et à appliquer les modifications (bouton Update!)

### À lire :

* clang-format -help
* [L’aide en ligne](https://clang.llvm.org/docs/ClangFormat.html) de clang-format
* La page [Clang-Format Style Options](https://clang.llvm.org/docs/ClangFormatStyleOptions.html)
* [Configuration en ligne du fichier .clang-format](https://zed0.co.uk/clang-format-configurator/)
* Allez lire aussi cette [page](http://www.labri.fr/perso/fleury/posts/programming/using-clang-tidy-and-clang-format.html)

