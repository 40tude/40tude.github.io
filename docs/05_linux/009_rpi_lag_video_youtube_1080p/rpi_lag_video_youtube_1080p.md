---
layout: default
title: "Raspberry lag vidéo YouTube 1080p"
parent: "Linux"
#math: mathjax
date: 2018-01-15 08:02:28
last_modified_date: 2023-01-12 00:11:09
---

# Raspberry lag vidéo YouTube 1080p

Cette fiche tente de résoudre les problèmes de lag vidéo sur le Raspberry 3 lors des lectures de vidéo sur YouTube. On va faire ça en mode télégraphe.

Pour commencer, il faut s'assurer que 128MB (ou plus) sont attribués au GPU

```bash
sudo raspi-config
```
* Advanced Options puis Memory Split

Ensuite

```bash
sudo apt-get update
sudo apt-get dist-upgrade -y
sudo apt-get install mesa-vdpau-drivers -y
```

Lancer alors Chromium et dans la barre d'adresse taper : ``chrome://flags``

Dans la page qui apparaît, retrouver et activer les switchs suivants

* ``#enable-gpu-rasterization``
* ``#enable-zero-copy``
* ``#ignore-gpu-blacklist`` (si Chromium est en version 60 et plus)

Modifier les paramètres de lancement de Chromium

```bash
sudo nano /usr/share/applications/chromium-browser.desktop
```

Remplacer la première par la seconde ligne

1. ``Exec=chromium-browser %U``
2. ``Exec=chromium-browser --enable-native-gpu-memory-buffers %U``

