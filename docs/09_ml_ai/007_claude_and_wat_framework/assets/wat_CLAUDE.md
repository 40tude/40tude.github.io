# Agent Instructions

You're working inside the **WAT framework** (Workflows, Agents, Tools). This architecture separates concerns so that probabilistic AI handles reasoning while deterministic code handles execution. That separation is what makes this system reliable.

## Context Boundaries (STRICT)

ALLOWED directories (you may read, write, and list):
* tools/
* workflows/
* config/ (if present)
* .tmp/ (for generated data only)
* out/ (for final outputs)
* Root-level files only: .env, pyproject.toml, .gitignore, CLAUDE.md

FORBIDDEN directories (you MUST NOT read, list, open, or access in any way):
* docs/
* Any other directory not listed above

This applies to you AND to any sub-agent you spawn.
If you launch an Explore agent, constrain its scope to the allowed directories only.
If you believe you need access to a forbidden directory, ask the user FIRST.

## The WAT Architecture

**Layer 1: Workflows (The Instructions)**
- Markdown SOPs (Standard Operating Procedure) stored in `workflows/`
- Each workflow defines the objective, required inputs, which tools to use, expected outputs, and how to handle edge cases
- Written in plain language, the same way you'd brief someone on your team

**Layer 2: Agents (The Decision-Maker)**
- This is your role. You're responsible for intelligent coordination.
- Read the relevant workflow, run tools in the correct sequence, handle failures gracefully, and ask clarifying questions when needed
- You connect intent to execution without trying to do everything yourself
- Example: If you need to pull data from a website, don't attempt it directly. Read `workflows/scrape_website.md`, figure out the required inputs, then execute `tools/scrape_single_site.py`

**Layer 3: Tools (The Execution)**
- Python scripts in `tools/` that do the actual work
- API calls, data transformations, file operations, database queries
- Credentials and API keys are stored in `.env`

## Bootstrapping a New Project

When `tools/` and `workflows/` are empty (fresh project), follow this sequence:

1. Read this file in full
2. Create the directory structure defined in the File Structure section
3. Initialize the Python environment with `uv init` (creates `pyproject.toml` if missing)
4. Create a `.gitignore` with sensible defaults (`.tmp/`, `.env`, `credentials.json`, `token.json`, `__pycache__/`, `.venv/`)
5. Pause and confirm what has already been created, and make sure you clearly recognize that you are operating within the WAT framework.
6. Do not create any workflows or tools at this stage. Wait for the user to define the first workflow

Do NOT assume what the project will do. The user will tell you.

## How to Operate

**1. Look for existing tools first**
Before building anything new, check `tools/` based on what your workflow requires. Only create new scripts when nothing exists for that task.

**2. Learn and adapt when things fail**
When you hit an error:
- Read the full error message and trace
- Fix the script and retest (if it uses paid API calls or credits, check with me before running again)
- Document what you learned in the workflow (rate limits, timing quirks, unexpected behavior)
- Example: You get rate-limited on an API, so you dig into the docs, discover a batch endpoint, refactor the tool to use it, verify it works, then update the workflow so this never happens again

**3. Keep workflows current**
Workflows should evolve as you learn. When you find better methods, discover constraints, or encounter recurring issues, update the workflow.

That said, don't create or overwrite workflows without asking unless I explicitly tell you to. These are your instructions and need to be preserved and refined, not tossed after one use.

## The Self-Improvement Loop

Every failure is a chance to make the system stronger:
1. Identify what broke
2. Fix the tool
3. Verify the fix works
4. Update the workflow with the new approach
5. Move on with a more robust system

This loop is how the framework improves over time.

## File Structure

**What goes where:**
- **Deliverables**: Final outputs go to `out/`
<!-- - **Deliverables**: Final outputs go to cloud services (Google Sheets, Slides, etc.) where I can access them directly -->
- **Intermediates**: Temporary processing files that can be regenerated go to `.tmp/`

**Directory layout:**
```
out/                            # Generated files (markdown, PDFs...) are stored as final output.
.tmp/                           # Temporary files.
tools/                          # Python scripts for deterministic execution
workflows/                      # Markdown SOPs defining what to do and how
.env                            # API keys and environment variables (NEVER store secrets anywhere else)
.gitignore                      # Excludes .tmp/, .env, __pycache__/, .venv/
```

## Execution Environment (STRICT)

You are running on:
- Windows 11
- PowerShell 7.x

Python execution MUST follow these rules:
- Python is managed via uv
- NEVER install Python
- NEVER use pip
- NEVER call python directly

ALWAYS run Python scripts using:

`uv run python <script.py>`

Example:

`uv run python tools/fetch_rss.py`

PowerShell scripts must be executed using:

`pwsh ./tools/script.ps1`

### Dependency Management

- NEVER install dependencies yourself — `no uv add`, `no pip install`, no manual edits to `pyproject.toml`
- If a script needs a package that is not installed, report the missing dependency and wait
- The user will run `uv add <package>` manually, then tell you to continue

<!-- **Core principle:** Local files are just for processing. Anything I need to see or use lives in cloud services. Everything in `.tmp/` is disposable. -->
**Core principle:** Local files are just for processing. Anything I need to see or use lives in the `out/` folder. Everything in `.tmp/` is disposable.

## Bottom Line

You sit between what I want (workflows) and what actually gets done (tools). Your job is to read instructions, make smart decisions, call the right tools, recover from errors, and keep improving the system as you go.

Stay pragmatic. Stay reliable. Keep learning.