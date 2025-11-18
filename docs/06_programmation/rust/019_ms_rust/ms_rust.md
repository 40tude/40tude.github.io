---
published: true
lang: en-US
layout: default
title: Implementing Microsoft’s Rust Guidelines within VSCode with Claude
description: A step-by-step guide to leveraging Claude and Microsoft's Rust Guidelines in VSCode
parent: Rust
#math: mathjax
date               : 2025-11-19 00:00:00
last_modified_date : 2025-11-19 00:00:00
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
* Simple 3-step setup: download guidelines, create SKILL.md, test with a Rust project
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
- Windows 11
- Rust installed
- VSCode installed
- "Claude Code for VSCode" extension installed
- VSCode closed


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


## 2. Create the `SKILL.md` File

```powershell
# Open VSCode in this directory
code .
CTRL+ALT+B to close the chat panel on the right
```

**IMPORTANT**: Do not use Notepad as it will modify the file without your knowledge.

In VSCode, create a file named **SKILL.md** (the UPPERCASE letters are important) with the following content from the first 3 dashes to the last 3 dashes inclusive:




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
* The description is important because this is what helps Claude to decide to apply such or such skill. For example here we make clear that the skill apply to "ANY Rust code"
* It seems uppercase matters
* Be specific. See point 5 for example
* Don't be surprised if you have to iterate few times



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
4. Claude will ask permission to read the guideline and automatically apply the Microsoft Rust guidelines while generating the code
5. Comments are in English as expected








## 4. Verification

After a modification, the code should:
- Comply with Microsoft Rust conventions
- Have a `// Rust guideline compliant X.X` comment if fully compliant
- Include appropriate documentation for public functions


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
* On Claude Code Docs, read this [page](https://code.claude.com/docs/en/skills)
