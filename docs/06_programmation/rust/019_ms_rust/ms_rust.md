---
published: true
lang: en-US
layout: default
title: Implementing Microsoft’s Rust Guidelines within VSCode with Claude
description: A step-by-step guide to leveraging Claude and Microsoft's Rust Guidelines in VSCode
parent: Rust
#math: mathjax
date               : 2025-11-19 01:00:00
last_modified_date : 2025-11-19 09:00:00
---



# Implementing Microsoft’s Rust Guidelines within VSCode with Claude
{: .no_toc }

A step-by-step guide to leveraging Claude and Microsoft's Rust Guidelines in VSCode
{: .lead }




<!-- <h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2> -->





## TL;DR
{: .no_toc }
* Automatically enforces Microsoft Rust coding standards through Claude skills
* Simple 3-step setup: download guidelines, create/tune `SKILL.md`, test with a Rust project
* Claude will automatically apply guidelines to all future `.rs` files
* VSCode + Win11 (not tested elsewhere)

<div align="center">
<img src="./assets/img00.webp" alt="" width="900" loading="lazy"/><br/>
<span>A step-by-step guide to leveraging Claude and Microsoft's Rust Guidelines in VSCode.</span>
</div>





## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}






## 0. Prerequisites
In the following I suppose:
- Windows 11
- Rust installed
- VSCode installed
- "Claude Code for VSCode" extension installed
- VSCode is closed

If you run Linux or MacOS, the procedure should be similar except may be, the names of the directories.

## 1. Create the Skill Structure

Open a PowerShell terminal (WIN+X then I)

```powershell
# Create the skill directory
New-Item "$env:USERPROFILE/.claude/skills/ms-rust" -ItemType Directory

# Navigate to the directory
cd "$env:USERPROFILE/.claude/skills/ms-rust"

# Download the Microsoft Rust guidelines
Invoke-WebRequest -Uri "https://microsoft.github.io/rust-guidelines/agents/all.txt" -OutFile "./rust-guidelines.txt"

# To verify
ls
```

If you are not sure of the path to Claude you can use:

```powershell
Test-Path "$env:USERPROFILE\.claude"
```
Read the doc, and try different directories. Indeed Claude Code (CLI) and Claude Desktop (GUI) are 2 different beasts.


**Side Note**
Read the [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/guidelines/index.html). Before becoming a set of recommendations for AI, the Pragmatic Rust Guidelines are first a collection of well-structured, readable guidelines presented as a Rustdoc-generated book.

**Side Note**
A long time ago, when C++11 was introduced, there was a need to help developers transition from"old" to "modern" C++. In addition, there were so many ways to accomplish the same thing in C++ that some form of guidance became necessary. Finally, it was crucial to explain what should be avoided (raw pointer), what should be promoted (RAII), and why. This is why the C++ Core Guidelines were created at that time (Bjarne Stroustrup, [CppCon 2015](https://isocpp.org/blog/2015/09/bjarne-stroustrup-announces-cpp-core-guidelines?utm_source=chatgpt.com)).

If you’re interested, you can [read this page](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines).



## 2. Create the `SKILL.md` File

```powershell
# Open VSCode in this directory
code .
CTRL+ALT+B to close the chat panel on the right
```

**IMPORTANT**: Do not use Notepad as it will modify the file without your knowledge.

In VSCode, create a file named **SKILL.md** (the UPPERCASE letters are important) with the following content from the first 3 dashes to the last 3 dashes inclusive (YAML Frontmatter):




```markdown
---
name: ms-rust
description: ALWAYS use this skill BEFORE writing or modifying ANY Rust code (.rs files), even for simple Hello World programs. Enforces Microsoft Rust coding guidelines, applies M-CANONICAL-DOCS documentation, adds compliance comments, and validates against rust-guidelines.txt. This skill is MANDATORY for all Rust development.
---

# Rust Development
This skill automatically enforces Rust coding standards and best practices when creating or modifying Rust code.

## Instructions

**CRITICAL**: This skill MUST be invoked for ANY Rust code operation, including:
- Creating new .rs files (even simple examples like Hello World)
- Modifying existing .rs files (any change, no matter how small)
- Reviewing Rust code
- Refactoring Rust code

**Process**:
1. Read the [rust-guidelines.txt](rust-guidelines.txt) to understand all compliance requirements
2. Before writing/modifying ANY Rust code, ensure edits are conformant to the guidelines
3. Apply proper M-CANONICAL-DOCS documentation format
4. Add compliance comments
5. Comments must ALWAYS be written in American English, unless the user explicitly requests ‘write comments in French’ or provides another clear instruction specifying a different comment language.
6. If the file is fully compliant, add a comment: `// Rust guideline compliant {date}` where {date} is the guideline date/version

**No exceptions**: Even for trivial code like "Hello World", guidelines must be followed.

---

```


* **⚠️ Warning**: The `name` field must contain only lowercase letters and hyphens (no underscores).
* Based on my experience... I **strongly** recommend to use the same name for the directory and the skill (`ms-rust` in our case.)
* The `description` is important (see the Side Note below) because this is what helps Claude to decide to apply such or such skill. For example here we make clear that the skill apply to "ANY Rust code"
* It seems uppercase matters. I did some tests to confirm it. See ALWAYS, ANY and **CRITICAL** above.
* Be specific. See point 5 for example
* Don't be surprised if you have to iterate few times



### Side Note
{: .no_toc }

For what I understood, here is what happen when starting a conversation
1. The skills are already indexed - Claude does NOT browse the `.claude/skills` directory at the beginning of each conversation. The Claude Code System has already scanned this directory and provided Claude with a list of available skills.
2. Claude receives a prepared list - In its system instructions, there is a <available_skills> section that lists the available skills with their name and description (extracted from `SKILL.md` ). For example:
    ```xml
    <available_skills>
    <skill>
        <name>ms-rust</name>
        <description>ALWAYS use this skill BEFORE writing or modifying ANY Rust code...</description>
    </skill>
    </available_skills>
    ```
3. Claude don't read `SKILL.md` at startup - It never reads the entire contents of `SKILL.md` files before we  ask something. It just see the short description. This is why it is important.
4. Invoking the skill - When Claude decides to use a skill (based on its description), it use the Skill tool with the name of the skill. That's when the entire contents of `SKILL.md` are injected into the conversation.
5. Applying the instructions - Once the skill is invoked, Claude sees all the detailed instructions from `SKILL.md` and have to follow them for the task at hand.

The flow looks like:
* Start of conversation → Claude knows the names/descriptions of the available skills.
* Rust task requested → Claude sees that ms-rust corresponds → It invokes the skill.
* Skill invoked → The complete content of `SKILL.md` appears → It reads and apply the instructions.




### File Organization
{: .no_toc }

```
C:\Users\<your_name>\.claude\
└── skills\
    └── ms-rust\
        ├── SKILL.md
        └── rust-guidelines.txt
```





## 3. Test the Skill

Return to the terminal which should still be open.

```powershell
# Create a test project
cd $env:TEMP
cargo new test_skills
cd test_skills/

# Open the directory in VSCode
code .
```

In VSCode:
1. Close the chat panel that takes up space (CTRL+ALT+B)
2. Open the `src/main.rs` file
3. Open Claude and ask to modify the code, for example:
   - "Modify `main.rs` to display the first 10 Fibonacci numbers"
4. Claude will ask permission to read the `SKILL.md` then `rust-guidelines.txt` and automatically apply the Microsoft Rust guidelines while generating the code.









## 4. Verification

After the modifications have been made, the code should:
- Comply with Microsoft Rust conventions
- Have a `// Rust guideline compliant <date>` comment if fully compliant
- Include appropriate documentation for public functions
- Comments be in English


```rust
// Rust guideline compliant 2025-11-18

/// Prints the first 10 Fibonacci numbers to stdout.
///
/// This function generates and displays the Fibonacci sequence where each number
/// is the sum of the two preceding ones, starting from 0 and 1.
///
/// # Examples
///
/// ```
/// main();
/// // Output:
/// // Fibonacci number 1: 0
/// // Fibonacci number 2: 1
/// // ...
/// // Fibonacci number 10: 34
/// ```
fn main() {
    print_fibonacci(10);
}

/// Calculates and prints the first n Fibonacci numbers.
///
/// # Examples
///
/// ```
/// print_fibonacci(5);
/// ```
///
/// # Panics
///
/// Panics if n is 0.
fn print_fibonacci(n: usize) {
    let mut prev = 0u64;
    let mut curr = 1u64;

    println!("Fibonacci number 1: {}", prev);

    if n > 1 {
        println!("Fibonacci number 2: {}", curr);
    }

    for i in 3..=n {
        let next = prev + curr;
        println!("Fibonacci number {}: {}", i, next);
        prev = curr;
        curr = next;
    }
}
```













## 5. Webliography
* [The Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/guidelines/index.html)
* On Claude Code Docs, read this [page](https://code.claude.com/docs/en/skills)
* About SKILL, read this [page](https://docs.claude.com/en/docs/agents-and-tools/agent-skills/best-practices)
* [C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines).