---
published: true
author: 40tude
lang: fr
layout: default
title: "Mise à jour Linux Mint 21 vers 22 (Partie 1) : le grand ménage"
description: "Une soirée pour faire le ménage sur un vieux portable sous Mint 21.3 avant la migration. Avec toutes les commandes, les erreurs et les allers-retours."
image: docs/05_linux/016_migration_mint_22/assets/img00.webp
twitter:
  card: summary_large_image
parent: "Linux"
# nav_order: 36
# math: mathjax
date:               2026-11-20 11:15:26
# last_modified_date is updated by tools/hooks/pre-commit
last_modified_date: 2026-11-20 12:52:26
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

* Mon Dell Latitude E7440 (2013) sous Linux Mint 21.3 avait le disque plein à **84 %**. Avant de migrer vers Mint 22, il fallait faire le ménage.
* Les coupables : une **corbeille de 35 Go**, une vieille VM VMware, une copie de mon site web, **45 Go d'instantanés Timeshift** et des dizaines de paquets inutiles.
* Toujours **simuler** une purge `apt` avec `-s` et **lire** la sortie avant de lancer pour de vrai.
* Le swap de 16 Go n'est pas surdimensionné quand on **hiberne**. Le réduire aurait cassé l'hibernation.
* Mettre à jour Docker Desktop sous Linux, c'est télécharger un `.deb`... puis se battre avec `gpg` et `pass` pour pouvoir se connecter.
* Résultat : le disque passe de 84 % à 44 % et la machine est prête pour la [Partie 2]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_2.md%}), la migration.



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00.webp"
    alt="Mon vieux portable Dell sous Linux Mint en plein ménage"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Mon vieux portable Dell sous Linux Mint en plein ménage</figcaption>
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

J'ai un Dell Latitude E7440. Il date de 2013, il tourne sous Linux Mint depuis des années (voir [Linux Mint Installation and Configuration]({%link docs/05_linux/012_install_config_mint/install_config.md%})) et je ne m'en sers quasiment que pour du développement et des tests. Le hic, c'est que je n'y avais pas touché depuis plus de 6 mois. Il était temps de :

1. faire le ménage, parce qu'il y a plein de trucs que je n'utilise pas (LibreOffice et compagnie)
2. passer de Mint 21 à Mint 22

Avant de commencer, il faut que je sois honnête : je n'aurais **pas** été capable de faire tout ça seul. J'ai passé la soirée (de 22h à 2h du matin, plus une bonne partie de la matinée du lendemain) avec Claude à qui je posais mes questions et envoyais mes sorties de terminal. Je vais citer ses réponses de temps en temps.

Je ne cache rien dans ce qui suit : les erreurs (les miennes comme les siennes), les fausses pistes, les moments où j'ai eu envie de tout jeter par la fenêtre. Je pense que c'est plus utile qu'un tuto lisse où tout marche du premier coup. Parce que dans la vraie vie, rien ne marche du premier coup.

L'histoire est en deux épisodes :
* **Partie 1** (ici) : le grand ménage
* **Partie 2** : [la migration vers Mint 22]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_2.md%})

Une dernière chose. Sur cette machine il n'y a rien de critique. Si vous faites pareil sur une machine importante, **faites une sauvegarde** avant. Vraiment.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Changer de distribution ?

La toute première question que j'ai posée, c'était : compte tenu de mon cas d'usage (des tests, 3 lignes de code...) est-ce que je ne devrais pas carrément changer de distribution ? La réponse a été directe :

> *Pour un poste quasi 100 % dev, je garderais Mint : changer de distribution coûte une réinstallation pour un gain faible. Fedora ou Debian se justifient seulement si vous voulez des paquets plus récents ou un système plus minimal.*

OK, on garde Mint. Et puis, tant qu'à faire, je n'avais pas envie de tout réinstaller.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Un premier jet un peu trop rapide

Dans la foulée, Claude m'a proposé ça :

```bash
sudo apt purge 'libreoffice*' thunderbird hexchat rhythmbox celluloid hypnotix pix transmission-gtk
sudo apt autoremove --purge && sudo apt clean
flatpak uninstall --unused
sudo apt update && sudo apt full-upgrade
```

Avec du recul, c'est une liste **générique**. Elle a été écrite sans savoir ce qui était réellement installé sur ma machine. Ça peut marcher, mais ce n'est pas comme ça qu'on devrait procéder. On verra plus bas une version bien plus propre, avec une simulation d'abord.

Au passage, je ne savais pas ce que faisait la ligne `flatpak`. Réponse :

> *`flatpak uninstall --unused` supprime les runtimes Flatpak (GNOME, KDE, pilotes GL...) dont plus aucune application installée n'a besoin. Elle ne touche à aucune de vos applications, et libère souvent plusieurs Go.*










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Où j'en suis ?

Je ne me souviens jamais des commandes pour savoir quelle version tourne. Les voilà :

```bash
cat /etc/linuxmint/info      # Mint version and edition
lsb_release -a               # distribution and Ubuntu base
uname -r                     # running kernel
inxi -S                      # summary: kernel, desktop, distro
inxi -Fxz                    # full hardware and system report
```

Chez moi, ça donnait (sortie raccourcie) :

```text
RELEASE=21.3
CODENAME=virginia
EDITION="Cinnamon"
DESCRIPTION="Linux Mint 21.3 Virginia"
...
5.15.0-203-generic
...
System:
  Kernel: 5.15.0-203-generic x86_64 bits: 64 compiler: gcc v: 11.4.0
    Desktop: Cinnamon 6.0.5 Distro: Linux Mint 21.3 Virginia
    base: Ubuntu 22.04 jammy
Machine:
  Type: Laptop System: Dell product: Latitude E7440 v: 00
CPU:
  Info: dual core model: Intel Core i7-4600U bits: 64 type: MT MCP
...
Drives:
  Local Storage: total: 238.47 GiB used: 195 GiB (81.8%)
  ID-1: /dev/sda vendor: Samsung model: SSD PM851 mSATA 256GB
Partition:
  ID-1: / size: 233.18 GiB used: 194.99 GiB (83.6%) fs: ext4 dev: /dev/sda3
Swap:
  ID-1: swap-1 type: file size: 16 GiB used: 0 KiB (0.0%) file: /swapfile
Info:
  Processes: 254 Uptime: 49m Memory: 15.52 GiB used: 2.46 GiB (15.9%)
  Init: systemd runlevel: 5 Compilers: gcc: 11.4.0 Packages: 2289 Shell: Bash
```

En résumé :
* **Mint 21.3 Cinnamon**, basé sur Ubuntu 22.04, noyau 5.15. C'est la dernière version de la branche 21, supportée jusqu'en avril 2027. Donc pas d'urgence, mais autant s'y prendre maintenant.
* Le disque est plein à **84 %**. Ça, c'est pas normal. J'ai dû oublier des trucs. Et une mise à niveau majeure demande de la place. Donc le ménage passe en premier.

Gardez en tête les **16 Go de swap** avec 0 Ko utilisé (voir la ligne `Swap: ID-1: swap-1 type: file size: 16 GiB used: 0 KiB (0.0%) file: /swapfile`). On va en reparler, et ce sera l'occasion d'une belle erreur.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Adieu Anaconda

Premier candidat évident : Anaconda. Je ne l'utilise plus du tout, je suis passé à `uv` (j'en parle dans [UV Python Package Manager: The Essentials]({%link docs/06_programmation/python/005_uv/uv.md%})). Pour le retirer proprement :

```bash
conda init --reverse --all        # remove conda hooks from .bashrc etc.
rm -rf ~/anaconda3 ~/.conda ~/.condarc ~/.continuum
```

La première ligne est importante. Elle retire le bloc que `conda` avait ajouté dans `~/.bashrc` (et compagnie) pour s'activer à chaque ouverture de terminal. Si on supprime le répertoire sans faire ça, chaque nouveau terminal se plaint de ne plus trouver `conda`.

{: .note }
> Si `conda` ne répond déjà plus, il faut retirer à la main, dans `~/.bashrc`, tout le bloc compris entre `>>> conda initialize >>>` et `<<< conda initialize <<<`. Ensuite, ouvrir un nouveau terminal.

On verra un peu plus loin qu'il restait encore `~/.anaconda` et `~/.ipython` qui traînaient. Ils sont partis eux aussi.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Qui prend toute la place ?

Pour trouver les gros consommateurs, trois commandes :

```bash
sudo apt install ncdu && ncdu ~      # interactive, browse with arrows
docker system df                     # space used by Docker
du -sh ~/.cache ~/.local/share/Trash
```

`ncdu` est un petit outil en mode texte qui affiche la taille des répertoires, triés du plus gros au plus petit. On navigue avec les flèches. Indispensable.

<!-- TODO: screenshot of ncdu ~ -->

Voilà ce que donnaient les deux dernières commandes :

```text
Cannot connect to the Docker daemon at unix:///home/philippe/.docker/desktop/docker.sock. Is the docker daemon running?
2,4G	/home/philippe/.cache
35G	/home/philippe/.local/share/Trash
```

Oui, oui **35 Go dans la corbeille.** Pour la vider, soit on passe par Nemo, soit :

```bash
rm -rf ~/.local/share/Trash/*
```

`ncdu` remontait aussi d'autres pistes :
* `~/vmware` : 11 Go
* `~/.docker` : 5 Go (Docker Desktop, qui était arrêté, d'où l'erreur ci-dessus)
* `~/.anaconda` et `~/.ipython` : des restes d'Anaconda
* `~/.cache` : 2.4 Go, qu'on peut vider sans risque











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## VMware Player et un site piraté

J'avais VMware Player installé, avec des VM dont je ne me servais plus. Après avoir vérifié qu'aucune ne contenait quelque chose à garder :

```bash
sudo vmware-installer -l                      # shows the exact product name
sudo vmware-installer -u vmware-player
rm -rf ~/vmware ~/.vmware
```

Si un jour j'ai de nouveau besoin de VM, VirtualBox ou virt-manager s'installent directement avec `apt`.

En fouillant, je suis aussi tombé sur une énième copie de mon site web. Il avait été piraté en avril 2024, puis récupéré dans les larmes et le sang. Environ 29 Go. Poubelle (et cette fois, je vide la poubelle). La récupération "à la main" du site web, les migrations de WordPress vers Jekyll et d'OVH vers GitHub méritent une paire d'articles mais j'ai pas trop le temps.

Après ce premier passage, mon dossier personnel est passé à **12 Go**. Pas mal pour un début.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Docker : Desktop ou dockerd ?

Pour voir la place occupée par Docker, il fallait le démarrer. Et là, ça faisait tellement longtemps que je ne savais plus comment. C'est `dockerd`, non ? Pas tout à fait :

> *Le socket `~/.docker/desktop` montre que vous utilisez Docker Desktop.*

```bash
systemctl --user start docker-desktop       # Docker Desktop (runs in a VM)
sudo systemctl start docker                 # classic Docker engine (dockerd)
docker context ls                           # which one is active
```

Il y a donc deux "Docker" possibles sous Linux :
* **Docker Desktop** : une application graphique qui fait tourner le moteur Docker dans une machine virtuelle. Elle se lance au niveau de l'utilisateur (d'où le `--user`).
* **Docker Engine** (`dockerd`) : le moteur classique, directement sur la machine, géré par `systemd` en mode système.

Au démarrage, Docker Desktop m'annonce que ma version n'est plus supportée et qu'il faut la mettre à jour. Évidemment.

Claude m'a alors suggéré de supprimer purement et simplement Docker Desktop, qui fait tourner une VM gourmande sur un portable modeste, et de ne garder que le moteur Docker (mis à jour par `apt`, lui). C'est probablement un bon conseil. Mais j'ai décidé de garder Docker Desktop. On verra plus bas que je l'ai un peu regretté.

Par contre, avant de le mettre à jour, je voulais faire le ménage dans les applications.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Les paquets qui ne servent à rien

Cette fois, on part de ce qui est **vraiment** installé :

```bash
apt-mark showmanual | sort > ~/apt-manual.txt   # packages installed explicitly (by me or by Mint)
flatpak list --app
snap list 2>/dev/null
```

Les deux dernières commandes ne renvoient rien. Pas de Flatpak, pas de Snap. C'est déjà ça.

J'ai envoyé `apt-manual.txt` à Claude, qui m'a proposé une purge. Mais attention, **en simulation d'abord**, grâce à l'option `-s` :

```bash
sudo apt purge -s \
  'libreoffice*' 'libuno*' ure ure-java uno-libs-private liblibreoffice-java fonts-opensymbol \
  hexchat-common rhythmbox-data librhythmbox-core10 gir1.2-rb-3.0 transmission-common pix-data brasero-common \
  '*-dbg' \
  docker docker-compose docker-doc docker-registry \
  zfsutils-linux zfs-initramfs zfs-zed \
  qemu-efi-aarch64 qemu-efi-arm \
  orca brltty xbrlapi onboard gamemode \
  linux-image-5.15.0-76-generic linux-modules-5.15.0-76-generic linux-modules-extra-5.15.0-76-generic \
  linux-headers-5.15.0-76 linux-headers-5.15.0-76-generic linux-headers-5.15.0-88-generic
```

Ligne par ligne, ce qu'on retire :
* les restes de **LibreOffice** et de ses bibliothèques (`uno`, `ure`...)
* les restes d'applications que je n'utilise pas (HexChat, Rhythmbox, Transmission, Pix, Brasero)
* tous les paquets de **débogage** (`*-dbg`). Ils ne servent qu'à déboguer Cinnamon, Nemo, etc.
* le **vieux Docker** de Ubuntu et `docker-compose` v1, en doublon avec Docker Desktop et `docker-ce` qui, eux, restent
* **ZFS**, inutile puisque mon disque est en `ext4`
* des firmwares **QEMU pour ARM**
* les outils d'**accessibilité** (lecteur d'écran, braille, clavier visuel) et `gamemode`
* les **anciens noyaux** 76 et 88. Le noyau courant (203) n'est évidemment pas dans la liste.

La consigne de sécurité était claire :

> ***Vérifiez la simulation*** *: si elle annonce le retrait de `cinnamon`, `mint-meta-*` ou `docker-desktop`, ne lancez pas.*

Voilà ce que j'ai vu (sortie raccourcie) :

```text
Note : sélection de libreoffice-l10n-en-gb pour l'expression rationnelle « libreoffice* »
Note : sélection de libreoffice-spellcheck-zh-tw pour l'expression rationnelle « libreoffice* »
...
Les paquets suivants ont été installés automatiquement et ne sont plus nécessaires :
  linux-headers-5.15.0-174 linux-headers-5.15.0-174-generic linux-headers-5.15.0-88 linux-image-5.15.0-174-generic
  linux-modules-5.15.0-174-generic linux-modules-extra-5.15.0-174-generic python3-docker python3-dockerpty python3-docopt python3-dotenv
  python3-jsonschema python3-pyrsistent python3-texttable python3-websocket wmdocker
Veuillez utiliser « sudo apt autoremove » pour les supprimer.
Les paquets suivants seront ENLEVÉS :
  brasero-common* brltty* cinnamon-control-center-dbg* cinnamon-dbg* docker* docker-compose* docker-doc* docker-registry* fonts-opensymbol*
  gamemode* gir1.2-rb-3.0* hexchat-common* libbrasero-media3-1* libc6-dbg* libcinnamon-desktop-dbg* libcinnamon-menu-3-0-dbg* libcjs-dbg*
  libcvc-dbg* liblibreoffice-java* librhythmbox-core10* libuno-cppu3* libuno-cppuhelpergcc3-3* libuno-purpenvhelpergcc3-3* libuno-sal3*
  libuno-salhelpergcc3-3* libunoloader-java* linux-headers-5.15.0-76* linux-headers-5.15.0-76-generic* linux-headers-5.15.0-88-generic*
  linux-image-5.15.0-76-generic* linux-modules-5.15.0-76-generic* linux-modules-extra-5.15.0-76-generic* muffin-dbg* nemo-dbg* onboard* orca*
  pix-data* qemu-efi-aarch64* qemu-efi-arm* rhythmbox-data* transmission-common* uno-libs-private* ure* ure-java* xbrlapi* xed-dbg*
  xreader-dbg* xviewer-dbg* zfs-initramfs* zfs-zed* zfsutils-linux*
0 mis à jour, 0 nouvellement installés, 51 à enlever et 0 non mis à jour.
```

J'ai lu la liste des paquets ENLEVÉS. Pas de `cinnamon` tout court (seulement `cinnamon-dbg`), pas de `mint-meta-*`, pas de `docker-desktop`. À mon avis on est bon. Claude confirme. On lance la même commande **sans** le `-s`, puis on fait le ménage des dépendances orphelines :

```bash
sudo apt purge \
  'libreoffice*' 'libuno*' ure ure-java uno-libs-private liblibreoffice-java fonts-opensymbol \
  hexchat-common rhythmbox-data librhythmbox-core10 gir1.2-rb-3.0 transmission-common pix-data brasero-common \
  '*-dbg' \
  docker docker-compose docker-doc docker-registry \
  zfsutils-linux zfs-initramfs zfs-zed \
  qemu-efi-aarch64 qemu-efi-arm \
  orca brltty xbrlapi onboard gamemode \
  linux-image-5.15.0-76-generic linux-modules-5.15.0-76-generic linux-modules-extra-5.15.0-76-generic \
  linux-headers-5.15.0-76 linux-headers-5.15.0-76-generic linux-headers-5.15.0-88-generic
sudo apt autoremove --purge
```

Le `autoremove` emporte au passage le noyau 174 et les dépendances Python de `docker-compose` v1 (celles listées comme "plus nécessaires" ci-dessus).

Petite question pendant que ça tourne : faut-il arrêter Docker Desktop ? Non. Cette purge ne touche à aucun paquet qu'il utilise.

Et maintenant, le moment de vérité :

```bash
df -h /
```

```text
Sys. de fichiers Taille Utilisé Dispo Uti% Monté sur
/dev/sda3          234G     97G  126G  44% /
```

De **84 % à 44 %**. Environ 98 Go libérés. Je suis content.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Où sont passés les 85 Go ?

Sauf que... Claude fait le calcul : mon dossier perso ne fait plus que 12 Go, et il y a 97 Go d'utilisés. Il reste donc 85 Go *ailleurs*. Pour chercher sur tout le disque (le `-x` évite de sortir de la partition) :

```bash
sudo ncdu -x /
```

Les suspects habituels selon Claude : `/var/lib/docker`, `/var/log`, `/opt` et le `/swapfile` de 16 Go.

Pendant que `ncdu` mouline, moi je mets une pièce sur Timeshift. Les instantanés Timeshift sont stockés dans `/timeshift`, sur le **même disque**. Et ça faisait des mois (des années ?) qu'il en accumulait.

<!-- TODO: screenshot of sudo ncdu -x / showing /timeshift -->

Pièce gagnée : **45 Go** pour Timeshift. Et le pire, c'est que ces vieux instantanés contiennent encore tout ce qu'on vient de purger. Le plus simple est de tout supprimer, puis d'en refaire un propre une fois le ménage terminé :

```bash
sudo timeshift --list                                      # list snapshots
sudo timeshift --delete --snapshot '<name>'                # delete one snapshot
sudo timeshift --delete-all                                # delete them all
sudo timeshift --create --comments "clean before Mint 22"  # create a new one
```

{: .note }
> Timeshift n'est **pas** une sauvegarde. C'est un filet de sécurité pour le système : si une mise à jour casse tout, on revient en arrière. Mais il est sur le même disque. Si le disque meurt, les instantanés meurent avec lui. Et par défaut, Timeshift ignore `/home`.

Pour le reste, `ncdu` ne montrait rien d'anormal. Sauf le swap.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Le swap, ou l'erreur que j'ai failli avaler

Claude m'explique que le `/swapfile` de 16 Go est surdimensionné, "*vu qu'il n'est jamais utilisé*", et qu'on peut le ramener à 4 Go.

Jamais utilisé ? Même par l'hibernation ? Parce que moi, j'hiberne.

> *Bonne remarque, j'ai été trop affirmatif. `inxi` montrait 0 Ko utilisé, mais seulement après 49 minutes de fonctionnement.*

Voilà. `inxi` avait été lancé 49 minutes après le démarrage. Forcément, le swap était vide. Ça ne veut pas dire qu'il ne sert jamais. Et pour hiberner, le contenu de la RAM est écrit dans le swap, donc il doit être au moins aussi gros que la RAM. J'ai 16 Go de RAM.

Pour vérifier si l'hibernation est configurée :

```bash
grep -rs resume /etc/default/grub /etc/initramfs-tools/conf.d/
```

```text
/etc/default/grub:GRUB_CMDLINE_LINUX_DEFAULT="resume=UUID=c57aaff4-6edb-485d-ae68-d0a7a2315beb resume_offset=5410816 quiet splash"
```

Elle l'est (je l'avais configurée lors de l'[installation de Mint 21]({%link docs/05_linux/012_install_config_mint/install_config.md%}#enable-hibernation)). Et ce qui est intéressant ici, c'est le `resume_offset`. Il indique l'emplacement **exact**, sur le disque, du fichier `/swapfile`. Pour info, voilà la recette que Claude avait proposée pour réduire le swap :

```bash
# DO NOT RUN if you hibernate (see below)
sudo swapoff /swapfile
sudo rm /swapfile
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

Supprimer puis recréer le fichier, c'est le mettre ailleurs sur le disque. Le `resume_offset` aurait alors pointé dans le vide, et l'hibernation aurait été cassée. Sans compter que 4 Go, c'est trop petit pour y ranger 16 Go de RAM.

Donc :
* si on hiberne : on garde les 16 Go et on ne touche à rien
* si on n'hiberne jamais : on peut réduire, mais il faut aussi retirer `resume=...` et `resume_offset=...` de `/etc/default/grub`, puis lancer `sudo update-grub`

J'hiberne. Je garde.

{: .important }
> C'est la première leçon de la soirée. Une IA peut être très affirmative et se tromper. Ici, c'est parce que je savais que j'utilisais l'hibernation que j'ai posé la question. Bref, quand une réponse paraît bizarre, faut pas hésiter à demander pourquoi.

Dans cette façon de travailler à quatre mains il y a, je trouve, un petit côté binôme de "L'arme fatale". Bien sûr c'est moi Mel Gibson, le beau gosse de servie.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Mettre à jour Docker Desktop

Avant de faire l'instantané Timeshift, je me dis qu'il est temps de mettre Docker à jour. Comme ça, l'instantané capturera un système à jour. Bonne logique, et comme Timeshift ignore `/home`, les images de Docker Desktop (qui sont dans `~/.docker`) ne vont pas gonfler l'instantané.

Sous Linux, Docker Desktop ne se met **pas** à jour tout seul. Il faut télécharger le nouveau `.deb` et l'installer. Cette fois, il vaut mieux arrêter Docker Desktop avant :

```bash
systemctl --user stop docker-desktop
cd ~/Téléchargements
wget https://desktop.docker.com/linux/main/amd64/docker-desktop-amd64.deb
sudo apt install ./docker-desktop-amd64.deb
```

À noter, `docker-ce`, le moteur qui vient du dépôt Docker, avait déjà été mis à jour par le `full-upgrade` du début.

À la fin de l'installation, je vois ça et je me dis "Oups" :

```text
Note : sélection de « docker-desktop » au lieu de « ./docker-desktop-amd64.deb »
Les paquets suivants seront mis à jour :
  docker-desktop
...
Dépaquetage de docker-desktop (4.99.0-245822) sur (4.67.0-222858) ...
Paramétrage de docker-desktop (4.99.0-245822) ...
Enabling use of privileged ports by Docker Desktop
kubernetes.docker.internal already in /etc/hosts
...
Skipping the installation of the AppArmor profile as this version of AppArmor does not seem to support the profile bundled with Docker Desktop.
...
N: Le téléchargement est effectué en dehors du bac à sable en tant que « root » car le fichier « /home/philippe/Téléchargements/docker-desktop-amd64.deb » n'est pas accessible par l'utilisateur « _apt ». - pkgAcquire::Run (13: Permission non accordée)
```

En fait, pas d'oups. Tout s'est bien passé, on est passé de la 4.67 à la 4.99 :
* la ligne `N:` finale est une simple note qui apparaît quand le `.deb` est dans notre dossier perso (que l'utilisateur système `_apt` n'a pas le droit de lire)
* le message AppArmor est bénin sous Ubuntu 22.04

On peut supprimer le `.deb` et relancer Docker Desktop :

```bash
rm ~/Téléchargements/docker-desktop-amd64.deb
systemctl --user start docker-desktop
```

Dans la foulée, je suis même passé en 4.100, de la même façon.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## La galère du login Docker

Docker Desktop est à jour, mais impossible de me connecter à mon compte. La [page d'explication de Docker](https://docs.docker.com/desktop/setup/install/linux/){:target="_blank"} est, à mon goût, tout sauf claire.

Ce qu'il faut savoir : sous Linux, Docker Desktop range nos identifiants dans `pass`. C'est un gestionnaire de mots de passe en ligne de commande, qui chiffre tout avec une clé GPG. Il faut donc créer une clé GPG, puis initialiser `pass` avec cette clé. Une seule fois. Première tentative :

```bash
gpg --generate-key            # enter your name and email, choose a passphrase
gpg --list-secret-keys        # copy the long ID shown under "sec"
pass init <ID>
```

Je relance Docker Desktop, il me demande un mot de passe. Mais lequel ? L'ID de la clé ? Un mot de passe de `pass` ?

> *C'est la phrase de passe que vous avez choisie pendant `gpg --generate-key`. L'ID n'est qu'un identifiant, pas un secret, et `pass` n'a pas de mot de passe propre : il utilise celui de la clé GPG.*

Sauf que je n'ai jamais choisi de phrase de passe. J'ai juste donné mon nom et mon adresse mail. En fait je commence à fatiguer. `gpg` a ouvert une petite fenêtre pour la phrase de passe pendant la création. J'ai saisi un truc mais au moment du login dans Docker ça passait pas (fatigue à mon avis). Du coup "panique" et on recommence tout gentiment :

```bash
gpg --delete-secret-and-public-key <ID>
gpg --full-generate-key       # defaults are fine, set a passphrase you will remember
gpg --list-secret-keys
pass init <NEW_ID>
```

À ce stade, je commence à trouver ça carrément lourd. Moi, je veux juste lancer Docker. Et évidemment :

```text
gpg --list-secret-keys
...
sec   rsa3072 2026-11-19 [SC]
      <NEW_ID>
uid          [  ultime ] Philippe BAUCOUR <xxx@xxx.com>
ssb   rsa3072 2026-11-19 [E]

pass init <NEW_ID>
Password store initialized for <NEW_ID>
docker-credential-helpers/aHR0cHM6Ly9pbmRleC5kb2NrZXIuaW8vdjEvcmVmcmVzaC10b2tlbg==/40tude: reencrypting to <SUBKEY_ID>
gpg: échec du déchiffrement : Pas de clef secrète
docker-credential-helpers/aHR0cHM6Ly9pbmRleC5kb2NrZXIuaW8vdjEv/40tude: reencrypting to <SUBKEY_ID>
gpg: échec du déchiffrement : Pas de clef secrète
...
```

`pass init` essaie de rechiffrer, avec la nouvelle clé, d'anciens jetons Docker qui avaient été chiffrés avec une clé qui n'existe plus (celle que je venais de supprimer). Forcément, il n'y arrive pas.

Bonne nouvelle quand même : pour utiliser Docker, la connexion n'est **pas** nécessaire. Les images publiques se téléchargent sans compte. J'aurais pu m'arrêter là. Mais ces jetons ne sont que des jetons de session, on peut les jeter sans regret :

```bash
rm -rf ~/.password-store/docker-credential-helpers
```

Je redémarre Docker Desktop, je me connecte, il recrée des jetons avec la nouvelle clé. Ça marche. Je suis en 4.100 et je suis connecté.

Mais franchement, quelle galère. Je suis étonné qu'il y ait encore des utilisateurs et des clients pour Docker Desktop sous Linux. Avec le recul, le conseil de tout à l'heure (ne garder que le moteur Docker) était sans doute la bonne chose à faire. Allez, je le note et je m'en félicite.

{: .note }
> TODO: Virer Docker Desktop et ne garder que le moteur Docker.










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Chrome et les petites applis

Je n'utilise que Edge. Chrome peut partir :

```bash
sudo apt purge google-chrome-stable
sudo rm -f /etc/apt/sources.list.d/google-chrome.list
rm -rf ~/.config/google-chrome ~/.cache/google-chrome
```

La deuxième ligne retire le dépôt de Google, sinon `apt` continuerait à le consulter à chaque mise à jour. Retenez cette ligne. Dans la [Partie 2]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_2.md%}), on va découvrir qu'elle n'a pas fait le travail.

Ensuite, j'ai fait le tour du menu Démarrer. Tout ce qui ne me sert jamais (Scanner, Warpinator...) est parti avec un clic droit sur l'icône, puis "Désinstaller". Je n'ai rien touché dans la partie Administration. Avec le recul, j'aurais sans doute pu en enlever un peu plus dans Accessoires.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Contrôle technique avant l'instantané

Avant de faire l'instantané Timeshift "propre", je voulais être sûr que la machine était saine. Voilà la check-list :

```bash
sudo apt autoremove --purge          # leftover dependencies
dpkg -l | grep '^rc' | awk '{print $2}' | xargs -r sudo apt purge -y   # leftover config files
sudo apt clean                       # empty the downloaded packages cache
sudo apt update && sudo apt full-upgrade
sudo dpkg --audit && sudo apt-get check   # no output = healthy
dpkg -l 'linux-image-*' | grep ^ii   # should list only the current kernel
df -h /
```

La deuxième ligne mérite une explication. Quand on désinstalle un paquet avec `remove` (et pas `purge`), ses fichiers de configuration restent sur le disque. `dpkg -l` les liste avec le statut `rc` (*removed, config files remain*). On récupère leurs noms avec `awk` et on les purge.

Tout est muet ou propre, sauf les noyaux :

```text
ii  linux-image-5.15.0-203-generic          5.15.0-203.213 amd64        Signed kernel image generic
ii  linux-image-5.15.0-204-generic          5.15.0-204.214 amd64        Signed kernel image generic
ii  linux-image-generic                     5.15.0.204.176 amd64        Generic Linux kernel image
```

Deux noyaux ? En fait, c'est parfait. Le 204 vient d'être installé par la mise à jour, et le 203 est celui qui tourne encore. Il servira de secours si le 204 pose problème. Il suffit de redémarrer pour passer sur le 204.

Je redémarre, puis je fais l'instantané :

```bash
sudo timeshift --create --comments "clean before Mint 22"
```

Il est 00h21. La machine est propre.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

Bilan de cette première partie :
* le disque est passé de **84 % à 44 %** d'occupation
* Anaconda, VMware Player, LibreOffice, Chrome et une cinquantaine de paquets inutiles sont partis
* Docker Desktop est à jour (et je peux m'y connecter, au prix d'un passage par GPG)
* la machine est à jour, saine, et un instantané Timeshift propre est prêt

Ce que je retiens :
* **Vider la corbeille.** Sérieux, 35 Go ?
* **Surveiller Timeshift.** Ses instantanés sont sur le même disque, et ils gardent tout ce qu'on a supprimé depuis.
* **Simuler avant de purger** (`apt purge -s`) et lire la liste des paquets ENLEVÉS. Si on y voit `cinnamon`, `mint-meta-*` ou un outil qu'on utilise, on s'arrête.
* **Ne pas toujours croire sur parole.** Même une IA sûre d'elle peut se tromper (le swap "jamais utilisé"). Quand ça paraît bizarre, on demande pourquoi.

Il est minuit vingt. Allez, on passe à la suite ? C'est dans la [Partie 2 : la migration vers Mint 22]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_2.md%}). Spoiler : je ne suis pas allé me coucher tout de suite.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliographie

* [Linux Mint Installation and Configuration]({%link docs/05_linux/012_install_config_mint/install_config.md%})
* [UV Python Package Manager: The Essentials]({%link docs/06_programmation/python/005_uv/uv.md%})
* [Install Docker Desktop on Linux](https://docs.docker.com/desktop/setup/install/linux/){:target="_blank"}
* [Timeshift on GitHub](https://github.com/linuxmint/timeshift){:target="_blank"}
* [Partie 2 : la migration vers Mint 22]({%link docs/05_linux/016_migration_mint_22/migration_mint_22_2.md%})
