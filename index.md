---
title: 40tude.fr
layout: home
# nav_order: 1
nav_exclude: true
date:               2024-12-25 12:00:00 +0000
last_modified_date: 1964-02-25 11:14:06
---


# 40tude.fr

{% comment %}
  Home page = "latest updates" list.
  Note: site.pages counts every generated page (articles, but also category
  index.md files and this page), so the number below is an upper bound.
  Liquid comments like this one are stripped at build time: nothing reaches the HTML.
{% endcomment %}
Nombre d'articles sur le site : **{{ site.pages | size }}**

{% comment %}
  Table of the N most recently modified pages.
  - last_modified_date is set by the pre-commit hook (tools/hooks/pre-commit).
  - This page has last_modified_date in 1964 so it always sorts last
    and never shows up in its own list.
  Each row: thumbnail (150 px) | title + description.
{% endcomment %}
<table>
  <tbody>
  {% assign nb_articles = 10 %}  {% comment %} number of rows {% endcomment %}
  {% assign nb_words = 20 %}     {% comment %} description length, in words {% endcomment %}
  {% assign articles_sorted = site.pages | sort: 'last_modified_date' | reverse %}
  {% for page in articles_sorted limit: nb_articles %}
    <tr>
      <td>
        {% comment %}
          Thumbnail, first match wins:
            1. front matter `image:` (full path from repo root, same image as Open Graph)
            2. first <img> found in the page content
            3. site default /assets/images/40tude_150.webp
          `image` holds the raw src found in step 2 (usually "./assets/imgNN.webp"),
          `image_full_path` the final URL used in the <img> below.
        {% endcomment %}
        {% assign image = '' %}
        {% assign image_full_path = '' %}
        {% if page.image and page.image != '' %}
          {% comment %} 1. relative_url turns "docs/.../img00.webp" into "/docs/.../img00.webp" {% endcomment %}
          {% assign image_full_path = page.image | relative_url %}
        {% else %}
          {% comment %}
            2. Split on '<img', then look for src=" inside the first tag.
            Works whatever the tag layout: the article template writes
            <img on one line and src="..." on the next one.
          {% endcomment %}
          {% capture page_content %}{{ page.content }}{% endcapture %}
          {% assign parts = page_content | split: '<img' %}
          {% if parts.size > 1 %}
            {% assign src_parts = parts[1] | split: 'src="' %}
            {% if src_parts.size > 1 %}
              {% assign image = src_parts[1] | split: '"' | first %}
            {% endif %}
          {% endif %}
        {% endif %}

        {% if image_full_path != '' %}
          {% comment %} already resolved in step 1, nothing to do {% endcomment %}
        {% elsif image == '' %}
          {% comment %} 3. no image anywhere: site default {% endcomment %}
          {% assign image_full_path = '/assets/images/40tude_150.webp' %}
        {% else %}
          {% comment %}
            Step 2 found a src relative to the article (e.g. "./assets/img00.webp").
            The home page lives at "/", so prefix it with the article's folder:
            page.url "/docs/a/b/article.html" -> page_dir "/docs/a/b/".
            (Splitting on '/' gives a leading empty part, which restores the first '/'.)
          {% endcomment %}
          {% assign page_dir = '' %}
          {% assign parts = page.url | split: '/' %}
          {% for part in parts %}
            {% unless forloop.last %}
              {% assign page_dir = page_dir | append: part | append: '/' %}
            {% endunless %}
          {% endfor %}
          {% assign image_full_path = page_dir | append: image %}
        {% endif %}

        <img src="{{ image_full_path }}" alt="Illustration de {{ page.title }}" width="150" loading="lazy" class="lightbox-image" />
      </td>
      <td>
        {% comment %} Text: front matter description if any, else the first nb_words words of the content {% endcomment %}
        <h2><a href="{{ page.url }}">{{ page.title }} :</a></h2>
        {% if page.description and page.description != '' %}{{ page.description | truncatewords: nb_words }}{% else %}{{ page.content | markdownify | strip_html | truncatewords: nb_words }}{% endif %}
      </td>
    </tr>
  {% endfor %}
  </tbody>
</table>





### À propos
* Il faut lire "**fortitude point fr**". Il n'y a pas de sens caché, j'ai toujours aimé cette façon d'écrire ce mot qui en Anglais désigne la force intérieure ou le courage moral dont une personne fait preuve face à l'adversité, à la douleur ou aux difficultés. Une qualité qui permet de rester ferme, résilient et digne dans les épreuves, sans céder à la peur ou au découragement. Perso, pour moi, la fortitude c'est ça :

<div align="center">
<img src="./assets/images/40tude_tree.webp" alt="" width="253" loading="lazy"/>
</div>

* Typiquement en Corse, en rando à chaque fois que je vois un pin tordu par les vents, enraciné dans une crevasse rocheuse en haut d’une montagne, qui survit malgré le manque de sol, l’aridité, le froid ou les tempêtes, je me dit que c'est une belle incarnation de ce qu'est la fortitude. Il ne s’agit pas d’une force éclatante ou agressive, mais plutôt d’une persévérance silencieuse, d’une capacité à tenir bon dans l’adversité, à résister au temps. La fortitude n’est pas seulement humaine. Elle peut aussi s’observer dans les formes de vie qui refusent de céder, là où tout semble hostile. Ce pin n’a pas choisi la facilité, mais il a choisi de vivre. Pour moi c’est ça, la fortitude. Bon allez, j'arrête là.
* Heu... Pas de référence non plus à [l'opération de désinformation](https://fr.wikipedia.org/wiki/Op%C3%A9ration_Fortitude){:target="_blank"} de la seconde guerre mondiale.
* Les **commentaires** : ils sont les bienvenus. Il suffit d'aller dans cet [espace dédié](https://github.com/40tude/40tude.github.io/discussions). Techniquement le site est sur GitHub. Il est probable qu'avant de pouvoir écrire, on te demande de t'inscrire.
* Les **images** : faut pas hésiter à cliquer dessus. Elles s'affichent alors dans leur dimensions initiales et tu peux zoomer si besoin
* **Historique** : Le site précédent, hébergé chez **OVH** a été **piraté** en avril 2024. De mon point de vue d'utilisateur "lambda" qui souhaite simplement écrire et garder ses notes dans un blog, OVH n'a rien fait pour protéger mon site ni pour m'aider après coup (j'étais client depuis 2011 sans jamais avoir contacté le support...). D'un autre côté, je n'en pouvais plus de **WordPress**. Je parle surtout de l'édition des billets. Au début, en 2010, ça allait à peu près mais sur la fin c'est devenu n'importe quoi...  WP c'est sans doute très bien pour certains mais il ne correspond plus du tout à mes besoins. En plus, j'ai l'impression que ça part en vrille chez WP. [Lire ce billet sur Medium.](https://medium.com/notes-and-theories/this-man-controls-40-of-the-internet-and-its-a-problem-1b37a66e6185){:target="_blank"}
* **Update du 08/02/25** : il reste encore 84 billets à transferer. Essentiellement dans la catégorie Motos (roulage, bricolage ZX6R et RSV4). Pendant le transfert, même si j'apporte quelques modifications aux billets, je ne vais pas modifier les dates de dernière mise à jour. Je ne le ferai que si, après le transfert, je reviens sur une page pour y ajouter des compléments ou la réécrire complètement. Les dates de première et de dernière édition se trouvent **en bas de chaque page**, tandis que **l'estimation du temps de lecture** s'affiche en haut. J'essaie de rester focus sur le transfert mais j'avoue que je commence à tirer la langue.