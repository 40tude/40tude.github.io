---
layout: default
title: "Equation de la chaînette - explications étape par étape"
parent: "Maths"
math: mathjax
date: 2012-06-16 15:48:30
last_modified_date: 2022-11-25 17:01:53
---

## Introduction

L'autre jour en rentrant de Dunkerque, sur l'autoroute, alors que tout le monde dormait dans la voiture, j'ai, je ne sais pas pourquoi, commencé à regarder les pylônes électriques. Entre autres, je me suis demandé qu'elle était la courbe décrite par le fil électrique mais aussi et surtout quel était le paramètre qui était minimisé. De retour à la maison, un tour sur Google et zou, j'avais la réponse : équation de la chaînette. Cela dit, afin de retenir et de comprendre un peu mieux j'ai commencé à gribouiller sur le papier. Vous trouverez ci-dessous mes explications et ce que je crois avoir compris.

### Note :

Si vous êtes prof de math fuyez ! Je ne suis pas sûr que ce qui suit soit très orthodoxe...

## Schéma

<div align="center">
<img src="./assets/schema.webp" alt="" loading="lazy"/>
</div>

## Mise en équation

$$\sum{}{}\vec{F} = m \vec{\gamma} $$

Comme le système est au repos, l'accélération est nulle. On a donc :

$$\sum{}{}\vec{F} = \vec{0} $$

$$\vec{-T} + \vec{T+dT} + \vec{P} = \vec{0} $$

On projette sur les axes x et y

Sur x : $cos(\alpha)dT = 0$

Sur y : $sin(\alpha)dT - \mu gdL = 0$

Dans l'équation ci-dessus on a: $ P = mg = \mu dLg $

Où μ est la masse linéique, dL est un élément de longueur et g est l'accélération de pesanteur

Ensuite, on intègre l'équation de la projection sur l'axe x : $\int{}{}{cos(\alpha)dT} = K$

Il vient : $Tcos(\alpha) = K$

Autrement dit, à chaque endroit, le long du cable, la projection de la tension sur l'axe horizontal est constante. Si on se place à l'endroit le plus bas de la courbe, alors l'angle $\alpha$ est nul et K=To.

On intègre maintenant l'équation de la projection sur l'axe y : $\int{}{}{sin(\alpha)dT} = \int{}{}{\mu gdL}$

Il vient : $Tsin(\alpha) = \mu gL$

Mouai... Et on fait quoi maintenant avec les deux équations en question ?

(1) $Tcos(\alpha) = K$

(2) $Tsin(\alpha) = \mu gL$

Si on divise (2) par (1) il vient : $tg{\alpha} = \frac{\mu g}{K} L$

Mouai... Then what? Essayons de différencier l'équation ci-dessus : $d(tg{\alpha}) = \frac{\mu g}{K}d(L)$

Maintenant, remarquons que

$$tg{\alpha} = \frac{dy}{dx}$$

Et que

$$dL^2 = dx^2 + dy^2$$

Soit encore

$$dL = \sqrt{dx^2 + dy^2}$$

Bref, notre équation précédente devient

$$d(tg{\alpha}) = \frac{\mu g}{K} d(L)$$

$$d(\frac{dy}{dx}) = \frac{\mu g}{K} \sqrt{dx^2 + dy^2}$$

À ce niveau, ce qui serait bien, se serait d'avoir une équation homogène. Pour cela on aimerait avoir un dy/dx dans le membre de droite. Qu'à cela ne tienne allons-y. L'équation devient :

$$d(\frac{dy}{dx}) = \frac{\mu g}{K} \sqrt{dx^{2} + dy^{2}}$$

$$d(\frac{dy}{dx}) = \frac{\mu g}{K} \frac{\sqrt{dx^2 + dy^2}} {dx} dx$$

$$d(\frac{dy}{dx}) = \frac{\mu g}{K} \sqrt{\frac{dx^2}{dx^2} + \frac{dy^2}{dx^2}} dx$$

$$d(\frac{dy}{dx}) = \frac{\mu g}{K} \sqrt{1 + \frac{dy^2}{dx^2}} dx$$

$$d(\frac{dy}{dx}) = \frac{\mu g}{K} \sqrt{1 + \frac{dy}{dx}^{2}}dx$$

Faisons un peu le ménage histoire d'y voir plus clair

$$d(u) = \frac{\mu g}{K} \sqrt{1 + u^{2}}dx$$

$$du = C \sqrt{1 + u^{2}}dx$$

$$\frac {du}{\sqrt{1 + u^{2}}} = C dx$$

Bon, ben, on y va, on intègre

$$\int{}{}{\frac {du}{\sqrt{1 + u^{2}}}} = \int{}{}{C dx}$$

$$\int{}{}{\frac {du}{\sqrt{1 + u^{2}}}} = Cx$$

Hein, hein... Et l'intégrale de gauche... On fait comment ?

### Moyen mnémotechnique

En fait, je me rappelle que $cos^2(x) + sin^2(x) = 1 $ (ben, si, ça quand même je peux m'en rappeler) et que (en faisant migrer sin de l'autre côté) on a aussi $ch^{2}(x) = 1 + sh^{2}(x)$

Allez, on continue. Le membre de droite ressemble à ce qu'il y a sous la racine. J'effectue un changement de variable et je pose

$$u = sh(x)$$

En différenciant on a

$$du = ch(x)dx$$

L'intégrale (le membre de gauche) s'écrit alors

$$\int{}{}{\frac {du}{\sqrt{1 + u^{2}}}}$$

$$\int{}{}{\frac{ch(x)}{\sqrt{1 + sh^{2}(x)}}dx}$$

$$\int{}{}{\frac{ch(x)} {\sqrt{ch^{2}(x)}}dx}$$

$$\int{}{}{\frac{ch(x)}{ch(x)}dx}$$

$$\int{}{}{dx}$$

$$x$$

Comme on avait posé

$$u = sh(x)$$

C'est à dire

$$arcsh(u) = x$$

On en déduit que l'intégrale vaut arcsh(u)

On se retrouve donc avec

$$\int{}{}{\frac {du}{\sqrt{1 + u^{2}}}} = Cx$$

$$arcsh(u)= Cx$$

$$u= sh(Cx)$$

On a presque terminé... Faut cependant se rappeler qu'on avait écrit

$$u=\frac {dy} {dx}$$

Et donc, on se retrouve à résoudre l'équation suivante

$$\frac {dy} {dx}= sh(Cx)$$

$$dy= sh(Cx)dx$$

On pose

$$u = Cx$$

On a donc

$$du = Cdx$$

$$dx = \frac {du}{C}$$

L'équation devient

$$dy= sh(Cx)dx$$

$$dy= \frac {1}{C sh(u)}du$$

$$\int{}{}{dy}= \frac{1}{C} \int{}{}{sh(u)du}$$

$$y= \frac{1}{C} ch(u)$$

$$y= \frac{1}{C} ch(Cx)$$

Comme on a dit que

$C = \frac {\mu g}{K} $ avec K=Cte=To

$$y= \frac {K}{\mu g} ch ( \frac {K}{\mu g} x)$$

L'équation du fil électrique entre 2 poteaux est donc un cosh et non une simple parabolique comme on pourrait "intuitivement" s'y attendre. Voilà par ailleurs les 2 courbes (la parabole est rehaussée pour que les 2 points bas coïncident) :

<div align="center">
<img src="./assets/courbe.webp" alt="" loading="lazy"/>
</div>
