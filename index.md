---
title: Home
layout: home
nav_order: 1
date:               1964-02-25 20:00:00 +0000
last_modified_date: 2025-01-03 15:00:00 +0000
---

# 40tude.fr 
Faut lire "fortitude".  
Il n'y a pas de sens caché, j'ai toujours aimé cette façon d'écrire ce mot qui en anglais veut dire courage face à l'adversité.  
Pas de référence non plus à l'opération de désinformation de la seconde guerre mondiale.  

Le site 40tude.fr est en cours de migration (300 billets de 2011 à 2024).  
Laissez moi un peu de temps (j'espère avoir terminé fin janvier 2025)  
Soyez pas surpris si d'ici là, les menus changent, les billets et catégories disparaissent, apparaissent...
Pendant le portage, même si j'apporte quelques modifications aux billets, je ne vais pas changer les dates de dernières mise à jour.  

## TODO
* Faire un billet concernant le portage de WP vers markdown?
* Lightbox pour les images
* Ajouter un indicateur de durée de lecture en haut des pages
* Sur la 1ere page, la liste des 10 derniers billets édités
* Pages en anglais? avec lang=en avant le Head
    * À mon avis c'est mort mais bon je vais continuer à chercher
* Test du site avec outils Google etc.
    * Google Search Console
    * PageSpeed Insights
* What else ?

## DONE
* ~~Comment faire des liens (ancres) au sein d'une même page en markdown~~
    * Option 1 : `[Le planning en mode synthétique](#le-planning-synthétique)` avec accent
    * Option 2 : ## Un long titre de section <a id="short-id"></a> puis plus loin [Aller à cette section](#short-id)
* ~~Correcteur ortho FR dans VScode ?~~
* ~~Voir comment permettre le téléchargement de fichiers pdf, zip, xlsx...~~
    * `[Agenda de la Journee de Roulage](assets/agenda_roulage.pdf)`
* ~~Porter les pages de SOS pour tester les outils de recupération du site WP~~
* ~~Voir ces histoires de permalinks. Je suis pas sûr de comprendre de quoi on parle~~
    * Lire : <https://jekyllrb.com/docs/permalinks/#front-matter>
* ~~Back to Top~~
* ~~First et last Edit en bas de page~~

## À garder sous le coude
* `NOT YET TRANSFERED`

* `[Finalize Windows 11 installation]({% link docs/windows/finalize_windows_11_installation/finalize_windows_11_installation.md %})`

* Lister les 10 plus longs articles : 
    * ``Get-ChildItem -Path . -Recurse -Filter *.md | Sort-Object -Property Length -Descending | Select-Object -First 10 | Format-Table FullName, @{Name="Size (KB)"; Expression={"{0:N2}" -f ($_.Length / 1KB)}}``

* `{: .note }` `{: .warning }``{: .important }``{: .new }``{: .highlight }`
* Enlever le gras des titres CTRL+H + Exp Reg
    * ``(#+)\s\*\*(.*?)\*\*``
    * `$1 $2`

* Retrouver les $ qui sont seuls => ``(?<!\$)\$(?!\$)`` 
* Remplacer les $ seuls par $$   => ``(?<!\$)\$(?!\$)`` et `$$$`
* <https://just-the-docs.github.io/just-the-docs/>
* <https://jekyllrb.com/>
* <https://github.com/rouge-ruby/rouge/wiki/list-of-supported-languages-and-lexers>
* d droit ``\mathrm{d}``
* <https://www.mathematex.fr/guide-mathjax>
* <https://docs.mathjax.org/en/latest/input/tex/macros/index.html>

```
---
layout: default
title: "SOS2 Episode 0"
#parent: "Simple OS"
#nav_order: 2
# math: mathjax
date:               2023-11-19 00:20:43
last_modified_date: 2023-12-04 00:02:38
# permalink: /sos/
---

```

## Organisation des catégories ?

2. Simple OS
1. Maths
1. Windows
1. Linux
1. Programmation
    1. Python
    1. C++
    1. C
1. Moto
    1. Pilotage circuit
    1. RSV4
    1. ZX6R
    1. Divers
1. Snowboard débuter 
1. Divers
    * Billet Jeune
    * Billet Gravel
    * ... 
