C'est le portage du site 40tude.fr de WordPress + OVH vers markdown + Jekyll + Just the Docs + GitHub.

Publié sur <https://www.40tude.fr>. Un push sur `master` = publication (GitHub Actions, `.github/workflows/pages.yml`).

---

## Notes

Les notes de travail sont dans `_notes/`. Jekyll ignore les dossiers qui commencent par `_`, donc rien n'y est publié.

| Fichier                                        | Contenu                                                                            |
|------------------------------------------------|------------------------------------------------------------------------------------|
| [_notes/edition.md](_notes/edition.md)         | Aide-mémoire d'écriture : new-post, callouts, liens, front matter, équations, Liquid, notebooks Python |
| [_notes/exploitation.md](_notes/exploitation.md) | Faire vivre le site : Dependabot, warnings GitHub Actions, SEO, communication     |
| [_notes/todo.md](_notes/todo.md)               | Idées de billets, TODO, DONE                                                       |

---

## Arborescence

```text
docs/<NN_categorie>/            une catégorie, avec son index.md
    <NNN_slug>/
        <slug>.md               l'article
        assets/                 fichiers publiés (images en .webp)
        raw_assets/             ignoré par Git, OneDrive seulement
assets/_article_template.md     template d'article
tools/                          scripts, non publiés
scrap_book/                     brouillons, non publiés
_notes/                         notes de travail, non publiées
```
