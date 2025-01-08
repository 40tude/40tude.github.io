---
layout: default
title: "Equation différentielle du 1er ordre"
parent: "Maths"
#nav_order: 2
math: mathjax
date: 2012-06-05 23:34:21
last_modified_date: 2022-11-24 09:03:34
---


# Equation différentielle du 1er ordre, méthode et exemples

## Introduction

Résoudre une équation différentielle... Ça peut en laisser certains rêveurs. J'ai découvert (retenu et compris j'espère) cette méthode sur un site US. Comme je l'ai trouvé simple et efficace j'ai décidé de la retranscrire ici à travers différents exemples.

## La méthode en 4 points

### 1 - On écrit l'équation différentielle sous sa forme linéaire standard

$$y^{\prime}+ p(x)y = q(x)$$

Le but du jeu consiste à avoir 1 comme facteur de y'. Notons aussi que p(x) et q(x) peuvent être des fonctions de x. À part ça, rien de très tordu.

### 2 - On détermine ensuite le facteur d'intégration

Attention. La ruse de guerre se trouve ici. C'est vraiment le point qu'il faut comprendre. En fait, le membre de gauche de l'équation ressemble presque à un truc du style : f'g + g'f. Si on arrivait à se ramener à une expression de ce type on pourrait écrire la première équation sous la forme d'un truc du style : (fg)' = q

Je sais le membre de droite de l'égalité ci-dessus n'est pas juste mais ce n'est pas grave pour l'instant. En revanche, ce qui est beaucoup plus important à saisir c'est que si on arrive à écrire (fg)'=q alors c'est "gagné", yaka, faukon, il ne restera qu'une simple intégration à faire.

Bon allez, on y va, essayons, sur le cas général, de trouver ce fameux facteur d'intégration.

$$y^{\prime}+ p(x)y = q(x)$$

On cherche un facteur u(x) tel que :

$$y^{\prime} u(x)+ u(x)p(x)y = u(x)q(x)$$

Si on veut avoir une égalité du style :

$$f^{\prime}g + g^{\prime}f = u(x)q(x)$$

Alors, on n'a pas le choix, il faut que l'égalité ci-dessous soit respectée :

$$u^{\prime}(x) = u(x)p(x)$$

Résolvons l'équation en question. On commence par écrire

$$u^{\prime}(x) = \frac{\mathrm{d}u(x)}{\mathrm{d}x}$$

On a donc

$$\frac{\mathrm{d}u(x)}{\mathrm{d}x} = u(x)p(x)$$

$$\frac{\mathrm{d}u(x)}{u(x)} = p(x)\mathrm{d}x$$

On intègre et il vient :

$$\int{}{} \frac{\mathrm{d}u(x)}{u(x)} = \int{}{}p(x)\mathrm{d}x$$

$$\ln(u(x)) = \int{}{}p(x)\mathrm{d}x$$

$$u (x)= e^{\int{}{}p(x)\mathrm{d}x}$$

Pour l'instant, il ne faut pas trop s'inquiéter de la forme "un peu bizarre" de l'égalité ci-dessus.

### 3 - Une fois qu'on a le facteur d'intégration, on multiplie les 2 côtés de l'équation (1) par le facteur en question

On se retrouve avec une équation qui ressemble à :  $$f^{\prime}g + g^{\prime}f = u(x)q(x) $$

Elle se ramène à :  $$(fg)' = u(x)q(x) $$

### 4 - On intègre l'équation précédente

$$(fg)^{\prime} = u(x)q(x)$$

$$\int{}{}(fg)^{\prime} = \int{}{}u(x)q(x)dx$$

$$fg = \int{}{}u(x)q(x)\mathrm{d}x$$









## Exemple 1

On cherche à résoudre l'équation suivante :

$$xy^{\prime} - y = x^{3}$$

### Etape 1 : Ecriture sous la forme standard linéaire de l'équation précédente

$$y^{\prime} - \frac{y}{x} = x^{2}$$

### Etape 2 : On intègre le coef. de y de l'équation précédente pour trouver le facteur d'intégration.

Le facteur de y c'est :  $$\frac{-1}{x} $$

On cherche donc à intégrer :  $$\int{}{}\frac{-1}{x}dx $$

$$\int{}{}\frac{-1}{x}dx = -\int{}{}\frac{1}{x}dx = -\ln(x)$$

Le facteur d'intégration est donc :  $$e^{-\ln(x)} = (e^{\ln(x)})^{-1} = x^{-1} = \frac{1}{x} $$

### Etape 3 : On multiplie par le facteur d'intégration l'équation initiale

On reprend l'équation de départ :  $$y^{\prime} - \frac{y}{x} = x^{2} $$

On multiplie à gauche et à droite par le facteur d'intégration :  $$ \frac{1}{x} y^{\prime} - \frac{y}{x^2} = x $$

On en profite pour vérifier qu'on n'a pas fait d'erreur de calcul lors de la détermination du facteur d'intégration en vérifiant qu'on a bien :

$$(\frac{1}{x}y)^{\prime} = \frac{1}{x} y^{\prime} + \frac{-1}{x^2} y$$

On peut dès lors écrire l'équation (1) sous la forme :  $$(\frac{1}{x} y)^{\prime} = x  $$

### Etape 4 : On intègre la nouvelle version de l'équation initiale

$$(\frac{1}{x} y)^{\prime} = x $$

$$\int{}{}(\frac{1}{x} y)^{\prime} = \int{}{}x dx $$

$$\frac{1}{x} y = \frac{x^2}{2} + K$$

Pour finir la solution de l'équation est donc :

$$y = \frac{x^3}{2} + Kx$$

K est généralement déterminé avec les conditions initiales.









## Exemple 2

On souhaite résoudre :

$$\frac{dy}{dx} - 4y = 2$$

(1) Ecriture sous la forme standard linéaire de l'équation précédente

$$y^{\prime} -4 y = 2$$

(2) On intègre le coefficient de y de l'équation précédente pour trouver le facteur d'intégration.

Le facteur de y c'est -4.

$$\int{}{}-4dx = -4x $$

Le facteur d'intégration est donc :  $$e^{-4x}  $$

(3) On multiplie par le facteur d'intégration l'équation initiale

$$y^{\prime} e^{-4x} - 4y e^{-4x}= 2 e^{-4x}$$

$$(ye^{-4x})^{\prime}= 2 e^{-4x}$$

(4) On intègre l'équation précédente

$$(ye^{-4x})^{\prime}= 2 e^{-4x}$$

$$ye^{-4x}= \int{}{}2 e^{-4x}dx + K$$

$$ye^{-4x}= 2 e^{-4x} \frac {1}{-4} + K$$

$$y= - \frac{1}{2} + Ke^{4x}$$












## Exemple 3

On souhaite résoudre :

$$\frac{\mathrm{d}T}{\mathrm{d}t}  = k(T_{e} -T)$$

(1) Ecriture sous la forme standard linéaire de l'équation précédente

$$\frac{\mathrm{d}T}{\mathrm{d}t} + kT = kT_{e}$$

(2) On intègre le coefficient de T de l'équation précédente pour trouver le facteur d'intégration.

Le facteur de T c'est k.

$$\int{}{}kdt = kt $$

Le facteur d'intégration est donc :  $$e^{kt} $$

(3) On multiplie par le facteur d'intégration l'équation initiale

$$\frac{\mathrm{d}T}{\mathrm{d}t}e^{kt} + kTe^{kt} = kT_{e}e^{kt}$$

$$(e^{kt}T)^{\prime} = kT_{e}e^{kt}$$

(4) On intègre l'équation précédente

$$(e^{kt}T)^{\prime} = kT_{e}e^{kt}$$

$$e^{kt}T = \int{}{}kT_{e} e^{kt} dt + C$$

$$T = e^{-kt} \int{}{}kT_{e} e^{kt} dt + Ce^{-kt}$$

Attention, $$T_{e}$$ peut être une fonction du temps. Par exemple $$T_{e}(t)$$ peut être entretenue sinusoïdale, décroissante ou autre. La solution est donc plutôt du type :

$$T = ke^{-kt} \int{}{}T_{e}(t) e^{kt} dt + Ce^{-kt}$$











## Exemple 4

On souhaite résoudre :

$$\frac{\mathrm{d}s}{\mathrm{d}t} \cos(t) + s*\sin(t) = 1$$

(1) Ecriture sous la forme standard linéaire de l'équation précédente

$$\frac{\mathrm{d}s}{\mathrm{d}t} + s \frac{\sin(t)}{\cos(t)} = \frac{1}{\cos(t)}$$

$$\frac{\mathrm{d}s}{\mathrm{d}t} + s*\tan(t) = \frac{1}{\cos(t)}$$

(2) On intègre le coefficient de s de l'équation précédente pour trouver le facteur d'intégration.

Le facteur de s c'est $$\tan(t)$$.

$$\int{}{}\tan(t)\mathrm{d}t = \int{}{} \frac{\sin(t)}{\cos(t)}\mathrm{d}t = -\int{}{} \frac{-\sin(t)}{\cos(t)}\mathrm{d}t = -\ln(\cos(t))$$

Le facteur d'intégration est donc :  $$e^{-\ln(\cos(t))} = (e^{\ln(\cos(t))})^{-1} = \frac{1}{\cos(t)} $$

(3) On multiplie par le facteur d'intégration l'équation initiale

$$\frac{\mathrm{d}s}{\mathrm{d}t} \frac{1}{\cos(t)} + s \frac{\sin(t)}{\cos^{2}(t)} = \frac{1}{\cos^{2}(t)}$$

$$(\frac{s}{\cos(t)})^{\prime} = \frac{1}{\cos^{2}(t)} $$

(4) On intègre l'équation précédente

$$(\frac{s}{\cos(t)})^{\prime} = \frac{1}{\cos^{2}(t)} $$

$$\frac{s}{\cos(t)} = \int{}{}\frac{1}{\cos^{2}(t)}\mathrm{d}t + K$$

Si on se rappelle que

$$(\frac{\sin}{\cos})^{\prime} = \frac{\cos^2 + \sin^2}{\cos^2} = \frac{1}{\cos^2}$$

L'intégration devient

$$\frac{s}{\cos(t)} = \frac{\sin(t)}{\cos(t)} + K$$

$$s(t) = \sin(t) + K*\cos(t)$$

