C'est le portage du site 40tude.fr de WordPress + OVH vers markdown + Jekyll + Just the Docs + GitHub

---

## Table des matieres

- [Edition](#edition)
  - [Longueur des posts](#longueur-des-posts)
  - [VSCode Snippets](#vscode-snippets)
  - [Caracteres speciaux et tirets](#caracteres-speciaux-et-tirets)
  - [Callouts](#callouts)
  - [Video avec legende](#video-avec-legende)
  - [Image avec legende](#image-avec-legende)
  - [Liens et ancres](#liens-et-ancres)
  - [Note de bas de page](#note-de-bas-de-page)
  - [Table of content dans un article](#table-of-content-dans-un-article)
  - [Front matter](#front-matter)
  - [Sous-titre](#sous-titre)
  - [Tableaux](#tableaux)
  - [Regex utiles](#regex-utiles)
  - [PowerShell](#powershell)
  - [Liquid](#liquid)
- [Publication](#publication)
  - [Alerte de securite Dependabot](#alerte-de-securite-dependabot)
- [SEO](#seo)
  - [Tester et outils](#tester-et-outils)
  - [Indexation Google](#indexation-google)
- [Communication](#communication)
- [Idees de billets](#idees-de-billets)
- [TODO](#todo)
- [DONE](#done)

---

## Edition

### Longueur des posts
* 3_000 mots
* Decouper en serie le cas echeant
* Utiliser `word2read` ./blablabla.md (voir dans assets\wordcount)


### VSCode Snippets
Roue Crantee en bas a gauche > Snippets > Choisir markdown.json
Le fichier est dans `C:\Users\phili\AppData\Roaming\Code\User\snippets`


### Caracteres speciaux et tirets
* C cedille majuscule = ALT + 128 (du pave numerique)
* em dash — = ALT + 0150

```json
"em dash": {
    "prefix": "---",
    "body": ["—"],
    "description": "Insert em dash"
}
```

* `-` hyphen
* `–` en dash (tiret demi-cadratin)
* `—` em dash (tiret cadratin) : inserer une remarque parenthetique, plus expressif que la virgule

Supprimer les espaces vides en fin de ligne : `[ \t]+$`
(Voir aussi settings.json : `"files.trimTrailingWhitespace": true`)

Numerotes cercles :
* ❶ U+2776  ❷ U+2777  ❸ U+2778  ❹ U+2779  ❺ U+277A  ❻ U+277B  ❼ U+277C  ❽ U+277D  ❾ U+277E

Sans Serif :
* ➊ U+278A  ➋ U+278B  ➌ U+278C  ➍ U+278D  ➎ U+278E  ➏ U+278F  ➐ U+2790  ➑ U+2791  ➒ U+2792  ➓ U+2793

` NOT YET TRANSFERED `


### Callouts

Voir `_config.yml`

```
    purple        red              blue               green        yellow
{: .note }    {: .warning }    {: .important }    {: .new }    {: .highlight }
```

```
> A paragraph
>
> Another paragraph
>
> The last paragraph
```

```
{: .note-title }
> Side Note:
>
> A paragraph with a custom title callout
```

Reference : <https://just-the-docs.com/docs/ui-components/callouts/>

Bannieres a copier-coller dans un article en cours :

<h2 align="center">
<span style="color:red"><b>This post is still being written.</b></span>
</h2>

<h2 align="center">
<span style="color:orange"><b>This post is still being reviewed.</b></span>
</h2>


### Video avec legende

```html
<figure style="text-align: center;">
  <iframe width="560" height="315" src="https://www.youtube.com/embed/EZfM2VMs_vI?si=FHS-1PFIqBG70Ffs&amp;start=55" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
  <figcaption>I love the opening theme. It gives me goosebumps every time.</figcaption>
</figure>
```

Centrer une video YouTube sans legende :
```html
<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/ZbZSe6N_BXs?start=31" frameborder="0" allowfullscreen></iframe>
</div>
```


### Image avec legende

```html
<div align="center">
<img src="./assets/img_02.webp" alt="" width="450" loading="lazy"/><br/>
<span>1986...</span>
</div>
```

Sur une page avec permalink :
```html
<div align="center">
<img src="{%link docs/08_snowboard/assets/img_05.webp%}" alt="" width="450" loading="lazy"/>
</div>
```


### Liens et ancres

Lien sur une page du site :
```
[Coding Interview Patterns problems]({%link docs/06_programmation/rust/007_coding_interview_patterns/13_graphs/268_longest_increasing_path.md%})
```

Lien sur un fichier a telecharger :
```
[Agenda de la Journee de Roulage](assets/agenda_roulage.pdf)
```

Lien avec target blank :
```
[titre](https://youtu.be/57ivuBX1kLU?si=bI3xfPOcipMe5F3O&t=42){:target="_blank"}
```

Lien vers un autre article avec ancre :
```
<!--post-a.md -->
## My Important Section
This is the content I want to link to.
```

```
<!--post-b.md -->
See [this section]({% post_url 2025-09-01-post-a %}#my-important-section).
[expression]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#expressions)
[statement]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#statement)
[function]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#function-method-procedure)
[invariant]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#invariant)
[indirection]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#indirection)
```

Ancre dans la meme page (ou depuis une autre page) :
```
[Aller a cette section](#short-id)
## Un titre long pour une section <a id="short-id"></a>
```

```
[Three Steps]({%link docs/07_moto/001_notes_pilotage/003_three_steps/three_steps.md%}#le-point-de-corde-pc)
[Finalize Windows 11 installation]({% link docs/04_windows/001_finalize_windows_11_installation/finalize_windows_11_installation.md %})
```

Option avec accents :
```
[Le planning en mode synthetique](#le-planning-synthetique)
```


### Note de bas de page

Dans le texte :
```
une valeur entiere[^1]
```

Tout en bas de l'article :
```
----

[^1]: Ma note de bas de page.
```


### Table of content dans un article

```
Mettre {:toc} que sur les titres de niveau 2. Eviter 3 et +

# Corps cetoniques
{: .no_toc }

## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}

### Blablabla
{: .no_toc }


Pour exclure de la TOC les titres de niveaux 3+
   - CTRL + H + Exp Reg
   - `(#{3,}\s.+)`
   - `$1\n{: .no_toc }`
```


### Front matter

```yaml
---
published: true
lang: en-US
layout: default
title: "SOS2 Episode 0"
description: "From basic syntax to building plugins with once_cell and organizing Rust projects."
parent: "Simple OS"
#nav_order: 2
# math: mathjax
# permalink: /sos/
date:               2023-11-19 00:20:43
last_modified_date: 2023-12-04 00:02:38
---
```

Front matter complet avec SEO :
```yaml
published: true
author: 40tude
lang: en-US
layout: default
title: "From Monolith to Distributed Systems in Rust: A Practical Introduction"
description: "A hands-on journey through small, working projects to understand when and why architecture needs to evolve."
image: docs/06_programmation/rust/026_monolith_to_distributed/assets/img03.webp
twitter:
  card: summary_large_image
parent: "Rust"
nav_order: 35
date:               2026-02-11 10:00:00
last_modified_date: 2026-02-13 00:30:00
```

Image OG ideale : PNG 1200px x 630px
```
┌───────────────────────────────────────────────────┐
│                                                   │
│   🦀                                              │
│                                                   │
│   Learning Modular Monolith                       │
│   Architecture with Rust                          │
│                                                   │
│   From Hello World to Hexagonal Architecture      │
│   7 Steps - Traits - Ports & Adapters             │
│                                                   │
│                                  40tude.fr        │
└───────────────────────────────────────────────────┘
```


### Sous-titre

```
# Rust Traits: Defining Character
{: .no_toc }

From basic syntax to building plugins with once_cell and organizing your Rust projects.
{: .lead }
```


### Tableaux

```
| Aligne a gauche  | Centre          | Aligne a droite  |
| :--------------- |:---------------:| ----------------:|
| Aligne a gauche  | Ce texte        |  Aligne a droite |
```

Compter les articles du site / date de derniere modif :
```
<p>Nombre d'articles du site : {{ site.pages | size }}</p>
{% for page in site.pages limit: 1 %}
  <p>{{ page.last_modified_date }}</p>
{% endfor %}
```


### Regex utiles

Enlever le gras des titres :
```
CTRL + H + Exp Reg
(#+)\s\*\*(.*?)\*\*
$1 $2
```

Centrer les videos Youtube :
```
CTRL + H + Exp Reg
(<iframe.*?<\/iframe>)
<div align="center">\n$1\n</div>
```

Equations - remplacer les $ seuls par $$ :
```
CTRL + H + Exp Reg
(?<!\$)\$(?!\$)
$$$
```

```
$$y\_target$$           WORKS in both
$$\text{y_target}$$     Does NOT work in VSCode
$$\text{y\_target}$$    Does NOT work in rendering
```

Exemples d'equations :
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


### PowerShell

Lister les 10 plus longs articles :

```powershell
cd C:\Users\phili\OneDrive\Documents\40tude_to_repair\converted_to_markdown\docs

Get-ChildItem -Path . -Recurse -Filter *.md | Sort-Object -Property Length -Descending | Select-Object -First 10 | Format-Table FullName, @{Name="Size (KB)"; Expression={"{0:N2}" -f ($_.Length / 1KB)}}
```

Lister tous les fichiers qui ne sont pas .webp dans les sous-repertoires /assets :

```powershell
Get-ChildItem -Recurse -Directory -Filter "assets" | ForEach-Object {
    Get-ChildItem -Path $_.FullName -File | Where-Object {
        $_.Extension -ne ".webp"
    } | ForEach-Object {
        Write-Output $_.FullName
    }
}
```


### Liquid

```
    split ne fonctionne pas
    {% unless page.url contains '/index' %}
    {% endunless %}

    {% assign image = page.content | markdownify | split: '<img src="' | last | split: '"' | first %}

    {% assign image = page.content | split: '<img src="' | last | split: '"' | first %}
    {% assign image = image | slice: 2, image.size %}
    <pre>image = {{ image }}</pre>
    {% if image == page.content %}
        {% assign image = '/assets/images/40tude_307.webp' %}
    {% endif %}

    {% assign nb_articles = 5 %}
    {% assign nb_words = 20 %}
    {% assign articles_sorted = site.pages | sort: 'last_modified_date' | reverse %}
    {% for page in articles_sorted limit: nb_articles %}
        <tr>
            <td>
                <!-- Extract image if it exists -->
                {% assign image = '' %}
                {% capture page_content %}{{ page.content }}{% endcapture %}
                {% assign img_tag_start = '<img src="' %}
                {% assign img_tag_end = '"' %}
                {% assign img_start_index = page_content | index: img_tag_start %}
                {% if img_start_index %}
                    {% assign img_start_index = img_start_index | plus: img_tag_start.size %}
                    {% assign img_end_index = page_content | slice: img_start_index | index: img_tag_end %}
                    {% assign image = page_content | slice: img_start_index, img_end_index %}
                {% endif %}
                <pre>image before slice = {{ image }}</pre>
                {% if image == '' %}
                    {% assign image = '/assets/images/40tude_307.webp' %}
                {% else %}
                    {% assign image = image | slice: 2, image.size %}
                {% endif %}
                <pre>image after extract = {{ image }}</pre>

                <pre>page.url = {{ page.url }}</pre>

                <!-- Extract the directory from page.url -->
                {% assign page_dir = '' %}
                {% assign parts = page.url | split: '/' %}
                {% for part in parts %}
                    {% unless forloop.last %}
                        {% assign page_dir = page_dir | append: part | append: '/' %}
                    {% endunless %}
                {% endfor %}
                <pre>page_dir = {{ page_dir }}</pre>
                <img src="{{ page_dir }}{{ image }}" alt="Illustration de {{ page.title }}" width="100" loading="lazy"/>
            </td>
            <td>
                <a href="{{ page.url }}">{{ page.title }}</a>
            </td>
            <td>
                {{ page.content | markdownify | strip_html | truncatewords: nb_words }}
            </td>
        </tr>
    {% endfor %}




    {% assign nb_articles = 5 %}
    {% assign nb_words = 20 %}
    {% assign articles_sorted = site.pages | sort: 'last_modified_date' | reverse %}
    {% for page in articles_sorted limit: nb_articles %}
        <tr>
            <td>
                <!-- Extract image if it exists -->
                {% assign image = '' %}
                {% capture page_content %}{{ page.content }}{% endcapture %}
                {% assign img_tag_start = '<img src="' %}
                {% assign img_tag_end = '"' %}
                {% assign img_start_index = page_content | index: img_tag_start %}
                {% if img_start_index != nil %}
                    {% assign img_start_index = img_start_index | plus: img_tag_start.size %}
                    {% assign img_substring = page_content | slice: img_start_index %}
                    {% assign img_end_index = img_substring | index: img_tag_end %}
                    {% if img_end_index != nil %}
                        {% assign image = img_substring | slice: 0, img_end_index %}
                    {% endif %}
                {% endif %}
                <pre>image before slice = {{ image }}</pre>
                {% if image == '' %}
                    {% assign image = '/assets/images/40tude_307.webp' %}
                {% else %}
                    {% assign image = image | slice: 2, image.size %}
                {% endif %}
                <pre>image after extract = {{ image }}</pre>

                <pre>page.url = {{ page.url }}</pre>

                <!-- Extract the directory from page.url -->
                {% assign page_dir = '' %}
                {% assign parts = page.url | split: '/' %}
                {% for part in parts %}
                    {% unless forloop.last %}
                        {% assign page_dir = page_dir | append: part | append: '/' %}
                    {% endunless %}
                {% endfor %}
                <pre>page_dir = {{ page_dir }}</pre>
                <img src="{{ page_dir }}{{ image }}" alt="Illustration de {{ page.title }}" width="100" loading="lazy"/>
            </td>
            <td>
                <a href="{{ page.url }}">{{ page.title }}</a>
            </td>
            <td>
                {{ page.content | markdownify | strip_html | truncatewords: nb_words }}
            </td>
        </tr>
    {% endfor %}


```

Prompt type pour generer une version Medium d'un article :
```
Le fichier monolith_to_distributed.md est trop long et pas fait pour etre publie sur Medium.
Peux tu deposer dans raw_assets/ une version que je peux poster sur Medium.
Pense peut etre a utiliser les quelques unes des images img04 a img12 ou img14 pour illustrer.
Idealement je souhaite juste a avoir a coller le fichier que tu vas generer.
Evite les dash, les em-dash et les emojis. Ton de la discussion.
```

Afficher du JSON brut sans que Jekyll le parse :

{% raw %}
```json
{
  "win32RegValueName": "CodeOSS",
  "win32AppId": "{{E34003BB-9E10-4501-8C11-BE3FAA83F23F}",
  "win32AppUserModelId": "Microsoft.CodeOSS",
}
```
{% endraw %}

References :
* <https://just-the-docs.github.io/just-the-docs/>
* <https://jekyllcodex.org/without-plugins/>
* <https://mademistakes.com/>
* <https://just-the-docs.github.io/just-the-docs-tests/components/math/mathjax/index/>
* <https://jekyllrb.com/>
* <https://github.com/rouge-ruby/rouge/wiki/list-of-supported-languages-and-lexers>
* <https://www.mathematex.fr/guide-mathjax>
* <https://docs.mathjax.org/en/latest/input/tex/macros/index.html>
* <https://shopify.github.io/liquid/>

---

## Publication

### Alerte de securite Dependabot via Mail

Mail, titre typique : `[40tude] A security advisory on addressable affects at least one of your repositories`

0. Dans le mail, cliquer sur le lien en bas. Renvoie par exemple sur `https://github.com/40tude/40tude.github.io/security/dependabot/11`
1. Aller sur la page de l'alerte
2. Cliquer sur "Create Dependabot security update" (en haut a droite)
3. Dependabot cree une PR automatiquement
4. Aller sur la PR, cliquer "Merge pull request"
5. GitHub Pages rebuildera avec la version corrigee


### Alerte Dependabot sur le site GitHUb
0. Sur le GitHub du  site regarder "Securirty & quality"
1. Cliquer dessus si besoin
2. Aller dans la catégorie Dependabot
3. Cliquer sur `Create Dependabot security update` (bouton vert)
4. Cliquer sur `Review Security Update` (bouton vert)
4. Cliquer `Merge Pull Request` (bouton vert)
5. Cliquer `Confirm Merge` (bouton vert)
---

## SEO

### Tester et outils

Tester l'affichage sur Facebook/Twitter :
* <https://www.opengraph.xyz/>
* <https://metatags.io/>

Verifier qu'une page est bien indexee :
```
site:https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_02.html
```

Tooling :
* Google PageSpeed Insights : <https://pagespeed.web.dev/>
* Google Search Console : <https://search.google.com/search-console>
* Google Analytics : <https://analytics.google.com/analytics/web/>
* Web Page Test : <https://www.webpagetest.org/>
* GTMetrix : <https://gtmetrix.com/>
* Pingdom Tools : <https://tools.pingdom.com/>

Front matter SEO - ajouter ALT sur les images. Image OG ideale 1200x630px.


### Indexation Google

Pour s'assurer que toutes les pages de documentation sont indexees :

#### Etape A : Identifier la raison

Dans le menu **Pages** de Google Search Console, faire defiler sous le graphe. Raisons courantes :
* **Crawled - currently not indexed** : Google connait la page mais n'a pas decide de l'indexer (signal qualite/autorite)
* **Discovered - currently not indexed** : Google connait l'URL mais ne l'a pas encore crawlee
* **Excluded by 'noindex' tag** : Verifier si le generateur de site a ajoute un tag noindex par erreur

#### Etape B : Optimiser le maillage interne

* S'assurer que chaque page est liee depuis la sidebar, une table des matieres, ou un bouton Suivant/Precedent
* Google priorise les pages bien connectees

#### Etape C : Revalider le sitemap

1. Menu **Sitemaps** dans GSC
2. Verifier que l'URL du sitemap affiche "Success"
3. Si pas lu depuis des semaines, re-saisir l'URL et cliquer **Submit**

#### Etape D : Valider le correctif

Cliquer sur la raison identifiee (ex : "Crawled - currently not indexed"), puis cliquer sur **Validate Fix**.

---

## Communication

* LinkedIn
* <https://www.reddit.com/r/learnrust/>
* <https://www.reddit.com/r/rust/>
* <https://users.rust-lang.org/>
* Discord ?

Hashtags : `#RustLang` `@rustlang`

---

## Idees de billets

* Pareto et 50%-1%
* SOS Chapitres IV et suivants
* IA et ML - Relire les notes OneNote de preparation certification Architect IA
* 52 nuances de physique

---

## TODO

* Mettre a jour la page : Knee to knee
* Dans la home page faire apparaitre la categorie (avec un lien) et la date de derniere mise a jour ?
* Faire un billet sur le portage de WP vers markdown
* Mettre a jour la page : 022_compile_cpp_code_with_vscode
* Voir les histoires de balise `rel="canonical"`
* Voir les histoires de plugins
    * Va etre chaud
    * Faut installer ruby meme si on a un plugin sous forme de gem
    * Faut l'installer avec Bundle etc.
* SEO
    * Pas sur de comprendre grand chose au sujet
    * En plus je suis pas interesse par le sujet
* Optimization
    * Optimiser le site pour la vitesse ? just-the-docs.js = 1.6s je vois pas quoi faire
    * F12 ou Ctrl+Shift+I pour ouvrir les DevTools
    * Onglet Lighthouse > Performance, Accessibility, SEO, Best Practices, cliquer "Generate report"
    * Pagespeed Insights (<https://pagespeed.web.dev/>) : conseils prioritaires
    * WebPageTest (<https://www.webpagetest.org/>) : analyse depuis differents lieux

---

## DONE

* ~~Pages en anglais avec lang=en avant le Head~~
    * Voir `_layouts\default.html`. Si just the docs le modifie, recuperer la nouvelle version depuis `https://github.com/just-the-docs/just-the-docs/blob/main/_layouts/default.html`
* ~~Mettre les liens des urls dans les billets en gras~~
* ~~S'assurer que Google indexe bien le site~~
    * Y a toujours des soucis avec les vieilles pages pirates mais les bonnes pages sont bien indexees
* ~~Liens LinkedIn Youtube Github dans le footer de la navbar~~
* ~~Voir dans le repo GitHub si y a pas des articles a rapatrier~~
    * Notes de maths : Loi binomiale, Equation canonique
* ~~Lightbox pour zoomer sur les images~~
    * On peut cliquer sur toutes les images et zoomer sur PC et telephone
* ~~Ajouter un espace pour les discussions~~
* ~~Rapatrier la page Git Survival de GitHub ici~~
* ~~Rapatrier la page install Linux Mint qui est sur GitHub~~
* ~~Faire une feuille Sessions.pdf pour docs\07_moto\002_feuille_sessions\feuille_sessions.md~~
* ~~Faire une checklist.pdf~~
* ~~implement 404 page~~
* ~~Sur la 1ere page, la liste des 10 derniers billets mis a jour~~
* ~~Centrer les videos~~
* ~~Remplacer les CTRL + A par **CTRL + A** (verifier ALT et WIN)~~
* ~~Mettre a jour la facon d'ecrire Latex partout sur le site~~ : `$$\mathrm{\LaTeX}$$`
* ~~Page Linux Mint a rapatrier~~
* ~~Snowboard, ajouter des photos et des videos~~
* ~~Voir cette histoire de pull request de la part de Just The Docs~~
* ~~robots.txt~~ : verifier dans Google Search Console > Parametres
* ~~sitemap~~ : fix url in _config.yml + modify _includes\head_custom.html
* ~~Test du site avec outils Google~~ : Google Search Console, PageSpeed Insights
* ~~Ajouter un indicateur de duree de lecture en haut des pages~~
* ~~Installer Google Analytics~~
* ~~Configurer le site pour supporter les simple $ pour les equations~~
    * Impossible : <https://github.com/just-the-docs/just-the-docs/discussions/1593>
* ~~Comment faire des liens (ancres) au sein d'une meme page en markdown~~
* ~~Correcteur ortho FR dans VScode~~
* ~~Voir comment permettre le telechargement de fichiers pdf, zip, xlsx...~~
* ~~Porter les pages de SOS pour tester les outils de recuperation du site WP~~
* ~~Voir les histoires de permalinks~~ : <https://jekyllrb.com/docs/permalinks/#front-matter>
* ~~Back to Top~~
* ~~First et last Edit en bas de page~~
