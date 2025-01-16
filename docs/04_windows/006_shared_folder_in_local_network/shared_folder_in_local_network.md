---
layout: default
title: "Shared folders in local network"
parent: "Windows"
#math: mathjax
date: 2023-01-09 11:59:50
last_modified_date: 2023-01-12 00:20:10
---

# Shared folders in local network

## Introduction
In a personal context, we have several networked PCs. We want to implement Public directories on each machine to be able to share documents without having to go through the NAS, the cloud, or emails. PCs run Win11 22H2, Win10 22H2 and Linux

## Check that the network is private - PowerShell

* Open an elevated Terminal (AKA run as **Administrator)**
* **WIN + X,  I**
  + This means press 'WIN' key and 'x' key simultaneously. Then press 'i' key.
  + I will not list that step in the next PowerShell sections.
* Get-NetConnectionProfile
* Note the InterfaceIndex (third line).
  + It will be useful if you need to set the network private

```
PS C:\Users\phili> Get-NetConnectionProfile

Name                     : Livebox-AAE0
InterfaceAlias           : Ethernet
InterfaceIndex           : 21
NetworkCategory          : Private
DomainAuthenticationKind : None
IPv4Connectivity         : Internet
IPv6Connectivity         : Internet
```



## Check that the network is private - Win11

* **WIN + I**
  + This means press 'WIN' key and 'i' key simultaneously

<div align="center">
<img src="./assets/image-40.webp" alt="" width="900" loading="lazy"/>
</div>


* Network and Internet (on the left hand side)
* Properties

<div align="center">
<img src="./assets/image-41.webp" alt="" width="900" loading="lazy"/>
</div>





## Check that the network is private - Win10

* **WIN + I**
  + This means press 'WIN' key and 'i' key simultaneously

<div align="center">
<img src="./assets/Win10-System.webp" alt="" width="900" loading="lazy"/>
</div>


* Network and Internet
* Check the status of the network at the top of the window

<div align="center">
<img src="./assets/image-47_1.webp" alt="" width="900" loading="lazy"/>
</div>




## 2 - Set the network to private - PowerShell

* If needed

```
PS C:\Users\phili> Set-NetConnectionProfile -InterfaceIndex 21 -NetworkCategory Private
```

* Double check the updated network category calling Get-NetConnectionProfile.

## 3.1 - Set the sharing and discovery option correctly - Win11

* Reach the Network and Internet settings
  + **WIN + I**
  + Network and Internet
* Select Advanced parameters

<div align="center">
<img src="./assets/image-42.webp" alt="" width="900" loading="lazy"/>
</div>


* Select Advanced Sharing Parameters

<div align="center">
<img src="./assets/image-43.webp" alt="" width="900" loading="lazy"/>
</div>


There are 3 sections on the next page. Unfold each of them and copy the setting here-below.

### 1 - Private Networks

<div align="center">
<img src="./assets/image-44.webp" alt="" width="900" loading="lazy"/>
</div>


Network discovery is mandatory since we want our PC to be visible for the other PC.

### 2 - Publics Networks

<div align="center">
<img src="./assets/image-45.webp" alt="" width="900" loading="lazy"/>
</div>


Nothing is shared on public network.

### 3 - All Networks

<div align="center">
<img src="./assets/image-46.webp" alt="" width="900" loading="lazy"/>
</div>


The last parameter should be set if and only if you want to ask the users to enter their Id and Password otherwise leave it unset. In my case I do not set the parameter since the public directories are supposed to be shared between all the members of the private network. If we want to share specific directory with specific people, we can easily, do it with others means.




## Set the sharing and discovery option correctly - Win10

* Reach the Network and Internet settings
  + **WIN + I**
  + Network and Internet
* Select Network and Sharing

<div align="center">
<img src="./assets/Win10-Partage-1.webp" alt="" width="900" loading="lazy"/>
</div>


* Left hand side
* Select Modify advanced sharing parameters

<div align="center">
<img src="./assets/Win10-Centre-Reseau.webp" alt="" width="900" loading="lazy"/>
</div>


There are 3 sections on the next page. Unfold each of them and copy the setting here-below.

### 1 - Private Networks

<div align="center">
<img src="./assets/Win10-Private.webp" alt="" width="900" loading="lazy"/>
</div>


Network discovery is mandatory since we want our PC to be visible for the other PC.

### 2 - Public Networks

<div align="center">
<img src="./assets/Win10-Public.webp" alt="" width="900" loading="lazy"/>
</div>


Nothing is shared on public network.

### 3 - All Networks

<div align="center">
<img src="./assets/Win10-All.webp" alt="" width="900" loading="lazy"/>
</div>


The last parameter should be set if and only if you want to ask the users to enter their Id and Password otherwise leave it unset. In my case I do not set the parameter since the public directories are supposed to be shared between all the members of the private network. If we want to share specific directory with specific people, we can easily, do it with others means.











## Use cases

Repeat the process on the other PC and you should be good to go.

In the example below, SURF-PHILIPPE is a remote PC and 5PRO-PHILIPPE is my local machine.

First, note that there is a 'Public' directory under Users in your local machine

<div align="center">
<img src="./assets/image-48.webp" alt="" width="900" loading="lazy"/>
</div>


The 'Public' directory is pre-populated with sub-directories whose names help to dispatch the kind of documents to be shared. Obviously, there is no restriction, and you can add your own subdirectories.

<div align="center">
<img src="./assets/image-49.webp" alt="" width="900" loading="lazy"/>
</div>


Now, if I browse the local network and reach the PC named SURF-PHILIPPE. I'm asked an ID and password. I type one letter in the ID and click OK. In fact, to access the Public directory, the ID is not mandatory.

<div align="center">
<img src="./assets/image-51.webp" alt="" loading="lazy"/>
</div>


I can now reach the 'Users' directory...

<div align="center">
<img src="./assets/image-47.webp" alt="" width="900" loading="lazy"/>
</div>


and I the 'Public' sub-directory. For example, I can visit the Public Documents sub-directory.

<div align="center">
<img src="./assets/image-50.webp" alt="" width="900" loading="lazy"/>
</div>



## Linux

To learn more about the use case with a Linux remote PC you should [read that page]({%link docs/05_linux/006_shared_folder_in_windows_home_network/shared_folder_in_windows_home_network.md%}).

