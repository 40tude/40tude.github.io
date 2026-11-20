---
published: true
author: 40tude
lang: fr
layout: default
title: "Mise à jour Linux Mint 21 vers 22 (Partie 2) : la migration"
description: "Passer de Mint 21.3 à Mint 22 avec mintupgrade quand on a des dépôts tiers (Docker, Edge, VS Code, GitHub CLI, PowerShell). Une nuit, quatre murs et toutes les commandes."
image: docs/05_linux/016_migration_mint_22/assets/img00_2.webp
twitter:
  card: summary_large_image
parent: "Linux"
# nav_order: 36
# math: mathjax
date:               2026-11-20 11:15:26
# last_modified_date is updated by tools/hooks/pre-commit
last_modified_date: 2026-11-20 12:48:29
---



# {{ page.title }}
{: .no_toc }

{{ page.description }}
{: .lead }





<h2 align="center"><span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span></h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* `mintupgrade` fait passer de Mint 21.3 à Mint 22, mais il bute sur **tous les dépôts tiers** : clés expirées, dépôts fantômes, nouveau format `.sources`, dépôts liés à Ubuntu 22.04, paquets "étrangers".
* La parade : **mettre de côté** les dépôts qui gênent, désinstaller temporairement `gh`, migrer, puis **tout remettre en place** en 5 blocs de commandes.
* Comptez bien plus qu'une heure. J'ai commencé à 00h21, je me suis couché à 2h et j'ai fini le lendemain matin.
* Une fois en Mint 22, on passe à la **22.3** via le Gestionnaire de mises à jour, menu Édition.
* Bonus : la différence entre **veille** et **hibernation**, et comment faire apparaître un bouton "Hiberner" dans Cinnamon.
* À la fin, une **recette condensée** pour ceux qui veulent refaire la même chose sans les allers-retours.



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00_2.webp"
    alt="Un vieux portable Dell en pleine migration vers Linux Mint 22"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>TODO: Add a legend</figcaption>
</figure>










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Introduction

Ceci est la suite de la [Partie 1 : le grand ménage]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_1.md%}). Pour résumer l'épisode précédent : mon Dell Latitude E7440 tourne sous Linux Mint 21.3, il est propre, à jour, le disque est à 44 % et un instantané Timeshift "clean before Mint 22" vient d'être créé. Il est 00h21.

Comme dans la première partie, j'ai fait tout ça avec Claude à qui j'envoyais mes messages d'erreur et mes captures d'écran. Et comme dans la première partie, je ne cache rien. Surtout pas les moments où ça coince. Et ça coince beaucoup.

Petit détail qui a son importance : en fouillant dans les journaux de la machine, Claude avait repéré deux journaux `mintupgrade` datés du 9 novembre 2025. J'avais donc déjà tenté cette migration un an plus tôt, sans aller au bout. Je ne m'en souvenais même plus. On va vite comprendre pourquoi j'avais abandonné.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Lancement de mintupgrade

La consigne de départ : prévoir une heure, brancher le secteur et, de préférence, le câble réseau (ah, ah, une heure...).

```bash
sudo apt install mintupgrade
sudo mintupgrade
```

`mintupgrade` est l'outil officiel de Mint pour passer d'une version majeure à la suivante. Il s'ouvre dans une fenêtre graphique, vérifie le système, simule la mise à niveau, télécharge les paquets puis les installe, en demandant confirmation à chaque étape.

Et il ne va pas aller bien loin.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mur n°1 : des dépôts non signés

Première erreur, dès la vérification :

```text
W: GPG error: https://cli.github.com/packages stable InRelease: The following signatures couldn't be verified because the public key is not available: NO_PUBKEY 5612B36462313325
E: The repository 'https://cli.github.com/packages stable InRelease' is not signed.
W: GPG error: http://dl.google.com/linux/chrome/deb stable InRelease: The following signatures couldn't be verified because the public key is not available: NO_PUBKEY FD533C07C264648F
E: The repository 'http://dl.google.com/linux/chrome/deb stable InRelease' is not signed.
Error - Return code: 100
```

Dans la fenêtre, `mintupgrade` me demande de lancer `apt update` et de corriger les erreurs. Deux dépôts posent problème, et c'est probablement ce qui m'avait bloqué un an plus tôt.

**Chrome.** Mais... je viens de le désinstaller ? Oui, et dans la [Partie 1]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_1.md%}#chrome-et-les-petites-applis) on avait supprimé `/etc/apt/sources.list.d/google-chrome.list`. Sauf que le dépôt était aussi déclaré dans un fichier qui portait un autre nom. La commande précédente avait raté sa cible. Cette fois on cherche **par contenu**, pas par nom :

```bash
grep -rl dl.google.com /etc/apt/sources.list.d/ | xargs -r sudo rm
```

`grep -rl` liste les fichiers qui contiennent `dl.google.com`, et `xargs -r` les passe à `rm` (le `-r` évite de lancer `rm` sans argument si rien n'est trouvé).

**GitHub CLI.** La clé de signature du dépôt a changé entre-temps. Il faut récupérer la nouvelle :

```bash
sudo wget -qO /etc/apt/keyrings/githubcli-archive-keyring.gpg https://cli.github.com/packages/githubcli-archive-keyring.gpg
sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg
sudo apt update
```

Plus d'erreur. On relance `sudo mintupgrade`.

{: .note }
> Si l'erreur `gh` avait persisté, il aurait fallu regarder où est rangée la clé avec `grep -r cli.github.com /etc/apt/sources.list.d/`. Le fichier `.list` indique le chemin de la clé dans son option `signed-by=`.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mur n°2 : le format .sources

`mintupgrade` repart... et s'arrête de nouveau.

<!-- TODO: screenshot of mintupgrade complaining about .sources files -->

Cette fois, le problème vient des dépôts de Microsoft Edge et de VS Code. Ils sont déclarés dans le **nouveau format** `.sources` (format "deb822", plusieurs lignes `Clé: valeur`), et non dans l'ancien format `.list` (une ligne `deb ...`). Et `mintupgrade` ne sait pas encore les lire.

La solution : les mettre de côté le temps de la mise à niveau. Edge et VS Code restent installés, c'est juste leur dépôt qu'on débranche :

```bash
mkdir -p ~/apt-sources-backup
sudo mv /etc/apt/sources.list.d/microsoft-edge.sources /etc/apt/sources.list.d/vscode.sources ~/apt-sources-backup/
sudo apt update
sudo mintupgrade
```

On les remettra en place après.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mur n°3 : le bouton Réparer

Nouvel arrêt. `mintupgrade` signale un problème et propose un bouton "Réparer". Je clique. Une seconde fenêtre s'ouvre... et je ne sais absolument pas quoi y faire.

<!-- TODO: screenshot of the second window (Software Sources) -->

> *Fermez cette seconde fenêtre, elle ne gère que les dépôts Mint et Ubuntu.*

Le vrai coupable, c'est le dépôt Microsoft pour **Ubuntu 22.04**, installé à l'époque avec PowerShell (voir [Linux Mint 21 - Install PowerShell]({%link docs/05_linux/003_linux_mint_21_install_powershell/linux_mint_21_install_powershell.md%})). Un dépôt qui porte le numéro de version d'Ubuntu dans son adresse, ça ne peut pas survivre à un changement de version d'Ubuntu. On le met de côté lui aussi, en cherchant encore par contenu :

```bash
grep -rl packages.microsoft.com/ubuntu/22.04 /etc/apt/sources.list.d/ | xargs -r sudo mv -t ~/apt-sources-backup/
sudo apt update
```

Le `-t` de `mv` permet de donner le répertoire de destination en premier, ce qui est pratique avec `xargs` qui ajoute les fichiers à la fin.

Ensuite, on clique sur "Revérifier" dans `mintupgrade`. Après la migration, il faudra le remplacer par le dépôt Microsoft pour Ubuntu 24.04, sinon PowerShell ne sera plus mis à jour.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mur n°4 : gh, le paquet "étranger"

Encore un arrêt.

<!-- TODO: screenshot of mintupgrade proposing to downgrade gh -->

Là, c'est plus subtil. Pendant la vérification, `mintupgrade` a désactivé les dépôts tiers, dont celui de GitHub. Du coup, il voit mon `gh` (récent, installé depuis le dépôt de GitHub) comme un paquet "étranger" qui ne vient d'aucun dépôt connu. Et il propose de le remplacer par la version des dépôts Ubuntu... qui date de 2022.

Il ne faut pas accepter cette rétrogradation. Le plus simple est de désinstaller `gh` le temps de la migration :

```bash
sudo apt remove gh
```

Ma connexion à GitHub est stockée dans `~/.config/gh` et ne sera pas touchée. On réinstallera `gh` après. Revérifier.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Une info bleue, pas rouge

Encore une fenêtre. Je commence à craquer : en gros, les **seuls** logiciels que j'utilise sur cette machine posent problème.

<!-- TODO: screenshot of the info about the 4 orphan packages -->

> *Rassurez-vous, ce n'est qu'une information, l'icône est bleue et non rouge. Ces quatre paquets restent installés, mintupgrade signale juste que leurs dépôts sont débranchés pour l'instant. Vous pouvez continuer.*

OK. Bleu = information, rouge = problème. On continue. Cette fois, la mise à niveau démarre vraiment.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 01h06 : l'attente et le coup de gueule

La mise à niveau est en cours, et elle est très lente. Il est 1h du matin, et j'ai le temps de réfléchir.

Franchement, le processus n'est pas terrible. L'expérience utilisateur est bien moins bonne que sous Windows. Et qu'on ne me dise pas que c'est à cause des applications tierces : Windows aussi a des applications externes à supporter, et on n'est pas obligé de faire tout ça pour passer d'une version à l'autre. Ici, il a fallu :
* corriger une clé de dépôt expirée
* traquer un dépôt fantôme
* débrancher trois dépôts
* désinstaller un outil
* et comprendre des fenêtres dont les boutons ouvrent d'autres fenêtres qui ne servent à rien

Sans aide, j'aurais abandonné. D'ailleurs, c'est exactement ce que j'avais fait un an plus tôt.

Claude résume bien le problème :

> *La cohabitation avec les dépôts tiers est le point faible de ces mises à niveau majeures.*

Mint et Ubuntu maîtrisent leurs propres paquets. Mais tout ce qu'on a ajouté à la main (Docker, Edge, VS Code, GitHub CLI, PowerShell...) vient de dépôts externes, que `mintupgrade` ne connaît pas et qu'il préfère débrancher. Sur une machine de dev, ce sont justement les outils qu'on utilise.

En attendant, autant préparer la suite. Je demande à Claude les commandes à lancer après le redémarrage pour tout remettre en place.

> *Collez bloc par bloc. Le terminal exécute bien plusieurs lignes d'un coup, mais certaines commandes demandent une confirmation ou le mot de passe, et en cas d'erreur vous verrez tout de suite quel bloc a coincé.*











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 01h35 : Linux Mint 22 Wilma

Il est 1h35 du matin et une nouvelle fenêtre apparaît. Encore un problème ?

<!-- TODO: screenshot of the final cleanup window showing "Linux Mint 22 Wilma" -->

Non. Le titre de la fenêtre indique "Linux Mint 22 Wilma". La mise à niveau est faite. Ce qu'elle propose, c'est l'étape normale de nettoyage final : la liste des restes de Mint 21. On clique sur "Réparer" sans crainte.

Parmi ces paquets, `eza` (le remplaçant moderne de `ls`) va passer à une version un peu plus ancienne. Sans conséquence.

Je redémarre. Les commandes de remise en place attendront le lendemain. J'éteins finalement la machine à 2h.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Le lendemain : tout remettre en place

Avant tout, on vérifie qu'on est bien en Mint 22 :

```bash
cat /etc/linuxmint/info
```

Ensuite, les 5 blocs préparés la nuit dernière :

```bash
# 1. Edge and VS Code: their sources are release-independent, just put them back
sudo mv ~/apt-sources-backup/microsoft-edge.sources ~/apt-sources-backup/vscode.sources /etc/apt/sources.list.d/
```

Les dépôts d'Edge et de VS Code ne dépendent pas de la version d'Ubuntu. On les remet tels quels.

```bash
# 2. PowerShell: replace the old 22.04 Microsoft repo with the 24.04 one
rm -f ~/apt-sources-backup/*microsoft*
wget -q https://packages.microsoft.com/config/ubuntu/24.04/packages-microsoft-prod.deb -O /tmp/ms.deb
sudo apt install /tmp/ms.deb
```

Le paquet `packages-microsoft-prod.deb` installe le dépôt Microsoft et sa clé pour Ubuntu 24.04 (la base de Mint 22). Remarque : la ligne `rm -f` n'a pas supprimé l'ancien fichier, qui s'appelait `powershell.list` et ne contenait donc pas `microsoft` dans son nom. Sans conséquence, puisqu'il était dans le répertoire de sauvegarde et non dans `/etc/apt/sources.list.d/`. On le verra réapparaître plus bas.

```bash
# 3. Docker: point its repo to the new Ubuntu base (noble)
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu noble stable" | sudo tee /etc/apt/sources.list.d/docker.list
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
```

Le dépôt Docker, lui, contient le nom de code d'Ubuntu (`jammy` pour 22.04, `noble` pour 24.04). On réécrit le fichier pour pointer sur `noble`, et on rafraîchit la clé.

```bash
# 4. GitHub CLI: re-add its repo (key was refreshed earlier) and reinstall gh
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list
sudo apt update
sudo apt install gh
```

La clé avait été récupérée avant la migration (mur n°1). On redéclare le dépôt et on réinstalle `gh`. Ma connexion GitHub, qui était dans `~/.config/gh`, est toujours là.

```bash
# 5. Bring everything up to date and clean up
sudo apt full-upgrade
sudo apt autoremove --purge
```

Tout est passé sans erreur. J'en profite pour supprimer l'avant-dernier instantané Timeshift et en créer un nouveau :

```bash
sudo timeshift --create --comments "clean after Mint 22"
```











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Vérifications

```bash
cat /etc/linuxmint/info | grep RELEASE
sudo apt update          # should finish with no error
gh --version && code --version && pwsh --version && docker --version
ls ~/apt-sources-backup/ # can be deleted if everything works
```

```text
RELEASE=22
RELEASE_NOTES_URL=https://www.linuxmint.com/rel_wilma.php
Réception de :1 https://cli.github.com/packages stable InRelease [3 917 B]
Atteint :2 http://archive.ubuntu.com/ubuntu noble InRelease
Atteint :3 https://downloads.claude.ai/claude-desktop/apt/stable stable InRelease
Atteint :4 https://download.docker.com/linux/ubuntu noble InRelease
Atteint :5 http://archive.ubuntu.com/ubuntu noble-updates InRelease
Atteint :6 http://deb.gierens.de stable InRelease
Atteint :7 http://archive.ubuntu.com/ubuntu noble-backports InRelease
Atteint :8 https://packages.microsoft.com/repos/edge-stable stable InRelease
Atteint :9 https://packages.microsoft.com/ubuntu/24.04/prod noble InRelease
Atteint :10 http://security.ubuntu.com/ubuntu noble-security InRelease
Atteint :11 https://packages.microsoft.com/repos/code stable InRelease
Ign :12 http://packages.linuxmint.com wilma InRelease
Atteint :13 http://packages.linuxmint.com wilma Release
3 917 o réceptionnés en 1s (2 721 o/s)
Lecture des listes de paquets... Fait
Construction de l'arbre des dépendances... Fait
Lecture des informations d'état... Fait
Tous les paquets sont à jour.
gh version 2.105.0 (2026-11-17)
https://github.com/cli/cli/releases/tag/v2.105.0
1.147.0
8a14f0cc1b08d531eb83255ea3764ae22aa932b5
x64
PowerShell 7.7.0
Docker version 29.10.0, build fdd4b8e
.rw-r--r-- 135 root  2 Nov  2023 powershell.list
```

Tout est vert :
* on est bien en Mint 22
* tous les dépôts répondent, en `noble` (Ubuntu 24.04) pour ceux qui dépendent de la version. La ligne `Ign` sur le dépôt Mint est normale.
* `gh`, VS Code, PowerShell et Docker répondent
* il ne reste que le vieux `powershell.list` dans le répertoire de sauvegarde

On fait le ménage :

```bash
sudo rm -rf ~/apt-sources-backup
```











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Passer en 22.3

Mint 22, c'est bien. Mais il y a plus récent. Dans le Gestionnaire de mises à jour, d'abord on s'assure que tout est à jour.

<!-- TODO: screenshot of the Update Manager -->

Ensuite, menu **Édition**. Si une entrée "Mettre à niveau vers Linux Mint 22.x" apparaît, on la lance. Chez moi, c'était **Linux Mint 22.3 Zena**. Bonne nouvelle : cette mise à niveau mineure est bien plus rapide et bien plus simple que le passage de 21 à 22. Pas de dépôt à débrancher, rien. On redémarre à la fin.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Le mystère Timeshift

Après le redémarrage, je veux faire un instantané en 22.3. Cette fois je passe par l'application Timeshift. Je clique sur "Créer", il fait un truc (?), mais j'ai toujours le même nombre d'instantanés. Qu'est-ce qu'il a fait ?

Première explication de Claude :

> *Il a très probablement bien créé l'instantané, puis supprimé le plus ancien de lui-même. Timeshift garde un nombre fixe d'instantanés automatiques, et fait tourner les plus vieux.*

Ça se tient. Pour vérifier :

```bash
sudo timeshift --list
```

```text
Device : /dev/sda3
UUID   : c57aaff4-6edb-485d-ae68-d0a7a2315beb
Path   : /run/timeshift/2904/backup
Mode   : RSYNC
Status : OK
1 snapshots, 160.9 GB free

Num     Name                 Tags  Description
------------------------------------------------------------------------------
0    >  2026-11-20_09-59-09  O     clean after Mint 22
```

Raté. Un seul instantané, celui de 9h59, d'avant le passage en 22.3. Rien n'a été créé, et rien n'a été supprimé.

> *Je me suis trompé : il n'a rien créé.*

Encore une erreur de l'IA, reconnue aussitôt cette fois. On passe par la ligne de commande, qui a le mérite d'afficher ce qu'elle fait :

```bash
sudo timeshift --create --comments "Mint 22.3"
```

Bizarrement, je vois défiler les mêmes messages "Syncing files with rsync..." que dans la fenêtre de l'application tout à l'heure. Et à la fin :

```text
Creating new snapshot...(RSYNC)
Saving to device: /dev/sda3, mounted at path: /run/timeshift/2945/backup
Linking from snapshot: 2026-11-20_09-59-09
Syncing files with rsync...
Created control file: /run/timeshift/2945/backup/timeshift/snapshots/2026-11-20_10-21-27/info.json
RSYNC Snapshot saved successfully (23s)
Tagged snapshot '2026-11-20_10-21-27': ondemand
------------------------------------------------------------------------------
Removing snapshots (incomplete):
------------------------------------------------------------------------------
Removing '2026-11-20_10-15-43'...
Removed '2026-11-20_10-15-43'
```

Voilà le fin mot de l'histoire. L'application avait bien commencé un instantané à 10h15, mais il est resté **incomplet**, donc invisible dans la liste. La commande l'a détecté, l'a supprimé, puis a créé le sien à 10h21. Avec succès.

Moralité : pour Timeshift, je préfère la ligne de commande. Au moins, elle dit ce qu'elle fait.

Remarquez au passage les **23 secondes** : grâce au "Linking from snapshot", Timeshift ne copie que ce qui a changé depuis le précédent instantané. Les fichiers identiques sont des liens physiques.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Veille ou hibernation ?

Dernière vérification conseillée : l'hibernation, parce que c'est souvent ce qui casse après une mise à niveau majeure. Et souvenez-vous, dans la [Partie 1]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_1.md%}#le-swap-ou-lerreur-que-jai-failli-avaler), on avait gardé le swap de 16 Go exprès pour elle.

Je clique sur le bouton du menu, et le voyant du bouton power se met à clignoter très doucement. Pour moi, c'est une hibernation "courte" : pratique pour fermer le PC, changer de bureau et le rouvrir. Mais je ne peux pas le laisser comme ça une nuit ou une semaine.

En fait, je mélangeais deux choses :

> *Le voyant qui clignote doucement signale une mise en veille, pas une hibernation. En veille, la RAM reste alimentée et la batterie se vide en quelques jours. En vraie hibernation, le PC s'éteint complètement, voyant éteint, et peut rester ainsi des semaines.*

| | Veille (*suspend*) | Hibernation (*hibernate*) |
|---|---|---|
| Où est la session ? | En RAM | Écrite dans le swap |
| Alimentation | RAM alimentée, voyant qui clignote | Tout est éteint |
| Durée possible | Quelques jours (batterie) | Illimitée |
| Réveil | Instantané | Quelques secondes |

Pour tester une vraie hibernation :

```bash
sudo systemctl hibernate
```

Le PC s'éteint complètement. Au rallumage, on voit bien qu'il recharge la session depuis le disque, et je retrouve tout tel quel. L'hibernation fonctionne après la migration.

{: .note }
> Si `systemctl hibernate` refuse, c'est souvent le Secure Boot qui bloque l'hibernation.

Reste un problème. Dans l'interface, le seul bouton proposé dit "Mettre en veille". Il faut vraiment taper `sudo systemctl hibernate` à chaque fois ? Non. Mint masque le bouton par défaut, mais on peut l'afficher avec une règle `polkit` qui autorise les membres du groupe `sudo` à hiberner :

```bash
sudo tee /etc/polkit-1/rules.d/50-hibernate.rules <<'EOF'
polkit.addRule(function(action, subject) {
    if ((action.id == "org.freedesktop.login1.hibernate" ||
         action.id == "org.freedesktop.login1.hibernate-multiple-sessions") &&
        subject.isInGroup("sudo")) {
        return polkit.Result.YES;
    }
});
EOF
```

Claude annonçait qu'il faudrait redémarrer. Pas eu besoin : `polkit` a pris la règle en compte tout de suite, et le bouton "Hiberner" est apparu dans la fenêtre d'extinction. On peut aussi le choisir dans Paramètres, Gestion de l'alimentation, pour la fermeture du capot ou l'appui sur le bouton power.

<!-- TODO: screenshot of the shutdown dialog with the Hibernate button -->











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

Il est 10h33 le lendemain matin. Ma machine est allégée, en Mint 22.3, avec tous mes outils à jour, deux instantanés Timeshift propres et un vrai bouton "Hiberner". Est-ce que tout le travail en valait la peine ? Oui. Est-ce que j'ai trouvé ça agréable ? Non.

Ce que je retiens :
* **Les dépôts tiers sont le vrai problème.** Ce n'est pas Mint 22 qui pose souci, c'est tout ce qu'on a ajouté autour. Plus on a de dépôts externes, plus la migration sera pénible.
* **Chercher les dépôts par contenu, pas par nom.** `grep -rl <url> /etc/apt/sources.list.d/` trouve le fichier même s'il ne s'appelle pas comme on l'imagine.
* **Lire les icônes** : bleu = information, rouge = problème.
* **Ne jamais accepter une rétrogradation** d'un outil qu'on utilise (`gh`). Mieux vaut le désinstaller et le réinstaller après.
* **Préparer la remise en place pendant que ça tourne.** Le lendemain, il n'y avait plus qu'à coller 5 blocs.
* **Ne pas croire sur parole**, y compris une IA. Sur l'ensemble des deux parties, Claude s'est trompé au moins trois fois (le swap, le dépôt Chrome, Timeshift). À chaque fois c'est la vérification (une question, un `--list`, un message d'erreur) qui a remis les choses d'aplomb.

Et pour finir : sans aide, j'aurais abandonné, comme un an plus tôt. Avec, ça s'est fait en une soirée (longue) et une matinée. Ce n'est pas encore l'expérience de Windows Update, mais c'est faisable.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Annexe : la recette condensée

Si c'était à refaire, voilà ce que je ferais, dans l'ordre, sans les allers-retours. Adaptez à vos propres dépôts : chez moi c'était Edge, VS Code, Docker, GitHub CLI et PowerShell.

<!-- ---------------------------------------------------------------------- -->
### Avant la migration
{: .no_toc }

```bash
# System up to date and healthy
sudo apt update && sudo apt full-upgrade
sudo apt autoremove --purge && sudo apt clean
sudo dpkg --audit && sudo apt-get check   # no output = healthy

# Inventory of third-party repos: know what you will have to put back
ls /etc/apt/sources.list.d/
sudo apt update 2>&1 | grep -E '^(W|E):'   # any error here will block mintupgrade

# Remove repos of software you no longer use (search by content, not by name)
grep -rl dl.google.com /etc/apt/sources.list.d/ | xargs -r sudo rm

# Refresh keys that changed (GitHub CLI here)
sudo wget -qO /etc/apt/keyrings/githubcli-archive-keyring.gpg https://cli.github.com/packages/githubcli-archive-keyring.gpg
sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg

# Put aside what mintupgrade cannot handle
mkdir -p ~/apt-sources-backup
sudo mv /etc/apt/sources.list.d/*.sources ~/apt-sources-backup/                      # deb822 format
grep -rl packages.microsoft.com/ubuntu/22.04 /etc/apt/sources.list.d/ | xargs -r sudo mv -t ~/apt-sources-backup/
sudo apt remove gh                                                                   # avoids a downgrade
sudo apt update

# Clean snapshot
sudo timeshift --create --comments "clean before Mint 22"
```

{: .warning }
> `sudo mv /etc/apt/sources.list.d/*.sources` déplace **tous** les fichiers `.sources`. Chez moi, il n'y avait que ceux d'Edge et de VS Code. Regardez d'abord ce que vous avez avec `ls`.

<!-- ---------------------------------------------------------------------- -->
### La migration
{: .no_toc }

```bash
sudo apt install mintupgrade
sudo mintupgrade
```

Une info bleue n'est pas une erreur. À la fin, la fenêtre de nettoyage (titre "Linux Mint 22 Wilma") : Réparer, puis redémarrer.

<!-- ---------------------------------------------------------------------- -->
### Après la migration
{: .no_toc }

```bash
cat /etc/linuxmint/info

# Release-independent repos: put them back as is
sudo mv ~/apt-sources-backup/*.sources /etc/apt/sources.list.d/

# Microsoft repo (PowerShell) for Ubuntu 24.04
wget -q https://packages.microsoft.com/config/ubuntu/24.04/packages-microsoft-prod.deb -O /tmp/ms.deb
sudo apt install /tmp/ms.deb

# Docker repo on noble
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu noble stable" | sudo tee /etc/apt/sources.list.d/docker.list
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc

# GitHub CLI
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list
sudo apt update
sudo apt install gh

# Everything up to date, then check
sudo apt full-upgrade
sudo apt autoremove --purge
gh --version && code --version && pwsh --version && docker --version
sudo rm -rf ~/apt-sources-backup
sudo timeshift --create --comments "clean after Mint 22"
```

Puis : Gestionnaire de mises à jour, menu Édition, "Mettre à niveau vers Linux Mint 22.x", redémarrage, et un test d'hibernation avec `sudo systemctl hibernate`.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliographie

* [Partie 1 : le grand ménage]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_1.md%})
* [Linux Mint 22 "Wilma" release notes](https://www.linuxmint.com/rel_wilma.php){:target="_blank"}
* [Linux Mint 21 - Install PowerShell]({%link docs/05_linux/003_linux_mint_21_install_powershell/linux_mint_21_install_powershell.md%})
* [Installing PowerShell on Ubuntu](https://learn.microsoft.com/en-us/powershell/scripting/install/install-ubuntu){:target="_blank"}
* [Install Docker Engine on Ubuntu](https://docs.docker.com/engine/install/ubuntu/){:target="_blank"}
* [Installing gh on Linux](https://github.com/cli/cli/blob/trunk/docs/install_linux.md){:target="_blank"}
* [Timeshift on GitHub](https://github.com/linuxmint/timeshift){:target="_blank"}
