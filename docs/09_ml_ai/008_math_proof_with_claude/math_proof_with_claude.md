---
published: false
author: 40tude
# lang: fr
lang: en-US
layout: default
title: "TODO: Add a title"
description: "TODO: Add a description"
image: docs/09_ml_ai/008_math_proof_with_claude/assets/img00.webp
twitter:
  card: summary_large_image
parent: "ML & AI"
# nav_order: 36
math: mathjax
date:               2026-11-21 15:27:39
# last_modified_date is updated by tools/hooks/pre-commit
last_modified_date: 2026-11-21 18:55:02
---



# {{ page.title }}
{: .no_toc }

{{ page.description }}
{: .lead }





<h2 align="center"><span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span></h2>

<!--
Le prompt
La formalisation
Validation par Lean
  SandBox
  Installer pwsh : iex "& { $(irm https://aka.ms/install-powershell.ps1) } -UseMSI -Quiet"

  # Download the installer script (curl.exe, not the old PowerShell alias)
  curl.exe -O --location https://elan.lean-lang.org/elan-init.ps1

  # Run it, bypassing the script execution policy for this one call
  powershell -ExecutionPolicy Bypass -f elan-init.ps1

  # Clean up
  del elan-init.ps1

  # NEW TERMINAL then
  elan --version
  elan show   # lists installed toolchains

  Installer MinGit
    # Find the latest MinGit 64-bit release on GitHub
    $rel   = Invoke-RestMethod -Uri "https://api.github.com/repos/git-for-windows/git/releases/latest"
    $asset = $rel.assets | Where-Object { $_.name -match '^MinGit-[\d.]+-64-bit\.zip$' } | Select-Object -First 1

    # Download and extract to C:\MinGit
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile "$env:TEMP\mingit.zip"
    Expand-Archive -Path "$env:TEMP\mingit.zip" -DestinationPath "C:\MinGit" -Force

    # Add git to PATH: current session + user PATH (so new windows see it too)
    $env:Path = "C:\MinGit\cmd;$env:Path"
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    [Environment]::SetEnvironmentVariable("Path", "C:\MinGit\cmd;$userPath", "User")

    git --version


  lake new preuves math
  cd preuves
  lake exe cache get   # downloads precompiled Mathlib (~ a few GB, minutes instead of hours)

  Copier le fichier dans C:\Users\WDAGUtilityAccount\preuves\Preuves
  edit le fichier C:\Users\WDAGUtilityAccount\preuves\preuves.lean
  ajouter une seconde ligne "import Preuves.EvenInfinite"

  lake build           # compiles your files; no errors = proofs verified
  Corriger les erreurs
  lake build
  Ajuster
  lake build
  Remove-Item .\Preuves\Basic.lean

-->




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* Point 1
* Point 2



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00.webp"
    alt="TODO: Add a description"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>TODO: Add a legend</figcaption>
</figure>










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Introduction

Lorem ipsum dolor sit amet, consectetur adipiscing elit.

<!-- Link to a video -->
<figure style="max-width: 560px; margin: auto;">
<div style="position: relative; padding-bottom: 56.25%; height: 0;">
    <iframe
    src="https://www.youtube.com/embed/MIeYz6aMBbk"
    title="Add a title"
    style="position: absolute; inset: 0; width: 100%; height: 100%;"
    allowfullscreen>
    </iframe>
</div>
<figcaption style="text-align: center;">TODO: Add a legend</figcaption>
</figure>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Level 2 Section

Lorem ipsum dolor sit amet, consectetur adipiscing elit.

<!-- ###################################################################### -->
### Level 3 Section
{: .no_toc }

Lorem ipsum dolor sit amet, consectetur adipiscing elit.


Math inline $$x(t)$$ and, below, math offline:

$$
\frac{dS}{dt} = r \cdot S
$$


<!-- ---------------------------------------------------------------------- -->
#### Comments
Lorem ipsum dolor sit amet, consectetur adipiscing elit.









<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

Lorem ipsum dolor sit amet, consectetur adipiscing elit.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

* [Link to a web page](https://cleonis.nl/physics/phys256/energy_position_equation.php).
* [Link to a page of the site]({%link docs/03_maths/013_gauss_forme_integrale_differentielle/gauss_forme_integrale_et_differentielle.md%})