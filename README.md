C'est le portage du site 40tude.fr de WordPress + OVH vers markdown + Jekyll + Just the Docs + GitHub

## TODO
* Voir dans le repo GitHub si y a pas des articles à rapatrier
* Mettre à jour la page : 022_compile_cpp_code_with_vscode
* Faire un billet sur le portage de WP vers markdown?
* Optimiser le site pour la vitesse ? just-the-docs.js = 1.6s je vois pas quoi faire 
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

## DONE
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
* Revenir sur Codingame
* SOS Chapitres IV et suivants
* IA et ML 
    * Relire les notes OneNote de préparation certification Architect IA et voir ce que je peux rapatrier
* 52 nuances de physique (faut vraiment que je l'écrive un jour)









## À garder sous le coude

* ` NOT YET TRANSFERED `

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
Ancre
[Aller à cette section](#short-id) Dès qu'on met le # code propose une liste d'ancres

[Aller à cette section](#short-id)
## Un titre long pour une section <a id="short-id"></a>
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
   - CTRL + H + Exp Reg
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

### Remplacer les $ seuls par $$ 
```
CTRL + H + Exp Reg
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








