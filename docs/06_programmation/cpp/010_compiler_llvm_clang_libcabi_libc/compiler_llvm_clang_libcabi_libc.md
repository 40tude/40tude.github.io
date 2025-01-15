---
layout: default
title: "Compiler llvm, clang, libc++abi et libc++"
parent: "C++"
date: 2013-05-15 01:39:04
last_modified_date: 2020-05-03 23:09:21
---


# Compiler llvm, clang, libc++abi et libc++
{: .no_toc }

## Introduction
{: .no_toc }

Les dates de rédaction et de dernière modification de ce type d'article sont importantes (voir le pied pde page du billet). Par exemple, j'ai rencontré des problèmes qui ont été résolu depuis. 

Pour fixer les idées, je rappelle que je suis sous Windows 7, dans une VirtualBox 4.2.12 où je fais tourner Debian 7.0 Wheezy 64 bits. J'espère toutefois que cet article donnera des pistes si jamais vous rencontrez des problèmes similaires.


## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}


## Articles qui peuvent être utiles
{: .no_toc }

Ne commencez pas par tout lire... Vous allez mourir car certains points sont un peu "chauds". Cela dit, dans quelques jours, quand vous en aurez marre et que vous voudrez tout jeter par la fenêtre... Il est possible que vous y trouviez, là aussi, 2 ou trois pistes.

1. <http://sylvestre.ledru.info/blog/>
1. <http://18739c.ece.cmu.edu/alpha-1/debugging-debian/tools/clang/>
1. <http://solarianprogrammer.com/2013/01/17/building-clang-libcpp-ubuntu-linux/>
1. <http://dragoonsheir.wordpress.com/2013/03/16/wayland-and-c11-programming-part-1-of-n/>
1. <http://clang.llvm.org/get_started.html>
1. <http://comments.gmane.org/gmane.comp.compilers.clang.devel/22540>
1. <http://lists.alioth.debian.org/pipermail/soc-coordination/2012-July/001316.html>
1. <http://lists.alioth.debian.org/pipermail/soc-coordination/2012-July/001316.html>







## Pour bien commencer

Bon allez, c'est parti... Dans VirtualBox, je commence par donner 3Go de RAM à la machine virtuelle (j'en ai 8 physiquement dans le PC) et j'en profite pour lui donner aussi 4 processeurs. Le coup des 3 Go est important, ne le négligez pas car sinon ça va partir en sucette à la compile ou au link. Je suppose que l''on démarre dans le répertoire ($HOME). On commence par récupérer ce dont on a besoin (oui, oui je sais on aurait pu mettre tous les apt-get sur une même ligne mais je trouve que c'est plus didactique comme ça.)

```bash
$ su
apt-get install cmake
apt-get install make
apt-get update
apt-get upgrade
apt-get install gcc
apt-get install g++
apt-get install subversion
exit
```

## llvm et clang

Ensuite il faut créer un répertoire ``llvm`` (ce n'est pas très malin comme nom, choisissez autre chose si possible) où on va télécharger llvm, clang et leurs amis...

```bash
$ mkdir llvm
$ cd llvm
$ svn co http://llvm.org/svn/llvm-project/llvm/trunk llvm
```

On a donc une arborescence qui ressemble à : $HOME/llvm/llvm/

{: .note } 
C'est pour ça que je disais que choisir "lvm" comme nom de premier répertoire dans $HOME n'était pas malin

```bash
$ cd llvm/tools
$ svn co http://llvm.org/svn/llvm-project/cfe/trunk clang

$ cd ../..
$ cd llvm/tools/clang/tools
$ svn co http://llvm.org/svn/llvm-project/clang-tools-extra/trunk extra
$ cd ../../../..

$ cd llvm/projects
$ svn co http://llvm.org/svn/llvm-project/compiler-rt/trunk compiler-rt
$ cd ~/llvm/
```

Histoire de ne pas "pourrir" notre belle arborescence on va créer un répertoire "build" pour créer llvm et clang. On aura donc 2 répertoires (llvm et build) dans le répertoire $HOME/llvm

```bash
$ mkdir build
$ cd build
$ ../llvm/configure --enable-optimized --enable-targets=host-only
```

Ça va prendre un peu de temps. Ceci fait, on lance la création de llvm et de clang avec make

```bash
$ make -j 8
$ su
# make install
# exit
```

Le ``-j 8`` propose à make d'utiliser jusqu'à 8 "jobs" en parallèle. Ça tombe bien on a donné 4 cœurs à la machine virtuelle. Cela dit l'opération va prendre beaucoup de temps mais normalement à ce stade ça doit rouler sans problème. Profitez-en peut être pour aller, sur le web, lire un ou deux des articles ci-dessus.

## Faisons un premier test

Faisons un test sur un code très, très simple.

```cpp
#include <iostream>
#include <vector>

using namespace std;

int main(){

  cout << "Hello World" << endl;

  vector<int> vec;
  vec.push_back(10);
  vec.push_back(20);
  for(int i:vec){
    cout << i << endl;
  }
  return 0;
}
```

Si on compile le code source en question avec g++

```bash
$ g++ -std=c++0x test1.cpp -o test1
```

Après avoir testé que tout marche bien, voilà ce que donne ldd

```bash
$ ldd test1
        linux-vdso.so.1 =>  (0x00007fffc11d8000)
        libstdc++.so.6 => /usr/lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007ff891901000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007ff89167f000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007ff891468000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007ff8910de000)
        /lib64/ld-linux-x86-64.so.2 (0x00007ff891c13000)
```

Maintenant qu'on a clang on peut aussi compiler ``test1.cpp`` avec :

```bash
clang++ -std=c++0x test1.cpp -o test1
```

Si on demande à ldd de nous indiquer les lib utilisées, voilà ce que l'on obtient :

```bash
$ ldd test1
        linux-vdso.so.1 =>  (0x00007fff2adff000)
        libstdc++.so.6 => /usr/lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007f260f583000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f260f301000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f260f0ea000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f260ed60000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f260f894000)
```

On remarque que l'exécutable utilise les bibliothèques fournies par gcc.

D'autre part, notez qu'on peut aussi invoquer la compilation via la ligne suivante :

```bash
$ clang++ -std=c++11 test1.cpp -o test1
```

Cela dit, même si la compilation du code test2.cpp (voir un peu plus bas) passe bien (pas de warning, rien...) à l'exécution ça part en vrille avec un message du style :

```bash
terminate called after throwing an instance of 'std::regex_error'
  what():  regex_error
Abandon
```

C'est normal. En effet test2 utilise des regular expression qui font partie des specs C++11, qui sont normalement supportées par clang mais pas encore par gcc. Enfin bref, il faut qu'on lie notre clang avec une bibliothèque qui supporte C++11.

## Faisons un autre test avec Sanitizer

```c
#include <stdlib.h>

int main() {

  char *x = (char*)malloc(10 * sizeof(char*));

  free(x);
  return x[5];
}
```

Si on compile avec :

```bash
$ clang -fsanitize=address test3.cpp -o test3
```

Lors de l'exécution voilà ce que l'on obtient :

```text
=================================================================
==3133==ERROR: AddressSanitizer: heap-use-after-free on address 0x60700000dfb5 at pc 0x42fcc3 bp 0x7fff85463610 sp 0x7fff85463608
READ of size 1 at 0x60700000dfb5 thread T0
    #0 0x42fcc2 (/home/philippe/devel/test3/test3+0x42fcc2)
    #1 0x7f087eaf9eac (/lib/x86_64-linux-gnu/libc.so.6+0x1eeac)
    #2 0x42fa2c (/home/philippe/devel/test3/test3+0x42fa2c)
0x60700000dfb5 is located 5 bytes inside of 80-byte region [0x60700000dfb0,0x60700000e000)
freed by thread T0 here:
    #0 0x421251 (/home/philippe/devel/test3/test3+0x421251)
    #1 0x42fc4e (/home/philippe/devel/test3/test3+0x42fc4e)
    #2 0x7f087eaf9eac (/lib/x86_64-linux-gnu/libc.so.6+0x1eeac)
previously allocated by thread T0 here:
    #0 0x421391 (/home/philippe/devel/test3/test3+0x421391)
    #1 0x42fc14 (/home/philippe/devel/test3/test3+0x42fc14)
    #2 0x7f087eaf9eac (/lib/x86_64-linux-gnu/libc.so.6+0x1eeac)
Shadow bytes around the buggy address:
  0x0c0e7fff9ba0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9bb0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9bc0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9bd0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9be0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
=>0x0c0e7fff9bf0: fa fa fa fa fa fa[fd]fd fd fd fd fd fd fd fd fd
  0x0c0e7fff9c00: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9c10: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9c20: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9c30: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x0c0e7fff9c40: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07
  Heap left redzone:     fa
  Heap right redzone:    fb
  Freed heap region:     fd
  Stack left redzone:    f1
  Stack mid redzone:     f2
  Stack right redzone:   f3
  Stack partial redzone: f4
  Stack after return:    f5
  Stack use after scope: f8
  Global redzone:        f9
  Global init order:     f6
  Poisoned by user:      f7
  ASan internal:         fe
==3133==ABORTING
```

## libc++abi et libc++

Pour pouvoir supporter C++11, il faut créer ``libc++abi`` et ``libc++`` (ABI = application binary interface). Afin de séparer les combattants je propose de créer un répertoire ``libstdcxx`` dans ``$HOME`` pour y construire les deux lib en question.

```bash
$ cd
$ mkdir libstdcxx
$ cd libstdcxx
$ svn co http://llvm.org/svn/llvm-project/libcxxabi/trunk libcxxabi
$ svn co http://llvm.org/svn/llvm-project/libcxx/trunk libcxx
```

À la fin, on a 2 répertoires ``libcxxabi`` et ``libcxx`` dans le répertoire $HOME/libstdcxx




### Création de libc++abi :

```bash
$ cd libcxxabi/include
$ echo | g++ -Wp,-v -x c++ - -fsyntax-only
```

Dans la sortie console, il faut noter la ligne qui ressemble à : ``/usr/lib/gcc/x86_64-linux-gnu/4.7/include``. Dans mon cas j'obtiens un truc du style :

```bash
/usr/include/c++/4.7
/usr/include/c++/4.7/x86_64-linux-gnu
/usr/include/c++/4.7/backward
/usr/lib/gcc/x86_64-linux-gnu/4.7/include
/usr/local/include
/usr/lib/gcc/x86_64-linux-gnu/4.7/include-fixed
/usr/include/x86_64-linux-gnu
/usr/include
```

Afin d'aider le compilateur, copions, à partir du répertoire qu'on vient d'identifier, le fichier ``unwind.h`` là où il l'attend :

```bash
$ cp /usr/lib/gcc/x86_64-linux-gnu/4.7/include/unwind.h .
```

{: .warning }
Il ne faut pas oublier le "." tout à la fin de la ligne ci-dessus. 


Ceci fait il faut éditer le script buildit

```bash
$ cd ../lib
$ nano buildit
```

Bon, là y a un peu de boulot. En effet il faut :

* Supprimer ``-stdlib=libc++`` de la ligne EXTRA_FLAGS
* Retrouver le case ``$TRIPLE`` puis un peu plus bas le case ``*)``
* Supprimer le ``-lstdc++`` de ``LDSHARED_FLAG`` (c'est tout à la fin de la définition)
* Retrouver la ligne ``for FILE in ../src/*.cpp; do``
* Ajouter ``-I../../libcxx/include`` juste après ``$EXTRA_FLAGS``

Sauver (CTRL+O), quitter (CTRL+X) nano puis taper dans la console

```bash
$ TRIPLE=-linux- ./buildit
```

Dans mon cas, cela n'a pas marché, j'avais des messages du style :

```bash
error: no member named 'aligned_alloc' in the global namespace
using ::aligned_alloc;
~~^
```

À la suite de diverses recherches et discussions (voir <http://dragoonsheir.wordpress.com/2013/03/16/wayland-and-c11-programming-part-1-of-n/> et <https://groups.google.com/forum/?fromgroups=#!topic/llvm-dev/5isiIWNpjv8> ) j'ai pu appliquer un patch. À partir d'un répertoire ``$HOME/tmp`` dans lequel j'avais un fichier ``linux2.diff``, j'ai dû faire :

```bash
$ cp ../libstdcxx/libcxx/include/__config ./__config.bak
$ patch -p0 --dry-run ../libstdcxx/libcxx/include/__config < linux2.diff
$ patch -p0 ../libstdcxx/libcxx/include/__config < linux2.diff
```

Une fois le patch appliqué on recommence la procédure afin de compiler ``lic++abi`` et dans la console on rentre la commande suivante :

```bash
$ TRIPLE=-linux- ./buildit
```

À la fin (si tout se passe bien) on se retrouve avec un fichier ``libc++abi.so.1.0`` dans ``./`` Il faut installer le fichier en question à la bonne place dans l'arborescence.

```bash
$ su
cp libc++abi.so.1.0 /usr/lib/
cd /usr/lib
ln -s libc++abi.so.1.0 libc++abi.so
ldconfig
cd -
exit
```




### Création de libc++ :

On approche du but! Il ne reste plus qu'à compiler ``libc++``. Dans le répertoire ``$HOME`` on va créer un répertoire build_libc++ à côté des répertoires ``libcxx`` et ``libcxxabi``.

```bash
$ cd ../../
$ mkdir build_libc++
$ cd build_libc++
$ CC=clang CXX=clang++ cmake -G "Unix Makefiles" -DLIBCXX_CXX_ABI=libcxxabi -DLIBCXX_LIBCXXABI_INCLUDE_PATHS="../libcxxabi/include" -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr ../libcxx
$ make -j 8
$ su
# make install
# exit
```

Ayé !!! Heu... En fait pas tout à fait. En effet, comme on a compilé clang avant ``libc++`` on ne peut pas utiliser clang uniquement avec ``-stdlib=libc++``. À ce stade voilà toutefois ce que l'on peut faire pour inciter clang à utiliser ``libc++`` et ``libc++abi`` :

```bash
$ clang++ -stdlib=libc++ test1.cpp -lc++abi -o test1
```

Si jamais le warning vous gêne on peut faire :

```bash
$ clang++ -std=c++11 -stdlib=libc++ test1.cpp -lc++abi -o test1
```




## Faisons un autre test

Si on compile ``test1`` avec la ligne ci-dessus et si on lance ldd dessus voilà ce que l'on obtient.

```bash
$ ldd test1
       linux-vdso.so.1 =>  (0x00007fffc51ff000)
       libc++abi.so.1 => /usr/lib/libc++abi.so.1 (0x00007fb8964d4000)
       libc++.so.1 => /usr/lib/libc++.so.1 (0x00007fb89621b000)
       libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fb895f98000)
       libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fb895d82000)
       libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fb8959f8000)
       libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007fb8957db000)
       librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007fb8955d3000)
       /lib64/ld-linux-x86-64.so.2 (0x00007fb89672c000)
```

Pour rappel voilà ce que l'on avait la dernière fois quand on avait compilé test1 avec clang

```bash
$ ldd test1
        linux-vdso.so.1 =>  (0x00007fff2adff000)
        libstdc++.so.6 => /usr/lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007f260f583000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f260f301000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f260f0ea000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f260ed60000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f260f894000)
```

On voit donc que dorénavant ``test1`` est lié avec les lib de clang (``libc++abi`` et ``libc++``).

{: .note } 
Pour le reste, je ne comprends pas trop la présence de ``libgcc_s``, ``libc`` etc. J'aurai pensé qu'elles auraient été remplacées par des lib issues de clang. Faudra que je cherche/comprenne avant de revenir sur le sujet (stay tuned).





## Compilation de clang avec clang

Puisque ``libc++`` est construite, on peut dorénavant compiler clang en utilisant ``libc++`` en lieu et place de ``libstdc++``. Super ! On va compiler clang avec clang !

```bash
$ cd ~/llvm/build
$ ../llvm/configure --enable-optimized --enable-targets=host --enable-cxx11=yes --enable-libcpp=yes --with-extra-ld-options="-lc++abi" 2>&1 | tee -a configure_trace.txt
$ VERBOSE=1 make -j 8 2>&1 | tee -a make_trace.txt
```

Rappelons-nous qu'avant on avait fait simplement :

```bash
$ ../llvm/configure --enable-optimized --enable-targets=host
$ make -j 8
```

Pour ceux qui le souhaitent, la liste des options possibles pour "configure" se trouve dans le fichier ``~/llvm/llvm/configure`` aux alentours de la ligne 1404 en ce qui me concerne. Au pire faites une recherche sur **Optional Features**. Bien sûr le ``VERBOSE=1`` permet d'avoir plus d'informations à l'écran lors du make. En fait j'ai été obligé d'utiliser ce mode car, comme on le verra, j'ai eu des problèmes. À propos du  ``2>&1 | tee -a filename.txt`` faut pas trop s'inquiéter. C'est juste un moyen d'avoir dans le fichier "filename.txt" la totalité de ce qui se passe à l'écran lors du ``configure`` ou du ``make``. 

Comme je l'ai dit, en ce qui me concerne j'ai eu des problèmes (encore) au niveau du ``make``. En gros j'ai rencontré 2 types de soucis.

### Premier type de problème

À priori y avait des problèmes de fichiers include. Pour résoudre le problème j'ai relancé la commande suivante :

```bash
$ echo | g++ -Wp,-v -x c++ - -fsyntax-only
```

Comme précédemment cela me donnait la liste des chemins d'include utiles. Ensuite il a fallu modifier le fichier InitHeaderSearch.cpp en faisant :

```bash
$ cd ~/llvm
$ nano llvm/tools/clang/lib/Frontend/InitHeaderSearch.cpp
```

Ensuite, dans le fichier en question il faut faire une recherche (CTRL W sous nano) du mot "FIXME:" et modifier le code en s'inspirant de ce qui suit :

```bash
// FIXME: temporary hack: hard-coded paths.
//AddPath("/usr/local/include", System, true, false, false);

AddPath("/usr/include/c++/4.7", System, true);
AddPath("/usr/include/c++/4.7/x86_64-linux-gnu/.", System, true);
AddPath("/usr/include/c++/4.7/backward", System, true);
AddPath("/usr/lib/gcc/x86_64-linux-gnu/4.7/include", System, true);
AddPath("/usr/local/include", System, true);
AddPath("/usr/lib/gcc/x86_64-linux-gnu/4.7/include-fixed", System, true);
AddPath("/usr/include/x86_64-linux-gnu", System, true);
AddPath("/usr/include", System, true);
```

Notez qu'il n'y a bien que 3 paramètres (et pas 5 comme on peut le voir sur d'autres sites web) dans la fonction AddPath. Ceci fait, on sauve (CTRL O), on quitte (CTRL X) et c'est parti. Rebelotte :

```bash
$ cd ~/llvm/build
$ ../llvm/configure --enable-optimized --enable-targets=host --enable-cxx11=yes --enable-libcpp=yes --with-extra-ld-options="-lc++abi" 2>&1 | tee -a configure_trace.txt
$ VERBOSE=1 make -j 8 2>&1 | tee -a make_trace.txt
$ su
# make install
# exit
```

### Deuxième type de problème

C'est ceux sur lesquels je suis en ce moment (18/05/2013). En fait avec la version que je viens de récupérer je n'ai plus rencontré les problèmes de chemins à modifier dans InitHeaderSearch.cpp (le patch a été appliqué). Dorénavant, tout se passe bien à la compilation mais ça ratatouille au link. Voilà ce que je vois ce soir :

```bash
llvm[4]: Linking Release+Asserts Shared Library libclang.so
clang++ -I/home/philippe/llvm/build/include -I/home/philippe/llvm/build/tools/clang/tools/libclang -I/home/philippe/llvm/llvm/include -I/home/philippe/llvm/llvm/tools/clang/tools/libclang  -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS$
          -lclangFrontend -lclangDriver -lclangTooling -lclangSerialization -lclangParse -lclangSema -lclangARCMigrate -lclangRewriteFrontend -lclangRewriteCore -lclangAnalysis -lclangEdit -lclangAST -lclangLex -lclangBasic -lclangForma$
llvm[4]: Building Release+Asserts Archive Library libclang.a
/bin/rm -f /home/philippe/llvm/build/Release+Asserts/lib/libclang.a
ar cru /home/philippe/llvm/build/Release+Asserts/lib/libclang.a /home/philippe/llvm/build/tools/clang/tools/libclang/Release+Asserts/ARCMigrate.o /home/philippe/llvm/build/tools/clang/tools/libclang/Release+Asserts/CIndex.o /home/philip$
ranlib /home/philippe/llvm/build/Release+Asserts/lib/libclang.a
make[4]: quittant le répertoire « /home/philippe/llvm/build/tools/clang/tools/libclang »
make[4]: entrant dans le répertoire « /home/philippe/llvm/build/tools/clang/tools/c-index-test »
llvm[4]: Compiling c-index-test.c for Release+Asserts build
if  clang -I/home/philippe/llvm/build/include -I/home/philippe/llvm/build/tools/clang/tools/c-index-test -I/home/philippe/llvm/llvm/include -I/home/philippe/llvm/llvm/tools/clang/tools/c-index-test  -D_DEBUG -D_GNU_SOURCE -D__STDC_CONST$
                then /bin/mv -f "/home/philippe/llvm/build/tools/clang/tools/c-index-test/Release+Asserts/c-index-test.d.tmp" "/home/philippe/llvm/build/tools/clang/tools/c-index-test/Release+Asserts/c-index-test.d"; else /bin/rm "/home$
llvm[4]: Linking Release+Asserts executable c-index-test (without symbols)
clang++ -I/home/philippe/llvm/build/include -I/home/philippe/llvm/build/tools/clang/tools/c-index-test -I/home/philippe/llvm/llvm/include -I/home/philippe/llvm/llvm/tools/clang/tools/c-index-test  -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTAN$
        -lLLVMBitReader -lLLVMAsmParser -lLLVMX86Disassembler -lLLVMX86AsmParser -lLLVMX86CodeGen -lLLVMSelectionDAG -lLLVMAsmPrinter -lLLVMMCParser -lLLVMCodeGen -lLLVMObjCARCOpts -lLLVMScalarOpts -lLLVMInstCombine -lLLVMTransformUtils$
/home/philippe/llvm/build/Release+Asserts/lib/libclang.so: undefined reference to `std::basic_string<char, std::char_traits<char>, std::allocator<char> >::basic_string(std::string const&)'
/home/philippe/llvm/build/Release+Asserts/lib/libclang.so: undefined reference to `std::basic_string<char, std::char_traits<char>, std::allocator<char> >::basic_string(char const*, std::allocator<char> const&)'
/home/philippe/llvm/build/Release+Asserts/lib/libclang.so: undefined reference to `std::string::_Rep::_M_destroy(std::allocator<char> const&)'
/home/philippe/llvm/build/Release+Asserts/lib/libclang.so: undefined reference to `std::string::_Rep::_S_empty_rep_storage'
/home/philippe/llvm/build/Release+Asserts/lib/libclang.so: undefined reference to `std::string::assign(std::string const&)'
clang: error: linker command failed with exit code 1 (use -v to see invocation)
make[4]: *** [/home/philippe/llvm/build/Release+Asserts/bin/c-index-test] Erreur 1
make[4]: quittant le répertoire « /home/philippe/llvm/build/tools/clang/tools/c-index-test »
make[3]: *** [all] Erreur 1
make[3]: quittant le répertoire « /home/philippe/llvm/build/tools/clang/tools »
make[2]: *** [all] Erreur 1
make[2]: quittant le répertoire « /home/philippe/llvm/build/tools/clang »
make[1]: *** [clang/.makeall] Erreur 2
make[1]: quittant le répertoire « /home/philippe/llvm/build/tools »
make: *** [all] Erreur 1
```

J'ai beau chercher pour l'instant je ne trouve pas de solution. Ce qu'il y a de bizarre c'est qu'en allant sur le web ce type d'erreur (``undefined reference``) ressemble aux erreurs générées lorsque c'est  ``libstc++`` (gcc) qui est liée au lieu de ``libc++`` (clang). À ce stade je cherche, cherche...

Un gros merci [fulvioesposito](http://dragoonsheir.wordpress.com/) qui via son blog m'a donné **LA** solution. Faut juste faire un ``make clean`` afin de mettre tout le monde d'accord. Les commandes à utiliser deviennent donc :

```bash
$ cd ~/llvm/build
$ make clean
$ ../llvm/configure --enable-optimized --enable-targets=host --enable-cxx11=yes --enable-libcpp=yes --with-extra-ld-options="-lc++abi" 2>&1 | tee -a configure_trace.txt
$ VERBOSE=1 make -j 8 2>&1 | tee -a make_trace.txt
$ su
# make install
# exit
```

Tout doit bien se passer maintenant





## Faisons un dernier test

Si on compile le code ``test2.cpp`` ci-dessous avec la commande suivante tout se passe bien :

```bash
$ clang++ -std=c++11 -stdlib=libc++ test2.cpp -lc++abi -o test2
```

Pour info voici le code de test2:

```cpp
#include <iostream>
#include <regex>
#include <string>

using namespace std;

int main(){
  string input;
  regex rr("((\\+|-)?[[:digit:]]+)(\\.(([[:digit:]]+)?))?((e|E)((\\+|-)?)[[:digit:]]+)?");
  //As long as the input is correct ask for another number
  while(true){
    cout<<"Give me a real number!"<<endl;
    cin>>input;
    //Exit when the user inputs q
    if(input=="q")
      break;
    if(regex_match(input,rr))
      cout<<"float"<<endl;
    else{
      cout<<"Invalid input"<<endl;
    }
  }
}
```

## Conclusion temporaire

Si on s'amuse à faire

```bash
$ ldd /usr/local/bin/clang
```

Voilà ce que l'on obtient

```text
linux-vdso.so.1 =>  (0x00007fffde0df000)
libc++abi.so.1 => /usr/lib/libc++abi.so.1 (0x00007f699b8be000)
libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f699b6a2000)
librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007f699b499000)
libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f699b295000)
libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f699b013000)
libc++.so.1 => /usr/lib/libc++.so.1 (0x00007f699ad58000)
libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f699ab42000)
libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f699a7b8000)
/lib64/ld-linux-x86-64.so.2 (0x00007f699bb16000)
```

Si maintenant on fait

```bash
ldd /lib/x86_64-linux-gnu/libgcc_s.so.1
```

Voilà ce que l'on voit :

```bash
linux-vdso.so.1 =>  (0x00007fffb27ff000)
libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f76b1bed000)
/lib64/ld-linux-x86-64.so.2 (0x00007f76b2198000)
```

Autrement dit si on arrive à s'affranchir de ``ligcc_s`` dans clang on supprimera aussi ``libc``. D'après ce que je crois avoir compris, la recommandation est de construire clang à partir de la lib compiler-rt pour y arriver. C'est ce sur quoi je suis actuellement. La suite au prochain épisode.




## Teaser à propos de la suite

*Ne perdez pas trop de temps ici. Pour l'instant cette section est surtout pour moi et c'est vraiment "en travaux".*

Pour finir, comment ça marche toutes ces lib ? Voilà ce que je crois avoir compris dans le cas de gcc

```text
Application - libstdc++ - libgcc-s - libc
```

* Application : c'est mon code c++
* libstdc++ : lib qui permet le support du c++ (STL)
* libgcc-s : bibliothèque partagée pour le support des exceptions
* libc :

Dans le cas de clang, finalement, si on veut une pile complètement indépendante de gcc, il faudra que l'on ait un truc du style

```text
Application - libc++ - libc++abi - libunwind - librt - libc
```

* Application : c'est mon code c++
* libc++ : c'est la lib qui permet le support du c++11 (STL, remplace libstdc++)
* libc++abi : lib de bas niveau
* libunwind : bibliothèque de support pour les exceptions (remplace libgcc-s)
* libcxxrt : c'est la bibliothèque runtime du compilateur (remplace libgcc-s)
* libc :

À ce stade il faut donc qu'on arrive à intégrer ``compiler-rt`` et ``libunwind`` dans notre pile. C'est les deux pièces manquantes. En effet on a ``clang``, ``libc++`` et ``libc++abi``.

Ce qui n'est pas très cool c'est qu'en cherchant un peu, d'un côté on nous dit que ``compiler-rt`` ne peut être crée toute seule (la lib fait partie de l'arborescence llvm) et d'un autre on nous dit qu'on peut créer ``libc++`` en s'appuyant sur ``libcxxrt`` mais que cette dernière doit être compilée et disponible là où le linker l'attend... Heu on se mord la queue là. Non ?

