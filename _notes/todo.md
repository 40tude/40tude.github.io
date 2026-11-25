# TODO

Idées de billets, choses à faire, choses faites.

## Table des matières

- [Idées de billets](#idées-de-billets)
- [TODO](#todo-1)
- [DONE](#done)

---

## Idées de billets

* GPG & Pass
* Pareto et 50%-1%
* SOS Chapitres IV et suivants
  * Faire un driver de carte réseau avec Claude?
* IA et ML - Relire les notes OneNote de préparation certification Architect IA
* 52 nuances de physique
* Faire un billet sur le portage de WP vers markdown
* Mettre à jour la page : 022_compile_cpp_code_with_vscode
* Mettre à jour la page : Knee to knee

---

## MAINTENANCE & BUGS
* .

## TODO
* Dans la home page faire apparaître la catégorie (avec un lien) et la date de dernière mise à jour ?
* Voir les histoires de balise `rel="canonical"`
* Voir les histoires de plugins
    * Ça va être chaud
    * Faut installer Ruby même si on a un plugin sous forme de gem
    * Faut l'installer avec Bundle etc.
* SEO
    * Pas sûr de comprendre grand-chose au sujet
    * En plus je suis pas intéressé par le sujet
* Performance : suite possible, voir « Pistes restantes » dans [exploitation.md](exploitation.md#pistes-restantes)
    * Façade YouTube
    * `defer` sur `lunr.min.js` et `just-the-docs.js`
* Réparer la couverture (absente de la page d'accueil) de 3 articles
    * `docs/07_moto/500_stage_de_radigues_rider_school_drrs` : `image:` avec des `\` au lieu de `/`
    * `docs/06_programmation/rust/020_some` : `image:` en `assets/assets/...`
    * `docs/03_maths/010_onde_stationaire_onde_progressive` : première image = URL externe

---

## DONE
* ~~07_moto/500_stage_de_radigues… : le champ image: utilise des \ au lieu de /, donc l'image de l'accueil est cassée.~~
* ~~03_maths/010_onde_stationaire… : la première image est une URL externe, et l'accueil la transforme en chemin local cassé.~~
* ~~Optimiser le site pour la vitesse (just-the-docs.js = 1.6s je vois pas quoi faire)~~ Fait en nov. 2026 : recherche différée, vignettes, icônes SVG, lightbox `defer`, CSS maison dans le thème. Banc de mesure, commandes et acronymes dans [exploitation.md](exploitation.md#performance)
* ~~Voir ces histoire de Warning "Node.js 20 is deprecated" dans GitHub Actions~~ Voir le fix et les explication dans [exploitation.md](exploitation.md)
* ~~Ordonner les articles Rust (pas envie de faire des sous catégories)~~
* ~~Pages en anglais avec lang=en avant le Head~~
    * Voir `_layouts\default.html`. Si just the docs le modifie, récupérer la nouvelle version depuis `https://github.com/just-the-docs/just-the-docs/blob/main/_layouts/default.html`
* ~~Mettre les liens des urls dans les billets en gras~~ (règle dans `_sass/custom/custom.scss`)
* ~~S'assurer que Google indexe bien le site~~
    * Y a toujours des soucis avec les vieilles pages pirates mais les bonnes pages sont bien indexées
* ~~Liens LinkedIn Youtube Github dans le footer de la navbar~~
* ~~Voir dans le repo GitHub si y a pas des articles à rapatrier~~
    * Notes de maths : Loi binomiale, Équation canonique
* ~~Lightbox pour zoomer sur les images~~
    * On peut cliquer sur toutes les images et zoomer sur PC et téléphone
* ~~Ajouter un espace pour les discussions~~
* ~~Rapatrier la page Git Survival de GitHub ici~~
* ~~Rapatrier la page install Linux Mint qui est sur GitHub~~
* ~~Faire une feuille Sessions.pdf pour docs\07_moto\002_feuille_sessions\feuille_sessions.md~~
* ~~Faire une checklist.pdf~~
* ~~implement 404 page~~
* ~~Sur la 1re page, la liste des 10 derniers billets mis à jour~~
* ~~Centrer les vidéos~~
* ~~Remplacer les CTRL + A par **CTRL + A** (vérifier ALT et WIN)~~
* ~~Mettre à jour la façon d'écrire LaTeX partout sur le site~~ : `$$\mathrm{\LaTeX}$$`
* ~~Page Linux Mint à rapatrier~~
* ~~Snowboard, ajouter des photos et des vidéos~~
* ~~Voir cette histoire de pull request de la part de Just The Docs~~
* ~~robots.txt~~ : vérifier dans Google Search Console > Paramètres
* ~~sitemap~~ : fix url in _config.yml + modify _includes\head_custom.html
* ~~Test du site avec outils Google~~ : Google Search Console, PageSpeed Insights
* ~~Ajouter un indicateur de durée de lecture en haut des pages~~
* ~~Installer Google Analytics~~
* ~~Configurer le site pour supporter les simples $ pour les équations~~
    * Impossible : <https://github.com/just-the-docs/just-the-docs/discussions/1593>
* ~~Comment faire des liens (ancres) au sein d'une même page en markdown~~
* ~~Correcteur ortho FR dans VScode~~
* ~~Voir comment permettre le téléchargement de fichiers pdf, zip, xlsx...~~
* ~~Porter les pages de SOS pour tester les outils de récupération du site WP~~
* ~~Voir les histoires de permalinks~~ : <https://jekyllrb.com/docs/permalinks/#front-matter>
* ~~Back to Top~~
* ~~First et last Edit en bas de page~~
