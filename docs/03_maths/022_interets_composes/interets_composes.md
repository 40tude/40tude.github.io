---
published: true
author: 40tude
lang: fr-FR
layout: default
title: "Intérêts Composés"
description: "."
image: docs/03_maths/022_interets_composes/assets/img00.webp
twitter:
  card: summary_large_image
parent: "Maths"
# nav_order: 36
math: mathjax
date:               2026-11-10 11:00:00
# last_modified_date is updated by .git/hooks/pre-commit
last_modified_date: 2026-11-13 10:01:30
---



# {{ page.title }}
{: .no_toc }

{{ page.description }}
{: .lead }





<h2 align="center"><span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span></h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* Ne pas oublier les frais composés


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img00.webp"
    alt="."
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>.</figcaption>
</figure>










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Introduction

Y a deux ans, j'étais allé chez Mamie et à l'occasion de mon 16 eme anniversaire on avait fêté ce dernier à sa manière. Je vais pas me plaindre, j'étais sorti de là avec 5 500€ de plus sur mon compte. Si tu ne me crois pas, ou si tu te pose des questions à propos des pourcentages, vas lire cet [excellent article]({%link docs/03_maths/002_pourcentages/pourcentages.md%}).

Cette année c'est bizarre. On a fêté mes 18 ans à la maison avec les parents et dans la soirée Mamie m'a envoyé un message WhatApps un peu énigmatique me demandant de venir la voir ce matin à 10H00 avec mon PC portable...

J'aime toujours autant ma grand-mère alors ni une ni deux, à 10H00 pétante je sonne à sa porte.

Salut Marty chéri. Je sais que tu as un truc de prévu ce midi avec tes potes alors on va faire court et efficace. Tiens, prends un café et assieds toi là. Alleluia, il va pleuvoir des grenouilles demain mais pour une fois ton père m'a écouté... La semaine dernière il t'a ouvert un [PEA Jeune](https://www.yomoni.fr/apprendre/pea-jeune). En deux mots c'est une enveloppe fiscale. Arrête de me regarder avec ces yeux là tu vas voir c'est simple. En gros tu vas pouvoir l'utiliser pour acheter, entre autres, des actions. Tu ne pourras pas y déposer plus de 20 k€ mais même si un jour tes intérêts font que la somme montant+dividende va au delà de ce montant, ce ne sera pas un problème. En effet seuls les versements sont limités à 20 k€, pas la somme totale. Mais bon, à ce stade on s'en fout car à mon avis tu ne seras pas concerné. Quand tu quittera fiscalement le foyer fiscal de tes parents (t'es pas un Tanguy, rassure moi, tu vas partir avant tes 25 ans) il sera automatiquement transformé en PEA. Là ce sera cool car il ne perdra pas son antériorité, tu pourras le garder toute ta vie, y déposer jusqu'à 150 k€ et comme il aura plus de 5 ans, les taxes à payer seront un peu allégées.

Ok Mamie, super, mais j'ai rien compris... T'es bien le fils de ton père! Gentil mais il percute pas vite. Bon, alors là tout de suite ce qu'il faut que tu retiennes c'est que tu a un compte, que tu peux acheter des actions, laisser l'argent travailler pour toi etc. Moi, ce que je veux voir avec toi ce matin, c'est comprendre comment ça marche afin que tu puisse en tirer le meilleur. Et tu vas voir que c'est encore une question d'intérêts composés.

Alors mon ptit gars... Est-ce que tu te rappelle le marché que je t'avais proposé y a 2 ans et le raisonnement que tu avais dû poser afin de me répondre? Tu avais le choix entre 1 000 € tout de suite ou la somme correspondant à 1 000 € placé à 5% pendant 35 ans.

Tu m'étonnes. Bien sûr que je m'en rappelle. Je crois que c'est LA formule que je n'oublierai jamais...






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## La formule

C'était un truc du style :

$$V_n = V_0 * (1 + 5\%)^n$$

Dans lequel $$V_n$$ c'était la valeur au bout de $$n$$ années, $$V_0$$ le montant initial et $$n$$ le nombre d'années. Tu vois, je m'en rappelle comme si c'était hier. Mais bon pourquoi tu me pose la question?

Patience petit Padawan... Bon t'es plus un gamin, est que tu peux m'écrire la formule qui donne le gain au bout de $$n$$ années?

Ben oui... Je vais partir de l'égalité précédente. Ensuite j'ai envie d'écrire un truc du style:
$$
\begin{align*}
\textrm{Montant final} &= \textrm{Montant initial} * (1 + 5\%)^n \\
\textrm{Gain} &= \textrm{Montant final} - \textrm{Montant initial} \\
\textrm{Gain} &= \textrm{Montant initial} * (1 + 5\%)^n - \textrm{Montant initial} \\
G &= M * (1 + 5\%)^n - M \\
G &= M [(1 + 5\%)^n - 1] \\
G &= M [(1 + r\%)^n - 1] \\
G &= M [(1 + r)^n - 1]
\end{align*}
$$

où $$M$$ est la mise initiale, $$r$$ le rendement annuel et $$n$$ le nombre d'années.

OK Marty. Imagine que pour tes 18 ans je te donne 1000 € à placer sur ton PEA Jeune. Non, non ne me remercie pas car tu ne les a pas encore gagné. J'imagine que dans ton école vous apprenez à écrire des scripts en Python. Si c'est pas le cas, sors de cette maison je te déshérite, hahaha! Plus sérieusement, ouvre ton PC et écris moi un script de 3 lignes qui calcule le gains au bout de 35 ans de tes 1 000 € placés à 5%.

OK... Du coup j'ouvre mon PC puis VSCode et dans un notebook `interets_composes.ipynb` j'écris:

```python
M = 1_000  # mise initiale (EUR)
r = 0.05   # rendement annuel
n = 35     # nombre d'années

G = M * ((1 + r) ** n - 1)

print(f"Montant final : {M + G:,.2f} €".replace(",", " "))
print(f"Gain          : {G:,.2f} €".replace(",", " "))
```

À l'exécution je vois :

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img02.webp"
    alt="Calcul du gain de 1 000 € placés à 5% sur 35 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Calcul du gain de 1 000 € placés à 5% sur 35 ans</figcaption>
</figure>

Mamie, c'est pas pour me vanter mais l'honneur est sauf. On retrouve bien les 5 500 € de mes 16 ans mais ici comme on imagine que la mise de fond est de 1 000 €, à la fin le gain est de 4 500 € environ.

Bien, bien, bien... Je vois que tu as au moins appris 1 ou 2 trucs dans ton école. Est-ce que tu peux afficher tout ça dans un graphe?

Tes désirs sont des ordres...

```python
import matplotlib.pyplot as plt

M = 1_000  # mise initiale (€)
r = 0.05   # rendement annuel
N = 35     # duree max (annees)

annees = list(range(N + 1))
gains = [M * ((1 + r) ** n - 1) for n in annees]

fig, ax = plt.subplots(figsize=(9, 5))
ax.plot(annees, gains, marker='o', markersize=3)
ax.set_xlabel("Années")
ax.set_ylabel("Gain (€)")
ax.set_title(f"Évolution du gain de {M:,.2f} € placés à {r:.0%} sur {N} ans".replace(",", " "))
ax.grid(True, alpha=0.3)
fig.tight_layout()
plt.show()

```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img03.webp"
    alt="Évolution du gain de 1 000 € placés à 5% sur 35 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Évolution du gain de 1 000 € placés à 5% sur 35 ans</figcaption>
</figure>



Super. Maintenant on peut commencer à discuter. Tiens, affiche moi les 3 graphes quand on double
1. le montant initial (2 000€ au lieu de 1 000 €)
1. le rendement (10% au lieu de 5)
1. la durée (70 ans au lieu de 35)

Voilà ce que je vois. Respectivement 9 000 €, 27 000 € et 29 000 €

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img04.webp"
    alt="Évolution du gain de 2 000 € placés à 5% sur 35 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Évolution du gain de 2 000 € placés à 5% sur 35 ans</figcaption>
</figure>

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img06.webp"
    alt="Évolution du gain de 1 000 € placés à 10% sur 35 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Évolution du gain de 1 000 € placés à 10% sur 35 ans</figcaption>
</figure>

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img05.webp"
    alt="Évolution du gain de 1 000 € placés à 5% sur 70 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Évolution du gain de 1 000 € placés à 5% sur 70 ans</figcaption>
</figure>


Et du coup Marty qu'est-ce qui te vient en tête?

J'ai pas l'impression que de doubler la mise au départ soit si intéressant. En fait si, c'est rentable car on gagne 2 fois plus (9k au lieu de 4.5k) mais bon c'est linéaire. Dans les autres cas, c'est une toute autre histoire. Au bout de 70 ans, troisième cas, j'aurai une hanche en titane, plus de dents mais je pourrai m'acheter une Rolex avec les 29 k€ de gains mais... Le mieux, c'est peut être le compromis du second cas et de trouver un placement qui double les intérêts car alors à 53 ans ($$18+35=53$$) je pourrai alors m'acheter et profiter pendant quelques années encore d'une très belle moto à 27 000 balles.

Tu es mon digne petit fils. Oui, il y a 3 paramètres indépendants dans ton équation et en jouant sur les valeurs, on voit que certains ont plus d'influence que d'autres. Alors... Commence pas à faire ta mijaurée ou ton wokiste de base. Imagine... Imagine que tu te balades sur une plage et qu'en marchant, tu rattrapes une fille que tu vois de dos. Qu'est ce qui va faire que tu va vouloir engager la conversation? La longueur des cheveux, ses fesses, sa hauteur... Bon ben voilà, t'as tout compris. Là aussi y a plusieurs paramètres mais certains sont plus importants que d'autres.

Du coup, la question à 1M$ c'est: dans le cadre de ton équation avec ses 3 paramètres, est-ce que tu es capable d'aller plus loin que les tests de doublement de valeur qu'on a fait et de retrouver, par calcul, l'influence de chacun d'entre eux?

Ouai... Normalement c'est possible. On a vu ça en physique et mon prof qui lui n'est pas un obsédé avait un exemple tout aussi pertinent. Il disait: imaginez que vous êtes au sommet d'une petite colline avec votre snowboard. Le terrain c'est une surface dont l'altitude est une fonction de $$x$$ et de $$y$$, un truc du style $$z = f(x, y)$$. Du coup si tu pars vers le nord, $$x$$ ne change pas, seul $$y$$ varie et la pente est peut être douce. Si tu pars vers l'ouest, $$y$$ ne change pas, seul $$x$$ varie et la pente est peut être plus prononcée, et donc plus de fun. Donc les gars, pas de panique, calculer une dérivée partielle ça revient à étudier la formule initiale en bloquant tous les paramètres sauf un et à regarder alors ce qui se passe.

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img07.webp"
    alt="Dérivées partielles en snowboard"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Dérivées partielles en snowboard</figcaption>
</figure>

Allez, c'est parti.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Dérivation partielle

Je reprends la formule de départ.

$$
G = M \left[ (1+r)^n - 1 \right]
$$



### Qu'est-ce qui se passe si le montant varie?

Je ré-écris l'équation sous la forme

$$
y = x \left[ (1+r)^n - 1 \right]
$$

Il faut bien voir que $$x$$ est la variable et que $$r$$ et $$n$$ doivent être considérés comme des constantes. À l'extreme, on pourrait écrire l'équation précédente sous la forme :

$$
\begin{align*}
y = x \left[ (1+r)^n - 1 \right] \\
y = x K \\
y = K x
\end{align*}
$$

Si je dérive j'obtiens

$$
\begin{align*}
y^\prime &= K \\
y^\prime &= \left[ (1+r)^n - 1 \right] \\
y^\prime &= (1+r)^n - 1 \\
G^\prime &= (1+r)^n - 1
\end{align*}
$$


Mais comme on veut se rappeler que ce qu'on a sous les yeux, c'est la variation du gain $$G$$ quand le montant $$M$$ varie, on écrit pas $$G^\prime$$ mais plutôt $$\frac{\partial G}{\partial M}$$. Un c'est trop la classe et deux ça veut juste dire, dérivée partielle de $$G$$ par rapport à $$M$$. Du coup j'écris :


$$
\frac{\partial G}{\partial M} = (1+r)^n - 1
$$

Et tu comprends ça comment mon petit Marty?

Ben... Je me dis que si $$r$$ et $$n$$ sont des constantes, 5% et 35 ans par exemple, alors le membre de droite de l'égalité est constant lui aussi. Ici, avec les valeurs précédentes il vaut 4.5160. Y a pas de surprise. On avait $$ y = K x$$. La pente c'est $$K$$ (4.5160). Le gain varie linéairement avec le montant. Si on injecte 1000 € au début on va récupérer 4 516 € à la fin. Plus important, cela explique pourquoi quand on double le montant au départ (2000 € vs 1000 € par exemple) à la fin, on a "juste" doublé le gain.

**À retenir** : La dérivée partielle du gain par rapport au montant initial est une constante donc le gain varie linéairement avec ce dernier.






### Qu'est-ce qui se passe si le rendement varie?

Je pars de l'équation

$$
G = M \left[ (1+r)^n - 1 \right]
$$

Je la ré-écris sous la forme

$$
y = M \left[ (1+x)^n - 1 \right]
$$

Ça ressemble à $$y = M [f(x) - 1]$$ dont la dérivée est de la forme $$y^\prime = M f^\prime(x)$$

Ensuite la dérivée de $$(1+x)^n$$ c'est la dérivée de $$u^n$$ soit $$nu^{n-1}u^\prime$$ soit ici $$n (1+x)^{n-1} \, 1$$ car la dérivée de $$x$$ c'est $$1$$

Du coup j'écris

$$
\frac{\partial y}{\partial x} = M \, n \, (1+x)^{n-1}
$$

Soit avec les noms des variables du début

$$
\frac{\partial G}{\partial r} = M \, n \, (1+r)^{n-1}
$$

Et qu'est-ce que ça veut dire en Français dans le texte?

La dérivée partielle du gain par rapport au rendement est une fonction qui ressemble bigrement à $$y^\prime=K \,X^N$$. J'ai remplacé $$1+r$$ par $$X$$ et $$n-1$$ par $$N$$ pour bien faire ressortir le fait qu'on a une "simple" fonction puissance de $$X$$, qui grimpe vite, très vite.







Mouais... C'est bien joli tout ça mon petit Marty, mais tes $$X^N$$, ils me disent pas combien je gagne, moi. Tu peux me faire une application numérique? Tu gardes les 1 000 € et les 35 ans, et tu fais varier le rendement de 1% à 10% par exemple.

Pas de souci Mamie. Je retourne dans mon notebook, je créé une section "Qu'est-ce qui se passe si le rendement varie?" et après quelques tâtonnements j'obtiens ce code :

```python
M = 1_000  # mise initiale (€)
n = 35     # nombre d'années

def gain(r):
    return M * ((1 + r) ** n - 1)

def derivee(r):
    return M * n * (1 + r) ** (n - 1)

def fmt(x):
    return f"{x:,.0f} €".replace(",", " ")

print(f"{'r':>5} | {'Gain G':>10} | {'dG/dr':>12} | {'Pente / point':>13} | {'G(r+1%) - G(r)':>14}")
print("-" * 68)
for pct in range(1, 11):
    r = pct / 100
    print(f"{r:>5.0%} | {fmt(gain(r)):>10} | {fmt(derivee(r)):>12} | "
          f"{fmt(derivee(r) / 100):>13} | {fmt(gain(r + 0.01) - gain(r)):>14}")
```

La fonction `gain()` c'est la formule du début et la fonction `derivee()` c'est la dérivée partielle qu'on vient de calculer. Ensuite, pour chaque rendement de 1% à 10%, j'affiche une ligne du tableau. À l'exécution je vois :

```text
    r |     Gain G |        dG/dr | Pente / point | G(r+1%) - G(r)
--------------------------------------------------------------------
   1% |      417 € |     49 090 € |         491 € |          583 €
   2% |    1 000 € |     68 624 € |         686 € |          814 €
   3% |    1 814 € |     95 617 € |         956 € |        1 132 €
   4% |    2 946 € |    132 801 € |       1 328 € |        1 570 €
   5% |    4 516 € |    183 867 € |       1 839 € |        2 170 €
   6% |    6 686 € |    253 786 € |       2 538 € |        2 990 €
   7% |    9 677 € |    349 234 € |       3 492 € |        4 109 €
   8% |   13 785 € |    479 155 € |       4 792 € |        5 629 €
   9% |   19 414 € |    655 494 € |       6 555 € |        7 688 €
  10% |   27 102 € |    894 168 € |       8 942 € |       10 472 €
```

Attends, attends... Doit y avoir une erreur dans le code? Ligne 5%, ta dérivée elle vaut 183 867 €. Avec 1 000 € de mise? Tu vas me faire croire que je vais gagner 183 k€?

Non, non Mamie... Regarde l'en-tête de la colonne. C'est pas un montant c'est une dérivée, un taux d'accroissement. Elle dit de combien bouge $$G$$ quand $$r$$ bouge de 1. Sauf que $$r = 1$$ ça veut dire 100%. Autrement dit 183 867 €, c'est ce qu'on gagnerait en plus si le rendement passait de 5% à 105%, et encore, seulement si la pente restait la même tout du long. Personne ne fait ça. Nous, ce qui nous intéresse, c'est de passer de 5% à 6%, donc un point de rendement, donc $$\Delta r = 0.01$$. Il faut donc diviser par 100 :

$$
\Delta G \approx \frac{\partial G}{\partial r} \times \Delta r = 183\,867 \times 0.01 \approx 1\,839 \text{ €}
$$

C'est comme le compteur de la voiture. Si tu roules à 90 km/h pendant une minute, tu ne fais pas 90 km. Tu fais $$90/60 = 1.5$$ km. Le compteur donne une vitesse "par heure", la dérivée donne une pente "par 100% de rendement". Dans les deux cas, il faut ramener ça à la durée, ou à la variation, qui nous intéresse. C'est ce que montre la colonne "Pente / point" : à 5%, un point de rendement en plus rapporte environ 1 839 € de plus au bout de 35 ans.

Mouais. Et comment je sais que ton truc est juste?

Facile, c'est la dernière colonne. Là je ne fais aucune dérivée, je calcule bêtement le gain à 6% puis le gain à 5% et je fais la différence : $$6\,686 - 4\,516 = 2\,170$$ €. On est dans les mêmes eaux que les 1 839 € mais la dérivée sous-estime un peu. Et c'est normal. La dérivée c'est la pente **au point** 5%. Or entre 5% et 6% la pente ne reste pas constante, elle continue de grimper (à 6%, elle vaut déjà 2 538 € par point). Si on utilise la pente du départ pour tout le trajet, on "oublie" l'accélération. C'est comme une voiture qui accélère : si tu estimes la distance de la prochaine minute avec ta vitesse actuelle, tu te trompes forcément par défaut. D'ailleurs, regarde, la vraie valeur (2 170 €) est bien coincée entre la pente à 5% (1 839 €) et la pente à 6% (2 538 €).


Tiens, pour bien visualiser ces histoires d'erreur, dans le notebook je créé une section "La tangente sous-estime une courbe qui accélère" dans laquelle je te propose le code et l'illustration suivante avec la fonction $$x^2$$. Le code est "compliqué" car il y a beaucoup de choses à faire pour avoir un graphe "sympa". Sinon on trace juste la fonction $$y=x^2$$, la tangente à la courbe en $$x=2$$ et on calcule, en $$x=3$$, la différence entre la valeur de la fonction et la hauteur à laquelle se trouve la dérivée.

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img07_5.webp"
    alt="La tangente sous-estime une courbe qui accélère"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>La tangente sous-estime une courbe qui accélère</figcaption>
</figure>




Ok c'est clair maintenant. Et sinon, si tu lis le tableau de haut en bas, tu vois quoi?

Que la colonne "Pente / point" s'envole. Un point de rendement en plus, ça vaut 491 € quand on est à 1% et presque 9 000 € quand on est à 10%. C'est 18 fois plus pour le même point! C'est ça la "fonction puissance qui grimpe vite" de tout à l'heure. Et c'est très différent du cas du montant où la dérivée était une constante : chaque millier d'euros en plus au départ rapportait 4 516 € de plus, que l'on parte de 1 000 € ou de 10 000 €.

Parfait. Peux tu illustrer tout ça avec un graphe?

Dans le notebook, je crée une section que je nomme "Illustration du tableau précédent", je me gratte un peu la tête et finalement j'obtiens le code ci-dessous et le graphe :

```python
import matplotlib.pyplot as plt

M = 1_000  # mise initiale (€)
n = 35     # nombre d'années
r0 = 0.05  # point où l'on trace la tangente

rs = [i / 1000 for i in range(0, 101)]  # 0 % à 10 %
gains = [M * ((1 + r) ** n - 1) for r in rs]

G0 = M * ((1 + r0) ** n - 1)
pente = M * n * (1 + r0) ** (n - 1)
tangente = [G0 + pente * (r - r0) for r in rs]

fig, ax = plt.subplots(figsize=(9, 5))
ax.plot([r * 100 for r in rs], gains, label="Gain G(r)")
ax.plot([r * 100 for r in rs], tangente, linestyle="--",
        label=f"Tangente en r = {r0:.0%} (pente : {pente / 100:,.0f} € par point)".replace(",", " "))
ax.plot([r0 * 100], [G0], marker="o", color="black")
ax.set_xlabel("Rendement annuel r (%)")
ax.set_ylabel("Gain (€)")
ax.set_title(f"Gain de {M:,.0f} € placés sur {n} ans selon le rendement".replace(",", " "))
ax.yaxis.set_major_formatter(lambda y, _: f"{y:,.0f}".replace(",", " "))
ax.grid(True, alpha=0.3)
ax.legend()
fig.tight_layout()
plt.show()
```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img08.webp"
    alt="Gain de 1 000 € placés sur 35 ans selon le rendement, avec la tangente en r = 5%"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Gain de 1 000 € placés sur 35 ans selon le rendement, avec la tangente en r = 5%</figcaption>
</figure>

C'est graphique mais bon Marty, comment on lit ça?

Cette fois-ci, en abscisse on n'a plus les années mais le rendement, de 0% à 10%. La durée est bloquée à 35 ans et la mise à 1 000 €. C'est exactement l'idée de la dérivée partielle : on bloque tout sauf le paramètre qui nous intéresse (ici le rendement $$r$$).

1. **La courbe pleine** c'est le gain $$G(r)$$, la colonne "Gain G" du tableau mais avec beaucoup plus de points. Elle est presque à l'horizontale à gauche et elle se redresse de plus en plus vers la droite. Plus on avance, plus elle est raide. Ça, c'est la colonne "Pente / point" qui s'envole.
1. **Le point noir** c'est notre cas de référence : 5%, 4 516 €.
1. **Les pointillés** c'est la tangente à la courbe en ce point. C'est la droite qui "frôle" la courbe en $$r=5\%$$ et qui a exactement la même pente qu'elle à cet endroit. Tu te rappelles l'équation de la tangente que tu m'avais fait réviser en long et en large quand j'étais en Première? $$y = f(a) + f^\prime(a)(x - a)$$. Ben c'est exactement la ligne `tangente = [G0 + pente * (r - r0) for r in rs]` du code, avec $$a = 5\%$$ et $$f^\prime(a) = 183\,867$$.

Et maintenant Mamie, le plus intéressant. Pose ton doigt sur $$r=6\%$$ en abscisse. La tangente donne $$4\,516 + 1\,839 = 6\,355$$ €. La courbe, elle, donne 6 686 €. L'écart entre les deux (331 €), c'est exactement l'écart entre les colonnes "Pente / point" et "G(r+1%) - G(r)" du tableau ($$2\,170 - 1\,839$$). Le tableau et le dessin racontent la même histoire.

On voit aussi que la tangente reste partout **sous** la courbe (la courbe est convexe). C'est pour ça que l'approximation par la dérivée est toujours un peu trop pessimiste ici. Enfin, si on s'éloigne trop de 5%, la tangente raconte n'importe quoi. Vers la gauche elle finit même par passer sous zéro, ce qui n'a aucun sens pour un gain. La dérivée c'est un outil local, un zoom autour d'un point. Pour des petites variations elle est top, pour des grandes variations il vaut mieux refaire le calcul complet.


**À retenir** : La dérivée partielle du gain par rapport au rendement n'est pas constante. Elle augmente très vite avec $$r$$. Un point de rendement en plus vaut d'autant plus que le rendement est déjà élevé (491 € à 1%, 8 942 € à 10%, avec 1 000 € sur 35 ans).





### Qu'est-ce qui se passe si la durée varie?

Je pars de l'équation

$$
G = M \left[ (1+r)^n - 1 \right]
$$

Je la ré-écris sous la forme

$$
y = M \left[ (1+r)^x - 1 \right]
$$


$$
\frac{\partial G}{\partial n} = M \, (1+r)^n \ln(1+r)
$$








### Application numérique

**Exemple chiffré** : $$M = 10\,000\ \text{€}$$, $$r = 7\ \%$$, $$n = 20$$ ans, soit un gain $$G \approx 28\,700\ \text{€}$$.

- $$+1\ \text{€}$$ investi rapporte $$\approx 2{,}87\ \text{€}$$ de gain en plus.
- $$+1$$ point de rendement ($$7\ \% \rightarrow 8\ \%$$) rapporte $$\approx +7\,200\ \text{€}$$ ($$+7\,900\ \text{€}$$ en calcul exact).
- $$+1$$ année rapporte $$\approx +2\,600\ \text{€}$$ ($$+2\,830\ \text{€}$$ en calcul exact).

Ces dérivées ont des unités différentes (€, %, années), donc on ne peut pas les comparer telles quelles. Pour savoir quel levier est le plus puissant, il faut raisonner en **élasticités**, c'est-à-dire en effet relatif : « +1 % sur le paramètre donne combien de % de gain ? »

$$
E_x = \frac{\partial G}{\partial x} \cdot \frac{x}{G}
$$

- **Montant** : $$E_M = 1$$. C'est proportionnel, ni plus ni moins.
- **Taux** :

$$
E_r = \frac{n \, r \, (1+r)^{n-1}}{(1+r)^n - 1} \approx 1{,}76
$$

- **Durée** :

$$
E_n = \frac{n \, (1+r)^n \ln(1+r)}{(1+r)^n - 1} \approx 1{,}82
$$


### Ce qu'il faut en retenir

1. **La durée et le taux sont des leviers « super-linéaires »**, et leur élasticité grandit avec $$n$$. À 40 ans, elle dépasse 3. Commencer tôt et ne pas vendre est donc le meilleur levier.
2. **Le taux, on le subit en grande partie**, sauf sur un point : les **frais**. Ils se soustraient directement à $$r$$. Passer de $$0{,}8\ \%$$ à $$0{,}2\ \%$$ de frais revient à gagner $$+0{,}6$$ point de rendement, soit environ $$+4\,000\ \text{€}$$ dans l'exemple, sans prendre aucun risque supplémentaire.
3. **Le montant est le levier le plus prévisible**, mais seulement linéaire. En pratique, des versements réguliers combinent montant et durée.

Une précision : $$r$$ doit être le rendement **géométrique** moyen (le CAGR), pas la moyenne arithmétique des rendements annuels. Avec la volatilité, la moyenne arithmétique surestime le résultat.



<!-- ###################################################################### -->
### The Setup
{: .no_toc }








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Elasticité

Oui. Le point clé, c'est que **$$1{,}76$$ ne correspond pas à « +1 point de taux »** (de $$7\ \%$$ à $$8\ \%$$). Il correspond à **« +1 % du taux »**, c'est-à-dire de $$7\ \%$$ à $$7{,}07\ \%$$. C'est une variation relative.

**1. La définition de l'élasticité**

L'élasticité du gain $$G$$ par rapport au taux $$r$$ est :

$$
E = \frac{\text{variation relative de } G}{\text{variation relative de } r} = \frac{\partial G}{\partial r} \cdot \frac{r}{G}
$$

Elle répond à la question : « si $$r$$ augmente de 1 % de sa valeur, de combien de % augmente $$G$$ ? »

**2. On remplace par les formules**

$$
G = M \left[ (1+r)^n - 1 \right]
$$

$$
\frac{\partial G}{\partial r} = M \, n \, (1+r)^{n-1}
$$

Donc :

$$
E = \frac{M \, n \, (1+r)^{n-1} \cdot r}{M \left[ (1+r)^n - 1 \right]}
$$

Le $$M$$ disparaît (il est en haut et en bas), ce qui donne :

$$
\boxed{E = \frac{n \, r \, (1+r)^{n-1}}{(1+r)^n - 1}}
$$

**3. Application numérique** ($$n = 20$$, $$r = 0{,}07$$)

$$
(1{,}07)^{19} \approx 3{,}6165
$$

$$
(1{,}07)^{20} \approx 3{,}8697
$$

Numérateur :

$$
20 \times 0{,}07 \times 3{,}6165 = 1{,}4 \times 3{,}6165 \approx 5{,}063
$$

Dénominateur :

$$
3{,}8697 - 1 \approx 2{,}870
$$

D'où :

$$
E = \frac{5{,}063}{2{,}870} \approx 1{,}76
$$

**4. Vérification « à la main »**

On prend un taux de $$7\ \% \times 1{,}01 = 7{,}07\ \%$$, avec $$M = 10\,000\ \text{€}$$ et $$n = 20$$ :

- À $$7\ \%$$ : $$G = 10\,000 \times (3{,}8697 - 1) \approx 28\,697\ \text{€}$$
- À $$7{,}07\ \%$$ : $$(1{,}0707)^{20} \approx 3{,}9207$$, donc $$G \approx 29\,207\ \text{€}$$

Le gain augmente de $$510\ \text{€}$$. Rapporté à $$28\,697\ \text{€}$$, cela fait :

$$
\frac{510}{28\,697} \approx +1{,}78\ \%
$$

C'est bien environ $$1{,}76$$ fois la hausse de 1 % du taux. Le petit écart vient du fait que la dérivée est une approximation linéaire.

**5. Et pour +1 point ($$7\ \% \rightarrow 8\ \%$$) ?**

Dans ce cas, le taux augmente de $$\frac{1}{7} \approx +14{,}3\ \%$$ en relatif. Le gain passe de $$28\,697\ \text{€}$$ à $$36\,610\ \text{€}$$, soit $$+27{,}6\ \%$$. Le rapport :

$$
\frac{27{,}6}{14{,}3} \approx 1{,}93
$$

est un peu plus élevé que $$1{,}76$$, parce que la courbe s'accentue quand $$r$$ augmente.






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

Lorem...
























<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography


* [Pourcentages, croissance, gestion des ressources…]({%link docs/03_maths/002_pourcentages/pourcentages.md%})


<!-- * [Link to a web page](https://cleonis.nl/physics/phys256/energy_position_equation.php).
* [Link to a page of the site]({%link docs/03_maths/013_gauss_forme_integrale_differentielle/gauss_forme_integrale_et_differentielle.md%})
-->

