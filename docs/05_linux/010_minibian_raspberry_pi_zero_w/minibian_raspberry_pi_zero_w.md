---
layout: default
title: "Minibian sur Raspberry Pi Zero"
parent: "Linux"
#math: mathjax
date: 2017-03-12 15:20:32
last_modified_date: 2020-05-03 22:05:08
---


# Minibian sur Raspberry Pi Zero

Le Raspberry Pi Zero est très bien mais il n'est pas très véloce. Il n'est sans doute pas inutile de lui installer plutôt une Minibian.

## Copie de l'image sur Micro [SD Card]({%link docs/99_divers/001_cartes_micro_sd/cartes_micro_sd.md%})

* On est sous PIXEL sur le RPI3
* Ouvrir une console et aller dans le répertoire Téléchargements
* Récupérer le tar.gz de Minibian (225 Mo vs 1.5 Go pour une Raspbian)

```bash
wget https://sourceforge.net/projects/minibian/files/2016-03-12-jessie-minibian.tar.gz
```

* Décompresser l'image

```bash
tar xvf 2016-03-12-jessie-minibian.tar.gz
```

* On se retrouve avec un fichier ``2016-03-12-jessie-minibian.img`` (830 Mo)
* Il faut le copier sur une carte Micro SD Card. La procédure est identique à celle [expliquée ici]({%link docs/05_linux/008_install_rpi0_from_rpi/install_rpi0_from_rpi.md%}).
* Ne rien connecter au RPI3 et faire l'inventaire des périphériques connectés.

```bash
df -h
```

* Insérer une Micro SD Card dans un porteur USB et le brancher ce dernier sur le Raspberry 3
* Relancer la commande précédente pour découvrir comment s'appelle la clé USB

```bash
df -h
```

* Noter le nom de la clé. Ici elle s'appelle : ``/dev/sda1`` et elle est montée en ``/media/philippe/ESD-USB``. À noter que le 1 à la fin de sda1 indique que c'est la partition 1. Il peut y avoir plus d'une partition.
* Démonter la partition (réitérer l'opération s'il y a plusieurs partitions ``/dev/sda2`` etc.)

```bash
umount /dev/sda1
```

* Si on tape ``dd --version`` dans la console et si le N° de version qui sort est inférieur à 8.24 alors il faut passer par l'étape suivante si on veut voir une barre de progression à l'écran. C'est à faire une seule fois. Installer le package ``pv`` pour suivre l'avancement de la commande suivante :

```bash
sudo apt-get install pv
```

* Copier l'image sur la Micro SD Card. Attention il n'y a pas de N° à la fin de "sda" (on va machouiller toute la carte et pas une partition particulière)

```bash
pv 2016-03-12-jessie-minibian.img | sudo dd bs=4M of=/dev/sda
```

* Si le N° de version de coreutils est supérieur à 8.24 on peut invoquer la commande suivante (à noter le paramètre status=progress ) :

```bash
dd bs=4M status=progress if=2016-03-12-jessie-minibian.img  of=/dev/sda
```

* Vider le cache avant de retirer la Micro SD Card

```bash
sync
```

* Si vous êtes pressé, il faut insérer la SD Card dans un Raspberry Pi, brancher clavier, écran et alimentation. Sinon lisez les paragraphes suivants à propos de chroot etc.
* Boot ("root" & "raspberry" - Attention c'est "root" et pas "pi" et le clavier est "qwerty" pour l'instant)

À faire le plus tôt possible même si le clavier est encore en "qwerty"

```bash
apt-get update
apt-get upgrade
```

* Quand tout fonctionne, penser à supprimer sur le RPI3, dans le répertoire "Téléchargements", le fichier ``2016-03-12-jessie-minibian.tar.gz`` ainsi que le fichier image ``2016-03-12-jessie-minibian.img``.



## Utiliser chroot pour customiser l'installation de Minibian

Je suppose que l'image est copiée sur la Micro SD Card qui est dans une clé USB elle-même connectée à un Raspberry Pi. J'imagine que la Micro SD Card n'a pas encore été insérée dans un Raspberry pour le faire booter (voir fin des instructions précédentes). Ici, on va "préparer" le Minibian qui est sur la carte Micro SD avant de procéder à un vrai premier boot. En fait ce qui m'a motivé au départ ce sont ces histoires de clavier. Enfin bref...

* Débrancher la clé USB (dans laquelle est insérée la Micro SD Card)
* Rebrancher la clé
* Avec le gestionnaire de fichiers aller voir dans /media/$USER$
* Il y a deux répertoires qui correspondent aux 2 partitions qui sont sur la Micro SD Card
* Une des partitions possède un nom en majuscule et comprend un sous répertoire "overlays". Ce n'est pas elle qui nous intéresse
* L'autre répertoire a un nom à ralonge et comporte l'arborescence de la distribution
* Dans une console, aller dans le répertoire en question

```bash
cd /media/bob/f55....
```

* Bien vérifier qu'il y a les sous-répertoires ``/bin /etc / sbin ...``
* Modification du fichier ``source.list`` de Minibian (bien voir le "." au début de ./etc/.....)

```bash
sudo nano ./etc/apt/sources.list
```

* Le modifier pour qu'il ne contienne que la ligne que l'on a normalement dans une Raspbian classique

```text
deb http://mirrordirector.raspbian.org/raspbian/ jessie main contrib non-free rpi
```

* Saisir ensuite les commandes suivantes

```bash
sudo mount -o bind /proc ./proc
sudo mount -o bind /dev ./dev
sudo mount -o bind /dev/pts ./dev/pts
sudo cp /etc/resolv.conf ./etc/resolv.conf
```

* Faire très **ATTENTION** au "." devant ./proc par exemple.
* Pour finir lancer chroot

```bash
sudo chroot . /bin/bash
```

* Le prompt change. Il devient ``root@NOM_MACHINE``. Il est important de comprendre qu'à partir de ce moment, **tout se passe comme si** notre système de fichier avait sa racine dans le répertoire f55... À titre d'exemple, si on tape "pwd" dans la console la réponse est "/". Les manipulations précédentes avec ./proc etc. sont là pour faire en sorte que tout ne se passe pas trop mal dans notre sessions chroot.
* On peut vérifier que notre session ``chroot`` est bien connectée au réseau via un ``ifconfg``

### Faire l'update et la mise à jour de Minibian dans chroot

* Ci-dessous, pas besoin de "sudo" car on est "root"
* L'upgrade va prendre son temps...

```bash
apt-get update
apt-get upgrade
```

### Installer le clavier Français dans chroot

```bash
dpkg-reconfigure locales
```

* Laisser la case cochée en face de ``en_GB.UTF-8 UTF-8``
* Cocher ``fr_FR.UTF-8 UTF-8``
* Choisir ``fr_FR.UTF-8`` dans la boite de dialogue suivante

```bash
apt-get install keyboard-configuration
```

* Dans la boîte de dialogue choisir "Other"
* Dans la longue liste, choisir "French"
* Choisir encore "French" dans la boite de dialogue suivante

```bash
apt-get install console-data
```

* Choisir "Select keymap from full list"
* Choisir "pc / azerty / French / Same as X11 (latin 9) / Standard"

Normalement c'est réglé, à part le symbole euro. Au pire j'y reviendrai plus tard. Pendant que l'on y est, réglons la time zone

```bash
dpkg-reconfigure tzdata
```

* Choisir Europe
* Choisir Paris




### Régler le problème de CRDA au boot

Lors d'un boot de Minibian "normal" je vois des messages du type :

```bash
cfg80211: Calling CRDA to update world regulatory domain
```

Après "une certains nombre" d'essais infructeux les tentatives sont abandonnées. Ce n'est pas critique mais bon c'est embêtant car cela arrive pendant qu'on saisi son Id et son mot de passe. En fait il semble que Minibian charge le module ``cfg80211`` qui tente de faire un update du crda (à ce jour, je ne suis pas sûr d'avoir bien compris de quoi il s'agit. [Voir ici si besoin](https://wireless.wiki.kernel.org/en/developers/regulatory/crda)). crda n'est pas installé. Il faut y remédier et autant le faire pendant qu'on est dans la session chrootée.

```bash
apt-get install crda
```

Le boot sera un tout petit peu plus long mais on aura plus les messages



### Installer nano

C'est plus à titre d'exemple qu'autre chose mais comme "nano" n'est pas présent par défaut sur Minibian et que j'ai beaucoup de mal avec Vim (qui lui est installé par défaut) on va corriger le tir maintenant.

```bash
apt-get install nano
```




### Installer le réseau

* Je souhaite installer Minibian sur un Raspbery Pi Zero qui n'a pas de Wifi.
* J'ai un dongle wifi Belkin Surf N150. Quand je branche ce dernier sur un Raspberry Pi 3 et que j'invoque la commande ``lsub`` je "découvre" que ce dernier est à base de Realtek RTL 8188CUS.
* J'installe donc les soft suivants :

```bash
apt-get install firmware-brcm80211
apt-get install firmware-realtek
apt-get install wpasupplicant
```

* Edition du fichier ``wpa-supplicant.conf``

```bash
nano /etc/wpa_supplicant/wpa_supplicant.conf
```

* Voilà le contenu du fichier ``wpa_supplicant.conf`` (ne pas oublier les guillemets)

```text
network={
   ssid="Nom Du Reseau"
   psk="Mot de passe"
}
```

* **CTRL + O** puis **CTRL + X** pour sauver, quitter
* Edition du fichier interfaces

```bash
nano /etc/network/interfaces
```

* Voilà l'intégralité du fichier interfaces

```text
auto lo
iface lo inet loopback

auto eth0
iface eth0 inet dhcp

allow-hotplug wlan0
iface wlan0 inet dhcp
wpa-conf /etc/wpa_supplicant/wpa_supplicant.conf
```

* **CTRL + O** puis **CTRL + X** pour sauver, quitter




### Quitter chroot et tester Minibian

* On a terminé la "préparation" de la Minibian
* Taper ``exit`` dans la console
* On quitte la session chroot et on revient dans le "monde normal".
* Pour information après l'upgrade et les différentes installations, Minibian est passée de 440Mo à 490Mo
* Éjecter la Micro SD Card et l’insérer dans un Raspberry (moi ce sera un RP Zero)
* Boot ("root" & "raspberry"). La toute première fois, je branche un écran "au cas où..."
* Si un clavier est connecté, ce dernier est en français, nano est dispo etc.
* En ce qui me concerne, sur le Zéro, le dongle clignote, je retrouve l'adr via ma box ADLS et je peux faire du ssh à partir de Bash sous Windows 10. Que du bonheur... Enfin presque parce qu'il reste encore une chose à faire...




## Redimensionner la partition sur la Micro SD Card

* Il faut simplement suivre les instructions qui sont [sur cette page](https://minibianpi.wordpress.com/how-to/resize-sd/).
* Je suppose que l'on a booté Minibian sur un Raspberry et que l'on a un clavier et un écran.
* En ce qui me concerne, Minibian est sur un Raspberry Pi Zero Headless. Je fais tout via ssh à partir de Bash sous Windows 10.
* On est root. Il n'y aura donc pas de "sudo" par la suite.
* Optionnel. Faire la liste des points de montage

```bash
df -h
```

* Optionnel. Faire la liste des partitions

```bash
mount
mount | grep "mmc"
```

* Il y a trop de choses avec "mount" tout seul. La seconde ligne permet de filtrer les choses.
* Normalement on observe que le périphérique ``/dev/mmcblck0p1`` est monté sur ``/boot`` et que ``/dev/mmcblck0p2`` est monté sur ``/``
* Lancer ``fdisk`` sur ``mmcblck0``. Il n'y a pas ``p1`` ou ``p2`` à la fin car c'est le device ``mmcblck0`` et pas la partition ``p1`` (ou ``p2``) que l'on veut modifier.

```bash
fdisk /dev/mmcblk0
```

* Appuyer sur "m" pour voir l'aide en ligne
* Appuyer sur "p" pour voir la liste des partitions

Dans la suite, on va supprimer de la table de partition la seconde partition (celle de type Linux quand on a appuyé sur "p"). Ensuite on va recréer dans la table de partition une partition qui aura le même secteur de départ mais qui utilisera tout l'espace libre de la clé.

* Noter le nombre qui correspond à "Start" pour la seconde partition (125056 chez moi)
* Appuyer sur "d" puis sur "2" pour supprimer la partition Linux
* Appuyer sur "p" pour vérifier qu'il n'y a plus qu'une seule partition dans la table
* Appuyer sur "n", puis sur "p" puis sur "2"
* Taper "125056". Vérifier que ce nombre est égal à celui qui marquait le début de la seconde partition.
* Appuyer sur "ENTER" pour la nouvelle partition utilise tout l'espace libre
* Appuyer sur "w" pour écrire la nouvelle table de partition
* Taper "reboot"
* Quand le boot est terminé, saisir :

```bash
resize2fs /dev/mmcblk0p2
```

* Pas de panique. Resize2fs prend un peu de temps...

```bash
df -h
```
* Normalement le point de montage "/" doit avoir une taille homogène avec celle de la clé.


