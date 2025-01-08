---
layout: default
title: "Patch"
parent: "Linux"
#math: mathjax
date: 2023-11-30 15:22:18
last_modified_date: 2023-11-30 15:35:48
---

# Patch

<div align="center">
<img src="./assets/image-38.webp" width=450 alt="" loading="lazy"/>
</div>

C'est juste une cheat-sheet car je ne me rappelle jamais comment on doit procéder. J'en profite pour coller ici un moyen de faire la différence entre 2 répertoires.
Attention, j'ai pas creusé ni encore compris ces histoire de p0, P1... (voir dernier lien au bas de cette page)

## Différence entre 2 répertoires

```
git diff --no-index dir1/ dir2/                 # forcément faut git sur la machine
diff -qrN dir1/ dir2/
```




## Sur un fichier

### Apply

```
patch foo.c < my.patch                   # the patch "knows" the name of the file to be patched
```
### Undo

```
patch -R < /path/to/file
```
### Create

```
diff -u hello.cpp hello-new.cpp > my.patch
cat my.patch
patch < hello.patch                      # the patch "knows" the name of the file to be patched
```




## Sur un répertoire

### Faire une copie

```
cp -R origin new_version
```
### Faire des changements

```
cd new_version
...
```
### Créer le patch

```
cd ..
diff -Naur origin new_version > my.patch
```
### Appliquer le patch

```
cp -R origin origin_bak
patch --dry-run -ruN -d origin < my.patch
patch -ruN -d origin < my.patch
```



## Lire

* <https://www.cyberciti.biz/faq/appy-patch-file-using-patch-command/>
* <https://www.howtogeek.com/415442/how-to-apply-a-patch-to-a-file-and-create-patches-in-linux/>
* <https://doc.ubuntu-fr.org/patch>

