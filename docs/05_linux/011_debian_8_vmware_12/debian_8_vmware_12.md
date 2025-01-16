---
layout: default
title: "Installer une Debian dans VMWare Workstation 12 Player"
parent: "Linux"
#math: mathjax
date: 2017-02-26 01:56:03
last_modified_date: 2020-05-04 09:44:52
---


# Installer une Debian dans VMWare Workstation 12 Player

## Introduction

À la date de rédaction de ce billet, installer une Debian dans VMWare Workstation ne pose aucun problème. Le truc se sont les quelques étapes supplémentaires à faire ensuite. Comme je n'ai pas envie de consommer des cases mémoire pour ça je préfère me faire une **checklist**.

Je suis sous Windows 10 mais la version de Windows n'est pas essentielle.

Je fais une installation de [VMware Workstation 12 Player](https://www.vmware.com/fr/products/workstation-player/workstation-player-evaluation.html)


{: .warning }
Janvier 2025. Le lien précédent ne fonctionne certainement plus. Essayez celui-ci : <https://www.minitool.com/news/download-vmware-workstation-pro-17-for-free.html>

Je crée et j'installe une Debian 8 Mate 64 bits (attention bien choisir "64 bits install" au tout début 😀 )

Pendant l'installation VMWare va demander a télécharger les VMWare Tools for Linux. Demander lui de vous le rappeler plus tard.

Au premier démarrage, commencer par éjecter le CD. Ensuite dans un terminal :

1. ``su``
2. ``visudo``
3. En dessous de la ligne : ``%sudo ALL=(ALL:ALL) ALL``
4. Je rentre une ligne du style : ``philippe ALL=(ALL:ALL) ALL``
5. **CTRL + O** pour écrire
6. **CTRL + X** pour quitter
7. ``exit`` (pour quitter le mode super user)

Dans un terminal :

```bash
sudo apt-get install build-essential
sudo apt-get install linux-headers-$(uname -r)
```
Quand c'est fait faut laisser VMWare télécharger les VMWare Tools for Linux. Quand c'est fait :

1. Double clic sur le CD VMWare Tools
2. Clic droit sur ``VMWareTools....tar.gz``
3. Choisir ``Extraire dans...``
4. Choisir un truc du style ``~/Téléchargements``

Ouvrir un terminal 

1. Aller dans ``~/Téléchargements/vmware-tools-distrib``
2. ``sudo ./vmware-install.pl``
3. Taper "yes" à la toute première question
4. Accepter toutes les autres réponses par défaut (ENTER)
5. Redémarrer la VM

Sur le Surface Pro, je ne vois rien car on est à 215 points par pouce au niveau de la résolution de l'écran

1. Système
2. Préférences
3. Onglet Polices
4. Bouton Détails
5. Régler la résolution au plus haut (idéalement faudrait 215) mais je le règle sur 150

 En se rappelant que j'ai déjà installé build-essentail, j'installe maintenant :

1. La dernière version de [**clang**](https://www.40tude.fr/installer-la-derniere-version-de-llvm-clang-sous-debian/) (3.9.1 en Fev 2017)
2. **cmake** (``sudo apt-get install cmake``)
3. **git** (``sudo apt-get install git``)
4. **catch** (dans ``~/Documents/C++/`` taper un truc du style git clone https://github.com/philsquared/Catch.git)
5. **cppcheck** (sudo apt-get install cppcheck)
6. **boost**
    1. Aller lire cette [page](http://www.linuxfromscratch.org/blfs/view/svn/general/boost.html)
    1. Récupérer le fichier .7z sur la page download (il arrive dans mon répertoire Téléchargements)

```bash
sudo apt-get install p7zip
cd ~/Téléchargements
7z x boost_1_63_0.7z
cd boost_1_63_0
./bootstrap.sh --prefix=/usr 
./b2 stage threading=multi link=shared 
sudo ./b2 install threading=multi link=shared (voir /usr/include/boost et /usr/lib)
```

* Avec `bootstrap.sh`, c'est à ce moment là que le script b2 est créé
* b2 va prendre un bon moment dans la VM

Faut penser à faire le ménage de ~/Téléchargments

*La suite au prochain épisode...*
