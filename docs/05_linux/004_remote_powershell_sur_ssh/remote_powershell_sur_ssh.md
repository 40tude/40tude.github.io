---
layout: default
title: "Remote PowerShell sur SSH"
parent: "Linux"
#math: mathjax
date: 2023-02-01 10:49:10
last_modified_date: 2023-02-01 10:57:58
---

# Remote PowerShell sur SSH

# Introduction

Ce billet fait suite à [celui-ci]({%link docs/04_windows/004_remote_powershell_sur_ssh/remote_powershell_sur_ssh.md %}) qui traitait de PowerShell remoting protocol (PSRP) over SSH mais dans un contexte Windows. Là, on va faire la même chose mais avec une machine distante (un serveur) Linux.

Afin d'éviter trop de redites je vous propose de lire [le billet en question]({%link docs/04_windows/004_remote_powershell_sur_ssh/remote_powershell_sur_ssh.md %}). 

Si besoin, n'hésitez pas à lire [cet autre billet]({%link docs/04_windows/003_configurer_remote_powershell_winrm_dans_un_workgroup/configurer_remote_powershell_winrm_dans_un_workgroup.md%}) qui traite de la configuration de remote PowerShell dans un contexte WORKGROUP (à la maison) et dans lequel je parle de WinRM.

Enfin bref, ici on va faire court...

Les coordonnées de la machine distante

* 192.168.1.44
* MINT-PHILIPPE
* PowerShell y est installé. [Lire ce billet]({%link docs/05_linux/003_linux_mint_21_install_powershell/linux_mint_21_install_powershell.md%}) si besoin.

Je suis connecté dessus à distance via XRDP. Lire [ce billet]({%link docs/04_windows/005_windows_remote_access_to_linux_xrdp/remote_access_to_linux_xrdp.md%}) si besoin.





## Sur la machine distante - Linux

### Configuration de SSH

Ouvrir une console et saisir les 2 lignes ci-dessous

```
sudo apt install openssh-client
sudo apt install openssh-client
```

Editer le fichier de configuration du serveur SSH pour, entre autres, y enregistrer PowerShell comme sous-système SSH

```
sudo nano /etc/ssh/sshd_config
```

3 choses à faire

* Enlever le # devant : PasswordAuthentication yes
* Enlever le # devant : PubkeyAuthentication yes
* Ajouter la ligne suivante

```
subsystem powershell /usr/bin/pwsh -sshs -nologo
```

Après avoir sauvegardé le fichier, il faut redémarrer le serveur SSH

```
sudo systemctl restart sshd.service
```


## Sur la machine locale - Win11

### Invoke-Command

<div align="center">
<img src="./assets/image.webp" alt="" width="900" loading="lazy"/>
</div>


### Enter-PSSession

<div align="center">
<img src="./assets/image-1.webp" alt="" width="900" loading="lazy"/>
</div>
