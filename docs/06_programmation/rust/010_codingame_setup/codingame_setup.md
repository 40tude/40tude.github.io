---
published: true
layout: default
title: "Rust, mon setup pour CodinGame"
parent: "Rust"
#math: mathjax
date               : 2025-07-03 16:00:00
last_modified_date : 2025-07-03 16:00:00
---

# Rust, mon setup pour CodinGame

<div align="center">
<img src="./assets/cover.webp" alt="" width="900" loading="lazy"/>
</div>


## TL;DR

## Introduction
On est en 2025 et je suis toujours en train d'apprendre Rust. Après avoir codé en Rust les solutions du livre Coding Patterns Interview je suis parti faire un tour sur CodinGame. Personnellement, je ne suis pas fan des tournois et autres mais j'adore y retourner et prendre mon temps pour résoudre un puzzle.

Le truc c'est qu'à l'instar de ce que j'ai déjà fait pour les quiz en C++ et Python, il faut que me remonte un setup. Je suis sous l'excellent Windows 11, j'utilise VSCode et typiquement je veux :
* Un repo sur GitHub
* Des répertoires Easy, Medium, Hard...
* Un sous-répertoire par puzzle
* Être capable de travailler en local en alimentant le programme avec un fichier `input.txt` par exemple
* Être capable de déboguer dans la joie et la bonne humeur  
* Pouvoir sélectionner l'ensemble de la fenêtre de code (CTRL+A), de coller le tout dans CodinGame et 
    * Ca sous-entend que l'utilisation du fichier input.txt doit pas interférer quand on est sur CodinGame
* Alors que je suis au fin fond de mon projet être capable de "commiter"

Je ne vais pas parler du mon Setup VSCode pour coder en Rust. J'ai déjà expliquer tout ça dans ce post. Par contre, je viens de terminer un premier puzzle "Easy" (je débute, poussez pas...), le setup semble fonctionner, je t'explique tout ça.

## Organisation des répertoires

### Phase 1
Là c'est simple. Je veux ça :

```
rust_codingame/
├── .git/
├── .vscode_to_copy/
├── README.md
├── .gitignore
├── easy/
│   ├── projet_01/
│   └── projet_02/
└── medium/
    ├── projet_03/
    └── projet_04/
```
1. Je crée le répertoire `rust_codingame`. Bah, si, ça reste quand même gérable...
1. J'ouvre VSCode depuis ce répertoire (`code .` ou click droit)
1. Je créé un fichier `README.md` avec bout de texte.
1. Je créé un fichier `.gitignore` qui contient une seule ligne `target/`
    * Ca va éviter de pousser sur GitHub les executables, les fichiers pour le debug etc. 
1. Depuis VSCode je crée un repo public et je pousse le répertoire sur GitHub
    * Le répertoire `.git` est créé à ce moment là. 

Allez, c'est bon on a terminé, je quitte VSCode et je vais faire un tour sur GitHub pour voir si tout est OK.


### Phase 2
w
1. Il est minuit, je viens d'allumer mon PC et on imagine que j'ai décidé de m'attaquer à un puzzle (exemple [Crop-Circles](https://www.codingame.com/ide/puzzle/crop-circles)). 
1. Que ce soit avec Win Terminal ou File Explorer, je vais dans le répertoire `rust_codingame` et je créé un répertoire `easy` (y en avait pas encore)
1. Quand c'est fait je vais dans ce répertoire, et depuis un Terminal Windows je tape `cargo new crop_circles`
1. Il est sympa, il crée tout ce dont j'ai besoin et entre autres un sous-répertoire `crop_circles`.
1. Je "descend" dans le répertoire du puzzle (`cd .\crop_circles\`) et je lance VSCode depuis ce terminal (`code .`)
1. Il est possible que VSCode te fasse remarquer qu'il a trouvé un repo Git dans un répertoire parent, blablabla. Tu cliques sur `OK`
1. Bon, du coup, on peut travailler et si tu ouvres dans VSCode un terminal (CTRL+ù) tu peux lancer un `cargo run`. Il va créer les répertoires `target` puis `debug` et tout un ensemble de fichiers dont l'exécutable qu'il va lancer.
1. Avant d'aller plus loin il est temps de faire un commit et de pousser tout ça dans le repo distant.

## Github

1. Là, c'est hyper simple
1. CTRL + SHIFT + G pour aller dans la section Source Code Control
1. Un petit message et commit
1. Et tu synchronises.

## Debug
Pour être tout à fait complet, j'ai essayé de monter un setup à base de Workspace Rust mais bon ça n'a pas marché. Quoi qu'il en soit, en faisant comme on a fait depuis le début, ça va être relativement simple.
1. Depuis VSCode
1. Si ce n'est pas déjà fait, tu installe l'extension LLDB (bien sûr tu ne va le faire qu'un fois)
1. Ensuite, tu créé un sous-répertoire `.vscode` dans le répertoire du projet
1. Tu colles ensuite les 2 fichiers ci-dessous
    * Il n'y a aucune modification à faire

### launch.json
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_BACKTRACE": "1"
            },
            "sourceLanguages": ["rust"],
            "preLaunchTask": "cargo-build-debug",

        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Release",
            "program": "${workspaceFolder}/target/release/${workspaceFolderBasename}.exe",
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_BACKTRACE": "1"
            },
            "sourceLanguages": ["rust"],
            "preLaunchTask": "cargo-build-release"
        }
    ]
}

```

### tasks.json
```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo-build-debug",
            "type": "cargo",
            "command": "build",
            "args": [],
            "problemMatcher": [
                "$rustc"
            ],
            "group": {
                "kind": "build",
                "isDefault": true
            }
        },
        {
            "label": "cargo-build-release",
            "type": "cargo",
            "command": "build",
            "args": [
                "--release"
            ],
            "problemMatcher": [
                "$rustc"
            ]
        }
    ]
}
```

Normalement tout est prêt pour débuguer
1. Poses un point d'arrêt dans le  `main.rs` qu'à généré Cargo
1. Appuis sur F5
1. Ca démarre puis ça s'arrête sur la ligne où il y a le point d'arrêt. Après tu connais je te fais pas un dessin.

Allez, il est temps de faire un petit commit 

### Faire une copie de .vscode
Comme il faudra copier-coller le répertoire `.vscode` dans tous les projets où tu vas faire du debug (on est d'accord, tu vas pas faire des `eprintln!` de petit cochon...). Je te propose de copier-coller le répertoire `.vscode` et ses 2 fichiers deux crans au dessus, dans le répertoire `rust_codingame`. Comme ça tu saura où aller le chercher.

Bon ben voilà, y a plus qu'à coder notre solution...

## Travailler en local
Sur CodinGame notre code lit les entrées depuis l'entrée standard et il affiche sa réponse classiquement. Le truc c'est que moi, je veux pas passer ma vie sur le site. Je veux pouvoir travailler chez moi, dans mon VSCode etc.
Du coup, il faut que je me débrouille pour leurrer mon code de telle sorte qu'en local il aille chercher les paramètres dans un fichier 'input.txt' et qu'une fois sur CodinGame il lise les entrée depuis l'entre standard.

Voilà ce que je te propose

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Custom function to get the input reader (from file or stdin)
fn get_input_reader() -> Box<dyn BufRead> {
    let path = Path::new("input.txt");
    if path.exists() {
        let file = File::open(path).expect("Failed to open input.txt");
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    }
}

fn main() {

    let mut reader = get_input_reader();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let instructions: Vec<&str> = line.split_whitespace().collect();
    ...
}
```

C'est pas hyper compliqué. 
1. Côté programme, je fais un `reader.read_line(&mut line).unwrap();` qui alimente la variable `line`. 
1. Ensuite je manipule cette dernière comme je veux. 
    * Par exemple ici je fais un `.split_whitespace()` blablabla. 
1. La partie intéressante se trouve dans la fonction `get_input_reader()` car c'est elle qui cache la misère
    * On commence par chercher le fichier `input.txt` en local
    * Si on le trouve le programme obtiendra ses entrées depuis le fichier en question
    * Si le fichier n'existe pas le programme obtiendra ses entrées depuis `io::stdin()`, aka l'entrée standard

Voici par exemple le contenu de mon fichier `input.txt`

```
jm31 PLANTMOWjm27 PLANTMOWjm23
oe7 fg9 ls11 




 PLANTMOWjm19 PLANTMOWjm15 PLANTMOWjm11 PLANTMOWjm7 PLANTMOWjm1

oe7 fg9 ls11 

PLANTft9

ft17 PLANTft9 nf17 PLANTMOWnf9 PLANTjm5

fg9 ls11 oe7

```
Pas d'affolement. Dans ce puzzle, seule la première ligne compte. Du coup pour faire des tests je copie colle ce que je trouve sur CodinGame et je met en premier les paramètres à utiliser. Les autres lignes on s'en fiche on ne va pas les lire. Elles peuvent rester là, au cas où.

## Tester sur CodinGame
* Dans VSCode, dans l'éditeur, je fais CTRL+A pour sélectionner tout mon code
* Ensuite je passe sous la page du puzzle, sur le site CodinGame
* Coller (CTRL+V)
* Run

Après tu pleurs ou pas selon que ça passe, ou pas.
