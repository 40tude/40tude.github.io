C'est le portage du site 40tude.fr de WordPress + OVH vers markdown + Jekyll + Just the Docs + GitHub

## TODO
* Mettre à jour la page : Knee to knee
* Dans la home page faire apparaître la catégorie (avec un lien) et la date de dernière mise à jour ?
* Faire un billet sur le portage de WP vers markdown?
* Mettre à jour la page : 022_compile_cpp_code_with_vscode
* Voir ces histoires de balise rel="canonical"
* Voir ces histoires de plugins
    * Va être chaud
    * Faut installer ruby même si on a un plugin sous forme de gem
    * Faut l'installer avec Bundle etc.
* SEO?
    * Pas sûr de comprendre grand chose au sujet
    * En plus je suis pas intéressé par le sujet
* Optimization
    * Optimiser le site pour la vitesse ? just-the-docs.js = 1.6s je vois pas quoi faire
    * F12 ou Ctrl+Shift+I pour ouvrir les DevTools.
    * Onglet Lighthouse
    * Performance, Accessibility, SEO, Best Practices, cliquer sur "Generate report".
    * Pagespeed Insights (https://pagespeed.web.dev/) : Donne des conseils prioritaires.
    * WebPageTest (https://www.webpagetest.org/) : Pour analyser la vitesse depuis différents lieux.
## DONE
* ~~Pages en anglais? avec lang=en avant le Head~~
    * ~~À mon avis c'est mort mais bon je vais continuer à chercher~~
    * ~~Y a peut être une option avec les collections mais je sais pas si y a lang="en" avant le head~~
<span style="color:red"><b>IMPORTANT:</b></span>
    * Voir `_layouts\default.html`. Si jamais just the docs le modifie faudra aller chercher la nouvelle version
    * Voir `https://github.com/just-the-docs/just-the-docs/blob/main/_layouts/default.html`

* ~~Mettre les liens des urls dans les billets en gras~~
* ~~S'assurer que Google indexe bien le site. Pas sûr à 100% que ce soit le cas aujourd'hui 11 01 2025~~
    * Y a toujours des soucis avec les vieille pages pirates mais bon à priori les bonnes pages sont bien indexées
* ~~Liens LinkedIn Youtube Github dans le footer de la navbar ?~~
* ~~Voir dans le repo GitHub si y a pas des articles à rapatrier~~
    * Notes de maths
        * Loi binomiale
        * Equation canonique
* ~~Lightbox pour zoomer sur les images?~~
    * C'est sans doute perfectible mais on peut cliquer sur toutes les images
    * Ensuite on peut zoomer sur PC et sur telephone
    * Le seul truc c'est que si on zoom, la page en arrière plan zoom aussi
    * On va partir comme ça pour l'instant
* ~~Ajouter un espace pour les discussions~~
* ~~Rapatrier la page Git Survival de GitHub ici~~
* ~~Rapatrier la page install Linux Mint qui est sur GitHub~~
* ~~Faire une feuille Sessions.pdf pour docs\07_moto\002_feuille_sessions\feuille_sessions.md~~
* ~~Faire une checklist.pdf~~
* ~~implement 404 page~~
* ~~Sur la 1ere page, la liste des 10 derniers billets mis à jour~~
* ~~Centrer les vidéos~~
* ~~Remplacer les CTRL + A par **CTRL + A**~~
    * ~~Vérifier ALT et WIN~~
    * ~~WIN + X, A~~
* ~~Mettre à jour la façon d'ecrire Latex partout sur le site~~
    * $$\mathrm{\LaTeX}$$
* ~~Page Linux Mint à rapatrier~~
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




## IDEES DE BILLETS
* Pareto et 50%-1%
* SOS Chapitres IV et suivants
* IA et ML
    * Relire les notes OneNote de préparation certification Architect IA et voir ce que je peux rapatrier
* 52 nuances de physique (faut vraiment que je l'écrive un jour)
~~* Revenir sur Codingame~~


## VSCode Snippets
Roue Crantée en bas à gauche
Snippets
Choisir markdown.json
Le fichier est dans `C:\Users\phili\AppData\Roaming\Code\User\snippets`



## Longueur des posts
* 3_000 mots
* Découper en série le cas échéant
* Utiliser `word2read` ./blablabla.md (voir dans assets\wordcount)



<h2 align="center">
<span style="color:red"><b>This post is still being written.</b></span>
</h2>

<h2 align="center">
<span style="color:orange"><b>This post is still being reviewed.</b></span>
</h2>


## À garder sous le coude
* Ç = ALT + 128 (du pavé numérique)
* — = ALT + 0150 (tiret long, em dash, tiret cadratin, pas d'espace autour en anglais)
* Supprimer les espaces vides en fin de ligne : `[ \t]+$`
    * Voir aussi settings.json et `"files.trimTrailingWhitespace": true,`

```json

"em dash": {
    "prefix": "---",
    "body": ["—"],
    "description": "Insert em dash"
}
```

### The name of these dashes
* The character — is called an em dash (tiret cadratin)
* Not to be confused with:
    * `-` → hyphen
    * `–` → en dash (tiret demi-cadratin)
* What is the em dash used for?
    * The em dash is used to insert a parenthetical remark or clarification into a sentence, in a more expressive way than commas.


* ❶ U+2776
* ❷ U+2777
* ❸ U+2778
* ❹ U+2779
* ❺ U+277A
* ❻ U+277B
* ❼ U+277C
* ❽ U+277D
* ❾ U+277E

Sans Serif
* ➊ U+278A
* ➋ U+278B
* ➌ U+278C
* ➍ U+278D
* ➎ U+278E
* ➏ U+278F
* ➐ U+2790
* ➑ U+2791
* ➒ U+2792
* ➓ U+2793

* ` NOT YET TRANSFERED `




### Callout
* Voir _config.yml

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

Callouts : https://just-the-docs.com/docs/ui-components/callouts/



### Video avec légende
<figure style="text-align: center;">
  <iframe width="560" height="315" src="https://www.youtube.com/embed/EZfM2VMs_vI?si=FHS-1PFIqBG70Ffs&amp;start=55" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
  <figcaption>I love the opening theme. It gives me goosebumps every time.</figcaption>
</figure>

### Image avec légende
<div align="center">
<img src="./assets/img_02.webp" alt="" width="450" loading="lazy"/><br/>
<span>1986...</span>
</div>


### Lien sur une page du site
[Coding Interview Patterns problems]({%link docs/06_programmation/rust/007_coding_interview_patterns/13_graphs/268_longest_increasing_path.md%})



### Line sur fichier à télécharger
`[Agenda de la Journee de Roulage](assets/agenda_roulage.pdf)`





### Note de bas de page

Dans le texte :
```
une valeur entière[^1]
```

Tout en bas de l'article
```
----

[^1]: Ma note de bas de page.

```




### Target

```
[titre](https://youtu.be/57ivuBX1kLU?si=bI3xfPOcipMe5F3O&t=42){:target="_blank"}

Pour les images sur les pages où il y a un permalink
<div align="center">
<img src="{%link docs/08_snowboard/assets/img_05.webp%}" alt="" width="450" loading="lazy"/>
</div>


```







### Ancre sur une autre une page du site

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








### Ancre
Sur un paragraphe dans une autre page
```
[Three Steps]({%link docs/07_moto/001_notes_pilotage/003_three_steps/three_steps.md%}#le-point-de-corde-pc)

[Aller à cette section](#short-id) Dès qu'on met le # code propose une liste d'ancres

[Aller à cette section](#short-id)
## Un titre long pour une section <a id="short-id"></a>
```


### Table of content
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
   - CTRL + H + Exp Reg
   - `(#{3,}\s.+)`
   - `$1\n{: .no_toc }`

```




```
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

### sous titre
```
# Rust Traits: Defining Character
{: .no_toc }

From basic syntax to building plugins with once_cell and organizing your Rust projects.
{: .lead }
```


{% raw %}
```json
{
  "win32RegValueName": "CodeOSS",
  "win32AppId": "{{E34003BB-9E10-4501-8C11-BE3FAA83F23F}",
  "win32AppUserModelId": "Microsoft.CodeOSS",
}
```
{% endraw %}



```
<p>Nombre d'articles du site : {{ site.pages | size }}</p>
{% for page in site.pages limit: 1 %}
  <p>{{ page.last_modified_date }}</p>
{% endfor %}

```


* <https://just-the-docs.github.io/just-the-docs/>
* <https://jekyllcodex.org/without-plugins/>
* <https://mademistakes.com/>
* <https://just-the-docs.github.io/just-the-docs-tests/components/math/mathjax/index/>
* <https://jekyllrb.com/>
* <https://github.com/rouge-ruby/rouge/wiki/list-of-supported-languages-and-lexers>
* <https://www.mathematex.fr/guide-mathjax>
* <https://docs.mathjax.org/en/latest/input/tex/macros/index.html>
* <https://shopify.github.io/liquid/>




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
CTRL + H + Exp Reg
(#+)\s\*\*(.*?)\*\*
$1 $2
```


### Centrer les vidéos Youtube
```
CTRL + H + Exp Reg
(<iframe.*?<\/iframe>)
<div align="center">\n$1\n</div>
```


```
<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/ZbZSe6N_BXs?start=31" frameborder="0" allowfullscreen></iframe>
</div>


```


### Remplacer les $ seuls par $$
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


### Lister tous les fichiers qui sont pas .webp dans les sous-répertoires /assets

```powershell
Get-ChildItem -Recurse -Directory -Filter "assets" | ForEach-Object {
    # List files in the found "assets" directory
    Get-ChildItem -Path $_.FullName -File | Where-Object {
        $_.Extension -ne ".webp"
    } | ForEach-Object {
        # Output the full path of files that don't have a .webp extension
        Write-Output $_.FullName
    }
}
```


### liquid
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

### Tooling (à voir plus tard)
* Google PageSpeed Insights : <https://pagespeed.web.dev/>
* Google Search Console : <https://search.google.com/search-console>
* Google Analytics : <https://analytics.google.com/analytics/web/>
* Web Page Test : <https://www.webpagetest.org/>
* GTMetrix : <https://gtmetrix.com/>
* Pingdom Tools : <https://tools.pingdom.com/>






### Verifier qu'une page est bien indexée
`site:https://www.40tude.fr/docs/06_programmation/rust/016_errors/errors_02.html`





### How to reach your goal: Indexing all pages

To ensure all your technical documentation is indexed, follow these steps:

#### Step A: Identify the "Reason"

Scroll down below the graph in the **Pages** menu. Google lists the specific reasons why those 111 pages aren't indexed. Common ones include:

* **Crawled - currently not indexed:** Google knows the page exists but hasn't decided to put it in the index yet. This is often a "quality" or "authority" signal.
* **Discovered - currently not indexed:** Google knows the URL exists but hasn't even crawled it yet because it doesn't want to overload your server.
* **Excluded by ‘noindex’ tag:** Check if your documentation generator (like Hugo, Docusaurus, or MkDocs) accidentally added a noindex tag.

#### Step B: Optimize Internal Linking

Documentation sites often have "orphan pages" (pages with no links pointing to them).

* Ensure every page is linked from a sidebar, a table of contents, or a "Next/Previous" button.
* Google prioritizes pages that are well-connected.

#### Step C: Update and Re-submit your Sitemap

1. Go to the **Sitemaps** menu in GSC.
2. Ensure the sitemap URL is correct and showing "Success."
3. If it hasn't been read in weeks, you can re-enter the URL and click **Submit** to ping Google.

#### Step D: "Validate Fix"

Once you have identified the reason (e.g., "Crawled - currently not indexed"), click on that specific row and click the **Validate Fix** button. This tells Google to prioritize re-evaluating those 111 pages.



## SEO
Tester l'affichage sur Facebook/Twitter
Voir front matter : image: docs/06_programmation/rust/026_monolith_to_distributed/assets/img03.webp Ideal 1200x630px
https://www.opengraph.xyz/
https://metatags.io/

Faire une image spécifique: PNG 1200px width x 630px height
image: docs/06_programmation/rust/026_monolith_to_distributed/assets/og-img.png
┌───────────────────────────────────────────────────┐
│                                                   │
│   🦀                                              │
│                                                   │
│   Learning Modular Monolith                       │
│   Architecture with Rust                          │
│                                                   │
│   From Hello World to Hexagonal Architecture      │
│   7 Steps • Traits • Ports & Adapters             │
│                                                   │
│                                  40tude.fr        │
└───────────────────────────────────────────────────┘


ALT sur les images

published: true
----> author: 40tude
lang: en-US
layout: default
title: "From Monolith to Distributed Systems in Rust: A Practical Introduction"
description: "A hands-on journey through small, working projects to understand when and why architecture needs to evolve."
----> image: docs/06_programmation/rust/026_monolith_to_distributed/assets/img03.webp
----> twitter:
---->   card: summary_large_image
parent: "Rust"
nav_order: 35
date:               2026-02-11 10:00:00
last_modified_date: 2026-02-13 00:30:00



## Communication
* LinkedIn
* https://www.reddit.com/r/learnrust/
* https://www.reddit.com/r/rust/
* https://users.rust-lang.org/
* LinkedIn
* Discord ?

#RustLang
@rustlang



