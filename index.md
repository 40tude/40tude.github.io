---
title: Home
layout: home
nav_order: 1
date:               2024-12-25 12:00:00 +0000
last_modified_date: 2025-01-12 12:00:00 +0000
---


# 40tude.fr 

{: .note }
[**40tude.fr**](https://www.40tude.fr/) en phase de renovation alors que je suis en train de le transférer de (WordPress + OVH) vers (markdown + Jekyll + Just The Docs + GitHub).  
Le **12 janvier 2025** il reste **170 articles** à transférer (essentiellement dans les catégories C++, Snowboard et Moto). J'espère avoir terminé fin janvier 2025. Je ne suis pas sûr de transférer tous les billets car certaines ne sont vraiment plus d'actualité. D'un autre côté, j'ai souvent été bien content de retrouver une info dans une "vieille" page qui était toujours en ligne... Lire par exemple le premier épisode de la série d'article sur [Simple OS]({%link docs/02_simple_os/000_sos_2_le_retour_20_ans_apres_episode_0/sos_2_le_retour_20_ans_apres_episode_0.md%}). Je ne sais pas, je vais voir...  
Pendant le portage, même si j'apporte quelques modifications aux billets, je ne vais pas changer les dates de dernières mise à jour. Je le ferai si, après le transfert, je reviens sur telle ou telle page pour y apporter des compléments, la ré-écrire complètement... On retrouve les date de première et de dernière édition au bas de chaque page.   
Pour l'instant c'est pas fun mais je suis focus sur le transfert.

Pour le reste...   
* Il faut lire "**fortitude point fr**". Il n'y a pas de sens caché, j'ai toujours bien aimé cette façon d'écrire ce mot qui en anglais veut dire courage face à l'adversité. Pas de référence non plus à [l'opération de désinformation](https://fr.wikipedia.org/wiki/Op%C3%A9ration_Fortitude) de la seconde guerre mondiale.  
* Le site précédent, hébergé chez **OVH** a été **piraté** en Avril 2024. De mon point de vue d'utilisateur "de base" qui veut juste écrire sur son blog, OVH n'a rien fait pour protéger mon site ou m'aider après coup (j'étais client depuis 2011 sans aucun coup de fil au support...). 
* D'un autre côté, je n'en pouvais plus de **WordPress**. Je parle surtout de l'édition des billets. Au début, en 2010, ça allait à peu près mais sur la fin c'est devenu n'importe quoi...  WP c'est sans doute très bien pour certains mais cela ne correspond plus du tout à mes besoins. En plus, j'ai l'impression que ça pue chez WP. Lire ce billet sur Medium.
    * <https://medium.com/notes-and-theories/this-man-controls-40-of-the-internet-and-its-a-problem-1b37a66e6185>


Pas la peine de lire plus loin. Ce sont des notes que je garde sous le coude lors du transfer.










---
## TODO
* Sur la 1ere page, la liste des 10 derniers billets mis à jour
* Faire un billet sur le portage de WP vers markdown?
* Lightbox pour les images?
* PageSpeed Insights indique : just-the-docs.js = 1.6s de chargement
* Voir ces histoires de balise rel="canonical"
* S'assurer que Google indexe bien le site. Pas sûr à 100% que ce soit le cas aujourd'hui 11 01 2025
* Voir ces histoires de plugins
    * Va être chaud
    * Faut installer ruby meme si on a un plugin sous forme de gem 
    * Faut l'installer avec Bundle etc.
* SEO? 
    * Pas sûr de comprendre grand chose au sujet
    * Pas intéressé par le sujet
* Pages en anglais? avec lang=en avant le Head
    * À mon avis c'est mort mais bon je vais continuer à chercher
    * Y a peut être une option avec les collection mais je sais pas si y a lang="en" avant le head
* What else ?










---
## À garder sous le coude

* `NOT YET TRANSFERED`

```
    purple        red              blue               green        yellow     
{: .note }    {: .warning }    {: .important }    {: .new }    {: .highlight }
```


```
Target
[titre](https://youtu.be/57ivuBX1kLU?si=bI3xfPOcipMe5F3O&t=42){:target="_blank"}

Pour les images sur les pages où il y a un permalink
<div align="center">
<img src="{%link docs/08_snowboard/assets/img_05.webp%}" alt="" width="450" loading="lazy"/>
</div>


```


```
Mettre {:toc} que sur les titres de niveau 2. Éviter 3 et +

# Corps cétoniques
{: .no_toc }

## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}

### Blablabla
{: .no_toc }


Pour exclure de la TOC les titres de niveaux 3+
   - CTRL+H + Exp Reg
   - ``(#{3,}\s.+)``
   - `$1\n{: .no_toc }`

```




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




* <https://just-the-docs.github.io/just-the-docs/>
* <https://jekyllcodex.org/without-plugins/>
* <https://mademistakes.com/>
* <https://just-the-docs.github.io/just-the-docs-tests/components/math/mathjax/index/>
* <https://jekyllrb.com/>
* <https://github.com/rouge-ruby/rouge/wiki/list-of-supported-languages-and-lexers>
* <https://www.mathematex.fr/guide-mathjax>
* <https://docs.mathjax.org/en/latest/input/tex/macros/index.html>




```
| Aligné à gauche  | Centré          | Aligné à droite  |
| :--------------- |:---------------:| ----------------:|
| Aligné à gauche  | Ce texte        |  Aligné à droite |
```



```
[Finalize Windows 11 installation]({% link docs/04_windows/001_finalize_windows_11_installation/finalize_windows_11_installation.md %})
```




### Enlever le gras des titres 
```
CTRL+H + Exp Reg
(#+)\s\*\*(.*?)\*\*
$1 $2
```




### Remplacer les $ seuls par $$ 
```
CTRL+H + Exp Reg
(?<!\$)\$(?!\$) 
$$$
```

```
d droit ``\mathrm{d}``
$$\mathrm{\LaTeX}$$

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
```


### Lister les 10 plus longs articles : 

```powershell
cd C:\Users\phili\OneDrive\Documents\40tude_to_repair\converted_to_markdown\docs

Get-ChildItem -Path . -Recurse -Filter *.md | Sort-Object -Property Length -Descending | Select-Object -First 10 | Format-Table FullName, @{Name="Size (KB)"; Expression={"{0:N2}" -f ($_.Length / 1KB)}}
```













---
## DONE
* ~~Snowboard, ajouter des photos et des vidéos~~
* ~~Voir cette histoire de pull request de la part de Just The Docs~~
* ~~robots.txt~~
    * Dans Google Search Console aller verifier dans "Paramètres" si il le voit bien
* ~~sitemap~~
    * fix url in _config.yml
    * modify _includes\head_custom.html
* ~~Test du site avec outils Google etc.~~
    * Google Search Console
    * PageSpeed Insights
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
## IDEES DE BILLETS
* Pareto et 50%-1%
* Revenir sur Codingame
* SOS Chapitres IV et suivants
* IA et ML 
* Relire les notes OneNote de préparation certification IA et voir ce que je peux rapatrier
* Rapatrier la page install Linux Mint qui est sur GitHub 
* 52 nuances de physique (faut vraiment que je l'écrive un jour)










---
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
