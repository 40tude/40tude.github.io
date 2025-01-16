---
layout: default
title: "Les fonctions les plus utilisées dans les exemples CVI"
parent: "ANSI C"
#math: mathjax
date: 2013-12-26 00:14:15
last_modified_date: 2020-05-03 18:05:39
---


# Les fonctions les plus utilisées dans les exemples CVI

J'ai eu l'occasion de faire une analyse de tous les codes source des exemples qui viennent avec [CVI](https://www.ni.com/cvi) 2013. En fait je souhaitais établir la liste des fonctions les plus utilisées.

Il a fallu que je m'y reprenne à deux fois mais cette fois je crois disposer d'un résultat fiable. En effet au départ j'avais des problèmes avec les codes qui utilisaient le SDK ou .NET. Je ne faisais alors l'analyse que de 152 codes sources sur 360 disponibles. Dans un deuxième temps je tiens compte de pratiquement tous les fichiers sources. J'ai encore des problèmes avec 17 d'entre eux. Ce sont notamment des problèmes de header manquants je crois... 17 sources sur 360, je ne vais pas perdre de temps là-dessus.

Quoiqu'il en soit, en tenant compte des sources disponibles dans les répertoires ActiveX, .NET et SDK je me retrouve à analyser des sources du style ..\National Instruments\CVI2013\samples\dotnet\mscorlib2.c qui fait 16 Mo! C'est un peu ridicule, bourré de fonctions et à mon avis ça trouble les résultats.

Suite comparaison des deux listes des noms de fonctions des analyses "complète" et "un peu moins complète", mon sentiment est qu'on a une très bonne idée du Top 50 des fonctions les plus utilisées dans les exemples CVI si on s'intéresse aux fonctions de la liste suivante. Cette liste comprend 1 486 fonctions différentes et chacune est plus ou moins invoquée souvent. Par exemple, la plus utilisée c'est SetCtrlAttribute qui est invoquée 906 fois. Pour finir, sur un total de 11 485 appels de fonctions recensés, les fonctions suivantes sont appelées 6 530 fois. Elles représentent plus de 50% des appels de fonctions :

```
SetCtrlAttribute
GetCtrlVal@12
SetCtrlVal
sprintf
free
MessagePopup@8
DiscardPanel@4
LoadPanel@12
DisplayPanel@4
QuitUserInterface@4
InitCVIRTEEx@12
RunUserInterface@0
MakePoint@8
strcpy
malloc
GetCtrlAttribute@16
rand
OGLSetCtrlAttribute
DeleteGraphPlot@16
SetWaitCursor@4
GetPanelAttribute@12
SetPanelAttribute
MakeRect@16
CloseCVIRTE@0
PlotY@40
SetMenuBarAttribute
InsertListItem
ColorPicker_SetAttribute
printf
Fmt
Timer@0
MakeColor@12
strcat
strlen
calloc
memset
GenerateGraphs
SetTableCellAttribute
SetInputMode@12
fprintf
sin
GetGeneralErrorString@4
_GetFilesArray
InstallPopup@4
strcmp
ActiveIconFile
SetTreeItemAttribute
SetAxisScalingMode@32
Delay@8
HideBuiltInCtrlMenuItem@12
```

Je vous conseille donc de jeter votre œil. Si une fonction est inconnue, faites **CTRL + SHFT + P** dans l'éditeur de code de CVI, retrouvez-la et lisez la doc. Oui, oui bien sûr il ne faut pas mettre l'@ ni le chiffre s'il y en a un. Attention, je pense que la fonction _GetFilesArray est une fonction cachée.

{: .note }
Oubliez la fonction ``Fmt``. C'est un veau qui n'avance pas qui date de LabWindows/DOS (oui oui j'ai bien dit [MS-DOS](https://fr.wikipedia.org/wiki/MS-DOS) comme Disk Operating System)

Pour être complet voilà le Top 50 des fonctions utilisées lorsque l'on tient compte des exemples .NET, SDK, ActiveX etc. Fondamentalement il n'y a pas de grosses différences à part le nombre de fonctions .NET qui apparaissent. Attention, elles sont sur-représentées à cause des fichiers ``mscorlib2.c`` et ``mscorlib.c``

```
CDotNetInvokeGenericMember@56
_CDotNetGenDisposeScalar
CDotNetDiscardHandle@4
CA_MethodInvokeEx
CDotNetInvokeGenericStaticMember@60
CDotNetCreateGenericInstance@40
CDotNetInvokeGenericInterfaceMember@64
CDotNetCreateArray@24
_CDotNetGenDisposeArray
CDotNetGetArrayLength@12
CDotNetGetArrayElements@16
SetCtrlAttribute
GetCtrlVal@12
CDotNetLoadAssembly@8
CDotNetDiscardAssemblyHandle@4
SetCtrlVal
CDotNetGetEnumValue@8
CDotNetCreateTypedArray@28
MessagePopup@8
OGLSetCtrlAttribute
CA_DiscardObjHandle@4
sprintf
CA_RegisterEventCallback@28
CA_GetEventCallback@20
CDotNetCreateInstance@28
CA_GetInterfaceFromObjHandle@20
System_Reflection_BindingFlags__Create@12
free
CA_FillErrorInfoEx@20
CA_DefaultValueVariant@0
DiscardPanel@4
LoadPanel@12
CA_InvokeHelperEx
DisplayPanel@4
SetWaitCursor@4
QuitUserInterface@4
InitCVIRTEEx@12
RunUserInterface@0
CA_FreeMemory@4
CA_CreateObjHandleFromInterface@28
strcpy
MYPrintf
GetCtrlAttribute@16
CDotNetFreeMemory@4
MakePoint@8
malloc
CA_VariantClear@4
CDotNetGetEnumTypeFromName@12
CDotNetGetTypeFromName@16
strlen
```

