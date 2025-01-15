---
layout: default
title: "Mac (El Capitan) configurer le mode hibernate"
parent: "Divers"
#math: mathjax
date: 2016-06-15 19:30:51
last_modified_date: 2020-05-03 17:01:46
---

# Mac (El Capitan) configurer le mode hibernate

Sous El Capitan je n'arrivais pas à mettre le Mac en mode hibernate. En fait les modifs que je tentais de faire étaient bloquée par le ``SIP`` qu'il faut désactiver. Bref, voici la méthode en mode télégraphique :

1. Reboot du Mac en mode recovery (**⌘R**jusqu'à ce que la pomme soit à l'écran)
2. Dans une console taper :
   ```bash
    csrutil disable; reboot
   ```
3. Quand la session est ouverte, dans une console taper :
   ```bash
   sudo pmset -a hibernatemode 25
4. Normalement quand on ferme l'écran au bout de quelques secondes, la led s'éteint. Dans mon cas (16Go de RAM) je retrouve un fichier ``/private/var/vm/sleepimage`` de 8Go

