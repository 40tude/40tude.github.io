---
layout: default
lang: en-US
title: "Rust"
parent: "Programmation"
# math: mathjax
date:               2025-03-27 13:00:00 +0000
last_modified_date: 2026-11-21 11:06:41
#last_modified_date: 2025-01-05 14:00:00 +0000
nav_order: 0
permalink: /rust
---

# Rust
{: .no_toc }

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img src="./assets/img00.webp"
    alt="A smiling 1970s-style developer in a flowery suit and tinted glasses points at the viewer, sitting at a vintage computer showing code and a glowing gear, surrounded by psychedelic swirls, a disco ball, a lava lamp, and floating architecture diagrams."
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Yes, you. Grab a seat, the reading path starts right here.</figcaption>
</figure>

A reading path through the Rust articles of this site, from the first install to software architecture.
{: .lead }

## How to Use This Page
{: .no_toc }

The articles are listed in the sidebar in a suggested reading order. If you are new to Rust, start at the top and move down. If you already know the basics, jump to the step that matches your level.

The path is split into steps. Each step builds on the previous ones, but nothing is set in stone: if a topic catches your eye, go for it and come back later.

## Step 0: Get Started
{: .no_toc }

Install the tools and get a working environment.

* [Rust on Windows 11, My Complete Setup Guide]({%link docs/06_programmation/rust/005_my_rust_setup_win11/my_rust_setup_win11.md%}): the toolchain, VS Code, and the settings I use every day.
* [Using Rust in Jupyter Notebooks on Windows]({%link docs/06_programmation/rust/001_rust_jupyter/rust_jupyter.md%}): a nice way to experiment with small snippets.
* [Rust, mon setup pour CodinGame]({%link docs/06_programmation/rust/010_codingame_setup/codingame_setup.md%}) (in French): practice Rust on coding puzzles.

## Step 1: Language Basics
{: .no_toc }

The core ideas that make Rust different: bindings, ownership, references, and functions.

* [Bindings in Rust: Much More Than Just Variables]({%link docs/06_programmation/rust/004_mutability/mutability_us.md%}): mutability, shadowing, and ownership. A French version is also [available]({%link docs/06_programmation/rust/004_mutability/mutability.md%}).
* [How to Actually Read Rust's Standard Library Documentation]({%link docs/06_programmation/rust/021_doc/doc_00.md%}): the documentation is our best friend, provided we know how to read it. Read Part 1 now, and come back to Parts 2 and 3 once traits and generics feel familiar (Step 3).
* [Who Owns What in Rust Loops?]({%link docs/06_programmation/rust/008_for_loops/for_loops.md%}): ownership and borrowing applied to `for` loops.
* Rust Dereferencing vs Destructuring, [part 1]({%link docs/06_programmation/rust/009_dereferencing_destructuring/dereferencing_destructuring_01.md%}) and [part 2]({%link docs/06_programmation/rust/009_dereferencing_destructuring/dereferencing_destructuring_02.md%}): `*`, `&`, and patterns, explained step by step.
* [Rust and Functional Programming: A Beginner's Top 10 Functions]({%link docs/06_programmation/rust/006_rust_fp_top_10_functions/rust_fp_top_10_functions.md%}): `map`, `filter`, `fold`, and friends.
* [Functions, Methods, Closures, Iterators, and Coroutines in Rust]({%link docs/06_programmation/rust/023_coroutine/coroutine.md%}): a short reminder with working examples.

## Step 2: Organize the Code
{: .no_toc }

As soon as a project grows beyond a single file, we need modules, crates, and an understanding of what the build does.

* [From Folders to Crates: A Practical Guide to Modern Rust Module Trees]({%link docs/06_programmation/rust/013_no_more_mod_rs/no_more_mod_rs.md%}): modules without `mod.rs`. Many later articles rely on it.
* [From Source File to Executable: A Gentle Walk Through the Rust Build System]({%link docs/06_programmation/rust/014_build_system/from_src_to_exe.md%}): what happens between `cargo build` and the `.exe`.

## Step 3: Types and Abstractions
{: .no_toc }

`Option`, `Result`, and traits are everywhere in Rust code. These three series are best read in this order.

* [Option in Rust: 15 Examples from Beginner to Advanced]({%link docs/06_programmation/rust/020_some/some_00.md%}) (3 parts): from `if let` to combinators.
* [Rust Error Handling, Demystified]({%link docs/06_programmation/rust/016_errors/errors_00.md%}) (7 parts): `Result`, the `?` operator, custom errors, and beyond.
* [Rust Traits: Defining Character]({%link docs/06_programmation/rust/015_traits/traits_00.md%}) (5 parts): from basic syntax to plugins.

## Step 4: Projects
{: .no_toc }

Time to put everything together in complete programs.

* [Multithreaded Mandelbrot Sets in Rust]({%link docs/06_programmation/rust/003_mandelbrot_multithread/mandelbrot_multithread.md%}): threads and performance on a visual problem.
* [Building Conway's Game of Life in Rust with Pixels and Winit]({%link docs/06_programmation/rust/017_game_of_life/game_of_life_00.md%}), followed by [Understanding GPU Selection and Performance]({%link docs/06_programmation/rust/017_game_of_life/gpu.md%}).
* [How to Deploy Your Rust API on Heroku]({%link docs/06_programmation/rust/011_api_heroku/api_heroku.md%}), followed by [CI/CD for Rust]({%link docs/06_programmation/rust/012_api_heroku_2/api_heroku_2.md%}).
* [From Zero to Ready to Contribute: Illustrated with Microsoft Edit]({%link docs/06_programmation/rust/018_msedit/ms_edit.md%}): from the first fork to the first pull request on a real project.

## Step 5: Software Architecture
{: .no_toc }

How to structure an application so that it stays easy to change. Read these in order, each one builds on the previous.

* [SOLID Principles in Rust: A Practical Guide]({%link docs/06_programmation/rust/022_solid/solid_00.md%}) (5 parts).
* [Hexagonal Architecture in Rust: A Beginner's Guide]({%link docs/06_programmation/rust/024_hexagonal/hexagonal_lite.md%}).
* [Learning Modular Monolith Architecture with Rust]({%link docs/06_programmation/rust/025_modular_monolith/modular_monolith_00.md%}) (7 steps and a bonus).
* [From Monolith to Distributed Systems in Rust]({%link docs/06_programmation/rust/026_monolith_to_distributed/monolith_to_distributed.md%}).

## Step 6: Tools and AI
{: .no_toc }

Working with AI coding assistants on Rust projects.

* [Microsoft Pragmatic Rust Guidelines with Claude]({%link docs/06_programmation/rust/019_ms_rust/ms_rust.md%}): turn the guidelines into a Claude skill.
* [Spec-Driven Development with Rust and GitHub Spec Kit]({%link docs/06_programmation/rust/027_speckit/speckit.md%}): build and evolve a small Axum web app from specs.

## Practice
{: .no_toc }

* [Coding Interview Patterns in Rust]({%link docs/06_programmation/rust/007_coding_interview_patterns/index.md%}): my solutions to the problems of a classic coding interview book. A good playground at any level once Step 1 is done.
