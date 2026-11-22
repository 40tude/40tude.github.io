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

---

## Outils externes

### words2read

Compte les mots réellement lus d'un article, une fois converti en HTML : front matter, blocs de code et commentaires sont exclus. Au-delà de 3 000 mots, affiche le passage où la limite est franchie, pour savoir où découper l'article en épisodes.

- Source : `C:\Users\phili\OneDrive\Documents\Programmation\rust\00_cli\words2read` (projet Rust séparé, hors de ce dépôt).
- Installation (une seule fois, à refaire après une modification du code) :

```powershell
cd C:\Users\phili\OneDrive\Documents\Programmation\rust\00_cli\words2read
cargo install --path .
```

  L'exécutable est copié dans `~\.cargo\bin`, qui est dans le `PATH` : la commande marche ensuite depuis n'importe quel dossier.

- Utilisation :

```powershell
words2read .\docs\09_ml_ai\008_math_proof_with_claude\math_proof_with_claude.md
```

```text
Words to read : 6087

Excerpt after 3000 words:
offers three options....
```
