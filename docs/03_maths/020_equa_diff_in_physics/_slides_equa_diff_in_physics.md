---
title: "From Derivatives to Action: How Physics Describes Change"
theme: night
highlightTheme: monokai
revealOptions:
  transition: slide
  slideNumber: true
  controls: true
  progress: true
  center: true
  mathjax3:
    mathjax: "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"
    loader:
      load:
        - output/chtml
---

<style>
  .reveal { font-size: 28px; }
  .reveal h1 { font-size: 1.6em; line-height: 1.2; }
  .reveal h2 { font-size: 1.1em; }
  .reveal h3 { font-size: 0.9em; }
  .reveal p, .reveal li { font-size: 0.9em; }
  .reveal blockquote { font-size: 0.85em; }
  .reveal table { font-size: 0.8em; }
  .reveal pre, .reveal code { font-size: 0.75em; }

  /* Images */
  .reveal section img {
    max-width: 85%;
    max-height: 45vh;
    object-fit: contain;
    border-radius: 8px;
  }
  .reveal section img.img-full {
    max-height: 65vh;
  }
</style>

<!-- =====================================================
     TITLE
     ===================================================== -->

# From Derivatives to Action

### How Physics Describes Change

> Building intuition from local laws, differential equations,
> and variational principles

<small>40tude · 2026</small>

---

<!-- COVER IMAGE -->

<img src="assets/img00.webp" class="img-full">

> Among infinitely many possible trajectories, nature selects
> the one that **extremizes the action**.

---

## Roadmap

1. Why derivatives appear in physics
2. Heat, waves & quantum mechanics — a shared skeleton
3. Variational principles — the global view
4. Principle of stationary action & the Lagrangian
5. Examples: free particle, gravity, harmonic oscillator
6. Deriving $F = ma$ from $\delta S = 0$
7. Maxwell's equations from a single action
8. Feynman path integrals & the quantum connection

---

<!-- =====================================================
     PART 1 — WHY DERIVATIVES?
     ===================================================== -->

## Part 1

# Why Derivatives and
# Differential Equations?

---

## Physics Describes **Change**

Most physical phenomena involve change:

- Position changes with time
- Temperature changes in space
- Velocity changes due to forces
- Electric fields vary in space and time

The cleanest tool to describe change is the **derivative**.

$$v = \frac{dx}{dt} \qquad a = \frac{d^2x}{dt^2}$$

---

## Velocity & Acceleration — A Visual

<img src="assets/img21.webp">

<small>Position, speed and acceleration — the first two derivatives of $x(t)$</small>

---

## First-Order Equations → **Flow**

When the *rate of change* depends on the *current value*:

$$\frac{dN}{dt} = -\lambda N \qquad \text{(radioactive decay)}$$

Solution: exponential decay $N(t) = N_0\,e^{-\lambda t}$

<img src="assets/img09.webp">

<small>The simplest first-order ODE hidden behind a percentage!</small>

---

## Second-Order Equations → **Dynamics**

Newton's 2nd law states that force changes the **rate of change** of position:

$$F = m\,a = m\,\frac{d^2x}{dt^2}$$

<img src="assets/img07.webp">

---

## The Physics of **"Memory"**

A second-order system needs *two* initial conditions to predict the future:

| What we need | Meaning |
|---|---|
| Position $x(t_0)$ | Where it is |
| Velocity $\dot{x}(t_0)$ | Where it came from |

> A first-order universe has **no inertia**.
> A second-order universe **remembers its velocity**.

---

## How Did Newton Find $F = ma$?

**Galileo (1589+)** — 100 years before Newton — measured:

- Objects fall with *constant* acceleration
- Motion without forces ⟹ *constant* velocity

Experiments then showed:

$$F \times 2 \Rightarrow a \times 2 \qquad m \times 2 \Rightarrow a \div 2$$

$$\therefore \quad a \propto \frac{F}{m} \quad \Longrightarrow \quad F = ma$$

> **An empirical law, not a guess.**

---

## Why Physics Prefers **Local** Laws

Instead of
> *"Temperature everywhere depends on the whole system"*

nature says
> *"What happens here depends on what is happening just next to it."*

$$\text{locality} \quad \Longrightarrow \quad \text{derivatives}$$

---

<!-- =====================================================
     PART 2 — HEAT EQUATION
     ===================================================== -->

## Part 2

# A Concrete Example:
# The Heat Equation

---

## Two Physical Statements

**1. Fourier's Law** — heat flows from hot to cold:

$$q = -k\,\nabla T$$

**2. Energy conservation** — what enters must stay or leave:

$$\frac{\partial T}{\partial t} = -\nabla \cdot q$$

Combine them → **the heat equation**:

$$\boxed{\frac{\partial T}{\partial t} = \kappa\,\nabla^2 T}$$

---

## The Laplacian $\nabla^2$ — Intuition

$$\nabla^2 T = \frac{\partial^2 T}{\partial x^2} + \frac{\partial^2 T}{\partial y^2} + \frac{\partial^2 T}{\partial z^2}$$

It measures **how a point differs from its neighbors**:

| Sign of $\nabla^2 T$ | Meaning |
|---|---|
| $< 0$ | Hotter than surroundings → heat flows **out** |
| $> 0$ | Colder than surroundings → heat flows **in** |

<img src="assets/img10.webp">

---

<!-- =====================================================
     PART 3 — SHARED SKELETON
     ===================================================== -->

## Part 3

# Why Heat, Waves & Quantum Mechanics
# Look So Similar

---

## One Skeleton, Three Theories

The pattern **time evolution = spatial curvature** appears in:

| System | Equation | Behavior |
|---|---|---|
| Heat | $\partial_t T = \kappa\,\nabla^2 T$ | Diffusion |
| Waves | $\partial_t^2 u = c^2\,\nabla^2 u$ | Oscillations |
| Quantum | $i\hbar\,\partial_t\psi = -\frac{\hbar^2}{2m}\nabla^2\psi + V\psi$ | Probability waves |

> Same mathematical ingredient — different physics.

---

## Why the Laplacian Everywhere?

**1. Locality** — atoms interact only with neighbors.
The Laplacian compares a point to its neighbors.

**2. Symmetry** — laws must be invariant under translations and rotations.
The simplest such operator is $\nabla^2$.

**3. Energy minimization** — many systems minimize energy.
Energy contains $(\nabla u)^2$; minimizing it yields $\nabla^2 u$.

---

## Hills, Valleys & the Second Derivative

<img src="assets/img08.webp">

<small>$u'' > 0$ → valley &nbsp;|&nbsp; $u'' < 0$ → hill &nbsp;|&nbsp; Many physical processes react to these differences.</small>

---

## Three Deep Reasons for Differential Equations

1. **Locality** — what happens here depends on nearby values → derivatives in space
2. **Continuous time evolution** — future depends on current rate of change → derivatives in time
3. **Conservation laws** — energy, momentum, charge → expressed as divergences and derivatives

---

<!-- =====================================================
     PART 4 — VARIATIONAL PRINCIPLES
     ===================================================== -->

## Part 4

# A Deeper Perspective:
# Variational Principles

---

<img src="assets/img02.webp" class="img-full">

<small>From differential equations to variational principles — two languages for the same physics.</small>

---

## Local vs Global Description

| Local (Newton/Leibniz) | Global (Lagrange/Hamilton) |
|---|---|
| "Force causes acceleration at every instant" | "Among all possible paths, find the special one" |
| Differential equation | Extremum of a functional |
| Step by step in time | The entire trajectory at once |

> **Both descriptions are equivalent.**

---

## What is a Variational Principle?

Any statement of the form:

> *The physical solution is the one that makes a certain quantity **stationary**.*

$$\delta\,\mathcal{F} = 0$$

where $\mathcal{F}$ is a **functional** — a function of functions.

Examples beyond physics: shortest path (geodesics), minimal surfaces, optimal control...

---

## Why Extremize Anything?

Extremizing an integral is a **compact, global way to encode differential equations**.

For a functional $S[q] = \int_{t_1}^{t_2} L(q,\dot{q},t)\,dt$,
the condition $\delta S = 0$ with fixed endpoints gives the **Euler–Lagrange equations**:

$$\frac{d}{dt}\!\left(\frac{\partial L}{\partial \dot{q}}\right) - \frac{\partial L}{\partial q} = 0$$

> Not new physics — a more **elegant reformulation**.

---

<!-- =====================================================
     PART 5 — STATIONARY ACTION
     ===================================================== -->

## Part 5

# The Principle of Stationary Action

---

## The Action $S$

$$\boxed{S = \int_{t_1}^{t_2} L(x,\dot{x},t)\,dt}$$

- $S$ is a **score** for each possible trajectory
- $L$ is the **Lagrangian** — $L = T - V$ (kinetic minus potential energy)
- The **real** trajectory makes $\delta S = 0$

> "Stationary" = minimum, maximum, or saddle point.
> Correct name: **principle of stationary action**.

---

## Why $L = T - V$ and Not $T + V$?

$T + V = E$ is **conserved** — the same for *all* paths.
It cannot distinguish the real trajectory from the fake ones.

$T - V$ measures the **imbalance** between motion and position.
Minimizing the cumulative imbalance recovers Newton's laws.

> Historically: discovered by Lagrange & Hamilton through trial and error
> to make Euler–Lagrange equations reproduce $F = ma$.

---

## Historical Timeline

| Person | Year | Contribution |
|---|---|---|
| Fermat | ~1660 | Least time in optics |
| Maupertuis | 1744 | "Least action" — $\int p\,dq$ |
| Lagrange | 1760+ | Formalized $\delta S = 0$ |
| Hamilton | 1834 | $S = \int (T-V)\,dt$ |
| Noether | 1915 | Symmetry → conservation laws |
| Feynman | 1948 | Path integral formulation |

---

<!-- =====================================================
     PART 6 — EXAMPLES
     ===================================================== -->

## Part 6

# Three Examples

---

## Example 1: Free Particle

<img src="assets/img05.webp">

No forces → $V=0$, so $L = \frac{1}{2}m\dot{x}^2$

---

## Free Particle — Step by Step

**Action:** $S = \int \frac{1}{2}m\dot{x}^2\,dt$

**Perturb the path:** $x \to x + \varepsilon\eta$ (with $\eta(t_1)=\eta(t_2)=0$)

**First variation:** $\delta S = \varepsilon m\int \dot{x}\dot{\eta}\,dt$

**Integration by parts:** $= -\varepsilon m\int \ddot{x}\,\eta\,dt$

**For $\delta S = 0$ for any $\eta$:**

$$\ddot{x} = 0 \quad \Longrightarrow \quad x(t) = vt + x_0$$

> **Newton's First Law!**

---

## Candidate Paths vs Minimum Action

<img src="assets/img20.webp" class="img-full">

<small>The physical straight-line path (blue) has the **minimum** action among all candidate paths.</small>

---

## Example 2: Particle in Gravity

<img src="assets/img06.webp">

$L = \frac{1}{2}m\dot{x}^2 - mgx$

Same variational procedure → $m\ddot{x} = -mg$ → $\ddot{x} = -g$

> **Free fall equation recovered!**

---

## Example 3: Harmonic Oscillator

<img src="assets/img17.webp">

Mass on a spring: $L = \frac{1}{2}m\dot{x}^2 - \frac{1}{2}kx^2$

Euler–Lagrange gives:

$$m\ddot{x} + kx = 0 \quad \Longrightarrow \quad x(t) = A\cos(\omega t + \varphi),\; \omega = \sqrt{\frac{k}{m}}$$

---

## Harmonic Oscillator — Energy Exchange

<img src="assets/img19.webp" class="img-full">

<small>$T$ and $V$ exchange continuously — total energy $E = T + V$ stays perfectly **constant**.</small>

---

<!-- =====================================================
     PART 7 — EULER-LAGRANGE & F=ma
     ===================================================== -->

## Part 7

# Deriving $F = ma$ from $\delta S = 0$

---

## The Euler–Lagrange Equation

Starting from $\delta S = 0$ and integrating by parts:

$$\boxed{\frac{d}{dt}\!\left(\frac{\partial L}{\partial \dot{x}}\right) - \frac{\partial L}{\partial x} = 0}$$

For $L = \frac{1}{2}m\dot{x}^2 - V(x)$:

$$\frac{\partial L}{\partial \dot{x}} = m\dot{x} \qquad \frac{\partial L}{\partial x} = -\frac{dV}{dx}$$

---

## Newton's Second Law Recovered

Substituting into the Euler–Lagrange equation:

$$m\ddot{x} - \!\left(-\frac{dV}{dx}\right) = 0$$

$$m\ddot{x} = -\frac{dV}{dx} = F$$

$$\boxed{F = ma}$$

> Newton's laws are **consequences** of a deeper principle.

---

<img src="assets/img01.webp" class="img-full">

<small>Different paths to get back to base — but only one is the most efficient.</small>

---

<!-- =====================================================
     PART 8 — MAXWELL'S EQUATIONS
     ===================================================== -->

## Part 8

# Maxwell's Equations from a Single Action

---

<img src="assets/img18.webp" class="img-full">

<small>From particles to fields — the variational principle scales up.</small>

---

## From Particles to Fields

In mechanics, we vary a **trajectory** $x(t)$.

In electromagnetism, we vary **fields** $\phi(x,t)$ and $\mathbf{A}(x,t)$:

$$\mathbf{E} = -\nabla\phi - \frac{\partial\mathbf{A}}{\partial t} \qquad \mathbf{B} = \nabla\times\mathbf{A}$$

The action integrates over **all of space and time**:

$$S = \int \mathcal{L}(E, B, \rho, J)\;d^3x\,dt$$

---

## The Electromagnetic Lagrangian Density

$$\mathcal{L} = \frac{\epsilon_0}{2}\!\left(E^2 - c^2 B^2\right) - \rho\phi - \mathbf{J}\cdot\mathbf{A}$$

| Term | Meaning |
|---|---|
| $\frac{\epsilon_0}{2}E^2$ | Energy stored in the electric field |
| $-\frac{\epsilon_0}{2}c^2 B^2$ | Energy stored in the magnetic field |
| $-\rho\phi$ | Charge–scalar potential interaction |
| $-\mathbf{J}\cdot\mathbf{A}$ | Current–vector potential interaction |

---

## All Four Maxwell Equations from $\delta S = 0$

**Vary $\phi$:**
$$\nabla\cdot\mathbf{E} = \frac{\rho}{\epsilon_0} \quad \text{(Gauss's law)}$$

**Vary $\mathbf{A}$:**
$$\nabla\times\mathbf{B} = \mu_0\mathbf{J} + \frac{1}{c^2}\frac{\partial\mathbf{E}}{\partial t} \quad \text{(Ampère–Maxwell)}$$

**Built into the definitions of $\mathbf{E}$ and $\mathbf{B}$:**
$$\nabla\cdot\mathbf{B} = 0 \qquad \nabla\times\mathbf{E} = -\frac{\partial\mathbf{B}}{\partial t}$$

---

## A Beautiful Consequence: Light

Remove charges ($\rho=0$, $\mathbf{J}=0$) and combine Maxwell's equations:

$$\nabla^2\mathbf{E} = \frac{1}{c^2}\frac{\partial^2\mathbf{E}}{\partial t^2}$$

This is a **wave equation** — electromagnetic fields propagate at speed:

$$c = \frac{1}{\sqrt{\mu_0\epsilon_0}}$$

> **Maxwell predicted that light is an electromagnetic wave.**

---

## Why the Action is So Powerful

- **Dynamics** — describes how systems *evolve in time*
- **Generality** — works for particles, fields, relativity, quantum theory
- **Symmetry** — via Emmy Noether's theorem (1915):

$$\text{Every symmetry of the action} \;\Longrightarrow\; \text{a conservation law}$$

| Symmetry | Conservation Law |
|---|---|
| Time invariance | Energy |
| Space invariance | Momentum |
| Rotation invariance | Angular Momentum |

---

<!-- =====================================================
     PART 9 — FEYNMAN
     ===================================================== -->

## Part 9

# The Quantum Connection:
# Feynman Path Integrals

---

## Classical vs Quantum

**Classical:**
The particle follows *one* path — the one where $\delta S = 0$.

**Quantum (Feynman, 1948):**
The particle explores *all possible paths*.
Each path contributes with a complex amplitude:

$$\text{Amplitude} \sim e^{iS/\hbar}$$

The total amplitude is the **sum over all paths**.

---

## Why Classical Physics Emerges

When $\hbar \to 0$ (or equivalently at large scales):

- Most paths have *wildly varying* phases → **destructive interference**
- Near the classical path, phases vary slowly → **constructive interference**

$$\delta S = 0 \quad \Longleftrightarrow \quad \text{maximum constructive interference}$$

> The principle of stationary action is the **classical limit** of quantum mechanics.

---

<!-- =====================================================
     CONCLUSION
     ===================================================== -->

## Conclusion

# The Big Picture

---

## Two Languages, One Physics

```
Differential equations          Variational principles
(local description)             (global description)

"At every instant,              "Among all possible paths,
 acceleration = force/mass"      find the one that matters"

        ↕  equivalent  ↕
```

Both encode the **same physical laws** — choose whichever reveals more structure.

---

## From One Principle, Everything

$$\boxed{\delta S = 0} \qquad S = \int_{t_1}^{t_2} L(x,\dot{x},t)\,dt$$

- Newton's laws ✓
- Maxwell's equations ✓
- Schrödinger equation ✓
- General Relativity ✓
- Standard Model ✓

> *Nature seems to follow a universal rule:*
> *the laws of physics come from making an action stationary.*

---

## A Surprising Generalization

The same logic appears **beyond physics**:

- **Neural networks** minimize a loss function over all parameter configurations
- **Reinforcement learning** agents extremize a cumulative reward over sequences of decisions

> *Nature and intelligence share the same underlying move:*
> *among all possible paths, find the one that matters.*
>
> That is not a metaphor. It is the same mathematics.

---

## Summary

| Concept | Key formula | What it tells us |
|---|---|---|
| 1st derivative | $\dot{x} = dx/dt$ | Rate of change / flow |
| 2nd derivative | $\ddot{x} = d^2x/dt^2$ | Curvature / inertia |
| Laplacian | $\nabla^2 u$ | Difference from neighbors |
| Action | $S = \int L\,dt$ | Score for a trajectory |
| Stationarity | $\delta S = 0$ | Selects the real trajectory |
| Noether's theorem | symmetry ↔ conservation | Deepest structure of physics |

---

<!-- FINAL IMAGE -->

<img src="assets/img00.webp" class="img-full">

> *Among infinitely many possible trajectories,*
> *nature selects the one that extremizes the action.*

### Thank you

<small>40tude · 2026 · <em>From Derivatives to Action: How Physics Describes Change</em></small>
