# Exécuter le notebook `interets_composes.ipynb` dans VS Code (Windows)

Ces instructions partent de zéro : aucun outil n'est supposé installé, même pas VSCode. Seul prérequis : un PC sous Windows 11 avec une connexion Internet.

À la fin vous aurez :

- `uv` : un outil qui installe Python et les bibliothèques nécessaires
- VS Code : l'éditeur dans lequel on ouvre le notebook
- un environnement Python propre au dossier, avec `numpy`, `matplotlib`, `scipy` et `ipykernel`

Comptez 10 à 15 minutes la première fois.

Rien de mal ne peut arriver. Dans le pire des cas, supprimez le dossier et recommencez.

---

## 1. Récupérer le notebook

1. Téléchargez le fichier `interets_composes.ipynb` depuis l'article.
2. Créez un dossier dédié, par exemple `Documents\interets_composes`.
3. Déplacez le fichier `interets_composes.ipynb` dans ce dossier.

## 2. Ouvrir un terminal dans ce dossier

1. Ouvrez le dossier dans l'Explorateur de fichiers (`WIN+E`).
2. Faites un clic droit dans une zone vide du dossier, puis **Ouvrir dans le Terminal**.

Alternative : `WIN+X` puis `I` ouvre un terminal. Il faut alors se placer dans le dossier :

```powershell
cd "$HOME\Documents\interets_composes"
```

Vérifiez que vous êtes au bon endroit, le notebook doit apparaître dans la liste :

```powershell
Get-ChildItem # ls
```

Gardez ce terminal ouvert : toutes les commandes suivantes s'y tapent.

## 3. Installer `uv`

```powershell
winget install --id astral-sh.uv -e
```

Si `winget` n'est pas reconnu, utilisez plutôt :

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

## 4. Installer VS Code

```powershell
winget install --id Microsoft.VisualStudioCode -e
```

Si `winget` n'est pas disponible, téléchargez l'installeur sur <https://code.visualstudio.com/> et lancez-le (laissez cochée l'option **Ajouter à PATH**).

## 5. Fermer puis rouvrir le terminal

**Important** : les commandes `uv` et `code` ci-dessous ne sont reconnues que dans un **nouveau** terminal.

1. Fermez le terminal.
2. Rouvrez un terminal dans le dossier du notebook (voir étape 2).
3. Vérifiez que `uv` et VSCode sont bien installés :

```powershell
uv --version
code --version
```

Les deux commandes précédentes doivent afficher un numéro de version.

## 6. Créer l'environnement Python du notebook

Toujours dans le dossier où vous avez déposé le notebook :

```powershell
uv venv
uv pip install ipykernel numpy matplotlib scipy
```

- `uv venv` crée un sous-dossier `.venv` qui contient un Python isolé. Si Python n'est pas installé sur la machine, `uv` le télécharge tout seul.
- `uv pip install ...` installe dans `.venv` les bibliothèques utilisées par le notebook. `ipykernel` est indispensable : c'est lui qui permet à VS Code d'exécuter le code du notebook.

## 7. Installer les extensions VS Code Python et Jupyter

```powershell
code --install-extension ms-python.python
code --install-extension ms-toolsai.jupyter
```

## 8. Ouvrir le notebook dans VSCode

Dans le dossier du notebook :

```powershell
code .
```

Il y a bien un espace puis un point (`.`) après le mot `code`. Le `.` signifie "le dossier courant". VS Code s'ouvre sur le dossier.

1. Si VS Code demande **Faites-vous confiance aux auteurs des fichiers de ce dossier ?**, répondez **Oui**.
2. Dans le panneau de gauche, cliquez sur `interets_composes.ipynb`.

## 9. Choisir le bon kernel

C'est l'étape où l'on se trompe le plus souvent.

1. En haut à droite du notebook, cliquez sur **Select Kernel** (ou **Sélectionner un noyau**).
2. Choisissez **Python Environments...**
3. Choisissez l'entrée qui contient **`.venv`** (par exemple `.venv (Python 3.x.x)`).

Ne choisissez **PAS** un autre Python (par exemple un "Python 3.14" global) : il ne contient pas les bibliothèques installées à l'étape 6, et VS Code affichera un message indiquant qu'il manque des composants (`ipykernel`, `numpy`, `matplotlib`...).

Si `.venv` n'apparaît pas dans la liste : `Ctrl + Shift + P`, tapez `Developer: Reload Window`, validez, puis recommencez.

## 10. Exécuter le notebook

- Cliquez sur **Run All** (ou **Exécuter tout**) en haut du notebook,
- ou exécutez les cellules une par une avec `Shift + Entrée`.

Les graphiques `matplotlib` s'affichent sous les cellules.

---

## Les fois suivantes

Tout est déjà installé. Il suffit de :

1. ouvrir un terminal dans le dossier du notebook,
2. taper `code .`,
3. ouvrir le notebook. VS Code se souvient normalement du kernel `.venv`.

## En cas de problème

| Symptôme | Solution |
|---|---|
| `uv` ou `code` n'est pas reconnu | Fermez et rouvrez le terminal (étape 5). |
| VS Code dit qu'il manque `ipykernel` ou une bibliothèque | Mauvais kernel sélectionné. Reprenez l'étape 9 et choisissez `.venv`. |
| `ModuleNotFoundError: No module named 'numpy'` (ou `matplotlib`, `scipy`) | Idem : vérifiez le kernel. Sinon relancez `uv pip install ipykernel numpy matplotlib scipy` dans le dossier du notebook. |
| `.venv` absent de la liste des kernels | Vérifiez que VS Code a été ouvert avec `code .` **dans le dossier du notebook**, puis `Developer: Reload Window`. |
| Les extensions ne s'installent pas en ligne de commande | Dans VS Code, ouvrez l'onglet Extensions (`Ctrl + Shift + X`), cherchez **Python** puis **Jupyter** (éditeur : Microsoft) et installez-les. |

## Désinstaller

Pour tout effacer, supprimez simplement le dossier du notebook (le sous-dossier `.venv` part avec). VS Code et `uv` se désinstallent depuis **Paramètres > Applications** ou avec :

```powershell
winget uninstall Microsoft.VisualStudioCode
winget uninstall astral-sh.uv
```
