---
title: 40tude.fr
layout: home
nav_order: 1
date:               2024-12-25 12:00:00 +0000
last_modified_date: 2025-01-15 08:00:00 +0000
---


# 40tude.fr 

{: .note }
Je transfère [**40tude.fr**](https://www.40tude.fr/) de (WordPress + OVH) vers (markdown + GitHub).  
Le **16 janvier 2025** il une centaine de billets à transférer (essentiellement dans la catégorie Pilotage). J'espère avoir terminé fin janvier 2025. Je ne suis pas sûr de transférer tous les billets car certains ne sont vraiment plus d'actualité. Cela dit, j'ai souvent été bien content de retrouver une info dans une "vieille" page qui était toujours en ligne... Lire par exemple le premier épisode de la série [**Simple OS**]({%link docs/02_simple_os/000_sos_2_le_retour_20_ans_apres_episode_0/sos_2_le_retour_20_ans_apres_episode_0.md%}).     
Pendant le transfert, même si j'apporte quelques modifications aux billets, je ne vais **pas modifier les dates de dernière mise à jour**. Je ne le ferai que si, après le transfert, je reviens sur une page pour y ajouter des compléments ou la réécrire complètement. Les dates de première et de dernière édition se trouvent **en bas de chaque page**, tandis que **l'estimation du temps de lecture** est affiché en haut.  
Pour l'instant je suis focus sur le transfert.

### À propos du site   
* Il faut lire "**fortitude point fr**". Il n'y a pas de sens caché, j'ai toujours bien aimé cette façon d'écrire ce mot qui en anglais veut dire courage face à l'adversité. Pas de référence non plus à [l'opération de désinformation](https://fr.wikipedia.org/wiki/Op%C3%A9ration_Fortitude){:target="_blank"} de la seconde guerre mondiale.  
* Le site précédent, hébergé chez **OVH** a été **piraté** en Avril 2024. De mon point de vue d'utilisateur "lambda" qui souhaite simplement écrire sur son blog, OVH n'a rien fait pour protéger mon site ni pour m'aider après coup (j'étais client depuis 2011 sans jamais avoir contacté le support...).
* D'un autre côté, je n'en pouvais plus de **WordPress**. Je parle surtout de l'édition des billets. Au début, en 2010, ça allait à peu près mais sur la fin c'est devenu n'importe quoi...  WP c'est sans doute très bien pour certains mais il ne correspond plus du tout à mes besoins. En plus, j'ai l'impression que ça pue chez WP. [Lire ce billet sur Medium.](https://medium.com/notes-and-theories/this-man-controls-40-of-the-internet-and-its-a-problem-1b37a66e6185){:target="_blank"}




### Les articles mis à jour récemment 

Nombre d'articles sur le site : **{{ site.pages | size }}**

<table>
    <thead>
    <tr>
        <th>Image</th>
        <th>Titre</th>
        <th>Extrait</th>
    </tr>
    </thead>
    <tbody>
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
    </tbody>
</table>



