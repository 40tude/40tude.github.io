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
last_modified_date: 2026-11-12 11:06:28
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
1. la durée (70 ans au lieu de 35)
1. le rendement (10% au lieu de 5)

Voilà ce que je vois. Respectivement 9 000 €, 29 000 € et 27 000 €

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img04.webp"
    alt="Évolution du gain de 2 000 € placés à 5% sur 35 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Évolution du gain de 2 000 € placés à 5% sur 35 ans</figcaption>
</figure>

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img05.webp"
    alt="Évolution du gain de 1 000 € placés à 5% sur 70 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Évolution du gain de 1 000 € placés à 5% sur 70 ans</figcaption>
</figure>

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img06.webp"
    alt="Évolution du gain de 1 000 € placés à 10% sur 35 ans"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Évolution du gain de 1 000 € placés à 10% sur 35 ans</figcaption>
</figure>

Et du coup Marty qu'est-ce qui te vient en tête?

J'ai pas l'impression que de doubler la mise au départ soit si intéressant. En fait si, c'est rentable car on gagne 2 fois plus (9k au lieu de 4.5k) mais bon c'est linéaire. Dans les autres cas, c'est une toute autre histoire. Au bout de 70 ans, j'aurai une hanche en titane, plus de dents mais je pourrai m'acheter une Rolex avec les 29 k€ de gains mais... Le mieux, c'est peut être de trouver un placement qui double les intérêts car à 53 ans je pourrai alors m'acheter et profiter pendant quelques années encore d'une très belle moto à 27 000 balles.

Tu es mon digne petit fils. Oui il y a 3 paramètres indépendants dans ton équation et en jouant sur les valeurs, on voit que certains on plus d'influence que d'autres. Alors commence pas à faire ta mijaurée ou ton wokiste de base. Imagine... Imagine que tu te balade sur une plage et qu'en marchant, tu rattrape une fille que tu vois de dos. Qu'est ce qui va faire que tu va vouloir engager la conversation? La longueur des cheveux, ses fesses, sa hauteur... Bon ben voilà t'as tout compris. Là aussi y a plusieurs paramètres mais certains sont plus importants que d'autres.

Du coup, la question à 1M$ c'est: dans le cadre de ton équation avec ses 3 paramètres, est-ce que tu es capable d'aller plus loin que les tests de doublement de valeur qu'on a fait et de retrouver, par calcul, l'influence de chacun d'entre eux?

Ouai... Normalement c'est possible. On a vu ça en physique et mon prof qui lui n'est pas un obsédé avait un exemple tout aussi pertinent. Il disait: imaginez que vous êtes au sommet d'une petite colline avec votre snowboard. Le terrain c'est une surface dont l'altitude est une fonction de $$x$$ et de $$y$$, un truc du style $$z = f(x, y)$$. Du coup si tu pars vers le nord, $$x$$ ne change pas, seul $$y$$ varie et la pente est peut être douce. Si tu par vers l'ouest, $$y$$ ne change pas, seul $$x$$ varie et la pente est peut être plus prononcée, et donc plus de fun. Donc les gars, pas de panique, calculer une dérivée partielle ça revient à étudier la formule initiale en bloquant tous les paramètres sauf un et à regarder alors ce qui se passe.

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

$$
\frac{\partial G}{\partial M} = (1+r)^n - 1
$$


### Qu'est-ce qui se passe si la durée varie?


$$
\frac{\partial G}{\partial n} = M \, (1+r)^n \ln(1+r)
$$




### Qu'est-ce qui se passe si le rendement varie?

$$
\frac{\partial G}{\partial r} = M \, n \, (1+r)^{n-1}
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

