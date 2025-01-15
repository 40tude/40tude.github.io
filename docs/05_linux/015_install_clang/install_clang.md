---
layout: default
title: "Installer LLVM et Clang sous Debian"
parent: "Linux"
#math: mathjax
date: 2017-02-22 17:05:07
last_modified_date: 2020-05-04 09:10:33
---

# Installer LLVM et Clang sous Debian

* J'ai une machine virtuelle avec une Debian 8.
* Dans le cache de apt-get je ne trouve qu'une version 3.5 de clang (``apt-cache show clang``)
* Je souhaite installer la toute dernière version.
* Je télécharge la version déjà compilée pour Debian 64 de LLVM/Clang (3.9.1 à ce jour) sur llvm.org

Ensuite je fais :

```bash
sudo tar -C /usr/local -xJf clang+llvm-3.9.1-x86_64-linux-gnu-debian8.tar.xz --strip 1
```

* Et zou c'est terminé. 
* Là où c'est plus "merdique" c'est pour désinstaller. Il faut faire :

```bash
sudo rm -v /usr/local/bin/{clang*,llc,lli,llvm*,macho-dump,opt,bugpoint,c-index-test} && \
sudo rm -rfv /usr/local/docs/llvm && \
sudo rm -rfv /usr/local/include/{clang,clang-c,llvm,llvm-c} && \
sudo rm -v /usr/local/share/man/man1/clang.1 && \
sudo rm -rfv /usr/local/lib/clang && \
sudo rm -v /usr/local/lib/{BugpointPasses.so,libclang*,libLLVM*,libLTO*,libprofile_rt*,LLVM*}
```

{: .note }
Pour invoquer clang en tapant clang++ au lieu de clang++-3.9.1 on peut avoir besoin de faire des liens symboliques. Par exemple, à faire dans ``/usr/local/bin`` :

```bash
sudo ln -s /usr/local/bin/clang-3.9.1 /usr/local/bin/clang
sudo ln -s /usr/bin/clang++-3.9.1 /usr/local/bin/clang++
```

