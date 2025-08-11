---
layout: default
lang: en-US
title: "Linux Min 21 - Enable automatic Update"
parent: "Linux"
#math: mathjax
date: 2023-01-06 16:15:28
last_modified_date: 2023-01-10 08:59:30
---

# Linux Min 21 - Enable automatic Update

<div align="center">
<img src="./assets/image-21.webp" alt="" loading="lazy"/>
</div>

## By hand

```bash
sudo apt update
sudo apt dist-upgrade
sudo apt upgrade
```

## Automatic
```bash
sudo apt install unattended-upgrades
dpkg-reconfigure --priority=low unattended-upgrades
```

Press **Yes** on the unique dialogue box you will see.
You're done.

