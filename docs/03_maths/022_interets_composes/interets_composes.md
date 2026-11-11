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
last_modified_date: 2026-11-11 15:53:18
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

Lorem...
* intérêts composés
* frais composés


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## La formule


$$V_n = V_0 * (1 + 5\%)^n$$


$$
\begin{align*}
\textrm{Montant final} &= \textrm{Montant initial} * (1 + 5\%)^n
\textrm{Gain} &= \textrm{Montant final} - \textrm{Montant initial}
\textrm{Gain} &= \textrm{Montant initial} * (1 + 5\%)^n - \textrm{Montant initial}
G &= M * (1 + 5\%)^n - M
G &= M * (1 + 5\%)^n - M
G &= M [(1 + 5\%)^n - 1]
\end{align*}
$$





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Dérivation partielle

Avec :

$$
G = M \left[ (1+r)^n - 1 \right]
$$

où $$M$$ est la mise initiale, $$r$$ le rendement annuel et $$n$$ le nombre d'années, les trois dérivées partielles sont :

- **Montant** :

$$
\frac{\partial G}{\partial M} = (1+r)^n - 1
$$

- **Taux** :

$$
\frac{\partial G}{\partial r} = M \, n \, (1+r)^{n-1}
$$

- **Durée** :

$$
\frac{\partial G}{\partial n} = M \, (1+r)^n \ln(1+r)
$$

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

**Ce qu'il faut en retenir :**

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


* [Percentage and exponential growth]({%link docs/03_maths/002_pourcentages/pourcentages.md%})


<!-- * [Link to a web page](https://cleonis.nl/physics/phys256/energy_position_equation.php).
* [Link to a page of the site]({%link docs/03_maths/013_gauss_forme_integrale_differentielle/gauss_forme_integrale_et_differentielle.md%})
-->

