---
published: true
lang: en-US
layout: default
title: Understanding Gradient Descent
description:
parent: "Machine Learning"
math: mathjax
date:               2025-11-20 15:00:00 +0000
last_modified_date: 2025-11-20 15:00:00 +0000
# nav_order: 9
# permalink: /machine_learning/
---

# Understanding Gradient Descent
{: .no_toc }


<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>



## TL;DR
{: .no_toc }



<div align="center">
<img src="./assets/img00.webp" alt="" width="900" loading="lazy"/><br/>
<span>Gradient Descent.</span>
</div>


## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}






## Introduction

Gradient descent is a fundamental optimization method in machine learning used to minimize a cost (or loss) function.

**Why do we use it?**

The principle is simple: we search for model parameters that minimize the error between predictions and true values. Gradient descent achieves this by iteratively adjusting parameters in the opposite direction of the gradient (the slope) of the cost function. It's like descending a mountain by always following the steepest slope downward.

**Main use cases:**

The algorithm is ubiquitous in ML. It's used to train linear and logistic regressions, where we minimize squared error or cross-entropy. It's also at the core of neural network training via backpropagation, where we compute gradients layer by layer. We also find it in SVMs, recommendation algorithms, and practically every parametric model requiring optimization.

**Common variants:**

Several versions exist: batch gradient descent (uses all data at each iteration, accurate but slow), stochastic gradient descent or SGD (one data point per iteration, fast but noisy), and mini-batch gradient descent (a compromise with a small batch of data). More advanced optimizers like Adam, RMSprop, or AdaGrad improve convergence by dynamically adjusting the learning rate.

The key hyperparameter is the learning rate: too large, we risk diverging; too small, convergence will be very slow.














## The Laser Pointer Game
Imagine you're holding a laser pointer, aiming at a white sheet hung on the opposite wall. Behind that sheet, there's a target at a specific height - but you can't see it. Your friend standing behind the sheet can see both your laser dot and the target, and they shout feedback: "3 inches too high!" or "5 inches below!"

Your goal? Adjust the angle of your laser until you hit the target perfectly.

<div align="center">
<img src="./assets/img01.webp" alt="" width="450" loading="lazy"/><br/>
<span>Gradient Descent.</span>
</div>


Let's use this simple game to understand gradient descent - the fundamental algorithm that powers modern machine learning.

### The Setup
{: .no_toc }

Let's make this concrete with numbers. Say you're standing 10 feet away from the wall, and your laser starts at chest height (let's call that 0 inches for simplicity).

The laser beam follows a straight line that we can describe mathematically as:

$$y = a × x + b$$

Where:
- $$x$$ is the horizontal distance (10 feet to the wall)
- $$y$$ is the height where the laser hits
- $$a$$ is the angle (technically the slope/tangent of the angle)
- $$b$$ is your starting height (0 inches in our case)

Since you're holding the laser steady at chest height, $$b = 0$$, and you can only adjust the angle $$a$$. So our equation simplifies to:

$$y = a × 10$$

### First Attempt
{: .no_toc }

You start by pointing the laser straight ahead (angle = 0). Your friend shouts: "You're 20 inches too low!"

Now you need to adjust. But by how much?

### The Gradient Descent Algorithm
{: .no_toc }

Here's where gradient descent comes in. The algorithm has three key components:

1. **The Error (Loss)**: How far you are from the target
2. **The Gradient**: Which direction and how steeply to adjust
3. **The Learning Rate**: How aggressively to make adjustments

#### Step 1: Calculate the Error
{: .no_toc }

Your friend said "20 inches too low", so:

$$Error = -20$$ (

The error is in inches and it is negative because you're below the target.

#### Step 2: Understanding the Gradient
{: .no_toc }

The gradient tells us how changing the angle $$a$$ affects the height $$y$$. Looking at our equation $$y = a × 10$$, we can see that:
* If we increase $$a$$ by 1, then $$y$$ increases by 10.

We can also say:
* If the variation of $$a$$ is 1, then the variation of $$y$$ is 10
* The variation of $$y$$ is 10 times larger than the variation of $$a$$
* If rather using $$\Delta$$ wich is use to note large variations, we use $$\partial$$, then we can write this as:

$$\frac{\partial y}{\partial a} = 10$$

This is called the "partial derivative" - it's just the rate of change. In our case, it's simply the horizontal distance (10 feet).

#### Step 3: Calculate the Adjustment
{: .no_toc }

The gradient descent formula is:

$$new_a = old_a - \text{learning-rate} × \text{gradient} × \text{error}$$

Let's pick a learning rate of 0.1 (we'll explain why shortly).

$$
\begin{align*}
new_a & = 0 - 0.1 × 10 × (-20) \\
new_a & = 0 + 20 \\
new_a & = 20
\end{align*}
$$

Wait, that seems huge! Let's check: with $$a = 20$$, the height would be $$y = 20 × 10 = 200$$ inches. We'd overshoot dramatically!

This is why choosing the learning rate matters. Let's try $$learning-rate = 0.01$$:

$$
\begin{align*}
new_a & = 0 - 0.01 × 10 × (-20) \\
new_a & = 0 + 2 \\
new_a & = 2
\end{align*}
$$

Now with $$a = 2: y = 2 × 10 = 20$$ inches. Perfect! We hit the target in one shot!



### A More Realistic Scenario
{: .no_toc }

In practice, you usually won't hit the target perfectly in one try. Let's say the target is at 47 inches, and we use $$learning-rate = 0.01$$.

**Iteration 1:**
- Current angle: $$a = 0$$
- Current height: $$y = 0 × 10 = 0$$ inches
- Error: $$0 - 47 = -47$$ inches
- Adjustment: $$-0.01 × 10 × (-47) = 4.7$$
- New angle: $$a = 0 + 4.7 = 4.7$$
- New height: $$y = 4.7 × 10 = 47$$ inches

Again, we got lucky! But let's see what happens if we use a smaller learning rate of 0.005:

**Iteration 1:**
- $$a = 0, y = 0, error = -47$$
- Adjustment: $$-0.005 × 10 × (-47) = 2.35$$
- New angle: $$a = 2.35, new y = 23.5$$ inches
- Friend says: "23.5 inches too low!"

**Iteration 2:**
- $$a = 2.35, y = 23.5, error = -23.5$$
- Adjustment: $$-0.005 × 10 × (-23.5) = 1.175$$
- New angle: $$a  = 3.525, new y = 35.25$$ inches
- Friend says: "11.75 inches too low!"

**Iteration 3:**
- $$a = 3.525, y = 35.25, error = -11.75$$
- Adjustment: $$-0.005 × 10 × (-11.75) = 0.5875$$
- New angle: $$a = 4.1125, new y = 41.125 inches$$
- Friend says: "5.875 inches too low!"

**Iteration 4:**
- $$a = 4.1125, y = 41.125, error = -5.875$$
- Adjustment: $$-0.005 × 10 × (-5.875) = 0.29375$$
- New angle: $$a = 4.40625, new y = 44.0625$$ inches
- Friend says: "2.9375 inches too low!"

You can see the pattern: each time, we get closer to the target, and the adjustments get smaller as the error decreases. After about 10 iterations, we'd be within a fraction of an inch!

### Key Insights
{: .no_toc }

This simple laser pointer game reveals the core principles of gradient descent:

**1. Iterative Improvement**: We don't need to solve for the perfect answer directly. We make repeated small adjustments.

**2. The Gradient Guides Us**: The gradient ($$\frac{\partial y}{\partial a} = 10$$) tells us the relationship between our adjustable parameter and the outcome.

**3. Learning Rate is Critical**: Too large, and we overshoot. Too small, and we take forever to converge. It's a [Goldilocks problem](https://en.wikipedia.org/wiki/Goldilocks_principle).

**4. Errors Shrink Over Time**: As we get closer to the target, the error gets smaller, which naturally makes our adjustments smaller too.










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


## From Lasers to Machine Learning

In machine learning, instead of a laser angle, we're adjusting weights in a neural network. Instead of hitting a target on a wall, we're trying to minimize prediction errors across thousands or millions of data points. Instead of one parameter $$a$$, we might have millions of parameters.

But the fundamental principle remains exactly the same: calculate the error, find the gradient, make a small adjustment, repeat.

That's the beauty of gradient descent - it's a simple idea that scales to solve incredibly complex problems.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Conclusion


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Webliography