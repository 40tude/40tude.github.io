---
layout: default
title: "Codingame C++ Niveau Medium"
parent: "C++"
#math: mathjax
date: 2015-05-31 14:59:03
last_modified_date: 2020-05-05 23:48:08
---


# Codingame C++ Niveau Medium
{: .no_toc }


{:.warning}
Cet article commence à dater (2015). La plupart des informations à propos des algorithmes doivent être toujours correctes mais il y a sans doute eu des évolutions dans le mauvais sens du côté de Codingame. Par mauvais sens je veux dire : vous empêcher de développer chez vous à votre rythme puis coller votre solution quand VOUS le souhaitez. Vous obliger à ne coder que sur leur plateforme, ne plus vous permettre de récupérer les jeux d'entrainement et les solutions pour travailler en autonome... J'espère me tromper mais il sera intéressant d'y retrouner de temps à autre et de vérifier si ce que je dis se réalise (ou pas) 🤞 



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}






<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mai 2016 je remarque 3 choses :

1. Cela fait un an que je n'ai pas joué sur Codingame (le temps passe vite...)
2. Plus gênant... Il semble que les fichiers test que l'on pouvait télécharger ont disparus. Je trouve ça nul. Je préfère de loin tester mon code dans Visual Studio Community puis faire un copier-coller sur Codingame plutôt que d'être "forcé" à développer dans une IHM Web. En plus les fichiers en question permettaient de cogiter dans le train et en tout cas partout où on n'avait pas nécessairement de liaison web. Nul je vous dis.
3. APU Init Phase passait l'année dernière. Je viens de faire un test vite fait. J'ai l'impression qu'il y a beaucoup plus de Test case et malheureusement il ne passe plus. Ça me fatigue un petit peu car si cela arrive sur ce "challenge" en particulier cela peut très bien arriver sur d'autres. J'ai pas du tout envie de tout tester à nouveau car rien ne dit qu'il ne faudra pas encore tout tester à nouveau dans 6 mois. Je trouve que ce n'est pas très pro comme attitude. Par exemple ils auraient pu mettre un APU Init Phase V1, APU Init Phase V2 etc. et/ou laisser le challenge dans son état initial. Bref, ça fait suer. Je vais mettre un tag sur les commentaires, un truc du style "marche en Mai 2016" afin d'indiquer la date à laquelle les explications que je donne se rapportent. Ensuite si l'équipe de Codingame change ses tests et/ou la nature du puzzle je n'y peux malheureusement pas grand-chose.

Pour le reste, plutôt que de publier le post dans son intégralité une fois celui-ci totalement "fignolé", je préfère publier rapidement et itérer le cas échéant. Il est donc normal que certains challenges soient encore sans commentaires (cela dit j'ai une solution pour chacun d'entre eux sur Codingame et donc les commentaires vont arriver un jour ou l'autre, c'est sûr).





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Avril-Mai 2015

Il faut remarquer que Codingame a réorganisé un peu les choses et/ou a créé de nouvelles sections (Optimisation par exemple) ou de nouveaux challenges (APU Init Phase dans la catégorie Medium par exemple) et donc c'est normal si cette page n'est pas totalement "raccord" avec le site.







<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Introduction
C'est le second post à propos de Codingame ([premier post](https://www.40tude.fr/mes-notes-a-propos-de-codingame/)). Je souhaite juste partager ici mes impressions de codage à propos de Codingame - Niveau Medium.



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## APU: Init Phase




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Skynet: the Virus

*Le cas échéant, les explications ci-dessous concernent un code qui marchait encore en Mai 2016.*

Voilà comment je voulais que la fonction main() soit écrite

```cpp
int main() {

  Graph MyGraph(cin);

  while (true) {
    int SkynetIndex;                                                            // The index of the node on which the Skynet agent is positioned this turn
    cin >> SkynetIndex;
    MyGraph.CounterStrike(SkynetIndex);
  }
}
```

J'ai donc une class Graph (qui représente le réseau de nœuds) qui est construite à partir de ce qui va être lu sur cin. Honnêtement je ne sais pas si c'est une bonne ou une mauvaise idée d'avoir cin comme paramètre du ctor mais bon ici ça va faire l'affaire. Pour le reste voilà la classe Graph et la structure Node :

```cpp
struct Node {
  list<int>  ChildNodes;                                                        // list (vs vector) allow to call remove()
  int DistToAgent = 0;                                                          // default value
  int Parent = -1;
};

class Graph {
public:
  Graph(istream& is);
  void CounterStrike(const int index);

private:
  void CutFirstLink(const int t_Agent, const int t_Node);
  void UpdatePathsToAgent(const int t_Agent);
  void FillWithConsole(istream& is, const int l);
  void SetExitsWithConsole(istream& is, const int e);

private:
  int m_NbNodes;                                                                // Number of nodes including the gateways
  vector<Node> m_Nodes;
  list<int> m_Exits;                                                            // list (vs vector) allow to call remove()
};
```

En effet, on voit dans la partie private de la classe qu'un Graph est représenté en interne par un vecteur de nœuds. Chaque nœud est en fait une structure qui comporte entre autres une liste de nœuds enfants. J'utilise une liste plutôt qu'un vecteur car il est plus facile (moins cher) de supprimer un élément en plein milieu d'une liste que d'un vecteur. De manière "accessoire" on trouve aussi dans la classe Graph, une liste avec les indices des nœuds qui sont des sorties. Là aussi c'est une liste car lorsqu'une sortie n'est plus connectée à aucun nœud il faudra la supprimer de la liste des sorties.

Il est peut-être pas inutile de remarquer dans la structure Node les valeurs par défaut qui sont donné à DistToAgent (distance to agent) et Parent (l'indice du parent).

Voilà le constructeur de la classe Graph et deux fonctions accessoires.

```cpp
Graph::Graph(istream& t_is) {
  int l, e;                                                                     // number of nodes, links and exits in the graph
  t_is >> m_NbNodes >> l >> e;
  m_Nodes.resize(m_NbNodes);
  FillWithConsole(t_is, l);
  SetExitsWithConsole(t_is, e);
}

void Graph::FillWithConsole(istream& t_is, const int t_NbLinks) {
  for (int N1, N2, i = 0; i != t_NbLinks; ++i) {                                // N1 and N2 defines a link between these 2 nodes
    t_is >> N1 >> N2;
    m_Nodes[N1].ChildNodes.push_back(N2);
    m_Nodes[N2].ChildNodes.push_back(N1);
  }
}

void Graph::SetExitsWithConsole(istream& t_is, const int t_NbExits) {
  for (int ExitIndex, i = 0; i != t_NbExits; ++i) {
    t_is >> ExitIndex;                                                          // the index of a exit
    m_Exits.push_back(ExitIndex);
  }
}
```

Le qualificatif const est pas mal utilisé. A noter aussi que les paramètres ont des noms du style t_XXX histoire de ne pas les confondre avec les variables locales des fonctions. A noter aussi la façon dont sont déclaré N1 et N2 dans la boucle for. L'idée est d'éviter de les déclarer dans le corps de la boucle mais bien dans l'initialisation de cette dernière.

Pour finir il faut remarquer que la liaison entre N1 et N2 étant bi-directionnelle, on ajoute N2 à la liste des nœuds connectés à N1 et on ajoute N1 à la liste des nœuds connectés à N2.

Dans la suite du code j'utilise une fonction UpdatePathsToAgent() qui calcule pour tous les nœuds du graphe, la distance de chacun vis à vis de la position de l'agent. Pour ce faire j'utilise un algorithme nommé Breadth First Search que l'on trouve facilement sur Wikipedia ou sur le web. Par rapport à ces implémentations le code ci-dessous n'utilise pas de couleur noire mais uniquement les couleur white et grey pour indiquer si un nœud a déjà été visité ou non.

```cpp
void Graph::UpdatePathsToAgent(const int t_Agent) {                             // Breadth First Search
                                                                                // http://www.personal.kent.edu/~rmuhamma/Algorithms/MyAlgorithms/GraphAlgor/breadthSearch.htm
  enum class ColorNode { WHITE, GREY };
  vector<ColorNode> Colors(m_NbNodes, ColorNode::WHITE);

  Colors[t_Agent] = ColorNode::GREY;
  m_Nodes[t_Agent].Parent = -1;
  m_Nodes[t_Agent].DistToAgent = 0;

  queue<int> q;
  q.push(t_Agent);

  while (!q.empty()) {
    auto u = q.front();
    q.pop();
    for (auto idx : m_Nodes[u].ChildNodes) {                                    // for each nodes of the list of child
      if (Colors[idx] == ColorNode::WHITE) {
        Colors[idx] = ColorNode::GREY;
        m_Nodes[idx].Parent = u;
        m_Nodes[idx].DistToAgent = m_Nodes[u].DistToAgent + 1;
        q.push(idx);
      }
    }
  }
}
```

La dernière fonction digne d'intérêt est celle qui s'occupe de couper le lien entre la sortie qui a été désignée comme étant la plus dangereuse et le premier nœud qui se trouve sur le chemin qui va de la sortie en question à l'agent. Le truc à remarquer c'est qu'outre que la fonction soit récursive, elle se charge aussi de supprimer de la liste des sorties, une sortie qui ne serait plus connectée à aucun autre nœud (ligne 9 ci-dessous).

```cpp
void Graph::CutFirstLink(const int t_Agent, const int t_Node) {                 // Cut the link between t_Agent and the first node on the path to the closest exit
                                                                                // When called, t_Node is an exit
  static auto NodeBefore = -1;
                                                                                // The node with no parent (parent==-1) is the one hosting the Agent
  if (t_Node == t_Agent) {                                                      // So here we reached the agent's node.
    m_Nodes[t_Agent].ChildNodes.remove(NodeBefore);                             // In Agent's list of child, remove NodeBefore index
    m_Nodes[NodeBefore].ChildNodes.remove(t_Agent);                             // In NodeBefore's list remove t_Node index
                                                                                // Z! If the NodeBefore the agent is an exit with no connection to other nodes then remove the exit from the list of exits
    if (find(m_Exits.begin(), m_Exits.end(), NodeBefore) != m_Exits.end() && m_Nodes[NodeBefore].ChildNodes.size() == 0 ) m_Exits.remove(NodeBefore);
    cout << t_Agent << " " << NodeBefore << endl;
  } else {
    NodeBefore = t_Node;
    CutFirstLink(t_Agent, m_Nodes[t_Node].Parent);
  }
}
```

Je crois n'avoir rien oublié d'essentiel. Pour vous donner un ordre d'idée, le code fait 123 lignes.








<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Heat Detector

*Le cas échéant, les explications ci-dessous concernent un code qui marchait encore en Mai 2016.*

Après avoir lu l'énoncé voilà ce que j'ai écrit

```cpp
int main() {
  int Larg, Haut;                                                               // width & height of the building.
  cin >> Larg >> Haut; cin.ignore();

  int N;                                                                        // maximum number of turns before game over.
  cin >> N; cin.ignore();

  int x, y;
  cin >> x >> y; cin.ignore();                                                  // Batman initial position

  Heroe Batman(x, y, Larg, Haut);

  while (true) {
    string NewDir;                                                              // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L, UL)
    cin >> NewDir; cin.ignore();
    Batman.ProcessNextMove(BombDir[NewDir]);
    cout << Batman.x << " " << Batman.y << endl;
  }
}
```

En gros on comprend que Batman (ligne 11) est une instance de type Heroe que l'on initialise avec x, y, Larg et Haut. Ensuite on a une boucle infinie dans laquelle, à chaque tour de boucle, on récupère l'information à propos de la nouvelle direction à suivre (ligne 15), on détermine le prochain mouvement à faire (ligne 16) et enfin on affiche les prochains x et y. Il faut sans doute remarquer que NewDir est de type string et que l'on invoque ProcessNextMove() en lui passant un paramètre un peu compliqué. C'est peut-être pas très heureux. Je réalise qu'il eut été plus malin d'écrire un truc du style :

```cpp
Batman.ProcessNextMove(NewDir);
```

Et de cacher la misère en gérant la conversion de la chaîne de caractères NewDir au sein de la fonction ProcessNextMove(). Bon allez, je fais la modif, là tout de suite, car la première mouture du code de la fonction main est vraiment trop moche.

Ensuite au début de la fonction afin de convertir la chaine NewDir qui peut valoir "U", "UR" etc. en un entier sur lequel il est plus facile de prendre une décision, voilà ce que j'écris :

```cpp
void Heroe::ProcessNextMove(string NewDir) {

  int dir = BombDir[NewDir];

  switch (dir) {
...
```

Pour comprendre ce qu'il se passe à la ligne 3 ci-dessous il faut comprendre que BombDir est en fait une map qui met en relation des chaines "U", "UR" etc. avec des variables entieres U, UR. Pour ce faire j'utilise un enum nommé Direction. Bombdir et Direction sont déclaré de la façon suivante :

```cpp
enum Direction { U, UR, R, DR, D, DL, L, UL };

map<string, int>BombDir{
  pair<string, int>("U", U),
  pair<string, int>("UR", UR),
  pair<string, int>("R", R),
  pair<string, int>("DR", DR),
  pair<string, int>("D", D),
  pair<string, int>("DL", DL),
  pair<string, int>("L", L),
  pair<string, int>("UL", UL)
};
```

Cela peut paraître un peu tordu. Cela dit, le truc, c'est que je souhaitais vraiment pouvoir écrire la fonction ProcessNextMove() de la façon suivante :

```cpp
void Heroe::ProcessNextMove(string NewDir) {

  int dir = BombDir[NewDir];

  switch (dir) {
    case U:
      // Faire ceci
    break;

    case D:
      // Faire cela
    break;

...
```

Du point de vue du codeur, du coup le code est simple à écrire et à maintenir. On voit bien que si la bombe est au-dessus (cas U) alors on fait un truc, si la bombe est en dessous (cas D) on fait autre chose etc. Une fois que la logique est encodée on met à jour les paramètres x et y du Heroe car on va les utiliser dans la boucle infinie de la fonction main() du code. La fin de la méthode ProcessNextMove() ressemble à :

```cpp
  ...
  x = ??????;
  y = ??????;
}
```

Pour finir la classe Heroe ressemble à ce qui suit :

```cpp
class Heroe {
public:
  Heroe(int a, int b, int l, int h) : x{ a }, y{ b }, MinX{ 0 }, MaxX{ l }, MinY{ 0 }, MaxY{ h } {};
  void ProcessNextMove(string NewDir);
public:
  int x;
  int y;
private:
  int MinX;
  int MaxX;
  int MinY;
  int MaxY;
};
```

On retrouve le constructeur qui initialise diverses variables internes et la fonction ProcessNextMove() dont l'objectif est de mettre à jour les paramètres x et y (coordonnées du prochain saut à effectuer). Dans ce challenge la logique n'est pas très compliquée. Par contre j'ai pris le temps de mettre en place les structures de données (la map BombDir, l'enum Direction etc.) me permettant d'écrire un code lisible (enfin je crois).

Pour vous donner un ordre d'idée, le code fait 99 lignes.





<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Paranoid Android




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Winamax Sponsored Contest



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mars Lander - Level 2



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Stock Exchange Losses



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Indiana - Level 1



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Teads Sponsored Challenge



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Network Cabling



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conway Sequence




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Telephone Numbers




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Dwarfs standing on the shoulders of giants



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Bender, a depressed robot



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Scrabble



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Gift



<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mayan Calculation


