---
layout: default
title: "Windows : notes, tips, tricks, trucs et astuces diverses"
#math: mathjax
date: 2017-01-14 21:08:00
last_modified_date: 2023-11-25 15:25:42
---

# Windows : notes, tips, tricks, trucs et astuces diverses

## Introduction

Je regroupe ici un ensemble de notes diverses à propos de Windows 10 & 11. Les notes sont classées par ordre alphabétique. Sinon faites CTRL+F et tapez un mot clé : "Mise en veille" ou "ISO" par exemple.

Il est possible qu'avec les futures mises à jour de Windows 10, certaines boîtes de dialogue changent ou que la mise en page évolue. Cela dit vous devriez vous y retrouver. Si je fais des mises à jour importantes je l'indiquerai clairement.
Si vous cherchez des raccourcis clavier Windows 10, allez plutôt sur cette [page](https://www.40tude.fr/win10-raccourcis-clavier/).

## Application en mode admin

* WIN + R

* Saisir le nom de l'application (regedit par exemple)

* CTRL + SHIFT + ENTER

## Bash Shell (Linux)

### Mise à jour Déc 2022

Lire cette [page](https://devblogs.microsoft.com/commandline/the-windows-subsystem-for-linux-in-the-microsoft-store-is-now-generally-available-on-windows-10-and-11/) puis aller sur le [Windows Store](https://apps.microsoft.com/store/detail/windows-subsystem-for-linux/9P9TQF7MRM4R?hl=fr-fr&gl=fr)

À partir du prompt de la console, pour retrouver son répertoire d'utilisateur Windows il faut taper un truc du style :

```
cd /mnt/c/Users/phili/
```

À partir du prompt PowerShell, retrouver son répertoire d'utilisateur Linux.

```
cd C:\Users\phili\AppData\Local\lxss\home\philippe
```

Le répertoire "lxss" n’apparaît pas du tout dans l'explorateur de fichier (même si les fichiers cachés sont censés être visibles). On peut bien sûr saisir le chemin à la main dans la barre de chemin (voir la barre en haut de la fenêtre ci-dessous)

<div align="center">
<img src="./assets/lxss.webp" alt="" width="900" loading="lazy"/>
</div>

### Notes

* Lire cette [page](https://www.howtogeek.com/249966/how-to-install-and-use-the-linux-bash-shell-on-windows-10/) pour plus d'information (police etc.)

* Voir cette [vidéo](https://channel9.msdn.com/Events/Ignite/Australia-2017/WIN321b)

## Gestionnaire de tâches

* CTRL + SHIFT + Esc

## Menu contextuel long

### Nov 2023 (WIN11 23H2)

* Regedit

* **HKEY_CURRENT_USER\SOFTWARE\CLASSES\CLSID**

* Nouvelle clé
  + **86ca1aa0-34aa-4e8b-a509-50c905bae2a2**

  + Nouvelle clé
    - **InprocServer32**

    - Dans la fenêtre, double-cliquer sur "par défaut".

    - Aucune modification. OK pour appliquer une valeur par défaut

* Redémarrer

## Mise à jour avec une image ISO

Je suis sous Windows 10. Si il s'agit de faire la mise à jour tout en préservant ses fichiers, on peut appliquer la méthode suivante :

* Télécharger l'image ISO

* La mettre dans le répertoire "Téléchargements" par exemple

* Click droit sur le nom du fichier et choisir "Monter"

* Double click sur le "Setup.exe"

* Penser à bien préciser de ne pas tout supprimer mais de garder les users en place

* Window va redémarrer plusieur fois mais tout va bien se passer

* Pour vérifier à la fin la version de Window
  + Win + R

  + Taper "winver" puis ENTER

Pour refaire une installation complète à partir d'une clé USB, [consultez cette page](https://www.40tude.fr/how-to-install-windows-11-with-a-usb-key/).

## Mise en veille : déterminer ce qui empêche la mise en veille

2. Win + X

6. PowerShell (admin)

10. Saisir

```
 powercfg /requests
```

En ce qui me concerne voilà ce que je peux voir :

<div align="center">
<img src="./assets/sleep.webp" alt="" loading="lazy"/>
</div>


Dans ce cas très précis, saisir :

```
powercfg -requestsoverride DRIVER "srvnet" SYSTEM
powercfg -requestsoverride DRIVER "\FileSystem\srvnet" SYSTEM
```

Je "pense" mais je ne suis pas sûr à 100% que seule la première ligne est suffisante.

Si la raison est différente, faire une recherche sur le service/application qui pose problème.

## Mise en veille : faire apparaître l'option "Mise en veille prolongée"

2. Win + X

6. Options d'alimentation

10. Clic sur "Choisir l'action des boutons d'alimentation"

14. Clic sur "Modifier des paramètres actuellement non disponibles"

18. Cocher "Veille prolongée"

22. Ensuite, voilà ce que je choisis comme action

<div align="center">
<img src="./assets/boutons.webp" alt="" loading="lazy"/>
</div>

### Notes rapides

2. Quand j'appuie sur le bouton On/Off ou sur le bouton de mise en veille (Fn + F1 sur mon portable) je veux être sûr de ne rien perdre donc : "Mise en veille Prolongée".

6. Entre deux réunions, je rabats le capot. Une simple mise en veille est suffisante car je sais que je vais le rouvrir bientôt.

## Mise en veille prolongée

### Mise à jour Déc 2022

Il faut absolument aller vois cette vidéo

https://youtu.be/OHKKcd3sx2c

À **retenir** quand on utilise un portable :

2. Je débranche

6. Je rabats

Par exemple mon Surface Book ne supporte que le mode S0

<div align="center">
<img src="./assets/image-1.webp" alt="" width="900" loading="lazy"/>
</div>


Finalement, sin Win10 22H2, voilà mes paramètres actuels. Je garde en tête qu'en mode "veille prolongée" le PC est toujours susceptible de répondre à des solicitations issues du réseau (téléchargement de mises à jour...) et qu'il faut vraiment l'éteindre si on ne veut pas que la batterie se vide.

<div align="center">
<img src="./assets/image-2.webp" alt="" width="900" loading="lazy"/>
</div>


Pour info on voit que par rapport à ce que je pouvais avoir avec d'autres portables, j'ai pas le modi hybride etc.

<div align="center">
<img src="./assets/image-3.webp" alt="" loading="lazy"/>
</div>


Le truc à retenir c'est qu'il faut : être vigilant, prendre le temps d'aller voir la vidéo, mettre un pouce en l'air, espérer que Microsoft et les constructeurs de PC corrigent le tir.

### Texte original

Pour qu'en mon absence, le portable ce mette tout seul en veille prolongée au bout de 10 min.

2. Win + X

6. Options d'alimentation

10. Paramètres d'alimentation supplémentaires

14. Clic sur "Modifier les paramètres du mode"

18. Clic sur "Modifier les paramètres du mode avancés"

<div align="center">
<img src="./assets/OptionsDalim.webp" alt="" loading="lazy"/>
</div>

### Explications rapides

2. La veille hybride concerne les PC continuellement alimentés. Windows stocke en mémoire et sur disque les applications et les documents ouverts. Il met ensuite le PC dans un mode où il coupe presque tout mais où la mémoire reste alimentée. En cas de coupure de courant on retrouve normalement sa session.

6. La veille classique consomme trop. Fichiers et documents sont en mémoire. Si on "oublie" le PC, quand il n'y a plus de batterie on perd tout.

10. Le veille prolongée vide la mémoire sur disque et éteint le portable. C'est ce que je veux. Je met donc :
    2. Mise en veille classique à  "Jamais"

    6. Je désactive le veille hybride afin d'autoriser la veille prolongée

    10. Je met 10 min. pour la veille prolongée

14. Clic sur "Appliquer" puis sur "OK"

## OneDrive : supprimer le dossier dupliqué dans Explorer

2. Win + R

6. Taper "regedit" puis Enter.

10. Aller dans HKEY_CURRENT_USER

14. SOFTWARE

18. Microsoft

22. Windows

26. CurrentVersion

30. Explorer

34. Desktop

38. NameSpace

42. Cliquer sur les différentes entrées sous NameSpace (long code hexa). Lire dans la partie droite le nom du dossier.

46. Quand le coupable est trouvé, clic droit sur le code hexa et choisir "Supprimer"

50. Explorer est mi à jour instantanément

## OneDrive : éviter d'enregistrer dessus par défaut

2. Clic droit sur l'icone OneDrive

6. Paramètres

10. Onglet Enregistrement Automatique

14. Choisir "Ce PC uniquement"

## Point de restauration : appliquer

* Win + R puis saisir "RSTRUI". On peut aussi faire Win + S, taper "Restauration"

* Cliquer sur le bouton "Restauration du système..."

* Cliquer sur "Suivant>"

* Choisir le nom de la restauration à appliquer

## Point de restauration : créer

**Rappel** : on ne peut pas créer plus d'un point de restauration par jour.

* Win + S

* Taper "Restauration"

* Si besoin cliquer sur "Configurer" pour activer la protection

* Cliquer sur le bouton radio "Activer la protection du système"

* Je décide pour l'instant que la protection pourra utiliser jusqu'à 25% du disque

<div align="center">
<img src="./assets/ConfigRestauration.webp" alt="" loading="lazy"/>
</div>


* Cliquer sur Appliquer puis sur OK

* Cliquer alors sur le bouton "Créer..." de la boîte de dialogue "Propriétés système"

* Saisir une description. Exemple : "Restauration avant installation de XYZ"

* Cliquer sur le bouton "Créer"

Aller voir cette [page](http://winaero.com/blog/restore-point-windows-10-powershell/) pour le faire dans PowerShell.

Pour voir où sont les fichiers de restauration sur le disques dur

* Win + E

* Clic sur "Affichage"

* Puis clic sur "Options" (tout à droite)

* Clic sur l'onglet "Affichage"

* Décocher la case "Masquer les fichiers protégés du système d'exploitation"

* Clic sur le bouton "Oui" de la boite de dialogue

* Clic sur le bouton "OK"

* Aller sur c:\

* Double clic sur le répertoire "System Volume Information"

* Malheureusement même quand on le voit on ne peut pas l'ouvrir (même l'admin)

* Bon allez on referme tout (Affichage/Options/Affichage/Masquer)

## Point de restauration : supprimer

* Win + S

* Taper "Nettoyage"

<div align="center">
<img src="./assets/Nettoyage.webp" alt="" loading="lazy"/>
</div>


* Onglet "Autres options"

* Section "Restauration du système et clichés instantanés"

* Cliquer sur "Nettoyer...". Tous les points de restauration seront supprimés sauf le dernier

<div align="center">
<img src="./assets/Restauration-2.webp" alt="" loading="lazy"/>
</div>

## PowerShell : Power User Menu

2. Clic droit dans la barre de tâche

6. Choisir Paramètres

10. Choisir l'option qui va bien. Elle commence par "Remplacer Invite de commande..."

14. Ensuite avec Win + X on a les options PowerShell et Powershel (admin) dans le menu

Note : Penser à ouvrir de suite une console en mode Admin et à taper :

```
Update-Help
```

Ensuite c'est plus sympa (qu'on soit Admin ou pas) quand on cherche de l'aide sur une commande. Exemple : help ls

## PowerShell : autoriser les scripts

Entre autres, celui le script démarrage

2. Win + X

6. Console PowerShell en mode Admin

10. Saisir

```
Set-ExecutionPolicy RemoteSigned
```

2. Taper 'T'

## PowerShell : créer un script de démarrage

Pour faire démarrer PowerShell dans un répertoire particulier ou bien lancer le profile posh-git par exemple

2. Win + X

6. Choisir PowerShell

10. Taper :
    ```
    New-Item -path $profile -type file –force
    ```

14. Aller dans  C:\Users\xyz\Documents\WindowsPowerShell

18. Modifier le contenu du fichier Microsoft.PowerShell_profile.ps1

Pour modifier le répertoire on peut taper :

```
Set-location .\Documents\
```

## PowerShell : démarrer dans un répertoire particulier

2. Dans $HOME$/Documents/WindowsPowerShell

6. Editer le ficher Microsoft.PowerShell_profile.ps1 comme suit par exemple

```
# default directory
Set-location $HOME\Documents\
```

Pour prendre en compte le nouveau profile, pas besoin de fermer le PowerShell. Il suffit de taper **.$PROFILE** sur la ligne de commande.

## Power User Menu

* Win + x

## Raccourcis

Voir cette page pour quelques [raccourcis clavier](https://www.40tude.fr/win10-raccourcis-clavier/) utiles.

## Tâches au démarrage

Pour désactiver Adobe Acrobat Update Service par exemple

2. Win +R

6. Taper **msconfig**

10. Aller dans l'onglet Services

14. Décocher qui vous voulez!

## Tâches programmées

Pour désactiver CompatTelRunner.exe par exemple

2. Win + R

6. Taper **taskschd.msc**

10. Rechercher Microsoft Compatibility Appraiser (\Microsoft\Windows\Application Experience)

14. Click droit

18. Disable

## Test micro

### Nov 2023 (WIN11 23H2)

* https://fr.mictests.com/

* https://www.malekal.com/le-micro-non-detecte-windows-11/

* WIN + R

* mmsys.cpl

## Utilisateurs : gestion

Pour supprimer defaultuser0 par exemple.

2. Win +R

6. Taper **lusrmgr.msc**

10. Supprimer **defaultuser0**

## Variable d'environnement PATH

2. Win + Pause

6. Clic sur "Paramètres Système Avancés" (liste à gauche, tout en bas)

10. Clic sur bouton "Variables d'environnement" (tout en bas)

14. Double clic sur "PATH" dans la liste

18. Clic sur "OK", "OK", "OK"

22. Fermer et relancer la console Powershell (ou la console)

<div align="center">
<img src="./assets/Var-Envi.webp" alt="" loading="lazy"/>
</div>

## Version courante

Pour vérifier le N° de la version installée

2. Win + I

6. Système

10. Information Système (tout en bas à gauche)

14. On accède au numéro de Version

Retrouver la version en cours : <https://technet.microsoft.com/fr-fr/windows/release-info.aspx>

## VMWare : Problème de résolution écran sur Surface Pro

La résolution du Surface Pro est très grande et les machines virtuelles Debian ou Ubuntu se retrouvent avec des polices toutes petites. C'est très désagréable.

2. Touche Win

6. Cliquer droit sur l'icône VMWare Worksation 12

10. Choisir Plus/Emplacement du fichier

14. Cliquer droit et choisir Propriétés

18. Onglet Compatibilité

22. Cocher "Remplacez le comportement de mise à l'échelle..."

26. Choisir "Système amélioré" dans la liste

<div align="center">
<img src="./assets/PourVMWare.webp" alt="" width="900" loading="lazy"/>
</div>

## Windows Old : supprimer

2. Win + S

6. Taper "Nettoyage"

10. Bouton "Nettoyer les fichiers système"

14. Cocher "Précédente(s) installation(s) de Windows"

18. Cliquer sur "Ok" puis confirmer en cliquant sur "Supprimer les fichiers"

<div align="center">
<img src="./assets/Nettoyage_2017_01.webp" alt="" loading="lazy"/>
</div>

<div align="center">
<img src="./assets/Nettoyage_2017_01.webp" alt="" loading="lazy"/>
</div>
