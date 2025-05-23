---
layout: default
title: "Equation de la chaînette"
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

$$\vec{-T} + \vec{T+\mathrm{d}T} + \vec{P} = \vec{0} $$

On projette sur les axes x et y

Sur x : $$\cos(\alpha)\mathrm{d}T = 0$$

Sur y : $$\sin(\alpha)\mathrm{d}T - \mu g\mathrm{d}L = 0$$

Dans l'équation ci-dessus on a: $$ P = mg = \mu \mathrm{d}Lg $$

Où $$\mu$$ est la masse linéique (la masse par unité de longueur), $$\mathrm{d}L$$ est un élément de longueur et g est l'accélération de pesanteur

Ensuite, on intègre l'équation de la projection sur l'axe x : $$\int{}{}{\cos(\alpha)\mathrm{d}T} = K$$

Il vient : $$T\cos(\alpha) = K$$

Autrement dit, à chaque endroit, le long du cable, la projection de la tension sur l'axe horizontal est constante. Si on se place à l'endroit le plus bas de la courbe, alors l'angle $$\alpha$$ est nul et K=To.

On intègre maintenant l'équation de la projection sur l'axe y : $$\int{}{}{\sin(\alpha)\mathrm{d}T} = \int{}{}{\mu g\mathrm{d}L}$$

Il vient : $$T\sin(\alpha) = \mu gL$$

Mouai... Et on fait quoi maintenant avec les deux équations en question ?

(1) $$T\cos(\alpha) = K$$

(2) $$T\sin(\alpha) = \mu gL$$

Si on divise (2) par (1) il vient : $$\tan{\alpha} = \frac{\mu g}{K} L$$

Mouai... Then what? Essayons de différencier l'équation ci-dessus : $$\mathrm{d}(\tan{\alpha}) = \frac{\mu g}{K}\mathrm{d}(L)$$

Maintenant, remarquons que

$$\tan{\alpha} = \frac{\mathrm{d}y}{\mathrm{d}x}$$

Et que

$$\mathrm{d}L^2 = \mathrm{d}x^2 + \mathrm{d}y^2$$

Soit encore

$$\mathrm{d}L = \sqrt{\mathrm{d}x^2 + \mathrm{d}y^2}$$

Bref, notre équation précédente devient

$$\mathrm{d}(\tan{\alpha}) = \frac{\mu g}{K} \mathrm{d}(L)$$

$$\mathrm{d}(\frac{\mathrm{d}y}{\mathrm{d}x}) = \frac{\mu g}{K} \sqrt{\mathrm{d}x^2 + \mathrm{d}y^2}$$

À ce niveau, ce qui serait bien, se serait d'avoir une équation homogène. Pour cela on aimerait avoir un $$\frac{\mathrm{d}y}{\mathrm{d}x}$$ dans le membre de droite. Qu'à cela ne tienne allons-y. L'équation devient :

$$\mathrm{d}(\frac{\mathrm{d}y}{\mathrm{d}x}) = \frac{\mu g}{K} \sqrt{\mathrm{d}x^{2} + \mathrm{d}y^{2}}$$

$$\mathrm{d}(\frac{\mathrm{d}y}{\mathrm{d}x}) = \frac{\mu g}{K} \frac{\sqrt{\mathrm{d}x^2 + \mathrm{d}y^2}} {\mathrm{d}x} \mathrm{d}x$$

$$\mathrm{d}(\frac{\mathrm{d}y}{\mathrm{d}x}) = \frac{\mu g}{K} \sqrt{\frac{\mathrm{d}x^2}{\mathrm{d}x^2} + \frac{\mathrm{d}y^2}{\mathrm{d}x^2}} \mathrm{d}x$$

$$\mathrm{d}(\frac{\mathrm{d}y}{\mathrm{d}x}) = \frac{\mu g}{K} \sqrt{1 + \frac{\mathrm{d}y^2}{\mathrm{d}x^2}} \mathrm{d}x$$

$$\mathrm{d}(\frac{\mathrm{d}y}{\mathrm{d}x}) = \frac{\mu g}{K} \sqrt{1 + \frac{\mathrm{d}y}{\mathrm{d}x}^{2}}\mathrm{d}x$$

Faisons un peu le ménage histoire d'y voir plus clair

$$\mathrm{d}(u) = \frac{\mu g}{K} \sqrt{1 + u^{2}}\mathrm{d}x$$

$$\mathrm{d}u = C \sqrt{1 + u^{2}}\mathrm{d}x$$

$$\frac {\mathrm{d}u}{\sqrt{1 + u^{2}}} = C \mathrm{d}x$$

Bon, ben, on y va, on intègre

$$\int{}{}{\frac {\mathrm{d}u}{\sqrt{1 + u^{2}}}} = \int{}{}{C \mathrm{d}x}$$

$$\int{}{}{\frac {\mathrm{d}u}{\sqrt{1 + u^{2}}}} = Cx$$

T'es mignon... Et l'intégrale de gauche... On fait comment ?

### Moyen mnémotechnique

En fait, je me rappelle que $$\cos^2(x) + \sin^2(x) = 1 $$ (ben, si, ça quand même je peux m'en rappeler) et qu'en faisant migrer $$\sin$$ de l'autre côté on a aussi $$\cosh^{2}(x) = 1 + \sinh^{2}(x)$$ (c'est mnémotechnique, c'est pas exact...)

Allez, on continue. Le membre de droite ressemble à ce qu'il y a sous la racine. J'effectue un changement de variable et je pose

$$u = \sinh(x)$$

En différenciant on a

$$\mathrm{d}u = \cosh(x)\mathrm{d}x$$

L'intégrale (le membre de gauche) s'écrit alors :

$$\int{}{}{\frac {\mathrm{d}u}{\sqrt{1 + u^{2}}}}$$

$$\int{}{}{\frac{\cosh(x)}{\sqrt{1 + \sinh^{2}(x)}}\mathrm{d}x}$$

$$\int{}{}{\frac{\cosh(x)} {\sqrt{\cosh^{2}(x)}}\mathrm{d}x}$$

$$\int{}{}{\frac{\cosh(x)}{\cosh(x)}\mathrm{d}x}$$

$$\int{}{}{\mathrm{d}x}$$

$$x$$

Comme on avait posé

$$u = \sinh(x)$$

C'est à dire

$$arc \sinh(u) = x$$

On en déduit que l'intégrale vaut arc sinh(u)

On se retrouve donc avec

$$\int{}{}{\frac {\mathrm{d}u}{\sqrt{1 + u^{2}}}} = Cx$$

$$arcsinh(u)= Cx$$

$$u= \sinh(Cx)$$

On a presque terminé... Faut cependant se rappeler qu'on avait écrit

$$u=\frac {\mathrm{d}y} {\mathrm{d}x}$$

Et donc, on se retrouve à résoudre l'équation suivante

$$\frac {\mathrm{d}y} {\mathrm{d}x}= \sinh(Cx)$$

$$\mathrm{d}y= \sinh(Cx)\mathrm{d}x$$

On pose

$$u = Cx$$

On a donc

$$\mathrm{d}u = C\mathrm{d}x$$

$$\mathrm{d}x = \frac {\mathrm{d}u}{C}$$

L'équation devient

$$\mathrm{d}y= \sinh(Cx)\mathrm{d}x$$

$$\mathrm{d}y= \frac {1}{C}\sinh(u)\mathrm{d}u$$

$$\int{}{}{\mathrm{d}y}= \frac{1}{C} \int{}{}{\sinh(u)\mathrm{d}u}$$

$$y= \frac{1}{C} \cosh(u)$$

$$y= \frac{1}{C} \cosh(Cx)$$

Comme on a dit que $$C = \frac {\mu g}{K} $$ avec K=Cte=To

$$y= \frac {K}{\mu g} \cosh ( \frac {\mu g}{K} x)$$

L'équation du fil électrique entre 2 poteaux est donc un cosh() et non une simple parabolique comme on pourrait "intuitivement" s'y attendre. Voilà par ailleurs les 2 courbes. La parabole est rehaussée pour que les 2 extremums coïncident :

<div align="center">
<img src="./assets/courbe.webp" alt="" loading="lazy"/>
</div>
