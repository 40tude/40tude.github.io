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
  - [Images : format, taille, compression](#images--format-taille-compression)
  - [Note de bas de page](#note-de-bas-de-page)
  - [Table of content dans un article](#table-of-content-dans-un-article)
  - [Front matter](#front-matter)
  - [Sous-titre](#sous-titre)
  - [Tableaux](#tableaux)
  - [Regex utiles](#regex-utiles)
  - [Aperçu VSCode et équations](#aperçu-vscode-et-équations)
  - [PowerShell](#powershell)
  - [Template d'article](#template-darticle)
  - [Vignettes de la page d'accueil](#vignettes-de-la-page-daccueil)
  - [Styles et icônes du site](#styles-et-icônes-du-site)
  - [En cas de mise à jour de Just the Docs](#en-cas-de-mise-à-jour-de-just-the-docs)
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
    purple        red              blue               green        yellow           orange
{: .note }    {: .warning }    {: .important }    {: .new }    {: .highlight }    {: .fortytude }
```

Just the Docs 0.12 ne dessine que des bordures (aucun fond, que `callouts_level` soit `quiet` ou `loud`). Le fond teinté (alpha 0.12) est ajouté à la main dans `_sass/custom/custom.scss`. `callouts_opacity` n'a plus d'effet depuis les anciennes versions du thème.

Callout maison `fortytude` (titre « 40tude », couleurs du logo) :
- Le nom ne peut pas être `40tude` : une classe CSS ne peut pas commencer par un chiffre (le build casserait).
- Just the Docs n'a pas de couleur `orange` : `$orange-000` (`#fd851c`, orange du logo, bordure fine + fond) et `$orange-300` (`#b54708`, titre + bordure gauche, plus foncé pour le contraste) sont définis dans `_sass/custom/setup.scss`.
- Pour créer un autre callout : 1) définir `$<couleur>-000` et `$<couleur>-300` dans `setup.scss` si la couleur n'existe pas, 2) ajouter le callout dans `_config.yml` (section `callouts`), 3) ajouter sa ligne de fond dans `custom.scss`.

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

Faut utiliser le snippet `div+CTRL+TAB`

```html
<figure style="max-width: 560px; margin: auto;">
<div style="position: relative; padding-bottom: 56.25%; height: 0;">
    <iframe
    src="https://www.youtube.com/embed/EZfM2VMs_vI?start=55"
    title="I'm a legend"
    style="position: absolute; inset: 0; width: 100%; height: 100%;"
    allowfullscreen>
    </iframe>
</div>
<figcaption style="text-align: center;">
    I love the opening theme. It gives me goosebumps every time.
</figcaption>
</figure>
```




### Image avec légende

Faut utiliser le snippet `fig+CTRL+TAB`


```html
<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img00.webp"
    alt="Describe the image here"
    style="max-width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>
```

**`max-width: 100%` et pas `width: 100%` sur `<img>`** (depuis nov. 2026)
* `max-width: 900px` sur `<figure>` limite le *cadre*, pas l'image. Avec `width: 100%`, l'image remplit toujours le cadre : une image de 510 px est étirée à ~800 px et devient floue.
* Avec `max-width: 100%`, l'image garde sa taille native (1 pixel de l'image = 1 px CSS), centrée, et ne rétrécit que si le cadre est plus étroit (mobile, ou image plus large que 900 px).
* Conséquence : une petite image (zoom, crop) s'affiche petite. Pour qu'elle s'affiche plus grand, il faut une image plus grande (réexport), pas du CSS.
* Les articles antérieurs à nov. 2026 utilisent encore `width: 100%` (non migrés).



### Images : format, taille, compression

Choix appliqués sur le blog depuis nov. 2026 (série *How Attention Works*).

**Règle de base : le mode (avec ou sans perte) dépend du type d'image, le chiffre de qualité vient après.**

| Type d'image | Mode WebP | Qualité | Largeur |
|---|---|---|---|
| Couverture `img00.webp` (photo, illustration) | avec perte | 75 à 85 (80 par défaut) | **1200** (1200 x 675 en 16:9, idéal OG : 1200 x 630) |
| Photo dans le corps de l'article | avec perte | 75 à 85 | **1200** |
| Capture d'écran (interface, code, texte) | **sans perte** | sans objet | taille native, ne pas réduire |
| Schéma, diapo dense (draw.io, etc.) | avec perte en général | 85 à 90 | taille native, ne pas réduire sous **1800** (draw.io : export `-s 2`) |

**Largeur : deux chiffres, deux familles d'images**
* **1200** pour ce qui se regarde (couverture, photo). **1800 ou plus** pour ce qui se lit (texte, étiquettes, code).
* Pourquoi plus de 900 alors que les figures font `max-width: 900px` ? Ce sont des pixels *CSS*. Sur un portable Windows à 150 % ou un écran haute densité (2x), 900 px CSS = 1350 à 1800 pixels réels. Image plus petite = agrandie par le navigateur = floue.
* Une photo agrandie de 1200 à 1800 reste agréable. Un texte de 8 px agrandi devient pénible à lire.
* La lightbox (ci-dessous) affiche l'image à sa taille réelle, dans la limite de 90 % de la fenêtre. Une image de 1200 px y reste à 1200 px CSS ; un schéma de 2400 px remplit l'écran. Encore une raison de garder les schémas denses en grand.

**Lightbox : toutes les images du site sont cliquables**
* Script maison `assets/js/lightbox.js`, chargé par `_includes/footer_custom.html` (ajouté le 2025-01-25). Depuis le 2026-11-25 il est chargé avec `defer` : il s'exécute après la lecture de la page et ne retarde plus l'affichage.
* Au chargement, il ajoute la classe `lightbox-image` à chaque `<img>` dont le `src` finit par `.jpg`, `.jpeg`, `.png`, `.gif` ou `.webp`. Clic = image sur fond noir, `max-width` et `max-height` à 90 % de la fenêtre. Fermeture : croix en haut à droite ou clic à côté de l'image.
* Rien à faire dans l'article : pas besoin d'entourer l'image d'un `<a href>`.
* Si l'image a un attribut `data-full="..."`, la lightbox ouvre ce fichier au lieu du `src`. Sert à la page d'accueil : elle affiche la vignette, mais le clic ouvre la couverture en grand.
* `assets/js/lightbox_00.js` et `assets/css/lightbox_00.css` = restes d'un premier essai avec zoom (abandonné, commit « Retour en arriere le zoom fonctionne pas »). Non chargés.

**Pourquoi le texte bave en WebP avec perte**
* Le mode avec perte stocke la couleur à demi-résolution (1 pixel de couleur pour un bloc 2 x 2).
* Invisible sur une photo. Sur du texte fin, surtout coloré sur fond coloré (coloration syntaxique), ça bave et ça fait des halos.
* Monter la qualité à 90 ou 100 ne change rien : la perte de couleur est faite quelle que soit la qualité.
* Une capture d'écran = grands aplats de couleur unie : le sans perte la compresse très bien. Il est souvent **plus léger** que le avec perte.

**Mesures (nov. 2026)**

| Image | avec perte q80 | q90 | q100 | sans perte |
|---|---|---|---|---|
| Capture JupyterLab 1293 x 831 | 50 Ko | 65 Ko | 95 Ko | **35 Ko** |
| Schéma draw.io 925 x 2225 | **42 Ko** | 56 Ko | 88 Ko | 130 Ko |

Le schéma est l'exception : l'anti-aliasing du texte et les ombres créent beaucoup de nuances, le sans perte gonfle. Dans le doute, exporter les deux et garder le plus léger qui est propre.

**Choisir le chiffre de qualité (mode avec perte)**
1. Exporter à 90 et à 80.
2. Comparer à 100 % de zoom sur les zones difficiles : texte, bords nets, dégradés.
3. Garder la valeur la plus basse sans différence visible. Photo ou couverture : presque toujours entre 75 et 85.

Au-delà de 90, le poids grimpe vite pour un gain invisible (capture ci-dessus : 65 Ko à q90, 95 Ko à q100).

**Outils**
* Paint.NET + plugin WebPFileType : décocher « Sans perte » + qualité 80 pour une couverture, cocher « Sans perte » pour une capture.
* Paint.NET, redimensionner (Image > Redimensionner) :
  * « En taille absolue », « Conserver les proportions » coché, régler la largeur.
  * « Utiliser la correction gamma » **coché** : la moyenne se fait sur la luminosité réelle, les traits fins et le texte noir ne s'épaississent pas et ne foncent pas.
  * Résolution (pixels/pouce) et taille d'impression : sans effet sur le web, ignorer.
  * Ré-échantillonnage :

| Option | Usage |
|---|---|
| **Adaptatif (net)** | **par défaut** : couverture, photo. Choisit la méthode selon le facteur de réduction et accentue un peu. |
| **Fant** | schéma, capture avec texte. Moyenne exacte des pixels couverts, aucun halo, rendu un peu plus doux. |
| Bicubique | correct pour une réduction modérée (1675 -> 1200), moins bon pour une forte réduction. |
| Lanczos | très net mais petits halos clairs autour des traits noirs. Plutôt photos. |
| Bicubique (lisse), Bilinéaire | plus flous, sans intérêt ici. |
| Bilinéaire (basse qualité), Plus proche voisin | à éviter (crénelage). Le plus proche voisin ne sert qu'au pixel art ou pour agrandir des icônes. |

  * Contrôle : zoom 100 % autour du texte. Liseré clair autour des traits noirs avec « Adaptatif (net) » -> repasser en Fant.
  * Largeur cible : voir « Largeur : deux chiffres » plus haut (1200 pour une couverture ou une photo, pas sous 1800 pour un schéma dense).
* En ligne de commande (pas besoin d'ImageMagick) :

```powershell
# Couverture : mise à l'échelle + recadrage centré en 1200x675 (pas de déformation si la source n'est pas en 16:9), avec perte
uv run --no-project --with pillow python -c "from PIL import Image, ImageOps; ImageOps.fit(Image.open('img00.jpg').convert('RGB'), (1200, 675), Image.LANCZOS).save('img00.webp', 'WEBP', quality=80, method=6)"

# Capture d'écran : sans perte
uv run --no-project --with pillow python -c "from PIL import Image; Image.open('capture.png').save('img01.webp', 'WEBP', lossless=True, quality=100, method=6)"
```

* Schémas draw.io : script `raw_assets/export_views.ps1` de l'article (exemple : `docs/09_ml_ai/009_how_attention_works/`). Un zoom = recadrage d'une vue exportée, défini dans le script.

**Nommage et intégration**
* `assets/img00.webp` = couverture, aussi utilisée pour Open Graph et la vignette de la page d'accueil (`image:` du front matter). Ne jamais publier avec le placeholder gris.
* Vignette de la page d'accueil : `assets/img00_thumb.webp` (300 px, ~10 Ko), **générée automatiquement** au commit à côté de la couverture. Ne pas la faire à la main. Voir [Vignettes de la page d'accueil](#vignettes-de-la-page-daccueil).
* Série en plusieurs épisodes dans un même dossier : un hero par épisode, `img00_00`, `img00_01`... (badge « Épisode N/M »). Images du corps : numérotation unique pour toute la série, qui continue d'un épisode à l'autre.
* Images du corps : `img01`, `img02`... dans l'ordre de lecture. Une ou deux images ajoutées après coup : suffixe lettre (`img04a`) toléré. Au-delà, renuméroter avant publication (une fois publié, ne plus renommer : liens externes, cache).
* Toujours `<figure>` + `alt` descriptif + `<figcaption>`, `loading="lazy"`. Image haute et étroite : réduire `max-width` (450 px par exemple).
* Sources (PNG, `.drawio`, captures brutes) dans `raw_assets/` (ignoré par git), jamais dans `assets/`.

**Pièges**
* **Flouter** dans les captures d'écran la barre de favoris, les onglets, les comptes, avant l'export WebP.
* Coller une image dans VS Code crée un `image.png` **à côté de l'article** et insère `![alt text](image.png)` là où se trouve le curseur. Le fichier est alors suivi par git et publié par Jekyll. Vérifier `git status` avant de committer.


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
* Le même hook génère la vignette de la page d'accueil des articles modifiés (voir [Vignettes de la page d'accueil](#vignettes-de-la-page-daccueil))
* `tools/generate_last_update.ps1` lit ces dates et génère `tools/last_update_YYYYMMDD.md` (articles triés du plus récent au plus ancien)




### Vignettes de la page d'accueil

La page d'accueil (`index.md`) liste les 10 articles **modifiés** en dernier (pas les plus récents), chacun avec une image de 150 px. Au lieu de la couverture (100 à 180 Ko), elle affiche une vignette WebP de 300 px (~10 Ko) rangée à côté :

```text
docs/<categorie>/<article>/assets/img00.webp          couverture
docs/<categorie>/<article>/assets/img00_thumb.webp    vignette (générée)
```

* La couverture est trouvée comme le fait `index.md` : `image:` du front matter, sinon le premier `<img src="...">` de l'article.
* **Automatique** : au `git commit`, le hook `tools/hooks/pre-commit` lance `tools/thumbnails/make-thumbnails.mjs` sur les fichiers indexés. Article nouveau ou modifié : vignette créée si absente, refaite si la couverture est plus récente. Couverture modifiée seule : vignette refaite. Les vignettes sont ajoutées au même commit (la sortie du hook affiche `THUMB ...`).
* Donc mettre à jour un vieil article (qui remonte en haut de l'accueil) lui crée sa vignette au passage.
* S'il manque Node.js ou les paquets npm, le hook affiche un avertissement et le commit passe quand même. L'accueil utilise alors la couverture : rien de cassé, juste plus lourd.
* Pas de vignette = `index.md` reprend l'image d'origine.

Installation une fois par PC (après un clone) :

```powershell
# From the repository root. Node.js LTS is required (winget install OpenJS.NodeJS.LTS)
npm ci --prefix tools/thumbnails
git config core.hooksPath tools/hooks
```

À la main :

```powershell
# All articles (only missing or outdated thumbnails are written)
node tools/thumbnails/make-thumbnails.mjs --all

# One article
node tools/thumbnails/make-thumbnails.mjs docs/09_ml_ai/009_how_attention_works/how_attention_works_00.md
```

Réglages (300 px, qualité 75) en haut de `make-thumbnails.mjs`. Détails : `tools/thumbnails/README.md`.

### Styles et icônes du site

* **Styles maison** (liens en semi-gras, interligne serré des blocs de code) : `_sass/custom/custom.scss`. Just the Docs l'ajoute à la fin de sa propre feuille de style. Avant le 2026-11-25 c'était `assets/css/custom.css`, chargé à part (une requête bloquante de plus). Ne pas recréer de fichier CSS séparé.
* Du CSS normal est du SCSS valide : on peut coller du CSS tel quel dans `custom.scss`.
* **Icônes du bas de la barre latérale** (GitHub, LinkedIn, YouTube, RSS) : SVG écrits directement dans `_includes/nav_footer_custom.html` (icônes Simple Icons, domaine public). Plus de Font Awesome : il chargeait un CSS et une police depuis cdnjs (deux requêtes bloquantes) pour 4 icônes.
* Ajouter une icône : copier un `<li>` existant, prendre le `path` SVG de l'icône sur <https://simpleicons.org/>, garder `fill="currentColor"` (la couleur suit le texte ou le `style` du lien).
* Le CSS et les icônes des slides (`docs/.../slides/`) sont à part et n'ont pas changé.

### En cas de mise à jour de Just the Docs

Le thème est **figé** dans le `Gemfile` (`gem "just-the-docs", "0.12.0"`) : il ne change jamais tout seul. C'est voulu, car le site remplace deux fichiers du thème par des copies modifiées. Une montée de version automatique mélangerait ces vieilles copies avec un thème plus récent, avec des bugs silencieux (recherche cassée, langue des pages fausse...).

Dependabot (`.github/dependabot.yml`, `bundler`) peut ouvrir une PR de mise à jour du thème : **ne pas la fusionner telle quelle**, suivre la procédure ci-dessous (sur la branche de la PR ou sur une branche à soi).

#### Étape 1 : quelle version utilise le site ?

```powershell
# From the repository root
Select-String -Path Gemfile, Gemfile.lock -Pattern 'just-the-docs'
```

Les deux doivent afficher la même version (aujourd'hui `0.12.0`).

#### Étape 2 : quelle est la dernière version ?

```powershell
# Latest version published on RubyGems
(Invoke-RestMethod https://rubygems.org/api/v1/versions/just-the-docs/latest.json).version
```

Ou à la main : <https://rubygems.org/gems/just-the-docs> et le CHANGELOG <https://github.com/just-the-docs/just-the-docs/blob/main/CHANGELOG.md>.

Même version : rien à faire, s'arrêter là. Sinon, lire le CHANGELOG : chercher « breaking », et ce qui touche la recherche (`search`), `default.html`, les includes `*_custom.html`, `custom.scss`.

#### Étape 3 : monter la version sur une branche

```powershell
git switch master
git pull
git switch -c theme/just-the-docs-0.13.0      # adapt the version number

# Edit the Gemfile: gem "just-the-docs", "0.13.0"
bundle update just-the-docs                   # updates Gemfile.lock
Select-String -Path Gemfile.lock -Pattern 'just-the-docs'   # check
```

#### Étape 4 : reporter les fichiers modifiés du thème

Le site a **deux vraies surcharges** (fichiers du thème copiés puis modifiés). Il faut repartir de la nouvelle version du thème et y remettre nos modifications.

Où trouver les fichiers de la nouvelle version :

```powershell
# Folder of the installed gem (after bundle update)
bundle info just-the-docs --path
```

1. **`assets/js/just-the-docs.js`** (recherche différée, perf point 1)
    * Copier le nouveau `assets/js/just-the-docs.js` de la gem par-dessus le nôtre, puis remettre les blocs marqués `40tude` (chercher `40tude` dans l'ancienne version : `git show master:assets/js/just-the-docs.js | Select-String 40tude`).
    * Il y a 3 endroits : le commentaire d'en-tête, la ligne qui relance la recherche après chargement de l'index (`searchLoaded`), et le bloc « lazy search » qui remplace l'appel direct à `initSearch()`.
    * Si la nouvelle version charge déjà la recherche à la demande (à vérifier dans le CHANGELOG), supprimer notre copie : `git rm assets/js/just-the-docs.js`.
2. **`_layouts/default.html`** (langue de la page : `page.lang` avant `site.lang`)
    * Copier le nouveau `_layouts/default.html` de la gem par-dessus le nôtre, puis remettre la ligne :

```html
<html lang="{{ page.lang | default: site.lang | default: 'en-US' }}">
```

Les autres fichiers ne sont **pas** des copies : ce sont des points d'accroche prévus par le thème, vides à l'origine. Il suffit de vérifier dans le CHANGELOG qu'ils existent toujours :

| Fichier | Rôle |
|---|---|
| `_includes/head_custom.html` | ajouts dans `<head>` (sitemap, Google Analytics...) |
| `_includes/header_custom.html` | haut de page |
| `_includes/footer_custom.html` | bas de page, chargement de `lightbox.js` |
| `_includes/nav_footer_custom.html` | icônes SVG en bas de la barre latérale |
| `_sass/custom/custom.scss` | styles maison, ajoutés à la fin du CSS du thème |

`_includes/mathjax.html`, `_includes/reading-time.html` et `_layouts/mathjax.html` sont à nous, le thème ne les a pas.

Pour lister les surcharges si un doute (un fichier du dépôt qui existe aussi dans la gem) :

```powershell
$gem = bundle info just-the-docs --path
git ls-files _includes _layouts _sass assets | Where-Object { Test-Path (Join-Path $gem $_) }
```

#### Étape 5 : vérifier en local

```powershell
bundle exec jekyll serve      # http://localhost:4000
```

* La recherche : cliquer dans la barre, taper un mot, des résultats apparaissent (après une petite seconde la première fois).
* Une page `lang: fr` : F12 > Elements, `<html lang="fr">`.
* Les icônes en bas de la barre latérale, les liens en semi-gras, une image qui s'ouvre en grand (lightbox), la page d'accueil avec ses vignettes, une page avec des équations.
* Mode sombre si utilisé.

Puis mesurer, avant/après sur le même PC :

```powershell
./tools/perf/Measure-Perf.ps1 -Local -Label "just-the-docs 0.13.0"
```

Comparer avec la dernière mesure `local` du tableau de bord (`-DashboardOnly -Open`). Une note qui chute ou un TBT qui remonte à plusieurs secondes = la recherche différée a sauté (étape 4.1).

#### Étape 6 : publier et mesurer

```powershell
git add Gemfile Gemfile.lock assets/js/just-the-docs.js _layouts/default.html
git commit -m "Update just-the-docs to 0.13.0"
git push -u origin theme/just-the-docs-0.13.0
```

Ouvrir la PR sur GitHub, la fusionner, attendre que « Deploy Jekyll site to Pages » soit vert (onglet Actions), puis :

```powershell
git switch master
git pull
./tools/perf/Measure-Perf.ps1 -Label "après just-the-docs 0.13.0"
git add tools/perf
git commit -m "perf measurements après just-the-docs 0.13.0"
git push
```

Enfin mettre à jour la version citée dans cette section (étape 1).

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
