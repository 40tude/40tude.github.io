---
published: true
author: 40tude
lang: en-US
layout: default
title: "Spec-Driven Development with Rust and GitHub Spec Kit"
description: "Building and evolving a small Axum web app step-by-step using specs, tasks, and AI-assisted workflows."
image: docs/06_programmation/rust/027_speckit/assets/img04.webp
twitter:
  card: summary_large_image
parent: "Rust"
nav_order: 36
date:               2026-03-05 11:00:00
last_modified_date: 2026-03-05 13:00:00
---


# Spec-Driven Development with Rust and GitHub Spec Kit
{: .no_toc }

Building and evolving a small Axum web app step-by-step using specs, tasks, and AI-assisted workflows.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* For beginners, tinkerers, hobbyists, amateurs, and early-career developers...


**Note**
The [companion project](https://github.com/40tude/bmi_sdd) with is available on GitHub.




<figure style="text-align: center;">
<img src="./assets/img00.webp" alt="Description" width="900" loading="lazy"/>
<figcaption>Caption.</figcaption>
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
I suppose the following software are installed:
1. VScode
1. GitHub
1. GitHub CLI (gh)
    ```powershell
    winget install GitHub.cli
    gh auth login

    # Puis dans le wizard :
    #   Account → GitHub.com
    #   Protocol → HTTPS
    #   Authenticate Git with GitHub credentials → Yes
    #   How to authenticate → Login with a web browser

    gh --version
    gh auth status

    ```
1. Python
1. uv
    ```powershell
    winget install astral-sh.uv
    uv --version
    ```
1. Optional: Heroku
    * You have an account (free)
    * Install Heroku CLI. Read https://devcenter.heroku.com/articles/heroku-cli
1. Spec Kit
    * https://github.com/github/spec-kit
    ```powershell
    uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
    ```



**Note**
If you need to update specify

```powershell
uv tool install specify-cli --force --from git+https://github.com/github/spec-kit.git
specify check
```
Read : https://github.com/github/spec-kit/blob/main/docs/upgrade.md



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Setup

```powershell
cargo new bmi_sdd
cd ./bmi_sdd
cargo build
cargo run
# commit and push on GitHub
git add .
git commit -m "My first commit"

# Create the repo on GitHub with GitHub CLI (gh)
gh repo create my_project --public --source=. --remote=origin --push

# upgrade speckit
uv tool install specify-cli --force --from git+https://github.com/github/spec-kit.git

specify init . --ai claude

# Welcome home
code .

# CTRL + ù on FR keyboard
claude
/model # Set Opus 4.6 &  effort: full
```







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.constitution

I open a chat with claude in my web browser
I share the very first version of the README.md
I say :

> I work with the speckit workflow. After reading the README.md, suggest a prompt to use with /speckit.constitution. If anything is missing from the README, let me know as well.

Then we have a Q&A session. At the end I get the prompt below, which I paste into the Claude session I have in VSCode:

```text
/speckit.constitution

Create a BMI Calculator web application in Rust with the following requirements:

## Core Functionality
- Calculate Body Mass Index (BMI) using SI units (kg for weight, meters for height)
- Classify BMI into standard WHO categories:
  - Underweight: < 18.5
  - Normal: 18.5 – 24.9
  - Overweight: 25.0 – 29.9
  - Obese: ≥ 30.0
- Stateless application — no database, no persistence

## API
- Single endpoint: POST /api/bmi
- Request body (JSON): { "weight_kg": f64, "height_m": f64 }
- Success response (200): { "bmi": f64, "category": "string" }
- Error response (422): { "error": "string" } with meaningful messages (e.g., "weight_kg must be positive")
- Health check: GET /health returning 200 OK

## Tech Stack & Crates
- **Web framework:** Axum + Tokio (async runtime)
- **Serialization:** Serde (JSON request/response)
- **Error handling:** thiserror (domain/library errors) + anyhow (application-level errors)
- **Logging:** tracing + tracing-subscriber — all errors logged server-side
- **CLI config:** Clap (port, log level)
- **HTTP client:** Reqwest (for integration tests)
- **UI:** Bootstrap (CDN), served as embedded HTML via Axum

## Architecture
- Clean separation: domain logic, API layer, UI serving
- Domain module: pure functions for BMI calculation and classification (no I/O, no framework dependencies)
- API module: Axum handlers, JSON types, input validation, error mapping
- UI module: single HTML page with Bootstrap form, fetch-based submission to /api/bmi, result display

## Quality & Testing (TDD)
- Unit tests for domain logic (calculation accuracy, category boundaries, edge cases like zero/negative inputs)
- Integration tests for API endpoints using Reqwest (valid requests, invalid inputs, missing fields)
- All tests runnable via `cargo test`

## Deployment
- Run and test locally first — port configurable via --port CLI flag or PORT env var (Heroku convention)
- Deploy on Heroku using Rust buildpack
- PORT env var takes precedence over CLI flag when set
- Procfile included

## Non-goals
- No input range constraints beyond positivity
- No persistence or database
- No API versioning
- No authentication

```



* I review `.specify/memory/constitution.md`
* Commit msg: `add: project constitution v1.0.0`




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.specify

I continue the chat in Claude web. I ask:

> Now I need to use /speckit.specify but I'm not sure what to write. I feel like we've already said everything in the constitution. Can you suggest some prompts?

We go back and forth a few times. Finally I paste this commande in Claude in VSCode:


```
/speckit.specify

Specify the project with the following structure:

## Project Layout
- src/main.rs — CLI parsing (Clap) + server startup
- src/domain.rs — BmiInput struct, calculate_bmi(), BmiCategory enum, BmiResult struct
- src/api.rs — Axum handlers, JSON request/response types, error handling
- src/ui.rs — function returning the HTML page as a string
- src/error.rs — thiserror enum (InvalidWeight, InvalidHeight) mapped to 422 responses
- tests/api_tests.rs — integration tests with reqwest

## Key Design Decisions
- BMI rounded to 1 decimal place
- BmiCategory implements Display for the JSON "category" field
- Input validation happens in the domain layer, not the handler
- Tracing subscriber initialized in main with env-filter
- HTML page embedded as a const &str, not served from a file

## Expected Test Cases

### Domain Tests
- calculate_bmi(70.0, 1.75) → 22.9, Normal
- calculate_bmi(50.0, 1.80) → 15.4, Underweight
- calculate_bmi(90.0, 1.70) → 31.1, Obese
- calculate_bmi(0.0, 1.75) → Error: weight must be positive
- calculate_bmi(70.0, -1.0) → Error: height must be positive

### API Tests
- POST /api/bmi with valid JSON → 200 + correct result
- POST /api/bmi with missing field → 422 + error message
- POST /api/bmi with empty body → 422
- GET /health → 200

From these tests, derive the types, modules, and function signatures.

```


* This creates branch `001-bmi-calculator`
* I read specs\001-bmi-calculator\spec.md
* I read specs\001-bmi-calculator\checklists\requirements.md
* Commit msg: `After /speckit.specify`





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.clarify

```powershell
/clear           # check Opus is active
/speckit.clarify # No additional instruction
```

Commit msg: `After /speckit.clarify`




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.plan

```powershell
/clear          # check Opus is active
/specify.plan

```
Commit msg: `After /speckit.plan`


**Note:** Reading this page `https://github.com/github/spec-kit/blob/main/spec-driven.md`, I wonder if I should have put the list of tools to use here instead.






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.tasks

* **⚠️ IMPORTANT:** Remember to switch to Sonnet (full)

```powershell
/model # Select Sonnet, Full
/clear
/speckit.tasks
```

* Commit msg: `After /speckit.tasks`




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.analyze

```powershell
clear             # check Sonnet is active
speckit.analyze

```
* Commit msg: `After /speckit.analyze`



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.checklist

```powershell
/clear             # check Sonnet is active
/speckit.checklist

```
* Commit msg: `After /speckit.checklist`




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## /speckit.implement

```powershell
/clear               # check Sonnet is active
/speckit.implement
```

* Commit: `After /speckit.implement`





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Test & Check Local

### Checking
```powershell
# Default port 3000
cargo run

# Custom port via CLI flag
cargo run -- --port 8080

# Custom port via env var (takes precedence over --port)
$env:PORT='8086'; cargo run
# CTRL+C to stop
Remove-Item env:PORT
ls env:

# PORT only exists for the spawned process
Start-Process cargo -ArgumentList 'run' -NoNewWindow -Wait -Environment @{ PORT = '8086' }


# Custom log level
cargo run -- --log-level debug
cargo run -- --log-level "bmi_sdd=debug,hyper=debug,tower=debug"
```

The server starts at `http://localhost:3000` (or the configured port).

### Testing

```powershell
# Run all tests (unit + integration)
cargo test

# Unit tests only (domain logic + port resolution)
cargo test --lib
cargo test --bin bmi_sdd

# Integration tests only
cargo test --test api_test
```

### Manual Verification

With the server running (`cargo run`):

```powershell
# Valid BMI calculation
curl -X POST http://localhost:3000/api/bmi `
  -H "Content-Type: application/json" `
  -d '{"weight_kg": 70.0, "height_m": 1.75}'
# -> 200 {"bmi":22.9,"category":"Normal"}

# Invalid input
curl -X POST http://localhost:3000/api/bmi `
  -H "Content-Type: application/json" `
  -d '{"weight_kg": 0.0, "height_m": 1.75}'
# -> 422 {"error":"weight_kg must be positive"}

# Health check
$response = Invoke-WebRequest http://localhost:3000/health
$response.StatusCode
# -> 200

# Web UI -open in browser
start http://localhost:3000
```




I stay in the branch and I use Claude code to:
* Add `CTRL+C` support
* Add one tracing::debug! in src/api.rs


<figure style="text-align: center;">
<img src="./assets/img01.webp" alt="" width="900" loading="lazy"/>
<figcaption>...</figcaption>
</figure>


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Merging the branch manually

```powershell
# Switch to the branch (just to make sure)
git switch 001-bmi-calculator

# Push the feature branch
git push -u origin 001-bmi-calculator

# Create the pull request
gh pr create --title "feat: bmi-calculator" --body "First implementation" --base main

# gh pr merge with no argument use the current branch to identify the PR
# Merge + delete the remote branch
# `gh pr merge --delete-branch` delete :
#   * the remote branch
#   * the local branch if we are on another branch (we are on main)
git switch main
gh pr merge 001-bmi-calculator --squash --delete-branch

# Sync
git pull origin main
```




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Deployment to Heroku


### Prerequisites

- Run and test locally first
- Heroku account
- Heroku CLI installed
- Read the `.slugignore` file (avoid useless files on Heroku)
- Check the line `strip = "symbols"` in `Cargo.toml` (reduce size by removing symbol table entries from the final executable)


### Steps

1. Create a new Heroku app:
```powershell
heroku create rust-bmi-sdd
```

2. Set the buildpack:
```powershell
heroku buildpacks:set emk/rust
```

**Note:**
Combine 1 & 2 with:

```powershell
heroku create rust-bmi-sdd --buildpack emk/rust
```


3. Auth:
```powershell
heroku auth:token
```
Select and copy the token.


4. Deploy on Heroku:
```powershell
git push heroku main
```
* When the dialog box popup, enter **ANY** name and paste the token.
* Files are sent, the build process starts and the server is launched.
* Note the URL (for example: https://rust-bmi-sdd-XXXX.herokuapp.com/)

5. Open the app:
```powershell
heroku open
```
Alternatively point your browser to the previous URL (for example: https://rust-bmi-sdd-XXXX.herokuapp.com/)

**Note:**
Use
```powershell
heroku run bash
```
* To check the files deployed on Heroku.
* To check the size of the binary use `ls -al ./target/release/`

<figure style="text-align: center;">
<img src="./assets/img05.webp" alt="" width="900" loading="lazy"/>
<figcaption>...</figcaption>
</figure>


**Note:**
The process should be:
- Add features with Spec Kit, modify the app with Claude, test locally etc.
- Commit & push on GitHub
- Push on Heroku (`git push heroku main`)








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Adding a feature

Now I want to add an history with the last 5 calculated BMI.


### Process
On va créer une issue (#42 par exemple )
On va implémenter la feature dans une branche en suivant le workflow speckit
On va tester, améliorer etc.
On va merger la branche et fermer l'issue (`Closes #42` dans le message)

Suite discussion avec Claude dans mon Browser Web j'arrive au prompt suivant



```text
Create a GitHub issue on this project with the following details:

Title: "feat: add BMI calculation history (last 5 entries)"

Body:
## Summary
Add an in-memory session log that displays the last 5 BMI calculations
on the results page.

## Behavior
- After each BMI calculation, store the result in a shared in-memory list
- Display the last 5 entries in a table below the BMI result
- When a 6th entry is added, the oldest is evicted (FIFO using VecDeque)
- History is server-wide (shared across all users) — per-user session
  management is out of scope for this issue

## Implementation hints
- Use `VecDeque<BmiEntry>` with a max capacity of 5
- Wrap in `Arc<Mutex<...>>` and register as Axum shared state
- `BmiEntry` should store: weight, height, bmi value, category, timestamp

## Out of scope
- Persistent storage (database)
- Per-user session isolation (suggested as a follow-up exercise)

## Acceptance criteria
- [ ] History table appears after the first calculation
- [ ] Table shows at most 5 entries
- [ ] Oldest entry is removed when a 6th is added
- [ ] App compiles and deploys to Heroku without regression

Labels: enhancement

```


* À la fin je lis `Issue created: https://github.com/40tude/bmi_sdd/issues/2`. Je note que l'issue a le numéro 2
* Je peux aller sur GitHub pour voir l'issue #2 qui a été créée
* Ensuite je repars dans un workflow speckit "classique" pour implementer la feature

<figure style="text-align: center;">
<img src="./assets/img02.webp" alt="" width="900" loading="lazy"/>
<figcaption>...</figcaption>
</figure>


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### /speckit.specify

* **⚠️ IMPORTANT:** `/model` → Opus 4.6, effort: full
* `/clear`

Je colle
```text
/speckit.specify
We want to implement the feature described in GitHub issue #2:
BMI calculation history showing the last 5 entries using an
in-memory VecDeque, shared across all users.
```

* This creates branch `002-bmi-history`
* Creates specs/002-bmi-history
* It indicates : `No clarifications needed -- the GitHub issue was well-specified.`
    * I will **NOT** `/speckit.clarify`
* Commit msg: `After /speckit.specify`


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### /speckit.plan
* `/clear` # check Opus is active

```text
/specify.plan
```
* Generate plan.md, quickstart.md, research.md and data-model.md, api.md in `specs/002-bmi-history` and CLAUDE.md...
* Commit msg: `After /speckit.plan`





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### /speckit.tasks
* **⚠️ IMPORTANT:** Remember to switch back to `/model sonnet full`
* `/clear`
* `/speckit.tasks`
* Create `specs/002-bmi-history/tasks.md`
* Commit msg: `After /speckit.tasks`



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### /speckit.analyze
* `/clear` # check Sonnet is active
* `/speckit.analyze`
* I read the findings in the terminal

ID: C1
Category: Constitution
Severity: CRITICAL
Location(s): constitution.md:L64, plan.md:L32
Summary: Principle V declares "The application is stateless." Feature adds server-wide mutable state. Plan marks this as VIOLATION but the constitution itself has never been formally amended (no version bump, no text change).
Recommendation: Run /speckit.constitution to amend Principle V: add an exception clause permitting ephemeral in-memory state when explicitly required by spec and documented in Complexity Tracking.

C'est un très bon catch de Speckit. **Stateless** signifie que le serveur ne conserve aucune donnée entre les requêtes. Chaque requête est traitée indépendamment, sans mémoire du passé. C'est ce que dit ton `constitution.md` actuellement.

Un `VecDeque` en mémoire partagée via `Arc<Mutex<...>>` dans le state Axum, c'est exactement l'opposé — le serveur **conserve un état** qui persiste entre les requêtes.

Donc Speckit a raison de bloquer : il y a une contradiction formelle entre la constitution et la feature.

La bonne action c'est d'**amender la constitution** avant d'implémenter. Par exemple, modifier le principe V pour dire quelque chose comme :

> *"The application minimizes server-side state. Ephemeral in-memory state is permitted when explicitly documented and justified. Persistent storage requires a separate architectural decision."*

Et bumper la version (ex: `v1.0` → `v1.1`) avec une note expliquant pourquoi ce principe a été assoupli.

C'est un **excellent point**. Ca montre que SDD ne laisse pas passer les contradictions silencieusement. Une feature qui semble anodine force une décision architecturale consciente et traçable. C'est exactement la valeur du workflow.


Je répond "YES" à la question "Would you like me to suggest concrete remediation edits for the top 3 issues (C2, I1, W1)? I can produce the exact text changes for data-model.md and tasks.md for your review before you apply them."

Je le laisse faire. Il demande ensuite de bumper constitution. J'accepte


* Commit msg: `After /speckit.analyze`










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### /speckit.checklist

* `/clear` # check Sonnet is active
* `/speckit.checklist`
* C'est quoi /specify.checklist ? C'est une étape de validation de la spec avant d'implémenter. L'idée : avant de coder, on s'assure que la spec est suffisamment précise et complète pour qu't un dev (ou un agent) puisse l'implémenter sans avoir à deviner. C'est un filet de sécurité qui détecte les trous dans la spec.
En gros : "Est-ce que ce qu'on a écrit est assez précis pour être implémenté sans ambiguïté ?"
* Les 3 questions
    * Q1 — Sur quoi concentrer la checklist ?
    Pour un article de blog sur une feature simple, choisis D (All three equally) — tu veux une couverture équilibrée qui montre bien l'intérêt du processus. Si tu choisis A, B ou C tu rates des angles intéressants pour tes lecteurs.
    * Q2 — Niveau de profondeur ?
    B (Standard PR review) — c'est le bon équilibre pour l'article. Lightweight serait trop superficiel pour être pédagogique, et Formal release gate serait overkill pour un hello world.
    * Q3 — La concurrence est-elle un bloquant ?
    A (Hard blocker) — et c'est justement un super exemple pour l'article. Rust te force à gérer la concurrence explicitement (Arc<Mutex<...>>), donc autant que la spec le reflète comme une vraie exigence plutôt qu'une simple "assumption". Ça montre que SDD attrape les failles de sécurité avant le code.
* Je réponds : "D B A"
* Il créé un pr-review.md
* Commit msg: `After /speckit.checklist`








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
### /speckit.implement

* `/clear` # check Sonnet is active
* `/usage` # I read 54%
* `/speckit.implement`
* A un moment il indique

```text
Checklist Status

  ┌─────────────────┬───────┬───────────┬────────────┬────────┐
  │    Checklist    │ Total │ Completed │ Incomplete │ Status │ ├─────────────────┼───────┼───────────┼────────────┼────────┤
  │ requirements.md │ 13    │ 13        │ 0          │ PASS   │
  ├─────────────────┼───────┼───────────┼────────────┼────────┤
  │ pr-review.md    │ 28    │ 0         │ 28         │ FAIL   │
  └─────────────────┴───────┴───────────┴────────────┴────────┘

The pr-review.md is a post-implementation PR review checklist (all items are reviewer questions about spec gaps, not implementation prerequisites). It is not intended to be completed before coding.

Some checklists are incomplete. Do you want to proceed with implementation anyway? (yes/no)
```
C'est parfaitement logique:
* **`requirements.md` à 13/13**: c'est la checklist *pré-implémentation*. Elle vérifie que la spec est assez précise pour coder. Elle est complète, donc tu es prêt.
* **`pr-review.md` à 0/28**: c'est la checklist *post-implémentation*. Elle sert au moment de la PR/code review, quand le code existe déjà. Elle pose des questions du type "est-ce que l'implémentation respecte la spec ?". Il est normal qu'elle soit à 0 avant de coder, il n'y a rien à reviewer encore.


* Je réponde YES. La `pr-review.md` sera utile plus tard quand tu feras ta PR pour merger la feature et clore l'issue GitHub.

* Pour l'article: illustre que Speckit distingue bien les deux phases : *spec validation* avant le code, *implementation review* après.




* Commit msg: `After /speckit.implement`



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Checking and testing (local)








<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Improve the array rendering

<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

Je reste dans la branche et j'utilise Claude (pas Spec Kit)

* `/clear` # check Sonnet is active
* `/usage` # 71%
* J'utilise de ce prompt

```
Improve the visual appearance of the Calculation History table using
Bootstrap classes. Keep the existing functionality intact.

Requirements:
- Use Bootstrap's `table-striped table-hover table-bordered table-sm` classes
- Add a `thead-dark` (or `table-dark`) header row for contrast
- Color-code the Category cell based on value:
    - "Underweight" → badge badge-warning (yellow)
    - "Normal"      → badge badge-success (green)
    - "Overweight"  → badge badge-warning (orange)
    - "Obese"       → badge badge-danger  (red)
- Wrap the table in a `card` with a card-header titled "Calculation History"
- The timestamp format is fine as-is, but put date and time on the same line
- Keep the # column but right-align numeric columns (Weight, Height, BMI)

```


<div align="center">
<img src="./assets/img04.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>


### Merge of the branch

J'utilise le prompt ci-dessous:

```text
Merge the current feature branch into main, then close GitHub issue #2.

Steps:
- Ensure we are on the feature branch
- Merge into main with a descriptive commit message that includes "Closes #2"
- Push main to remote
- Delete the feature branch (local and remote)

```



Sinon je peux le faire à la main:

```powershell
# Switch to the branch just to make sure
git switch 002-bmi-history

# Push the feature branch
git push -u origin 002-bmi-history

# Create the pull request
gh pr create --title "feat: bmi-calculator" --body "Closes #2" --base main

# gh pr merge with no argument use the current branch to identify the PR
# Merge + delete the remote branch
# `gh pr merge --delete-branch` delete :
#   * the remote branch
#   * the local branch if we are on another branch (we are on main)
git switch main
gh pr merge 002-bmi-history --squash --delete-branch

# Sync
git pull origin main
```




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Deployment to Heroku
- Commit & push on GitHub
- Push on Heroku (`git push heroku main`)



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography
* [Spec Kit repo](https://github.com/github/spec-kit)