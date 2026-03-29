---
published: true
author: 40tude
lang: en-US
layout: default
title: "Running Claude Code in a Docker Linux Container (Windows 11 Host)"
description: "Hardening Our AI Workflow: Containerizing Claude Code to Protect Our Host System"
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

Hardening Our AI Workflow: Containerizing Claude Code to Protect Our Host System
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ## TL;DR
{: .no_toc } -->




<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00.webp"
    alt="An explosion of fire safely contained inside a glass cube in a sci-fi server room, symbolizing isolated and controlled execution"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Yolo mode, safely contained.</figcaption>
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
Claude Code's `--dangerouslySkipPermissions` flag (a.k.a. "yolo mode") lets Claude act autonomously without asking for confirmation on every file write, shell command, or package install. It is powerful but risky: a runaway agent on our host machine could delete files, corrupt configs, or trash our system.

The fix is simple: run Claude Code inside a Docker Linux container. The container is disposable. If something goes wrong, we kill it and start fresh. Our Windows host stays untouched.

There is a second benefit. Claude Code was built and trained in a Linux environment. It reasons about Linux paths, shell commands, and toolchains more naturally than it does about Windows. Running it in a native Linux container removes a whole class of friction where Claude suggests `apt install`, `curl | bash`, or `#!/usr/bin/env python` and we have to mentally translate.

The workflow is straightforward: build a minimal Docker image with Node, Claude Code, and our toolchain (Python via uv, Rust, etc.), mount our project folder as a volume, and run Claude inside. Our code is persisted on the host, the container itself is ephemeral.

This guide walks through the whole setup on a Windows 11 host: building the image, authenticating Claude (both Pro plan and API key), mounting a project, and running a first agentic session. Two Dockerfiles are provided : one with Python/uv, one adding Rust. By the end we will have a reusable container we can spin up in seconds for any new project.

Ready? Let's dive in!






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Why Use Docker with Claude?

Running Claude inside a Docker container is not just a convenience—it fundamentally changes how we interact with AI-assisted coding by making it safer, more reproducible, and easier to control.

### Isolation and Safety
{: .no_toc }

Claude Code is an autonomous agent capable of reading, modifying, and generating files. Running it directly on your host system means giving it access to your entire environment, which can be risky.

Docker provides a controlled sandbox:

* The agent only sees what we explicitly mount into the container
* It cannot access your full filesystem by default
* Any unintended changes are confined to the container

This isolation significantly reduces the risk of accidental file modifications or destructive operations.


### Reproducibility
{: .no_toc }

A Docker container ensures that Claude always runs in a consistent environment:

* Same OS
* Same dependencies
* Same tooling

This means:

* We avoid “it works on my machine” problems
* We can share your setup with others بسهولة
* We can recreate identical environments across projects


### Dependency Management
{: .no_toc }

Claude often interacts with codebases that require specific tools, runtimes, or libraries.

Instead of installing everything on your host machine, Docker allows us to:

* Keep dependencies scoped to a project
* Avoid version conflicts
* Maintain clean and minimal host environments


### Controlled File Access
{: .no_toc }

With Docker, we decide exactly what Claude can access:

* Mount only the project directory we want it to work on
* Keep sensitive files (SSH keys, configs, etc.) outside the container
* Use read-only mounts when needed

This gives us fine-grained control over what the agent can see and modify.


### Safer Experimentation
{: .no_toc }

Docker makes it easy to experiment without consequences:

* Test prompts that modify large parts of a codebase
* Let Claude refactor or reorganize files
* Try automation workflows

If something goes wrong, we can simply discard the container and start fresh.


### Better Automation
{: .no_toc }

Running Claude in Docker also opens the door to automation:

* Integrate it into scripts or CI/CD pipelines
* Run batch operations on repositories
* Execute repeatable workflows

Because the environment is isolated and reproducible, automation becomes much more reliable.


<!-- ### Summary
{: .no_toc }
Using Docker with Claude gives you:

* A safe execution environment
* Full control over file access
* Clean dependency management
* Reproducible setups
* Confidence to experiment and automate

In short, Docker turns Claude from a powerful tool into a controlled and production-ready assistant. -->



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Prerequisites

- Windows 11
- VSCode
- Docker Desktop for Windows installed, updated and running
    - `winget install -e --id Docker.DockerDesktop`
- An Anthropic account with a Pro or Max plan
    - Optionally an API key (from [console.anthropic.com](https://console.anthropic.com))


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 1 — Create project folder (PowerShell)

`Win + X + I` to open a terminal

```powershell
cd $env:tmp
New-Item -ItemType Directory -Path hello_uv
cd hello_uv
```


At this point we can load VSCode from the current directory (`code .`) or continue to use the terminal.







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

To create an image with Rust one can use the Dockerfile below:

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
    - In VSCode, we can open a terminal (`CTRL+ù`) and `cd make_image`




```powershell
docker build -t claude-uv:latest .
# docker build --no-cache -t claude-uv:latest .       # may help
```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img02.webp"
    alt="Terminal output of docker build completing successfully for the claude-uv image"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>docker build completes -- the claude-uv image is ready.</figcaption>
</figure>


The images should be available in Docker



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img01.webp"
    alt="Docker Desktop Images tab showing the claude-uv:latest image (626 MB) in the local image list"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>claude-uv:latest visible in Docker Desktop -- 626 MB, ready to run.</figcaption>
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

* `-v "${PWD}:/workspace"`: mounts the current project folder into `/workspace` inside the container
* `-v "${HOME}\.claude_docker:/root/.claude"`: persists Claude credentials and config outside the container so authentication survives restarts
* `-w /workspace`: sets the working directory inside the container
* `claude-uv:latest`: runs the image we built in Step 2
* `bash`: starts an interactive shell instead of the default CMD


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img03.webp"
    alt="Terminal showing the docker run command launching the claude-uv container with the workspace volume mounted"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Container started -- /workspace is mounted and we are inside the Linux shell.</figcaption>
</figure>



Check every thing works as expected

```bash
uv --version
claude --version
claude
```




<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img14.webp"
    alt="Terminal showing uv and claude version checks, then Claude Code v2.1.87 launching with its welcome screen inside the container"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>uv and Claude Code are both available -- Claude is ready for work.</figcaption>
</figure>














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Step 3 bis — If USING TOKENS. Start the container with the project folder mounted



We use a secret key so create a `.env` file and paste the key from [console.anthropic.com](https://console.anthropic.com)


```text
ANTHROPIC_API_KEY=sk-ant-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

Even if you do not plan to push the project on GitHub today, trust me, create a `.gitignore` file and add the `.env` file. You never know...


```text
.env
```

Reach the root of the project folder and run the image

```powershell
cd ..
docker run --rm -it `
  --env-file .env `
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
## Step 4 — Start Claude Code

Once inside the container, start Claude:

```bash
claude
```

**First launch (Pro plan):** Claude will display a URL. Copy it (`c`), paste it in a browser, log in, copy the code back, and paste it in the terminal. From that point on, credentials are stored in `~/.claude_docker` on the host and reused on every subsequent `docker run`.

**First launch (API key):** Claude picks up `ANTHROPIC_API_KEY` from the environment automatically -- no browser step needed.

#### **Enabling yolo mode**
{: .no_toc }

The whole point of running inside a container is to safely use autonomous mode. Start Claude with:

```bash
claude --dangerouslySkipPermissions
```

Claude will now read, write, and run commands without asking for confirmation at every step. Because we are inside a throwaway container, the risk to our host system is zero.




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
3. Execute `uv run main.py` and get the output




<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img10.webp"
    alt="Claude Code terminal output confirming the Hello World project was scaffolded with uv init, main.py created, and uv run executed successfully"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Claude scaffolded the project, wrote main.py, ran it -- all in 58 seconds.</figcaption>
</figure>


All generated files land in `/workspace/demo` inside the container, which maps back to `tmp\hello_uv\demo` on our Windows host (**fully persisted**).


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img13.webp"
    alt="VSCode Explorer showing the hello_uv project tree with the demo folder containing main.py, pyproject.toml, uv.lock and other generated files"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>The generated project is fully visible in VSCode -- persisted on the Windows host via the volume mount.</figcaption>
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
    alt="VSCode terminal showing docker ps output with the claude-dev container running from claude-uv:latest, up for 10 minutes"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>docker ps confirms the claude-dev container is still running -- ready to exec into.</figcaption>
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
    alt="VSCode showing main.py open in the editor alongside a terminal where uv run demo/main.py outputs Hello, World! from inside the container"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Edit in VSCode, run inside the container -- both sides see the same files.</figcaption>
</figure>



Since VSCode is open we can modify the Python script OR switch to the Claude terminal and continue.

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
New-Item -ItemType Directory -Path hello_uv2
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
    alt="VSCode terminal showing the docker run command for hello_uv2 and Claude Code prompting to confirm workspace trust before starting"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>New project, same workflow -- Claude asks for workspace trust on first launch.</figcaption>
</figure>

Start Claude in yolo mode:

```bash
claude --dangerouslySkipPermissions
```

Then type the prompt:

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
    alt="VSCode showing the Syracuse series code in main.py and a terminal running uv run syracuse/main.py with the computed series as output"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Claude generated the Syracuse series app -- code and output visible side by side.</figcaption>
</figure>










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Alternatives

While running Claude manually inside a Docker container is flexible and educational, it is not the only approach. Several newer or higher-level solutions exist, each with different trade-offs.

### Docker Sandbox (Official Approach)
{: .no_toc }

Docker now provides a more integrated way to run AI agents through its sandboxing features.

Instead of building and managing containers manually, we can use a simplified command to launch an isolated environment tailored for agent execution.

**Advantages:**

* Minimal setup
* Strong isolation by default
* Designed specifically for AI agents
* Less room for configuration mistakes

**Drawbacks:**

* Less control over the environment
* Harder to customize deeply
* Still evolving (APIs and features may change)

This approach is ideal if we want a quick, safe setup without managing Dockerfiles or container configuration.




<!-- ### Docker Model Runner (Local Models)
{: .no_toc }

Another emerging approach is running models locally instead of relying on external APIs.

In this setup:

* Claude Code (or similar tooling) interacts with a locally hosted model
* Docker is used to run both the agent and the model

**Advantages:**

* No external API calls
* Better privacy (data stays local)
* Potentially lower long-term cost

**Drawbacks:**

* Requires powerful hardware
* Setup is more complex
* Model quality may differ from hosted solutions

This is particularly interesting for advanced users or sensitive environments.
 -->

### Dev Containers (VS Code / GitHub Codespaces)
{: .no_toc }

Development containers provide a pre-configured environment tightly integrated with editors like VS Code.

**Advantages:**

* Seamless developer experience
* Built-in tooling and extensions
* Easy onboarding for teams

**Drawbacks:**

* Less isolation from the developer workflow
* Not specifically designed for autonomous agents
* May expose more of your environment than intended

This approach works well if Claude is part of a broader development workflow rather than an isolated agent.


<!-- ### Manual Docker Setup (This Guide)
{: .no_toc }

The approach described in this document sits between low-level control and usability.

**Advantages:**

* Full control over environment and permissions
* Highly customizable
* Works everywhere Docker runs

**Drawbacks:**

* Requires more setup and maintenance
* Easier to misconfigure if you're not careful -->


<!-- ### Choosing the Right Approach
{: .no_toc }

* Use **Docker Sandbox** for simplicity and safety
* Use **manual Docker** for control and customization
* Use **local model setups** for privacy and independence
* Use **dev containers** for integrated development workflows

There is no single “best” solution—only the one that fits your constraints and goals. -->


















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Limitations and Pitfalls

While Docker provides strong isolation and flexibility, it is not a silver bullet. Misconfiguration or misunderstanding can still lead to issues.

### File Permissions
{: .no_toc }

Containers often run as `root` by default.

This can lead to:

* Files created with incorrect ownership on the host
* Permission conflicts when editing files outside Docker

**Recommendation:**

* Use a non-root user inside the container when possible
* Align container user IDs with your host user


### Volume Misconfiguration
{: .no_toc }

Mounting volumes incorrectly can expose more data than intended.

Common mistakes:

* Mounting your entire home directory
* Forgetting that mounted paths are writable by default

**Recommendation:**

* Mount only what is necessary
* Use read-only mounts when possible


### Network Access
{: .no_toc }

By default, containers may have outbound network access.

This means:

* The agent can call external services
* Data may leave our environment unintentionally

**Recommendation:**

* Restrict network access if not needed
* Be explicit about what external communication is allowed


### False Sense of Security
{: .no_toc }

Docker is isolation—not full virtualization.

Risks still exist:

* Kernel-level vulnerabilities
* Misconfigured containers
* Overly permissive mounts

**Key point:**
Docker reduces risk, but does not eliminate it.


### API Costs and Resource Usage
{: .no_toc }

When using hosted models:

* Every interaction may incur cost
* Automated workflows can generate significant usage

**Recommendation:**

* Monitor API usage
* Set limits or budgets where possible


### Environment Drift
{: .no_toc }

If we rebuild containers inconsistently:

* Dependencies may change
* Behavior may differ between runs

**Recommendation:**

* Pin versions in our Dockerfile
* Use reproducible builds


### Debugging Complexity
{: .no_toc }

When something goes wrong, debugging can be harder:

* Is the issue in the host?
* The container?
* The agent itself?

**Recommendation:**

* Keep setups simple
* Add logging and visibility where possible


<!-- ### Summary

Docker provides powerful isolation and control, but requires careful use.

Common pitfalls include:

* Incorrect permissions
* Overexposed volumes
* Unrestricted network access
* Hidden costs from API usage

Understanding these limitations helps you use Docker with Claude safely and effectively. -->












<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion
Running Claude Code inside a Docker container is a small upfront investment that pays off immediately. We get yolo mode without the anxiety: Claude can install packages, rewrite files, and run arbitrary shell commands. If it goes sideways, one `docker rm` and we are back to a clean slate. Nothing on our beloved Windows host is ever at risk.

Beyond safety, the Linux environment just works better. No path translation, no shebang issues, no "this script assumes bash" friction. Claude operates in the environment it was designed for, and the results seem to show.

The image built here is intentionally minimal. From this base we can layer in anything our stack requires: databases, compilers, CLI tools and commit the Dockerfile alongside our project so the environment is fully reproducible. Our teammates (or our future self) get the exact same setup with a single docker build.

The mounted volume pattern means our code always lives on the host, version-controlled and safe, while the container remains throwaway. Spin it up, let Claude loose, close it down. Repeat.

That is the whole workflow. Simple, safe, and fast enough that we will not think twice about using it every day.

Is this how I will work from now on? I don’t know. First, Claude should be able to work perfectly on the most commonly used host; it should not force us to move to Linux or, even worse, to macOS. On the other hand, I really like my Windows setup.