---
layout: default
title: "Installer un Raspberry Pi Zero en ligne de commande"
parent: "Linux"
#math: mathjax
date: 2017-03-11 02:02:13
last_modified_date: 2020-05-04 14:28:07
---

# Installer un Raspberry Pi Zero en ligne de commande

## Introduction
Le but du jeu est de copier sur une Micro SD Card une image Raspbian afin de faire démarrer une Raspberry Zero W qui vient d'arriver aujourd'hui. On n'est pas obligé de faire de cette façon. Voir cette [autre page]({%link docs/05_linux/007_installation_raspberry/installation_raspberry.md%}) d'instructions par exemple.

* On est sous PIXEL sur le RPI
* Ouvrir une console et aller dans le répertoire Téléchargements
* Saisir, au choix, l'une des deux commandes suivantes :

```bash
wget -O rasbian_test.zip https://downloads.raspberry.org/raspbian_latest
wget -O rasbian_test.zip https://downloads.raspberry.org/raspbian_lite_latest
```

* Aller boire un thé ou manger une pomme. 
* Lire la suite de ce billet et retenir que par la suite, en fonction du fichier téléchargé il faudra ajouter "lite" ou pas.
* Vérifier l'intégrité du fichier
    * Ce n'est pas instantané comme procédure. Le processeur tourne à moins de 10%. Re-thé ou relecture...

```bash
sha1sum raspbian_latest.zip
```

* Si besoin aller sur le site <https://www.raspberrypi.org/downloads/raspbian/> pour vérifier que les deux nombres coïncident bien
* Dézipper le fichier (ça prend du temps encore cette histoire et le processeur ne tourne qu'entre 10 et 25%)

```bash
unzip raspbian_latest.zip
```

* À ce jour (11/03/2017) le fichier s'appelle dorénavant "2017-03-02-raspbian-jessie.img"
* Ne rien connecter au RPI3 et faire l'inventaire des périphériques en l'état.

```bash
df -h
```

* Insérer une Micro SD Card dans un porteur USB et le brancher le porteur sur le RPI3
* Relancer la commande précédente pour découvrir comment s'appelle la clé USB

```bash
df -h
```

* Noter le nom de la clé. Ici elle s'appelle : ``/dev/sda1`` et elle est montée en ``/media/philippe/ESD-USB``. À noter que le 1 à la fin de ``sda1`` indique que c'est la partition 1.
* Démonter la clé

```bash
umount /dev/sda1
```

* Si on tape ``dd --version`` dans la console et si le N° de version qui sort est inférieur à 8.24 alors il faut passer par l'étape suivante si on veut voir une barre de progression à l'écran. C'est à faire une seule fois. Installer le package "pv" pour suivre l'avancement de la commande suivante :

```bash
sudo apt-get install pv
```

* Copier l'image sur la Micro SD Card. Attention il n'y a pas de N° à la fin de "sdd" (on va machouiller toute la carte et pas une partition particulière)

```bash
pv 2017-03-02-raspbian-jessie.img | sudo dd bs=4M of=/dev/sda
```

* Si le N° de version de coreutils est supérieur à 8.24 on peut invoquer la commande suivante (à noter le paramètre ``status=progress`` ) :

```bash
dd bs=4M status=progress if=2017-03-02-raspbian-jessie.img  of=/dev/sda
```

* Vider le cache avant de retirer la Micro SD Card

```bash
sudo sync
```

* Insérer la Micro SD Card dans le Raspberry Pi Zero W, brancher clavier, souris, écran et alimentation.
* Boot ! (Attention cela peut prendre du temps...)
* Quand tout sera configuré il faudra penser à supprimer sur le RPI3, dans le répertoire "Téléchargements", le fichier zip téléchargé (rasbian_test.zip par exemple) ainsi que l'image disque associée (2017-03-02-raspbian-jessie.img par exemple).





## Configuration du Wifi

* Saisir la commande suivante

```bash
sudo nano /etc/wpa_supplicant/wpa_supplicant.conf
```

* Compléter le fichier avec les lignes suivantes (ne pas oublier les guillemets)

```bash
network={
    ssid="Nom du Wifi"
    psk="Mot de Passe"
}
```

* Mettre en route le réseau (pour arrêter c'est sudo ifdown wlan0)

```bash
sudo ifup wlan0
```

* Faire un test de réseau avec un ``ifconfg`` pour récupérer l'adr puis un ping

```bash
ifconfig
ping google.com
```




## Configuration du Wifi sans clavier ni écran

Je me place dans le cas où on vient de copier l'image sur la Micro SD Card et où on vient de vider le cache USB du Raspberry 3 avec la commande suivante :

```bash
sudo sync
```

De plus, je suppose qu'il y a une Box qui fait serveur DHCP qui distribue des adr.

* Retirer la clé USB qui porte la Micro SD Card
* Remettre la clé USB. Si besoin, sur le RPI3, cliquer sur le bouton "Annuler" de la boîte de dialogue qui apparaît sous PIXEL.
* Vérifier les périphériques montés avec

```bash
df -h
```

* Par exemple, sur mon RPI3 je vois la liste suivante

```bash
...
/dev/sda1            /media/bob/boot
/dev/sda2            /media/bob/8a907.....
```

* Aller dans le répertoire /media/bob/8a907.... et inspecter son contenu

```bash
cd /media/bob/8a907...
ls -all
```

* On retrouve l'arborescence du futur système de fichiers du Raspberry Pi  Zero W
* Saisir la commande suivante. 

{: .warning }
Il y a bien un "." devant "/etc" car on va éditer le fichier qui est sur la clé USB et **PAS** le fichier qui est sur le RPI3.

```bash
sudo nano ./etc/wpa_supplicant/wpa_supplicant.conf
```

* Compléter le fichier avec les lignes suivantes (ne pas oublier les guillemets) :

```bash
network={
    ssid="Nom du Wifi"
    psk="Mot de Passe"
}
```

* CTRL+O et CTRL+X pour quitter nano.

Ensuite on va modifier le fichier "interfaces" de la clé USB

* Ouverture du fichier

```bash
nano ./etc/network/interfaces
```

* Dans le fichier il faut trouver les lignes

```bash
allow-hotplug wlan0
iface wlan0 inet manual
...
```

* Et les modifier comme ci-dessous (ne pas oublier le dhcp en bout de ligne :-) )

```bash
auto wlan0
allow-hotplug wlan0
iface wlan0 inet dhcp
...
```

* CTRL+O et CTRL+X pour quitter nano.

Pour terminer il faut créer un fichier "ssh" dans le sous-répertoire boot car le protocole ssh n'est pas [supporté par défaut](https://www.raspberrypi.org/blog/a-security-update-for-raspbian-pixel/).

```bash
sudo touch ../boot/ssh
```

{: .warning }
On parle bien du répertoire ``/media/bob/boot``" et **PAS** du répertoire ``/media/bob/8a907.../boot``. Comme on est dans le répertoire ``/media/bob/8a907...`` il faut bien mettre deux points devant ``/boot`` dans la commande précédente. 

* Prendre le temps de bien vérifier qu'un fichier vide nommé ``ssh`` est bien dans ``/boot``
* Terminer avec les commandes suivantes avant de débrancher la clé USB qui porte la Micro SD Card du Raspberry 3

```bash
umount /dev/sda1
umount /dev/sda2
sudo sync
```

* Insérer la Micro SD Card dans le Raspberry Pi Zero W et brancher l'alimentation.
* Boot !
* Ensuite il faut retrouver via sa box, l'adresse IP du Raspberry Pi Zero W et se connecter dessus en SSH pour terminer la [configuration]({%link docs/05_linux/007_installation_raspberry/installation_raspberry.md%}).









## Utilisation du Bluetooth

Avec un mini clavier [RT-MWK02+](http://www.riitek.eu/FR/Produits/RT-MWK02%2B_FR.html) par exemple.

### Note

À ce jour (mars 2017) les manipes ci-dessous semblent à jour.

* Allumer le clavier et dans la console saisir la commande suivante :

```bash
bluetoothctl
```

* On rentre alors dans une session dont le prompt commence par "[bluetooth]#". Dans ce qui suit, après ce prompt c'est ce que l'on saisi. Le reste c'est la réponse de bluetoothctl. Saisir les commandes suivantes :

```bash
[bluetooth]# help
Available commands:
  list                       List available controllers
  show [ctrl]                Controller information
  select <ctrl>              Select default controller
  devices                    List available devices
  paired-devices             List paired devices
  power <on/off>             Set controller power
  pairable <on/off>          Set controller pairable mode
  discoverable <on/off>      Set controller discoverable mode
  agent <on/off/capability>  Enable/disable agent with given capability
  default-agent              Set agent as the default one
  scan <on/off>              Scan for devices
  info <dev>                 Device information
  pair <dev>                 Pair with device
  trust <dev>                Trust device
  untrust <dev>              Untrust device
  block <dev>                Block device
  unblock <dev>              Unblock device
  remove <dev>               Remove device
  connect <dev>              Connect device
  disconnect <dev>           Disconnect device
  version                    Display version
  quit                       Quit program

[bluetooth]# agent on
Agent registered

[bluetooth]# pairable on
Changing pairable on succeeded

[bluetooth]# scan on
Discovery started
...
[CHG] Device CC:C5:0A:1C:04:09 Name: Bluetooth 3.0 Macro Keyboard
...

bluetooth]# scan off
Discovery stopped
...

[bluetooth]# pair CC:C5:0A:1C:04:09
Attempting to pair with CC:C5:0A:1C:04:09
[CHG] Device CC:C5:0A:1C:04:09 Connected: yes
[agent] PIN code: 432703
[CHG] Device CC:C5:0A:1C:04:09 Modalias: usb:v05ACp8502d011B
[CHG] Device CC:C5:0A:1C:04:09 UUIDs:
        00001000-0000-1000-8000-00805f9b34fb
        00001124-0000-1000-8000-00805f9b34fb
        00001200-0000-1000-8000-00805f9b34fb
[CHG] Device CC:C5:0A:1C:04:09 Paired: yes
Pairing successful
[CHG] Device CC:C5:0A:1C:04:09 Connected: no

[bluetooth]# trust CC:C5:0A:1C:04:09
[CHG] Device CC:C5:0A:1C:04:09 Trusted: yes
Changing CC:C5:0A:1C:04:09 trust succeeded

[bluetooth]# connect CC:C5:0A:1C:04:09
Attempting to connect to CC:C5:0A:1C:04:09
[CHG] Device CC:C5:0A:1C:04:09 Connected: yes
Connection successful

[bluetooth]# info CC:C5:0A:1C:04:09
Device CC:C5:0A:1C:04:09
        Name: Bluetooth 3.0 Macro Keyboard
        Alias: Bluetooth 3.0 Macro Keyboard
        Class: 0x002540
        Icon: input-keyboard
        Paired: yes
        Trusted: yes
        Blocked: no
        Connected: yes
        LegacyPairing: yes
        UUID: Service Discovery Serve.. (00001000-0000-1000-8000-00805f9b34fb)
        UUID: Human Interface Device... (00001124-0000-1000-8000-00805f9b34fb)
        UUID: PnP Information           (00001200-0000-1000-8000-00805f9b34fb)
        Modalias: usb:v05ACp8502d011B

[bluetooth]# exit
```

### Note

Une fois que cela a été fait, en ce qui me concerne... Rien ne marchait. J'ai simplement éteint et rallumé le clavier et là les caractères apparaissaient bien à l'écran. Au moins il ne faut pas faire la manipe ``bluetoothctl`` à chaque fois 

Si par la suite, le clavier ne fonctionne pas, rentrer dans ``bluetoothctl`` et saisir ``connect CC:C5:0A:1C:04:09`` puis sortir.


{: .warning }
À ce jour (mars 2017) j'ai de très sérieux problème de charge CPU sur le Raspi Zero W avec Bluetooth. Je n'ai pas ces problèmes sur un Raspberry Pi 3 par exemple. Sur le Zero, si je "joue" avec le clavier, dès que ça marche, ``/usr/lib/bluetooth/bluetoothd`` fait grimper la charge à 100%. 

Je ne comprends pas ce qui se passe. Pour l'instant je m'en sors avec :

```bash
sudo systemctl stop bluetooth
```

Ce n'est pas du tout satisfaisant.

