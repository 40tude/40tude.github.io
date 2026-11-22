# Édition

Aide-mémoire pour écrire les articles : Markdown, kramdown, Liquid, équations, notebooks.

## Table des matières

- [Édition](#édition-1)
  - [Nouvel article (new-post)](#nouvel-article-new-post)
  - [Longueur des posts](#longueur-des-posts)
  - [Liens et ancres](#liens-et-ancres)
  - [Callouts](#callouts)
  - [VSCode Snippets](#vscode-snippets)
  - [Caractères spéciaux et tirets](#caractères-spéciaux-et-tirets)
  - [Vidéo avec légende](#vidéo-avec-légende)
  - [Image avec légende](#image-avec-légende)
  - [Note de bas de page](#note-de-bas-de-page)
  - [Table of content dans un article](#table-of-content-dans-un-article)
  - [Front matter](#front-matter)
  - [Sous-titre](#sous-titre)
  - [Tableaux](#tableaux)
  - [Regex utiles](#regex-utiles)
  - [Aperçu VSCode et équations](#aperçu-vscode-et-équations)
  - [PowerShell](#powershell)
  - [Template d'article](#template-darticle)
  - [Liquid](#liquid)
- [NoteBook Python](#notebook-python)

---

## Édition

### Nouvel article (new-post)

Script `tools/New-Post.ps1` qui crée un article à partir de `assets/_article_template.md`.

```powershell
cd docs\03_maths              # un dossier de catégorie (doit contenir un index.md)
new-post mon_article          # numéro auto : max du dossier + 1 (les 9xx sont ignorés)
new-post 088_mon_article      # numéro imposé
```

Résultat :

* `088_mon_article/` avec `assets/` et `raw_assets/` (vide, pas sur Git mais sur OneDrive)
* `088_mon_article/mon_article.md` copié depuis le template
* `assets/img00.webp` : image bidon copiée depuis `assets/_article_template_img00.webp`
* Front matter ajusté : `parent` (= `title:` de `index.md` du dossier courant), `image`, `date` et `last_modified_date` à l'heure courante. Le reste (`title`, `description`, `lang`...) reste comme dans le template.
* Ouvre le `.md` dans VSCode

Le script refuse si : pas dans un dossier sous `docs/`, pas d'`index.md`, dossier déjà existant ou numéro déjà pris.

**Attention** : la commande `new-post` vient d'une fonction ajoutée dans `$PROFILE` (`code $PROFILE` pour la voir). Si le site change de dossier ou de machine, la recréer :

```powershell
function new-post {
    & "$HOME\OneDrive\Documents\40tude.github.io\tools\New-Post.ps1" @args
}
```


### Longueur des posts
* 3_000 mots idéalement
* Dans une première version ne pas se poser de question mais ensuite, découper en épisodes le cas échéant
* Compter avec `words2read`
  * installation : voir le README du dépôt
  * Sinon aller voir dans `..\Documents\Programmation\rust\00_cli\words2read`

```powershell
# from ..\Documents\40tude.github.io
words2read .\docs\09_ml_ai\008_math_proof_with_claude\math_proof_with_claude.md

Words to read : 6087

Excerpt after 3000 words:
offers three options....
```


### VSCode Snippets
Roue crantée en bas à gauche > Snippets > Choisir markdown.json
Le fichier est dans `C:\Users\phili\AppData\Roaming\Code\User\snippets`


### Caractères spéciaux et tirets

```json
"em dash": {
    "prefix": "---",
    "body": ["—"],
    "description": "Insert em dash"
}
```

* `-` hyphen
* `–` en dash (tiret demi-cadratin)
* `—` em dash (tiret cadratin) : insérer une remarque parenthétique, plus expressif que la virgule - ALT + 0150
* `É` : ALT + 144
* `Ç` : ALT + 128
* ` ` : espace insécable ALT + 0160 (MS Edit par exemple)

Supprimer les espaces vides en fin de ligne : `[ \t]+$`
(Voir aussi settings.json : `"files.trimTrailingWhitespace": true`)

Numéros cerclés :
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

Référence : <https://just-the-docs.com/docs/ui-components/callouts/>

Bannières à copier-coller dans un article en cours :

<h2 align="center">
<span style="color:red"><b>This post is still being written.</b></span>
</h2>

<h2 align="center">
<span style="color:orange"><b>This post is still being reviewed.</b></span>
</h2>


### Vidéo avec légende

```html
<figure style="text-align: center;">
  <iframe width="560" height="315" src="https://www.youtube.com/embed/EZfM2VMs_vI?si=FHS-1PFIqBG70Ffs&amp;start=55" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
  <figcaption>I love the opening theme. It gives me goosebumps every time.</figcaption>
</figure>
```

Centrer une vidéo YouTube sans légende :
```html
<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/ZbZSe6N_BXs?start=31" frameborder="0" allowfullscreen></iframe>
</div>
```


### Image avec légende

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
[Coding Interview Patterns problems]({%link docs/06_programmation/rust/007_coding_interview_patterns/13_graphs/268_longest_increasing_path.md%}){:target="_blank"}
```

Lien sur un fichier à télécharger :
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
See [this section]({% post_url 2025-09-01-post-a %}#my-important-section){:target="_blank"}.
[expression]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#expressions){:target="_blank"}
[statement]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#statement){:target="_blank"}
[function]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#function-method-procedure){:target="_blank"}
[invariant]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#invariant){:target="_blank"}
[indirection]({%link docs/06_programmation/001_computer_science_vocabulary/computer_science_vocabulary.md%}#indirection){:target="_blank"}
```

Ancre dans la même page (ou depuis une autre page) :
```
[Aller à cette section](#short-id)
## Un titre long pour une section <a id="short-id"></a>
```

```
[Three Steps]({%link docs/07_moto/001_notes_pilotage/003_three_steps/three_steps.md%}#le-point-de-corde-pc)
[Finalize Windows 11 installation]({% link docs/04_windows/001_finalize_windows_11_installation/finalize_windows_11_installation.md %})
```

Option avec accents :
```
[Le planning en mode synthétique](#le-planning-synthetique)
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
Mettre {:toc} que sur les titres de niveau 2. Éviter 3 et +

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
last_modified_date: 2026-11-20 13:24:57
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
last_modified_date: 2026-11-20 13:24:57
```

Image OG idéale : PNG 1200px x 630px
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
| Aligné à gauche  | Centré          | Aligné à droite  |
| :--------------- |:---------------:| ----------------:|
| Aligné à gauche  | Ce texte        |  Aligné à droite |
```

Compter les articles du site / date de dernière modif :
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

Centrer les vidéos YouTube :
```
CTRL + H + Exp Reg
(<iframe.*?<\/iframe>)
<div align="center">\n$1\n</div>
```

Équations - remplacer les $ seuls par $$ :
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

Exemples d'équations :
L'alignement se fait sur le `&`. `align*` sans numérotation. `align` avec numérotation.

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


### Aperçu VSCode et équations

Objectif : que l'aperçu VSCode (`CTRL + K V`) affiche les équations comme le site dans Edge, pour relire un article sans surprise.

#### Qui affiche quoi

| Où                      | Moteur Markdown          | Moteur équations |
|-------------------------|--------------------------|------------------|
| Site (Edge)             | Jekyll + kramdown        | MathJax 3 (voir `_includes/mathjax.html`) |
| Aperçu VSCode `CTRL+K V`| Markdown Preview Enhanced (MPE) | MathJax (forcé dans `settings.json`) |

* `CTRL + K V` est associé à 2 commandes : l'aperçu natif de VSCode ET celui de MPE. C'est MPE qui gagne (raccourci d'extension prioritaire).
* Vérifier : `CTRL + K CTRL + S` > icône clavier "Record Keys" > taper `CTRL + K V`. Colonne Source.
* Vérifier dans l'aperçu : clic droit. Si on voit "Open in Browser", "Export"... c'est MPE.

#### Problème 1 : `\%` dans `\text{}`

Symptôme : `\text{ en \%}` OK dans VSCode, affiche `\%` en clair dans Edge.

* MathJax n'interprète PAS les commandes à l'intérieur de `\text{}` (il faudrait l'extension `textmacros`, non chargée sur le site).
* KaTeX (moteur par défaut de MPE et de l'aperçu natif) les interprète. D'où la différence.

Solutions :
1. Écrire le `%` hors du `\text{}` : `\text{Variation de } G \text{ en } \%` (garder l'espace dans `\text{ en }`).
2. Dans `settings.json` (`C:\Users\phili\AppData\Roaming\Code\User\settings.json`), MPE est passé en MathJax pour voir les mêmes erreurs que le site :

```json
"markdown-preview-enhanced.mathRenderingOption": "MathJax",
```

#### Problème 2 : `$$...$$` dans une phrase

Symptôme : dans l'aperçu VSCode, `Dans lequel $$V_n$$ c'était...` coupe le paragraphe, `V_n` se retrouve centré sur sa propre ligne. Sur le site, tout est OK.

Cause : kramdown et MPE ne lisent pas `$$` de la même façon.
* kramdown : `$$...$$` au milieu d'une ligne de texte = équation **inline**. Seul sur sa ligne (paragraphe à part) = équation **centrée**.
* MPE : `$$...$$` = toujours équation centrée.
* On ne peut pas passer le site aux `$` simples (voir [todo.md](todo.md), section DONE, discussion just-the-docs #1593). Donc on adapte l'aperçu, pas les articles.

Solution : le script `.crossnote/parser.js` (à la racine du repo).
* `.crossnote/` = dossier de config de MPE pour le workspace (MPE est basé sur la lib `crossnote`).
* MPE appelle `onWillParseMarkdown(markdown)` AVANT d'afficher le fichier. Le script reçoit le texte, le modifie et le renvoie. Le fichier `.md` sur disque n'est jamais modifié.
* Ce qu'il fait : remplace les `$$...$$` qui sont au milieu d'une ligne par `$...$` (inline pour MPE).
* Ce qu'il ne touche pas :
    * les blocs de code (```` ``` ```` ou `~~~`)
    * le code inline entre backticks
    * les lignes qui ne contiennent qu'une équation : `$$V_n = V_0 (1 + r)^n$$`
    * les blocs multi-lignes entre deux lignes `$$` (`\begin{align*}`...)
* Jekyll ignore les dossiers qui commencent par un point : `.crossnote/` n'est pas publié sur le site.
* Depuis MPE 0.8.x, `parser.js` tourne dans un bac à sable (QuickJS). Pas d'accès au disque ni au réseau, juste du JavaScript pur.

Exemple :
```text
Avant  : Dans lequel $$V_n$$ c'était la valeur au bout de $$n$$ années
Après  : Dans lequel $V_n$ c'était la valeur au bout de $n$ années     (vu par MPE seulement)
```

Après une modif de `parser.js` : fermer l'aperçu et le rouvrir. Sinon `CTRL + SHIFT + P` > "Developer: Reload Window".

Tester le script hors VSCode (affiche les lignes transformées) :
```powershell
$js = @'
const fs = require('fs');
const hooks = eval(fs.readFileSync(process.argv[2], 'utf8'));
hooks.onWillParseMarkdown(fs.readFileSync(process.argv[3], 'utf8')).then(o => console.log(o));
'@
Set-Content ./temp/test_parser.js $js
node ./temp/test_parser.js ./.crossnote/parser.js ./docs/03_maths/022_interets_composes/interets_composes.md
```

Limites connues :
* Un `$` isolé dans le texte (ex. "la question à 1M$") peut être pris pour un début d'équation par MPE. Sans conséquence si le paragraphe ne contient pas d'équation.
* Le script ne connaît que les cas ci-dessus. Si un rendu bizarre apparaît uniquement dans VSCode, penser à lui en premier.
* Désactiver : renommer ou supprimer `.crossnote/parser.js`, puis rouvrir l'aperçu.

Sur un autre PC : il suffit que `.crossnote/parser.js` soit commité et que MPE soit installé, avec le réglage MathJax ci-dessus dans `settings.json`.


### PowerShell

Lister les 10 plus longs articles :

```powershell
cd C:\Users\phili\OneDrive\Documents\40tude_to_repair\converted_to_markdown\docs

Get-ChildItem -Path . -Recurse -Filter *.md | Sort-Object -Property Length -Descending | Select-Object -First 10 | Format-Table FullName, @{Name="Size (KB)"; Expression={"{0:N2}" -f ($_.Length / 1KB)}}
```

Lister tous les fichiers qui ne sont pas .webp dans les sous-répertoires /assets :

```powershell
Get-ChildItem -Recurse -Directory -Filter "assets" | ForEach-Object {
    Get-ChildItem -Path $_.FullName -File | Where-Object {
        $_.Extension -ne ".webp"
    } | ForEach-Object {
        Write-Output $_.FullName
    }
}
```

### Template d'article

* Utilise Liquid pour écrire une seule fois le titre et la description
* La date et l'heure de dernière mise à jour (`last_modified_date`) sont maintenues par un hook de pre-commit versionné dans `tools/hooks/pre-commit`
    * `.git/hooks/` n'est pas versionné, donc Git est configuré pour utiliser `tools/hooks/`
    * Après un clone (autre PC, dossier déplacé), activer le hook une fois :

```powershell
git config core.hooksPath tools/hooks
git config core.hooksPath   # vérif : doit afficher tools/hooks
```

* `.gitattributes` force LF sur `tools/hooks/*` (sinon `autocrlf` passe en CRLF et bash plante)
* `tools/generate_last_update.ps1` lit ces dates et génère `tools/last_update_YYYYMMDD.md` (articles triés du plus récent au plus ancien)




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

Prompt type pour générer une version Medium d'un article :
```
Le fichier monolith_to_distributed.md est trop long et pas fait pour être publié sur Medium.
Peux-tu déposer dans raw_assets/ une version que je peux poster sur Medium.
Pense peut-être à utiliser quelques-unes des images img04 à img12 ou img14 pour illustrer.
Idéalement je souhaite juste avoir à coller le fichier que tu vas générer.
Évite les dash, les em-dash et les emojis. Ton de la discussion.
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

Références :
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

## NoteBook Python

Objectif : un billet + son notebook Jupyter, édités dans le VSCode ouvert sur la racine du site.
Exemple de référence : `docs/03_maths/022_interets_composes/`.

### Organisation

```text
docs/<categorie>/<NNN_billet>/
    <billet>.md                 article publié
    assets/                     images publiées (.webp)
    raw_assets/                 IGNORE PAR GIT (.gitignore : **/raw_assets/)
        .venv/                  env Python propre au notebook
        <billet>.ipynb          notebook de travail
        README.md               mode d'emploi lecteur (installer uv, VSCode, kernel...)
```

* `raw_assets/` n'est ni commité ni publié. Pour proposer le notebook en telechargement, le copier dans `assets/` et le lier depuis l'article.
* Modèle de README lecteur : `docs/03_maths/022_interets_composes/raw_assets/README.md` (pas sur GitHub, le garder sur OneDrive).

### Pourquoi VSCode ne trouve pas le `.venv`

VSCode ne détecte automatiquement un `.venv` qu'à la racine du dossier ouvert (`40tude.github.io`).
Un `.venv` dans `docs/.../raw_assets/` est trop profond : le kernel picker ne propose que les Python globaux de `uv` (`AppData\Roaming\uv\python\...`).
Ne PAS cliquer "Create Python Environment" : ça crée un `.venv` à la racine du site.

### Checklist nouveau notebook

1. Créer le dossier et l'environnement :

```powershell
cd docs/<categorie>/<NNN_billet>
New-Item -ItemType Directory -Force raw_assets/
cd raw_assets
uv venv
uv pip install ipykernel numpy matplotlib scipy
```

2. Vérifier que le kernel est utilisable :

```powershell
./.venv/Scripts/python.exe -c "import ipykernel; print('ipykernel ok')"
```

3. Copier le chemin complet du Python du `.venv` dans le presse-papier :

```powershell
(Resolve-Path ./.venv/Scripts/python.exe).Path | Set-Clipboard
```

4. Dans VSCode : `CTRL + SHIFT + P` > `Python: Select Interpreter` > `Enter interpreter path...` > `CTRL + V` > Entrée.
5. Créer ou ouvrir `raw_assets/<billet>.ipynb`.
6. En haut à droite du notebook : `Select Kernel` > `Python Environments...` > icône Refresh > choisir l'entrée `.venv` (prompt = `raw_assets`, chemin `...\raw_assets\.venv\Scripts\python.exe`).
    * Si absent : `CTRL + SHIFT + P` > `Developer: Reload Window`, puis recommencer l'étape 6.
7. Vérifier dans une cellule :

```python
import sys; print(sys.executable)   # doit finir par raw_assets\.venv\Scripts\python.exe
```

8. Ajouter un package plus tard (depuis `raw_assets/`) : `uv pip install <package>`. Pas besoin de rechoisir le kernel, juste `Restart` du kernel.
9. Rédiger billet et notebook en parallèle. Images du notebook à publier : export en `.webp` dans `assets/`.
10. Publication : copier le `.ipynb` (et le README lecteur si utile) dans `assets/`, ajouter le lien dans l'article :

```
[Télécharger le notebook](assets/<billet>.ipynb)
```

### Alternatives

* Ouvrir seulement le dossier du notebook : `code docs/<...>/raw_assets` (le `.venv` est alors à la racine, détecté tout seul). Pratique pour travailler le notebook seul, mais on perd le reste du site.
* `File > Add Folder to Workspace...` > `raw_assets` : le site reste ouvert et le `.venv` est détecté.

### Réinstaller l'env (autre PC, `.venv` cassé, version Python changée)

```powershell
cd docs/<...>/raw_assets
Remove-Item -Recurse -Force .venv
uv venv
uv pip install ipykernel numpy matplotlib scipy
```

Puis refaire les étapes 3 à 7 de la checklist.
