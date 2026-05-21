---
published: false
author: 40tude
# lang: en-US
layout: default
title: "TODO: Add a title"
description: "TODO: Add a description"
image: TODO: Full path to the first image. Example: docs/03_maths/020_equa_diff_in_physics/assets/img00.webp
twitter:
  card: summary_large_image
parent: "Motos"
# nav_order: 36
math: mathjax
date:               2026-04-10 15:00:00
# last_modified_date is updated by .git/hooks/pre-commit
last_modified_date: 2026-05-21 08:53:32
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

* Point 1
* Point 2



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00.webp"
    alt="TODO: Add a description"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>TODO: Add a legend</figcaption>
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
## 0. Préliminaires

1. Vérifier que la chaîne n'est pas trop tendue
    * Si c'est le cas, le bras oscillant ne peut pas bouger librement, la chaîne tire sur le PSB... Bref c'est pas une bonne idée.
1. Mesurer la hauteur du joint spi de fourche. **Exemple**: 7 mm pour un RS 660

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img02.webp"
    alt="Mesure de la hauteur du joint spi de fourche"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Mesure de la hauteur du joint spi de fourche</figcaption>
</figure>



1. Mesurer la hauteur du zip. **Exemple**: 2.5 mm

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img01.webp"
    alt="Poser un zip sur un des tubes de fourche"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Poser un zip sur un des tubes de fourche</figcaption>
</figure>


1. Poser un zip sur 1 des tubes de fourche

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img03.webp"
    alt="Mesure de la largeur du zip. On va dire 2.5 mm"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Mesure de la largeur du zip. On va dire 2.5 mm</figcaption>
</figure>


1. Pour l'AV et l'AR retrouver dans la documentation technique les moyens de régler ainsi que les valeurs usines de :
    - Pré compression (pré-contrainte)
    - Rebond
    - Compression
    - Rebond rapide
    - Compression rapide

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img04.webp"
    alt="Doc Aprilia. 1=précharge 2=compression (tube de droite) 3=détente (tube de droite)"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Doc Aprilia. 1=précharge 2=compression (tube de droite) 3=détente (tube de droite)</figcaption>
</figure>

**Exemple**:La doc Aprilia du RS660 factory indique qu'il n'y a qu'une configuration et que c'est la configuration piste qui a été choisie. Il n'indique aucune valeur pour un usage route.
- Précharge = -5 tours depuis la vers
-


1. Préparer les outils en conséquence



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 1. Fourche: Vérifier les réglages par défauts

### Précharge
1. Ouvrir (dévisser) et noter le nombre de tours de précharge sur les tubes gauche et droit
1. Fermer (visser) complètement et noter le nombre de tours disponibles
1. Ouvrir complètement

### Rebond (si disponible)
1. Ouvrir et noter le nombre de clics de rebond sur les tubes gauche et droit
1. Fermer complètement et noter le nombre de clics disponible
1. Ouvrir complètement


### Compression (si disponible)
1. Ouvrir et noter le nombre de clics de compression sur les tubes gauche et droit
1. Fermer complètement et noter le nombre de clics disponible
1. Ouvrir complètement



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 2. Déterminer le bottom out (buttée de fourche)
1. Retrouver dans la documentation de la moto. **Exemple**: 120 mm pour un RS 660
1. Étendre la fourche à son maximum
1. Mesurer du fourreau au pied (ne pas mesurer sous le joint). **Exemple**: 143 mm pour un RS 660
1. Application numérique: $$\textrm{Bottom Out} = 143 - 120 = 23 \textrm{ mm}$$







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 4. Mesure du static sag (moto SANS pilote)
1. Mesurer du fourreau au pied (ne pas mesurer sous le joint). **Exemple**: 115 mm pour un RS 660
1. Application numérique: $$\textrm{Static SAG} = 143 - 115 = 28 \textrm{ mm}$$
1. On veut un static sag en 15 et 25 mm (20 mm typique)
1. Avec 28 mm, la moto s'enfonce trop. Faut ajouter/fermer la précharge
1. On ferme de 4 tours
1. On mesure et on lit 118 mm.
1. $$\textrm{Static SAG} = 143 - 118 = 25 \textrm{ mm}$$
1. On touche plus à rien



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 4. Mesure du rider sag (moto AVEC pilote)
1. S'équiper: bottes, gants, casque, combine ou blouson
1. Monter DOUCEMENT sur la moto
1. Première mesure, 108 mm par exemple
1. $$\textrm{Rider SAG} = 143 - 108 = 35 \textrm{ mm}$$
1. Freiner de l'avant et pomper plusieurs fois la fourche
1. Faire une mesure. **Exemple**: 109 mm pour un RS 660
    1. C'est OK si la difference est de l'ordre du mm
    1. Si y a une grande difference, faire la révision de la fourche (huile, joints...)
1. Prendre la valeur moyenne des 2 valeurs précédentes
1. Objectifs
    * Route = 35-40 mm
    * Track = 30 mm
1. Ajustements:
* Si on lit 22 mm il faut ouvrir la précharge. Si on est tout ouvert et si on atteint toujours pas la valeur souhaitée, le pilote est trop léger. Faut changer les ressorts
* Si on lit 45 mm il faut fermer la précharge. Si on est tout fermé et si on atteint toujours pas la valeur souhaitée, le pilote est trop lourd. Faut changer les ressorts




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 5. Mettre à jour le Bottom Out en fonction du réglage de précharge


1. Étendre la fourche à son maximum
1. Mesurer du fourreau au pied (ne pas mesurer sous le joint). Exemple: 146 mm pour un RS 660 car on a mis +3 tours de précharge
1. Application numérique: $$\textrm{Bottom Out} = 146 - 120 = 26$$





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 6. Calculer la limite à surveiller quand on roule

1. On veut 10 mm de marge avec que la fourche ne touche
1. La distance entre le pied de fourche et le base du zip: $$\textrm{Limit} = 146 - 120 - 5 - 2 + 10 = 29 mm$$
1. Faut pas que le bas du zip descende sous les 29 mm

Ajustements:
* Si le bas du zip dépasse la limite (mesure inf à 29mm) il faut fermer la précharge et/ou vérifier les mesures
* Si le bas du zip reste loin de la limite on peut freiner plus fort et/ou vérifier les mesures





<!-- Link to a video -->
<figure style="max-width: 560px; margin: auto;">
<div style="position: relative; padding-bottom: 56.25%; height: 0;">
    <iframe
    src="https://www.youtube.com/embed/MIeYz6aMBbk"
    title="Add a title"
    style="position: absolute; inset: 0; width: 100%; height: 100%;"
    allowfullscreen>
    </iframe>
</div>
<figcaption style="text-align: center;">TODO: Add a legend</figcaption>
</figure>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Level 2 Section

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam luctus blandit tincidunt. Nunc et laoreet ipsum. Fusce neque nisi, blandit vitae magna nec, sollicitudin commodo felis. Morbi a viverra lorem, ut sollicitudin lacus. Pellentesque euismod magna et enim fermentum tempor. Etiam vel sagittis mauris. Praesent dictum nisl sit amet tellus rhoncus mollis. Aenean nisi nibh, tincidunt at massa eget, luctus aliquet lectus. Mauris ac massa dolor. Sed fringilla tristique est. Suspendisse nec leo et magna tincidunt ultrices. Suspendisse lacinia leo sed congue ornare. Mauris congue eu dui posuere ultricies. Duis volutpat volutpat erat, ut consequat nisl bibendum gravida. Curabitur fringilla tincidunt auctor.

<!-- ###################################################################### -->
### Level 3 Section
{: .no_toc }

Cras dui ex, scelerisque et magna et, lacinia convallis nunc. Proin sapien nunc, eleifend a mi semper, efficitur pharetra justo. Etiam sit amet ex lacinia, consequat orci sed, malesuada leo. Donec commodo felis eu commodo suscipit. Praesent vitae lorem a dui porta volutpat. Pellentesque efficitur pharetra velit, at placerat nulla iaculis in. Praesent placerat turpis sit amet mauris bibendum interdum. Sed consectetur massa lacus, tempus congue purus dictum nec.

Math inline $$x(t)$$ and, below, math offline:

$$
\frac{dS}{dt} = r \cdot S
$$











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

Maecenas in arcu id erat interdum tristique sed fermentum tortor. Donec eget felis ornare sem dapibus tincidunt at vitae justo. Mauris feugiat tristique augue at maximus. Vivamus euismod blandit mi, ut pretium libero tempor sit amet. In tristique nisi vel mi molestie, ac ornare enim blandit. Phasellus bibendum diam massa, in tempor purus imperdiet a. Curabitur mattis mauris eget cursus molestie. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.

Aliquam blandit malesuada purus at porta. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vestibulum efficitur sapien leo, id iaculis sem sagittis ac. Praesent dolor nisl, fringilla fermentum maximus id, ornare id justo. Morbi at gravida purus, eu imperdiet risus.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

* [Link to a web page](https://cleonis.nl/physics/phys256/energy_position_equation.php).
* [Link to a page of the site]({%link docs/03_maths/013_gauss_forme_integrale_differentielle/gauss_forme_integrale_et_differentielle.md%})