---
layout: default
lang: en-US
title: "Finalize Windows 11 installation"
parent: "Windows"
nav_order: 3
#math: mathjax
date: 2023-11-05 11:44:55
last_modified_date: 2023-12-03 15:34:12
---

<div align="center">
<img src="./assets/img01.webp" alt="" width="900" loading="lazy"/>
</div>


# Finalize Windows 11 installation


## Prelude

Read this page first. [Install Windows 11 with a USB key]({% link docs/04_windows/000_install_windows_11_with_a_usb_key/install_windows_11_with_a_usb_key.md %})

* Indeed, below, I assume [Chocolatey](https://chocolatey.org/) is up and running
* Most of the software installations are done in Windows Terminal using CLI

## Installations

### Open an Admin Terminal

```
Set-ExecutionPolicy RemoteSigned
Update-Help
```
### Chrome

```
choco install googlechrome -y
```
### MesloGM Nerd Font

* <https://www.nerdfonts.com/font-downloads>

* Download, unzip, select the fonts, right click, install

* Within Windows Terminal, go to Settings, select MesloGM Nerd Font

### Oh my Posh

```
Set-ExecutionPolicy Bypass -Scope Process -Force; Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://ohmyposh.dev/install.ps1'))
```

Later, for updates of Oh my Posh use the same command

### VSCode

```
choco install vscode -y
```
### Git

```
choco install git -y
Update-SessionEnvironment                   # update environment variables
git config --global user.name "Xxxx YYYYY"
git config --global user.email xxx.yyy@gmail.com
git config --global core.editor "code --wait"
git config --list
```
### Linux Sub System

```
wsl --install
```

* Reboot the host

### Powershell (!=Windows Powershell)

```
choco install powershell-core -y
Update-Help
```

* In order to see the Oh My Posh prompt, I had to copy the content of `Microsoft.PowerShell_profile.ps1` into `profile.ps1`

* In %USERPROFILE%/Documents check `PowerShell` and `WindowsPowerShell` directories

### Powertoys

```
choco install powertoys -y
```

* Reboot the host

### Docker

```
choco install docker-desktop -y
Update-SessionEnvironment
```
### Anaconda

* <https://www.anaconda.com/download>

* Download, install

* Open Anaconda Powershell Prompt

```
conda update conda
conda update -n base --all
```

* [Read this page](`NOT YET TRANSFERED`) to add Anaconda Prompt within Win Terminal

### Microsoft365

* Connect to your Microsoft account

* Select "Install Software"

* OfficeSetup.exe...

### Rufus

```
choco install rufus.portable -y
```

* When needed, type `rufusp` in a terminal or **WIN + R** then `rufusp`

* If needed, visit : C:\ProgramData\chocolatey\bin

* <https://github.com/pbatard/rufus>

### Adobe Acrobat Reader DC => Sumatra

```
choco install sumatrapdf.portable -y
```

* Edge or Chrome might be OK

* Install Sumatra Portable Version
  + When needed, type `sumatrapdf-64` in a terminal or **WIN + R** then `sumatrapdf-64`

  + If needed, visit : C:\ProgramData\chocolatey\bin

  + <https://github.com/sumatrapdfreader/sumatrapdf>

### Choco survival guide

* Update-SessionEnvironment comes with chocolatey

```
choco search vscode
choco install vscode -y
choco list
choco upgrade all
choco uninstall vscode
```
## TO DO

```
choco install filezilla -y
choco install gpu-z -y
choco install cpu-z.portable -y
```

* Anaconda or miniconda 

## What is next ?

* When possible install software using choco
* Select [portable version](https://en.wikipedia.org/wiki/Portable_application) of the software when it is available

