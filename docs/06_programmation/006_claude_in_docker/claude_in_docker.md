---
published: true
author: 40tude
lang: en-US
layout: default
title: "Running Claude Code in a Docker Linux Container (Windows 11 Host)"
description: "Hardening Your AI Workflow: Containerizing Claude Code to Protect Your Host System"
image: docs\06_programmation\006_claude_in_docker\assets\img00.webp
twitter:
  card: summary_large_image
parent: "Maths"
# nav_order: 36
math: mathjax
date:               2026-03-29 12:00:00
last_modified_date: 2026-03-29 12:00:00
---



# Running Claude Code in a Docker Linux Container (Windows 11 Host)
{: .no_toc }

Hardening Your AI Workflow: Containerizing Claude Code to Protect Your Host System
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }




<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
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




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Prerequisites

- Windows 11
- VSCode
- Docker Desktop for Windows installed, updated and running
    - `winget install -e --id Docker.DockerDesktop`
- An Anthropic account and a plan (PRO foo example)
    - Optionally an API key (from [console.anthropic.com](https://console.anthropic.com))


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 1 — Create  project folder (PowerShell)

`Win + X + I` to open a terminal

```powershell
cd $env:tmp
New-Item -ItemType Directory -Path hello_uv
cd hello_uv
```


At this point you can load VSCode from the current directory (`code .`) or continue to use the terminal.







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 2 — Create the Image

Create a `make_image/Dockerfile` file

```powershell
New-Item -ItemType Directory -Path make_image # create the `make_image` folder
cd make_image
New-Item Dockerfile # create the file
```

Copy the content below in `Dockerfile`

```dockerfile
FROM node:22-slim

RUN apt-get update && apt-get install -y \
    curl git ripgrep ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp
RUN curl -fsSL https://claude.ai/install.sh | bash
ENV PATH="/root/.local/bin:$PATH"

RUN curl -LsSf https://astral.sh/uv/install.sh | sh

# Pre-install Python 3.12 and set as default
RUN /root/.local/bin/uv python install cpython-3.12
RUN echo "3.12" > /root/.python-version

# Skip the onboarding wizard on every start
RUN echo '{"hasCompletedOnboarding":true,"installMethod":"native"}' > /root/.claude.json

WORKDIR /workspace
CMD ["bash"]
```


#### **Note**
{: .no_toc }

To create an image with rust one can use the Dockerfile below:

```dockerfile
FROM node:22-slim

RUN apt-get update && apt-get install -y \
    curl git ripgrep ca-certificates \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp
RUN curl -fsSL https://claude.ai/install.sh | bash
ENV PATH="/root/.local/bin:$PATH"

RUN curl -LsSf https://astral.sh/uv/install.sh | sh

# Install Rust (latest stable via rustup)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Skip the onboarding wizard on every start
RUN echo '{"hasCompletedOnboarding":true,"installMethod":"native"}' > /root/.claude.json

WORKDIR /workspace
CMD ["bash"]

```



Build the image
    - In VSCode, you can open a terminal (`CTRL+ù`) and `cd make_image`




```powershell
docker build -t claude-uv:latest .
# docker build --no-cache -t claude-uv:latest .       # may help
```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img02.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>


The images should be available in Docker



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img01.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 3 — If PRO PLAN. Start the container with the project folder mounted


Reach the root of the folder and run the image.


```powershell
cd ..
docker run --rm -it `
  --name claude-dev `
  -v "${PWD}:/workspace" `
  -v "${HOME}\.claude_docker:/root/.claude" `
  -w /workspace `
  claude-uv:latest `
  bash
```

#### **Note:**
{: .no_toc }

* `-v`:
* `-v`:
* `-w`: set current directory
* Run the latest version of `claude-uv` image
* `bash`:


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img03.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>



Check every thing works as expected

```bash
uv --version
claude --version
claude
```



<!-- Follow the instruction. Below select option 1 (you have a plan)

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img04.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>



Copy URL (press `c`) and paste in your favorite browser

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img05.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>


Copy the code from the browser

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img06.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>


Paste the code (CTRL+V) in Claude

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img07.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>


You are good to go

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img08.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>





 -->


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img14.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 3 bis — If USING TOKENS. Start the container with the project folder mounted



We use a secret key so create a `.env` file and paste the code you get from `https://platform.claude.com/settings/keys`


```text
ANTHROPIC_API_KEY=sk-ant-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

Even if you do not plan to push the project on GitHUb today, create a `.gitignore` file and add the `.env` file. You never know...


```text
.env
```

Reach the root of the project folder and run the image

```powershell
cd ..
docker run --rm -it `
  --name claude-dev `
  -v "${PWD}:/workspace" `
  -v "${HOME}\.claude_docker:/root/.claude" `
  -w /workspace `
  claude-uv:latest `
  bash
```






<!--


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img04.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>

 -->
































<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 5 — Ask Claude Code to create a `uv`-based Hello World project


Once Claude Code's interactive session is open, type the following prompt:

```
Create a demo folder
In that folder create a Python "hello world" project using uv.
Initialize it with `uv init`, create a main.py that prints "Hello, World!", and run it with `uv run`.
```

Claude Code will:
1. Run `uv init` to scaffold the project
2. Create (or edit) `main.py` with a `print("Hello, World!")` statement
3. Execute `uv run main.py` and show you the output




<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img10.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>


All generated files land in `/workspace/demo` inside the container, which maps back to `tmp\hello_uv\demo` on our Windows host (**fully persisted**).


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img13.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>

















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 6 — Open a terminal and connect to the running image

Open a new terminal in VSCode (`CTRL+SHIFT+ù`)
```bash
docker ps

```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img11.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>


```powershell
docker exec -it claude-dev bash

```

Once in the Linux context run the python script

```bash
uv run demo/main.py

```


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img12.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>



Since VSCode is open you can modify the Python script OR switch to the Claude terminal and continue.

Close everything
* Terminal 1: `/exit` CLAUDE then `exit` the image
* Terminal 2: `exit` the image
* Close VSCode




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 7 — Tomorrow morning

Open the folder in VSCode

Open a terminal in VSCode (at the root of the folder)

Run the command

```powershell
docker run --rm -it `
  --name claude-dev `
  -v "${PWD}:/workspace" `
  -v "${HOME}\.claude_docker:/root/.claude" `
  -w /workspace `
  claude-uv:latest `
  bash
```


Once in the image, run the app

```bash
uv run demo/main.py
```


Call Claude

```bash
claude

```

Open a second terminal (CTRL+SHIFT+ù) to the running image

```powershell
docker exec -it claude-dev bash
```






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 8 — A new project


```powershell
cd $env:tmp
New-Item -ItemType Directory -Path hello_uv
cd hello_uv2
```

* Open the folder with Code
* Open a terminal

```powershell
docker run --rm -it `
  --name claude-dev `
  -v "${PWD}:/workspace" `
  -v "${HOME}\.claude_docker:/root/.claude" `
  -w /workspace `
  claude-uv:latest `
  bash
```


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img15.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>

```
Create with uv a Python application that ask for an integer, check it is positive, report an error if not and generate the Syracuse series otherwise .
Initialize the project with `uv init`, and run it with `uv run`.
```

Open a new terminal in VSCode (`CTRL+SHIFT+ù`)

```powershell
docker exec -it claude-dev bash

```

Run the app

```bash
uv run syracuse/main.py
```


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img16.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion
