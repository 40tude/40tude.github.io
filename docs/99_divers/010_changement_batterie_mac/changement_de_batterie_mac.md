---
layout: default
title: "Changement de batterie Mac"
parent: "Divers"
#math: mathjax
date: 2016-06-04 00:02:09
last_modified_date: 2020-05-03 21:35:30
---

# Changement de batterie Mac

Symptômes : en fait kernel_task est à 400% ou un truc comme ça dans le moniteur système. Je ne peux rien faire. Cela dit, quand je remets la batterie originale tout fonctionne bien.

Sous El Capitain, boot en mode recovery (CMD+R au boot) puis pour désactiver le system integration protection, dans un shell, taper :

```bash
csrutil disable
```

Boot en mode normal puis dans un shell, taper :

```bash
system_profiler -detailLevel mini | grep "Model Identifier:"
Model Identifier: MacBookPro8.2
```

Bien sûr la réponse dépend du modèle (moi j'ai un MacBook Pro Early 2011)

Dans le shell taper :

```bash
cd /System/Library/Extensions/IOPlatformPluginFamily.kext/Contents/PlugIns/ACPI_SMC_PlatformPlugin.kext/Contents/Resources
```

Enfin saisir :

```bash
sudo mv MacBookPro8_2.plist MacBookPro8_2.old
```

Boot en mode recovery (CMD+R au boot) puis ré-activer le system integration protection en tapant dans un shell :

```bash
csrutil enable
```

Dernier reboot. Normalement c'est bon.

