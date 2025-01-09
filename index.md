---
title: Home
layout: home
nav_order: 1
date:               1964-02-25 20:00:00 +0000
last_modified_date: 2025-01-03 15:00:00 +0000
---


# 40tude.fr 

{: .note }
La page et le site sont en construction alors je suis en train des transférer le 40tude.fr de (WordPress + OVH) vers (markdown + Jekyll + Just The Docs + GitHub).

* De tout façon, je n'en pouvais plus de WP. Quelle bouse!
    * <https://medium.com/notes-and-theories/this-man-controls-40-of-the-internet-and-its-a-problem-1b37a66e6185>
* Chez OVH, mon site a été piraté en Avril 2024. Ils n'ont rien fait pour protéger le site ou m'aider après coup (j'étais client depuis 2011 sans aucun coup de fil au support...) 

Il faut lire "fortitude".  
Il n'y a pas de sens caché, j'ai toujours aimé cette façon d'écrire ce mot qui en anglais veut dire courage face à l'adversité.  
Pas de référence non plus à l'opération de désinformation de la seconde guerre mondiale.  

Le site 40tude.fr est en cours de migration (300 billets de 2011 à 2024).  
Laissez moi un peu de temps (j'espère avoir terminé fin janvier 2025)  
Soyez pas surpris si d'ici là, les menus changent, les billets et catégories disparaissent, apparaissent...
Pendant le portage, même si j'apporte quelques modifications aux billets, je ne vais pas changer les dates de dernières mise à jour.  

---
## TODO
* Sur la 1ere page, la liste des 10 derniers billets édités
* Lightbox pour les images?
* Voir ces histoires de plugins
    * Va être chaud
    * Faut installer ruby meme si on a un plugin sous forme de gem 
    * Faut l'installer avec Bundle etc.
* SEO? 
    * Pas sûr de comprendre grand chose...
    * Pas intéressé par le sujet
* Faire un billet concernant le portage de WP vers markdown?
* Pages en anglais? avec lang=en avant le Head
    * À mon avis c'est mort mais bon je vais continuer à chercher
* Test du site avec outils Google etc.
    * Google Search Console
    * PageSpeed Insights
* What else ?


---

## DONE
* ~~Ajouter un indicateur de durée de lecture en haut des pages~~
* ~~Installer Google Analytics~~
* ~~Configurer le site pour supporter les simple $ pour les equations ou sigles dans les paragraphes~~
    * Impossible
    * https://github.com/just-the-docs/just-the-docs/discussions/1593
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




---
## À garder sous le coude

| Aligné à gauche  | Centré          | Aligné à droite |
| :--------------- |:---------------:| -----:|
| Aligné à gauche  |   ce texte        |  Aligné à droite |

* `NOT YET TRANSFERED`

* `[Finalize Windows 11 installation]({% link docs/04_windows/001_finalize_windows_11_installation/finalize_windows_11_installation.md %})`


* `    purple       red            blue             green      yellow     `
* `{: .note }` `{: .warning }``{: .important }``{: .new }``{: .highlight }`



* Enlever le gras des titres CTRL+H + Exp Reg
    * ``(#+)\s\*\*(.*?)\*\*``
    * `$1 $2`

* Remplacer les $ seuls par $$ CTRL+H + Exp Reg
    * ``(?<!\$)\$(?!\$)`` 
    * `$$$`

* d droit ``\mathrm{d}``
* $$\mathrm{\LaTeX}$$

$$
\begin{align*}
(-4) \cdot (0) & = 0 \\
(-4) \cdot (5 - 5) & = 0 \\
(-4) \cdot 5 + (-4) \cdot (-5) & = 0 \\
-20 + (-4) \cdot (-5) & = 0 \\
(-4) \cdot (-5) & = 20
\end{align*}
$$

$$
\begin{align}
(-4) \cdot (0) & = 0 \\
(-4) \cdot (5 - 5) & = 0 \\
(-4) \cdot 5 + (-4) \cdot (-5) & = 0 \\
-20 + (-4) \cdot (-5) & = 0 \\
(-4) \cdot (-5) & = 20
\end{align}
$$


* Lister les 10 plus longs articles : 
    * `cd C:\Users\phili\OneDrive\Documents\40tude_to_repair\converted_to_markdown\docs`
    * ``Get-ChildItem -Path . -Recurse -Filter *.md | Sort-Object -Property Length -Descending | Select-Object -First 10 | Format-Table FullName, @{Name="Size (KB)"; Expression={"{0:N2}" -f ($_.Length / 1KB)}}``

* <https://just-the-docs.github.io/just-the-docs/>
* <https://just-the-docs.github.io/just-the-docs-tests/components/math/mathjax/index/>
* <https://jekyllrb.com/>
* <https://github.com/rouge-ruby/rouge/wiki/list-of-supported-languages-and-lexers>
* <https://www.mathematex.fr/guide-mathjax>
* <https://docs.mathjax.org/en/latest/input/tex/macros/index.html>

```
---
layout: default
title: "SOS2 Episode 0"
#parent: "Simple OS"
#nav_order: 2
# math: mathjax
# permalink: /sos/
date:               2023-11-19 00:20:43
last_modified_date: 2023-12-04 00:02:38
---

```

Mettre toc que sur les titres de niveau 2. Eviter 3 et +
```
# Corps cétoniques
{: .no_toc }

## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}
```

## Organisation des catégories ?

* 02_simple_os
* 03_maths
* 04_windows
* 05_linux
* 06_programmation
    *  Python
    *  C++
    *  C
* 07_moto
    *  Pilotage circuit
    *  RSV4
    *  ZX6R
    *  Divers
* 08_snowboard 
* 99_divers
    * Billet Jeune
    * Billet Gravel
    * ... 
