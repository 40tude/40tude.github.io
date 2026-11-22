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













# Proving a Math Theorem with Claude and Lean 4 — A Hands-On Checklist (Windows 11 Sandbox)

## 0. Context

- [ ] **Friday, November 20, 2026**: Levent Alpöge, a mathematician at Anthropic, announced that he believes he has found a proof of the **Riemann Hypothesis** by working with Claude. He also claims the generalized version.
- [ ] **Status as of Sunday, November 22**: the claim is **not yet verified**. Experts are still checking it.
- [ ] **Why this matters for us**: an AI can produce a proof that *looks* right. The question is how anyone can **trust** it. One answer is a **proof assistant** such as **Lean 4**, a program that mechanically checks every step of a proof. If Lean accepts it, the proof is correct, provided the *statement* says what we meant.
- [ ] **Side note (RSA / credit cards)**: even if the Riemann Hypothesis is proven, RSA is not broken. The hypothesis describes how primes are distributed *on average*. It gives no method to factor `n = p × q`, and cryptographers already assumed it was true. The real long-term threat to RSA is quantum computing.
- [ ] **Goal of this article**: reproduce the whole loop on a deliberately trivial theorem, *"the set of even natural numbers is infinite"*:
  1. Claude writes an informal proof.
  2. Claude translates it into Lean 4 + Mathlib.
  3. Lean checks it.
  4. Lean's errors go back to Claude until the proof compiles.

---

## 1. Prepare a clean environment (Windows Sandbox)

- [ ] Do everything inside **Windows Sandbox**, so the main machine stays clean. See my article on installing and creating a sandbox under Windows 11: `TODO: link to the sandbox article`.
- [ ] ⚠️ The sandbox is **ephemeral**. Closing it wipes everything: elan, the Lean toolchain, Mathlib and its multi-GB cache, and your files. **Copy your `.lean` files out before closing.**
- [ ] Install **Git for Windows** inside the sandbox. Lean 4 projects fetch their dependencies, Mathlib included, as Git repositories.
- [ ] Check the shell. The elan installer expects **Command Prompt** or **PowerShell ≥ 7.4.1**. A fresh sandbox only ships Windows PowerShell 5.1, so either use `cmd`, install PowerShell 7, or call `curl.exe` explicitly as shown below.

---

## 2. Install elan (the Lean version manager)

**Mental model (Rust analogy)**: elan is a fork of rustup, so the pieces line up like this:

| Lean      | Rust            | Role                                     |
|-----------|-----------------|------------------------------------------|
| `elan`    | `rustup`        | Installs and selects toolchain versions  |
| `lake`    | `cargo`         | Build tool and package manager           |
| `lean`    | `rustc`         | The compiler / proof checker             |
| `lean-toolchain` | `rust-toolchain.toml` | Pins the Lean version per project |
| `lakefile.toml`  | `Cargo.toml`  | Project configuration and dependencies   |

- [ ] Download and run the installer:

```powershell
# Download the installer (use curl.exe, not the old PowerShell "curl" alias)
curl.exe -O --location https://elan.lean-lang.org/elan-init.ps1
# Run it, bypassing the execution policy for this call only
powershell -ExecutionPolicy Bypass -f elan-init.ps1
# Clean up
del elan-init.ps1
```

- [ ] **Open a new terminal** so the updated `PATH` is picked up.
- [ ] Verify:

```powershell
elan --version
elan show   # lists installed toolchains
```

- [ ] Understand what just happened. elan puts small "shim" executables (`lean.exe`, `lake.exe`) in `%USERPROFILE%\.elan\bin`. When you run `lake` inside a project, the shim reads the project's `lean-toolchain` file, downloads that exact Lean version if it's missing, and then runs it.
- [ ] *(Optional)* Alternative: the **VS Code Lean 4 extension** can install elan for you, and it shows the proof state live while you edit.

---

## 3. Create a Mathlib project

- [ ] Create the project:

```powershell
lake new preuves math
```

  - `preuves` is the project name. lake creates a `preuves\` folder.
  - `math` is the template. It sets up a project that depends on **Mathlib**, Lean's large library of formalized mathematics. The default template would give a bare project.

- [ ] Look at what was generated:

```
preuves\
├── lakefile.toml        # declares the "Preuves" library and requires mathlib
├── lean-toolchain       # pins the Lean version Mathlib needs (v4.36.0 here)
├── Preuves.lean         # library root: a list of imports
└── Preuves\
    └── Basic.lean       # placeholder module (def hello := "world")
```

- [ ] Download the **precompiled Mathlib** cache. Don't skip this: compiling Mathlib from source takes hours.

```powershell
cd preuves
lake exe cache get
```

---

## 4. Ask Claude for an informal proof

- [ ] Prompt:

> Prove that the set of even natural numbers is infinite. Give a rigorous, step-by-step proof.

- [ ] Expected idea: for any natural number `a`, the number `2 × (a + 1)` is even and greater than `a`. The even numbers therefore have no maximum, so the set is infinite.
- [ ] Read the informal proof and make sure **you** understand it before going further.

---

## 5. Ask Claude to formalize it in Lean 4

- [ ] Prompt:

> Formalize this statement and proof in Lean 4 with Mathlib. Use Mathlib's `Even` definition. Target Lean v4.36 and the current Mathlib, which uses the module system.

- [ ] 💡 **Tip**: tell Claude your exact Lean version (from `lean-toolchain`). Models are often trained on older Mathlib releases and may use lemma names that have since been renamed or removed.
- [ ] Reference version of the file, `Preuves\EvenInfinite.lean`:

```lean
module

public import Mathlib

/-!
# The set of even natural numbers is infinite

Statement: `{n : ℕ | Even n}.Infinite`, where `Even n` is Mathlib's
definition `∃ r, n = r + r`.
-/

public section

-- The set of even natural numbers is infinite
theorem evens_infinite : Set.Infinite {n : ℕ | Even n} := by
  -- It suffices to find, for every a, an even number greater than a
  apply Set.infinite_of_forall_exists_gt
  intro a
  -- Witness: 2 * (a + 1), even and strictly greater than a
  exact ⟨2 * (a + 1), ⟨a + 1, by ring⟩, by omega⟩
```

- [ ] `TODO`: paste my final version here, with its two proofs: by contradiction using the maximum of a finite set, and by the injection `k ↦ 2 * k`.

---

## 6. Wire the file into the project

lake does **not** scan the folder for `.lean` files. It builds the library root (`Preuves.lean`) and everything that root imports. **A file nobody imports is ignored.** Rust works the same way: a module not declared from `lib.rs` is never compiled.

- [ ] Put the file in the `Preuves\` subfolder, using UpperCamelCase for the name:

```powershell
Copy-Item .\EvenInfinite.lean .\Preuves\EvenInfinite.lean
```

- [ ] Edit `Preuves.lean`, the library root:

```lean
module

-- import Preuves.Basic     -- template placeholder, no longer needed
import Preuves.EvenInfinite
```

  The module name maps to the path, with `.` instead of `\`: `Preuves.EvenInfinite` → `Preuves\EvenInfinite.lean`.

- [ ] *(Optional)* Delete the placeholder. If you do, also remove or comment out its import, or the build fails with "unknown module":

```powershell
Remove-Item .\Preuves\Basic.lean
```

- [ ] Alternatives for checking a single file without wiring it in:

```powershell
lake build Preuves.EvenInfinite                 # build just this module and its dependencies
lake env lean .\Preuves\EvenInfinite.lean       # check one file directly
```

---

## 7. Build and fix (the real feedback loop)

- [ ] Run:

```powershell
lake build
```

- [ ] When an error appears, copy it back to Claude:

> Here is Lean's error output. Fix the proof.

- [ ] Repeat until the build succeeds. These are the issues I actually hit, in order:

| # | Symptom | Cause | Fix |
|---|---------|-------|-----|
| 1 | `failed to read file '...Mathlib\...\X.olean.private'` + warning *"imports Mathlib ... designed for use with the module system"* | Recent Lean/Mathlib use the **module system**: compiled files are split into a public `.olean` and a private `.olean.private`. A plain (non-module) file that imports Mathlib needs the private parts, and some were missing from the cache. | Add `module` as the **first line** of the file. |
| 2 | `Unknown identifier 'even_zero'` | Claude used an **outdated Mathlib lemma name**. | Use `Even.zero`, or the version-proof term `⟨0, rfl⟩`. Or send the error back to Claude. |
| 3 | Same `.olean.private` error, now on `Preuves.lean` | The **library root** (and `Basic.lean`) were still plain files importing a module. | Add `module` at the top of `Preuves.lean` **and** `Preuves\Basic.lean`. |
| 4 | `Unknown identifier 'Set'` + *"imported privately; consider adding public import"*, followed by cascading errors (`introN failed ... ⊢ sorry`, type mismatch) | `public section` makes the theorem public, so the names in its statement must be imported **publicly**. | Use `public import Mathlib`, or remove `public section`. Fix the **first** error first: the others cascade from it. |
| 5 | Warning *"Copyright too short!"* | Mathlib's style linter expects a Mathlib copyright header. The `math` template turns these linters on in `lakefile.toml` (`weak.linter.mathlibStandardSet = true`). | Harmless; ignore it. Or add `weak.linter.style.header = false` under `[leanOptions]` in `lakefile.toml`. A `set_option` inside the file is too late to silence it. |
| 6 | Warning *"The current module only contains private declarations"* | In a module, declarations are **private by default**. | Harmless for checking proofs. Add `public section` (with a `public import`) to silence it. |

- [ ] Background on `module`, `public` and private: inside a `module`, declarations are private unless marked `public`. A private theorem is still **fully checked**; it just can't be reused by other files.
- [ ] 💡 Fix errors **top-down**. One real error (such as an unknown name in the statement) turns the goal into `sorry`, and that produces confusing follow-up errors.

---

## 8. Validate

- [ ] Success looks like:

```
⚠ [18250/18254] Built Preuves.EvenInfinite
warning: ... Copyright too short!
Build completed successfully (18254 jobs).
```

  `⚠` means "built with warnings only". `Replayed` instead of `Built` means lake reused a cached result because the file didn't change.

- [ ] Check that the output has **no `sorry` warning**. A `sorry` is an admitted, unproven gap.
- [ ] *(Optional)* For a stronger check, add this at the end of the file:

```lean
#print axioms evens_infinite
```

  The output should list only standard axioms (`propext`, `Classical.choice`, `Quot.sound`), never `sorryAx`.

- [ ] ⚠️ **Read the statement yourself.** Lean guarantees the proof proves the statement. It does **not** guarantee the statement says what you meant. Here: `Set.Infinite {n : ℕ | Even n}`, where `Even n` means `∃ r, n = r + r`.

---

## 9. Tear down

- [ ] **Copy your `.lean` files and screenshots out of the sandbox.**
- [ ] Close the sandbox with the **window's X button**, not *Start → Shut down*. Shutting down from inside makes the host show a scary *"Erreur: L'environnement distant est en cours d'arrêt"* dialog. It's harmless, but the X button closes the sandbox cleanly.

---

## 10. Key takeaways (for the article)

- [ ] **Division of labor**: Claude is good at *finding* proofs, and Lean is uncompromising at *checking* them. Together, the loop is *Claude proposes → Lean checks → errors go back to Claude*.
- [ ] **Trust shifts**: you no longer have to trust the AI's reasoning, only Lean's small trusted kernel and your own reading of the **statement**.
- [ ] **Expect friction from versions, not math**: almost every problem above came from tooling and library versions (module system, renamed lemmas, linters), not from the mathematics.
- [ ] **Next step for readers**: try something where Claude's first attempt is likely to fail, such as *the infinitude of primes*.

---

## 11. Conclusion

`TODO: my conclusion — "we live in amazing times", a tipping point...`
