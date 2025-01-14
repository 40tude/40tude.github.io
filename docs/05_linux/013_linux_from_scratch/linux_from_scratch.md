---
layout: default
title: "Linux from Scratch"
parent: "Linux"
#math: mathjax
date: 2013-06-09 23:41:51
last_modified_date: 2020-05-03 16:50:48
---

# Linux from Scratch

## Introduction

Quelques remarques dans le désordre concernant le [Linux from Scratch](https://www.linuxfromscratch.org/) 7.3 que je viens de terminer.

Ma config est constituée d'un PC avec deux gros disques sur lequel tourne une Debian 7. Pour l'instant, seul le premier disque est vraiment utilisé (sous LVM). Pour LFS, sur le deuxième disque, j'ai créé une partition de 50 Go et un swap de 8 Go.

## Remarque 1

Pour faire un Linux from Scratch à la main (pas en automatique) le mieux c'est de faire la plus grande partie du boulot à partir d'un autre PC. Dans mon cas je suis sous Windows, j'utilise putty et j'ai le pdf de LFS 7.3 sous les yeux. L'énorme avantage c'est la capacité à faire du copier-coller entre le pdf et la console ouverte dans putty. Ça fait gagner beaucoup de temps et ça évite les erreurs de typo.

Pour le reste si j'ai besoin de transférer des fichiers du PC vers la station j'utilise [FileZilla](https://filezilla-project.org/).

## Remarque 2

Tout à la fin (section Grub, 8.4) je me suis écarté du manuel. Sur la Debian j'ai ajouté un fichier 11_lfs dans /etc/grub.d/

```bash
#!/bin/sh -e
echo "Adding Linux From Scratch" >&2
cat << EOF
menuentry "Linux From Scratch 7.3" {
set root=(hd1,1)
search --no-floppy --fs-uuid --set=root c3c69696-33df-4183-befc-01e5c2251d85
linux /boot/vmlinuz-3.8.1-lfs-7.3 root=/dev/sdb1
}
EOF
```

Ensuite j'ai fait un ``update-grub``. Grub retrouve bien une partition inconnue mais ce n'est pas grave... Pour retrouver le ``uuid`` utilisé dans les scripts ci-dessus j'utilise la commande "blkid"

```bash
root@stark:~# blkid
/dev/sda5: UUID="Pqq7L7-MLiM-3Mqo-TLCp-dHZD-5MSM-xW3PS8" TYPE="LVM2_member"
/dev/sda1: UUID="frFLdX-2PXH-RHtJ-hIej-5cNB-Hc8U-1bi3Nv" TYPE="LVM2_member"
/dev/mapper/stark-root: UUID="2df98736-e2fa-4f97-9550-5366e4cf94e0" TYPE="ext4"
/dev/mapper/stark-swap_1: UUID="f3ffd285-559e-4ce7-bd9d-3cc1f033f98b" TYPE="swap"
/dev/mapper/stark-home: UUID="05795d84-3ad2-426d-b6a9-e5f69b7aee23" TYPE="ext4"
/dev/sdb1: LABEL="lfs" UUID="c3c69696-33df-4183-befc-01e5c2251d85" TYPE="ext3" SEC_TYPE="ext2"
/dev/sdb2: UUID="0428ba4d-b8c8-47b6-a0f0-8261bdb2e417" TYPE="swap"
```

En cas de doute, il ne faut pas hésiter à faire un ``mcview`` (ou autre) sur le fichier ``/boot/grub/grub.cfg`` et vérifier qu'il n'y a pas de problème dans la section 11_lfs

Si au reboot ça part en kernel panic... Pas de panique. Faut juste rebooter et repartir sous la Debian "normale" et modifier le fichier ``/etc/grub.d/11_lfs``






## Remarque 3

Si à un moment on quitte le chroot, si on reboot la station ou quoi ou qu'est-ce, il y a intérêt à na pas oublier à se mettre dans le bon contexte (et le bon chroot). Voir la note du paragraphe 6.4.



### Avant le chapitre 7

```bash
export LFS=/mnt/lfs
mount -v -t ext3 /dev/sdb1 $LFS
export MAKEFLAGS='-j 4'

mount -v --bind /dev $LFS/dev
mount -vt devpts devpts $LFS/dev/pts
mount -vt proc proc $LFS/proc
mount -vt sysfs sysfs $LFS/sys

if [ -h $LFS/dev/shm ]; then
link=$(readlink $LFS/dev/shm)
mkdir -p $LFS/$link
mount -vt tmpfs shm $LFS/$link
unset link
else
mount -vt tmpfs shm $LFS/dev/shm
fi

chroot "$LFS" /usr/bin/env -i \
HOME=/root TERM="$TERM" PS1='\u:\w\$ ' \
PATH=/bin:/usr/bin:/sbin:/usr/sbin:/tools/bin \
/bin/bash --login
```

### A partir du chapitre 7

```bash
export LFS=/mnt/lfs
mount -v -t ext3 /dev/sdb1 $LFS
export MAKEFLAGS='-j 4'

mount -v --bind /dev $LFS/dev
mount -vt devpts devpts $LFS/dev/pts
mount -vt proc proc $LFS/proc
mount -vt sysfs sysfs $LFS/sys

if [ -h $LFS/dev/shm ]; then
link=$(readlink $LFS/dev/shm)
mkdir -p $LFS/$link
mount -vt tmpfs shm $LFS/$link
unset link
else
mount -vt tmpfs shm $LFS/dev/shm
fi

chroot "$LFS" /usr/bin/env -i \
HOME=/root TERM="$TERM" PS1='\u:\w\$ ' \
PATH=/bin:/usr/bin:/sbin:/usr/sbin \
/bin/bash --login
```

## Remarque 4

À propos de la console et des locales je ne suis pas sûr d'avoir tout compris. Voir le paragraphe 7.10.

Voilà le contenu de mon fichier ``/etc/sysconfig/console``

```bash
# Begin /etc/sysconfig/console

UNICODE="1"
#KEYMAP="fr_FR-utf8"
#FONT="LatArCyrHeb-16"
KEYMAP="fr-latin9"
LEGACY_CHARSET="iso-8859-15"

# End /etc/sysconfig/console
```

Sur la Debian je ne retrouve pas de fichier console dans ``/etc/sysconfig`` dont je puisse m'inspirer. En toute logique j'aimerai mettre tout en UTF8 avec un truc du style fr-FR.utf8




## Remarque 5

À propos de la config du kernel... Bon là c'est un peu laborieux et va falloir que j'y revienne. En gros le seul truc sur lequel j'ai vraiment joué c'est sur la carte réseau (Intel 1000e). La compile... Tout se passe bien mais à l'heure actuelle j'ai un ``vmlinuz`` qui fait 4Go !

```bash
root@stark:/mnt/lfs/boot# ls -all
total 6408
drwxr-xr-x  2 root root    4096 juin   3 01:47 .
drwxr-xr-x 22 root root    4096 juin   1 15:04 ..
-rw-r--r--  1 root root   75043 juin   9 19:08 config-3.8.1
-rw-r--r--  1 root root 2237702 juin   9 19:08 System.map-3.8.1
-rw-r--r--  1 root root 4221152 juin   9 19:08 vmlinuz-3.8.1-lfs-7.3
```

* Quoiqu'il en soit le système boot et je peux "pinger" ``www.google.com``. 
* Je pense que je vais faire un test avec le ``.config`` de la Debian puis ne garder que ma carte réseaux. 
* Faudra aussi que j'aille lire la page : <http://elinux.org/Kernel_Size_Tuning_Guide>

