---
layout: default
title: "Bonnes pratiques C++"
parent: "C++"
#math: mathjax
date: 2017-03-09 18:17:31
last_modified_date: 2020-05-03 15:02:18
---

# Bonnes pratiques C++

{: .note }
Comme d'autres pages du site, celle-ci va évoluer au cours du temps en fonction de ce que j'aurai lu ou vu. Il ne faut donc pas s'affoler s'il y a des sections vide pour l'instant.


## Introduction

Je mets ici des liens que je trouve utiles et qui concernent les bonnes pratiques de développement en C++. L'idée est de garder sous la main des "références" dont il est bon de s'inspirer afin de produire du code qui soit "Expressif". De mon point de vue, "Code Expressif" cela signifie du code qui est agréable à lire (si, si c'est possible), que l'on peut facilement s'approprier et qui a la capacité à exprimer l'intention de l'auteur au lecteur (voir [ici](https://medium.com/%40daniel.oliver.king/writing-expressive-code-b69ef7a5a2fa#.cynqxz1z2)). Sans entrer dans les détails de ce dernier point, cela signifie par exemple que l'auteur a judicieusement nommé ses variables et que la majorité de son code est constituée de fonctions et de structures de codages qui sont comprises par 80% des développeurs.

Qu'on le veuille ou non, le principal intérêt de cette démarche est économique et c'est pourquoi l'Expressive Code est si important. En effet, si on peut transmettre facilement le code à d'autres personnes cela veut dire aussi qu'on ne "perdra pas de temps" en training ou en formation lors du passage de l'un à l'autre. Cela veut dire aussi que l'on peut se permettre d'avoir des qualifications différentes pour des tâches différentes sur un même code. Par exemple, ceux qui maintiennent le code ne sont pas payé au même tarif que ceux qui l'ont conçu. Cela permet aussi de mettre sur le marché des applications dont les sources sont viables plus longtemps. Si on peut facilement relier et comprendre les codes sources alors, en cas de reprise de code, on évite le risque de devoir tout réécrire. De manière plus accessoire (quoique) cela veut dire aussi que l'on n'a pas "honte" des sources qu'on dépose sur GiHub. De plus, le code étant facile à s'approprier, on embarquera plus facilement du monde dans le projet. Enfin un code Expressif doit nous permettre de revenir dessus 2 ans après sans trop de difficultés.

On ne parlera pas du tout ici de mise en forme de code. Il existe pour cela de très bons outils qui évitent les guerres de religion et qui permettent très facilement à chacun d'appliquer "son" style aux codes source qu'il reçoit (voir cette page d'introduction à propos de [clang-format](https://www.40tude.fr/a-propos-de-clang-format/) ou les outils de Visual Studio). Cela dit, la façon dont le code est formaté participe à sa bonne compréhension.

Nous ne parlerons pas non plus ici des choix qu'il y aurait à faire entre Performances et Expressivité du Code. En effet, je fais le choix de dire qu'avant tout, mon code doit être Expressif et qu'ensuite, et uniquement ensuite, on intéressera à ces histoires de performances. C'est un choix et en tant que tel on peut en discuter. Cela dit, cette décision a le mérite d'être respectueuse du vieil adage qui dit que "Premature optimization is the root of all evil" ([Donald Knuth](http://web.archive.org/web/20130731202547/http%3A//pplab.snu.ac.kr/courses/adv_pl05/papers/p261-knuth.pdf)) que l'on traduit en Français par "on ne met pas la charrue avant les bœufs" (oui, oui je sais la traduction est libre...)

Dans l'ordre il faut donc :

1. Comprendre le problème à résoudre
2. Sortir un code qui couvre les spécifications du problème et que chacun peut comprendre.
3. Enfin et si besoin, voir comment optimiser les choses. Pour cela on pourra :
   1. Garder le code initial et dans le cadre d'une optimisation en vitesse, après une phase de Profiling, s'intéresser au 20% du code dans lesquels on passe 80% du temps ([Pareto](https://fr.wikipedia.org/wiki/Principe_de_Pareto)). Dans la zone d’intérêt on améliorera les choses en faisant en sorte que le code reste Expressif (compréhensible). Pour fixer les idées, un exemple basique pourrait être l'utilisation d'un vecteur plutôt que d'une liste chaînée ou bien l'utilisation de reserve() pour éviter le redimensionnement dynamique d'un vecteur.
   2. Si cela ne suffit pas on pourra répondre au problème avec une approche différente. Par exemple, un tri un peu plus smart que l'approche de type force brute que l'on avait utilisé en première instance. Attention, il va de soi que la nouvelle stratégie est codée "proprement" et qu'il en résulte un Code Expressif.
   3. Si cela ne suffit toujours pas il faudra peut-être taper dans le dur. Par exemple, ne plus utiliser des structures de données génériques pour contenir des données spécifiques mais au contraire optimiser l'organisation des structures en fonction des données qu'elles contiennent (Data Optimization)









## Conseils et recommandations

### Les bases

1. Compiler en W4 et traiter les warning comme des erreurs
2. Compiler en activant les [Cpp Core Guidelines](https://github.com/isocpp/CppCoreGuidelines) (pas encore sûr à propos de [GSL](https://github.com/Microsoft/GSL) (Guideline Support Lib))

### Vidéos

* 2014 : [Modernizing Legacy C++ Code](https://www.youtube.com/watch?v=LDxAgMe6D18)
  + Compile at W4 (not /Wall). Then when no warning treat warning as error.
  + Compile C code as C++
  + Avoid #ifdef or keep it simple. Avoid Macro as well
  + Use RAII everywhere (see 24:09 and after. Check std::nothrow)
  + Introduce exception but carefuly
  + Const qualify everything
  + Eliminate C cast and use C++ cast where necessary
  + Avoid raw loops where possible
* 2014 : [Making C++ Code Beautiful](https://www.youtube.com/watch?v=BiYliKliFvs&t=1037s)
  + Macros are ugly
  + Walls of code are ugly
  + Lambda are OK
  + Invisible code is OK
  + Removing effort is OK
  + Use libraries (STL)
  + Comments are ugly (use GIT)

### Pages & Sites Web utiles

* [Fluent{C++}](http://www.fluentcpp.com/posts/)
* 2016 [Writing Expressive Code](https://medium.com/%40daniel.oliver.king/writing-expressive-code-b69ef7a5a2fa#.cynqxz1z2) (Medium)
  + Exprimer clairement l'intention du programmeur
  + Les codes sont lus plus souvent qu'ils ne sont écrits
* 2013 [Writing expressive code](https://robinwinslow.uk/2013/11/22/expressive-coding/) (Robin)
  + *Writing expressive code may help future coders to understand what's going on. It may even help you in the future. But it may also help you simply to understand the problem. Thinking carefully about how to define and encapsulate the components of your solution will often help you to understand the problem better, leading to a more logical solution.*
  + Self-documenting code : bien nommer variables et fonctions
  + Don't abbreviate : UserIdentifier vs UiD
  + Be specific : NewLinkClickEvent vs Event
  + Encapsulation : Cache misère dans une fonction
  + Do you need an else? : À lire
  + Depth : À lire
  + Tidiness : Code Format
  + Delete commented out code : GIT fera le reste
  + Trade-offs : expresiveness vs succinctness
  + Depth vs. encapsulation : penser à composer les objets
  + Special syntax : if then else vs Bob = 12 ? "Oui" : Non"
  + Single-use variables : mouai... à relire.
* [What Makes Readable Code: Not What You Think](https://simpleprogrammer.com/2013/04/14/what-makes-code-readable-not-what-you-think/) (Simple Programmer)
  + *An experienced developer is more focused on the actual concept being expressed by the code—what the purpose of the code is, not how it is doing it.*
  + *When a less experienced developer reads code, they are trying to understand the actual structure of the code. A beginner is more focused on the actual vocabulary of the language than what the expression of that language is trying to convey.*
* [Source Code Comments](https://blog.quasardb.net/source-code-comments/) (QuasarDb)

### Livres

* 2014 : [Effective Modern C++](https://www.amazon.fr/Effective-Modern-C-Scott-Meyers/dp/1491903996/ref%3Dsr_1_1?ie=UTF8&qid=1489072425&sr=8-1&keywords=Effective+Modern+C%2B%2B)
* 2005 : [Effective C++](https://www.amazon.fr/Effective-Specific-Improve-Programs-Designs/dp/0321334876/ref%3Dpd_sim_14_1?_encoding=UTF8&psc=1&refRID=F6NX98S5KCC1AZRQZHD1), Third Edition, Scott Meyer







## Apprendre la STL

### Les bases

* [for_each](http://en.cppreference.com/w/cpp/algorithm/for_each)
* [transform](http://en.cppreference.com/w/cpp/algorithm/transform)
* [accumulate](http://en.cppreference.com/w/cpp/algorithm/accumulate)
* partition
* rotate
* all_of
* any_of
* find
* find_if
* binary_search
* lower_bound
* upper_bound
* equal_range
* sort
* stable_sort
* copy_if
* unique
* remove

### Vidéos

* 2015 : [STL Algorithms in Action](https://channel9.msdn.com/events/CPP/CppCon-2015/CPPConD02V028)
* 2013 : [C++ Seasoning](https://channel9.msdn.com/events/GoingNative/2013/Cpp-Seasoning)
* 2012 : [STL11: Magic && Secrets](https://channel9.msdn.com/events/GoingNative/GoingNative-2012/STL11-Magic-Secrets)
* 2011 : [C9 Lectures: Stephan T Lavavej - Advanced STL, 1 of n](https://channel9.msdn.com/Series/C9-Lectures-Stephan-T-Lavavej-Standard-Template-Library-STL-/C9-Lectures-Introduction-to-STL-with-Stephan-T-Lavavej)
* 2010 : [C9 Lectures: Stephan T. Lavavej - Standard Template Library (STL), 1 of n](https://channel9.msdn.com/Series/C9-Lectures-Stephan-T-Lavavej-Standard-Template-Library-STL-/C9-Lectures-Introduction-to-STL-with-Stephan-T-Lavavej)

### Pages & Sites Web

* Fluent{C++} : [STL Learning ressource](http://www.fluentcpp.com/stl-learning-resource/)

### Livres

* 2012 : [The C++ Standard Library: A Tutorial and Reference](https://www.amazon.fr/Standard-Library-Tutorial-Reference/dp/0321623215/ref%3Dsr_1_1?ie=UTF8&qid=1489072845&sr=8-1&keywords=0321623215)



*La suite au prochain épisode...*

