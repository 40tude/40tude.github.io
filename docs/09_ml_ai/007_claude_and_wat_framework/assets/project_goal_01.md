# Project: Daily Tech Digest Generator

## Goal
Build a system that generates a Daily Tech Digest. The system should:
- Collect information from online sources (e.g., RSS feeds, tech blogs, GitHub, newsletters...)
- Extract relevant articles based on user-defined topics of interest
- Summarize key insights from each article
- Produce a clear, structured daily report (format TBD: markdown, email, Google Doc...)

## Constraint
All planning must align with the WAT framework defined in CLAUDE.md.
- Workflows go in workflows/
- Tools go in tools/
- The agent coordinates execution

## Context
- Target platform: Windows 11 / PowerShell 7.x
- Greenfield project (no existing code)
- Tech stack, architecture, workflows, and tools are not yet decided
- Everything should be designed incrementally

## Current Mode: PLANNING ONLY

Do NOT write code.
Do NOT jump to a final architecture.
We are exploring and making decisions.


## What I expect from you

### 1. Ask structured clarifying questions
Organize your questions into categories:
- User needs & expectations
- Data sources & ingestion
- Filtering & relevance logic
- Output format & delivery
- Operational constraints (frequency, performance, cost, reliability)

Challenge assumptions where relevant.


### 2. Map the problem space
Identify:
- The key architectural decisions we need to make
- The main unknowns or risks
- Dependencies between decisions


### 3. Propose options (not a single solution)
For each major decision (e.g., storage, scheduling, summarization approach, output format):
- Present 2–3 viable options
- Explain pros / cons
- Indicate when each option is appropriate


### 4. Help prioritize decisions
Clearly distinguish:
- Decisions we must make early
- Decisions we can defer


### 5. Define a minimal vertical slice
Propose a small, end-to-end version of the system that:
- Can run locally on Windows (PowerShell-friendly)
- Avoids unnecessary infrastructure
- Covers the full pipeline (ingest → filter → summarize → output)
- Is simple enough to build quickly but meaningful enough to validate the concept


### 6. Keep things pragmatic
- Avoid over-engineering
- Avoid premature optimization
- Prefer simple, testable approaches first
