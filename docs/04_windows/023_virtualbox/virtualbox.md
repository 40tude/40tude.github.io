---
layout: default
title: "VirtualBox"
parent: "Windows"
#math: mathjax
date: 2013-05-10 21:34:15
last_modified_date: 2023-01-12 00:15:55
---

# VirtualBox

## Introduction
Je viens d'installer VirtualBox 4.2.12 (Mai 2013) afin d'y faire tourner une Debian 64bit Standard (pas de fenêtres pour l'instant, juste une simple console pour faire des tests de compilation).

Vous trouverez dans cette checklist deux ou trois trucs que j'ai dû faire pour pouvoir configurer l'invité comme je voulais et dont je veux garder trace.


## Installer sudo

```actionscript3
# adduser philippe sudo
```

Allez, une bonne chose de faite. On passe à la suite.

## Installer mc et mcedit

```text
$ sudo apt-get install mc
```

Maintenant on peut "graphiquement" naviguer dans les répertoires avec "mc". On peut surtout éditer les fichiers avec "mcedit". En fait je préfère "mcedit" à "nano" car on a pas besoin de faire **CTRL + C** pour savoir sur quelle ligne on est (toutes les informations sont affichée en haut de la "fenêtre"). Enfin bref c'est du détail. Pour en apprendre plus on peut aller jeter un oeil sur : <http://www.thegeekstuff.com/2008/10/midnight-commander-mc-guide-powerful-text-based-file-manager-for-unix/>

## Pour installer les Additions invité

```bash
$su
#apt-get update
#apt-get upgrade
#aptitude install dkms // install gcc et les headers qui vont bien...
#apt-get install build-essential
```

Pas besoin d'installer les headers avec un truc du style (#apt-get install linux-headers-$(uname -r) car les fichiers sont déjà là.

Reboot de la machine pour activer les mises à jour

```bash
#reboot
```

Ejecter le cd virtuel si y en a un
Dans le menu de la machine virtuelle cliquer sur : Périphériques/Installer les Addition invité...

Si jamais vous ne savez pas comment retrouver le nom de dev du cdrom

```bash
#sysctl dev.cdrom.info
```

Dans mon cas je vois que c'est sr0

```bash
#mount /dev/sr0 /media/cdrom
#cd /media/cdrom
#sh ./VBoxLinuxAdditions.run
#reboot
```

Normalement c'est bon. Dans mon cas le script râle un peu car j'ai pas XWindow.

## Pour faire du Putty entre l'hôte (Windows7) et l'invité (Debian Console)

Openssh-server est déjà installé
Faut se mettre en "Accès par pont" dans la config réseau de la machine virtuelle
Lancer le guest
Dans le guest retrouver son adr ip

```bash
$ifconfig
```

Lancer putty.exe (à récupérer sur le [site](http://www.chiark.greenend.org.uk/~sgtatham/putty/download.html)) depuis le host windows.

Utiliser l'ard ip de la cible pour se connecter.

Penser à mettre UTF8 ou ISO-8859-15 dans les paramètres de "Translation"

## Changer la résolution de la console (par défaut 640x480)

```bash
$su
#nano /etc/default/grub
```

Faut changer la ligne GRUB_GFXMODE.

Par exemple dans mon cas je met:
GRUB_GFXMODE=1024×768

Ajouter la ligne suivante juste en dessous de la précédente.
GRUB_GFXPAYLOAD_LINUX=keep

Sauver/quitter
Ensuite faut pas oublier de mettre à jour grub

```bash
#update-grub2
#reboot
```

## Copier-Coller entre l'hôte et la console de l'invité

VirtualMachine's Settings->General->Advanced->Shared Clipboard (Bidirectional) puis reboot de la machine

 *Je ne suis pas sûr que ça marche encore comme je veux. Faut que je trouve un test significatif.*

Une solution qui ne marche pas trop mal : utiliser **Putty.exe** à partir de l'hôte Windows.

Si à un moment il faut copier-coller alors dans la console invité, click droit insère le texte copié dans l'hôte.

## Monter un répertoire partagé

```text
# mkdir shared
# mount -t vboxsf shared ./shared
```

