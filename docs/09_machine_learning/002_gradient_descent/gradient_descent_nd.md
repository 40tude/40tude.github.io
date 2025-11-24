---
published: true
lang: en-US
layout: default
title: Understanding Gradient Descent in N Dimensions
description: Generalizing to real-world machine learning problems
parent: "Machine Learning"
math: mathjax
date:               2025-11-24 12:00:00 +0000
last_modified_date: 2025-11-24 12:00:00 +0000
# nav_order: 9
# permalink: /machine_learning/
---

# Understanding Gradient Descent in N Dimensions
{: .no_toc }

Generalizing to real-world machine learning problems
{: .lead }


<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>


## TL;DR
{: .no_toc }

* The gradient is a vector pointing "uphill" in N-dimensional space
* Each dimension is updated independently using partial derivatives
* **Critical point**: Features must be on similar scales or one will dominate
* Feature normalization ensures all parameters contribute fairly
* The math generalizes beautifully: same principle, just more dimensions


<div align="center">
<img src="./assets/img00.webp" alt="" width="450" loading="lazy"/><br/>
<span>Gradient Descent in N Dimensions.</span>
</div>


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

You know what's beautiful about mathematics? Sometimes, when you understand something in one dimension, and then in two dimensions, you've already understood everything. Going from 2D to 42D is just... more of the same.

Let me show you why.

In [Part 1]({%link docs/09_machine_learning/002_gradient_descent/gradient_descent.md%}), we adjusted a single laser pointer position. In [Part 2]({%link docs/09_machine_learning/002_gradient_descent/gradient_descent_2d.md%}), we controlled both $$x$$ and $$y$$ coordinates. Now? We're going to control as many parameters as we want.

And here's the secret: **if you understood the 2D case, you've already understood the N-dimensional case**. The principle is identical. The only difference? More variables to track, and one critical new problem we need to address: **scale** between the dimensions.

But don't worry. As usual, we'll start gently, build intuition, and by the end you'll see that high-dimensional gradient descent is just "rinse and repeat" with a few important tweaks.

<!-- This post will give you the solid foundations you need before diving into more advanced books like Aurélien Géron's "Hands-On Machine Learning with Scikit-Learn, Keras, and TensorFlow." Think of this as your launchpad. -->




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## The Pattern We've Discovered

Let's recap what we learned so far:

**In Episode 1 (1D)**, we had:
- One variable: $$y\_laser$$
- One error: $$error = y\_laser - y\_target$$
- Cost function: $$C = error^2$$
- One gradient: $$\frac{dJ}{dy\_laser} = 2 \cdot error$$
- Update rule: $$y\_new = y\_old - \alpha \cdot \text{gradient}$$

**In Episode 2 (2D)**, we had:
- Two independent variables: $$x\_laser$$ and $$y\_laser$$
- Two errors: $$error\_x$$ and $$error\_y$$
- Cost function: $$C = \frac{1}{2}(error\_x^2 + error\_y^2)$$
- Two gradients: $$\frac{\partial C}{\partial error\_x}$$ and $$\frac{\partial C}{\partial error\_y}$$
- Two update rules: one for x, one for y

Do you see the pattern? **Each dimension is independent.** We calculate the gradient for each variable separately, and we update each one using the exact same formula.

In N dimensions? Same thing. Just... more of them.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## What Really Is a Gradient?

Before we jump to N dimensions, let's clarify one crucial concept: **what is a gradient?**

In 1D, the gradient was just a number telling us "how steep is the slope?" Positive means the function goes up, negative means it goes down.

In 2D, the gradient became a **vector** with two components:

$$\nabla C = \begin{bmatrix} \frac{\partial C}{\partial x} \\ \frac{\partial C}{\partial y} \end{bmatrix}$$

This vector points in the direction of **steepest ascent**. It's like a compass that always points "uphill."

<div align="center">
<img src="./assets/img30.webp" alt="" width="450" loading="lazy"/><br/>
<span>The gradient vector points uphill.</span>
</div>

Since we want to go **downhill** (minimize the cost), we move in the **opposite direction**: $$-\nabla C$$.

In N dimensions? The gradient is still a vector but it has N components:

$$\nabla C = \begin{bmatrix} \frac{\partial C}{\partial w\_1} \\ \frac{\partial C}{\partial w\_2} \\ \vdots \\ \frac{\partial C}{\partial w\_N} \end{bmatrix}$$

Where $$w\_1, w\_2, ..., w\_N$$ are our N parameters (weights).

**Key insight**: The gradient is a **direction** in N-dimensional space. It tells us which way to adjust all N parameters simultaneously to reduce the cost most effectively.


Imagine... Imagine you're in the Alps, in Serre-Chevalier. The name of the resort is important—otherwise this example doesn't work. You and your snowboard are at the top of the "Cucumelle" slope (a red run). Don't ask me why, but you're looking for the path with the steepest descent. Believe it or not, you're mentally computing the gradient at the spot where you're sitting. You won't traverse the slope from left to right because in that case you'd be almost perpendicular to the fall line and your speed would be limited. No—instead, you plan to point the nose of your board straight downhill, go like that for 200 meters, then carve to the right... You get the idea.

Have you ever watched a drop of water in a sink? It always follows the path where the gradient is strongest. Your are doing exactly the same thing (and may be, visit the local hospital later today) but on the snow.


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## N Dimensions = N Independent Dials

Now, imagine you're standing in front of a complex control panel with N dials. Each dial controls one aspect of a machine's output. Your job? Adjust all N dials until the machine produces the perfect output.


<div align="center">
<img src="./assets/img31.webp" alt="" width="450" loading="lazy"/><br/>
<span>N independent parameters to optimize.</span>
</div>

Here's the beautiful part: **each dial is independent**. Turning dial #3 doesn't magically change what dial #7 does. They all contribute to the final output independently.

Mathematically, if we have N parameters $$w\_1, w\_2, ..., w\_N$$, our prediction is:

$$\hat{y} = w\_1 \cdot x\_1 + w\_2 \cdot x\_2 + ... + w\_N \cdot x\_N$$

Or in more compact notation:

$$\hat{y} = \sum_{i=1}^{N} w\_i \cdot x\_i$$

**STOP!**. Did you *look at* the previous formula or did you read it? Nothing personal but I don't trust you. Could you please, give ma a favor? Say it loud : "$$y$$ hat, the predicted value, is the sum for $$i$$ equal one to $$i$$ equal $$N$$ of each observation $$x_i$$ multiplied by the value of ith knob.

The error is now:

$$error = \hat{y} - y\_true$$

And our cost function (using the squared error):

$$C = \frac{1}{2} \cdot error^2 = \frac{1}{2} \left( \sum_{i=1}^{N} w\_i \cdot x\_i - y\_true \right)^2$$

Do not start tp grumble. Remember the Alamo and remember what we had in 2D. It looks like that:

<div align="center">
<img src="./assets/img21.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>N independent parameters to optimize.</span> -->
</div>

Then we said the expression of $$C$$ was to complicated and we wrote : $$C = \frac{1}{2} \cdot ({error\_x}^2 + {error\_y}^2)$$. What you see above is nothing more than the same expression but in N rather than 2 dimensions. Again, don't look at, read and said it loud as if you were exampling it the formula to your invisible friend (good luck if you mother or wife rush into the room and ask you what's going on here!)

To minimize this cost in N dimensions, we need to compute the gradient with respect to **each** parameter $$w\_i$$:

$$\frac{\partial C}{\partial w\_i} = error \cdot x\_i$$

And then update each parameter:

$$w\_i\_new = w\_i\_old - \alpha \cdot \frac{\partial C}{\partial w\_i}$$

See? It's exactly what we did in 1D and 2D. Just... repeated N times.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## But There's a Catch: The Scale Problem

Now, here's where things get tricky. And this is **crucial** for real-world applications.

Imagine you have these four features:
- Study hours: values between 0 and 20
- Sleep hours: values between 4 and 10
- Classes missed: values between 0 and 15
- Previous exam score: values between 0 and 100

Let's say you're trying to predict a student's final exam score using this equation:

$$\text{exam\_score} = w\_1 \cdot \text{study} + w\_2 \cdot \text{sleep} + w\_3 \cdot \text{missed} + w\_4 \cdot \text{previous}$$

Now, suppose the student studied 10 hours, slept 7 hours, missed 2 classes, and scored 75 on the previous exam.

If all weights start at 1.0, the prediction is:

$$\hat{y} = 1 \cdot 10 + 1 \cdot 7 + 1 \cdot 2 + 1 \cdot 75 = 94$$

**Do you see the problem?** The "previous exam score" contributes 75 to the prediction, while "study hours" only contributes 10. The gradient will be dominated by $$w\_4$$ because $$x\_4$$ is so much larger!

Let me show you what happens to the gradients:

$$\frac{\partial C}{\partial w\_1} = error \cdot 10$$

$$\frac{\partial C}{\partial w\_4} = error \cdot 75$$

The gradient for $$w\_4$$ is **7.5 times larger** than for $$w\_1$$, not because $$w\_4$$ is more important, but simply because the feature values are on different scales.

This causes a nasty problem: gradient descent will spend most of its effort adjusting $$w\_4$$ and will barely touch $$w\_1$$, $$w\_2$$, and $$w\_3$$.

<!-- <div align="center">
<img src="./assets/img32.webp" alt="" width="600" loading="lazy"/><br/>
<span>Without normalization, one dimension dominates.</span>
</div> -->

Visually, imagine trying to navigate down a long, narrow valley. If the scales are different, gradient descent will "zigzag" down the valley instead of taking the direct path. It's inefficient and slow.

**The solution?** Feature normalization.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## The Fix: Feature Normalization

The idea is simple: **put all features on the same scale** before training.

The most common method is called **standardization** (also known as z-score normalization):

$$x\_i\_normalized = \frac{x\_i - \mu\_i}{\sigma\_i}$$

Where:
- $$\mu\_i$$ is the mean of feature $$i$$
- $$\sigma\_i$$ is the standard deviation of feature $$i$$

After standardization, each feature has:
- Mean = 0
- Standard deviation = 1

This puts all features on equal footing. Now the gradients reflect the **true importance** of each feature, not just their arbitrary scales.

**Side Note:** Another popular method is min-max scaling where we write

$$x\_i\_normalized = \frac{x\_i - \min(x\_i)}{\max(x\_i) - \min(x\_i)}$$

Sorry to insist. Do NOT look at the formula. Read it, understand it, explain it to yourself and say it loud. It scales all features to the range [0, 1].

**Which one to use?** Standardization is generally preferred because it handles outliers. Outliers are the guys that are very far from average. Think of Victor Wembanyama, 2.24m, in a bus of supporters. Anyway, standardization is better because it is less sensitive to the exact min/max values in your dataset.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Let's play with Python

Let's build a complete example with real code to predicting student exam scores. We'll predict final exam scores based on four features, and we'll compare gradient descent **with** and **without** feature normalization.

```python
import numpy as np
import matplotlib.pyplot as plt

# Set random seed for reproducibility
np.random.seed(42)

# Generate synthetic student data
n_students = 100

# Features with different scales
study_hours = np.random.uniform(0, 20, n_students)          # 0-20 range
sleep_hours = np.random.uniform(4, 10, n_students)          # 4-10 range
classes_missed = np.random.randint(0, 15, n_students)       # 0-15 range
previous_score = np.random.uniform(40, 100, n_students)     # 40-100 range

# True relationship (with some noise)
# exam_score = 2*study + 5*sleep - 3*missed + 0.3*previous + noise
exam_score = (2 * study_hours +
              5 * sleep_hours -
              3 * classes_missed +
              0.3 * previous_score +
              np.random.normal(0, 5, n_students))

# Combine features into a matrix
X = np.column_stack([study_hours, sleep_hours, classes_missed, previous_score])
y = exam_score

# Add bias term (column of ones)
X = np.column_stack([np.ones(n_students), X])

print("Feature ranges (before normalization):")
print(f"Study hours: [{study_hours.min():.1f}, {study_hours.max():.1f}]")
print(f"Sleep hours: [{sleep_hours.min():.1f}, {sleep_hours.max():.1f}]")
print(f"Classes missed: [{classes_missed.min()}, {classes_missed.max()}]")
print(f"Previous score: [{previous_score.min():.1f}, {previous_score.max():.1f}]")
print()

# Function to perform gradient descent
def gradient_descent(X, y, learning_rate, n_iterations):
    """
    Perform gradient descent on the dataset.

    Args:
        X: Feature matrix (n_samples, n_features)
        y: Target vector (n_samples,)
        learning_rate: Step size alpha
        n_iterations: Number of iterations

    Returns:
        weights: Final weight vector
        cost_history: Cost at each iteration
    """
    n_samples, n_features = X.shape
    weights = np.zeros(n_features)
    cost_history = []

    for iteration in range(n_iterations):
        # Make predictions
        predictions = X @ weights

        # Calculate error
        errors = predictions - y

        # Calculate cost (MSE)
        cost = (1 / (2 * n_samples)) * np.sum(errors ** 2)
        cost_history.append(cost)

        # Calculate gradient
        gradient = (1 / n_samples) * (X.T @ errors)

        # Update weights
        weights = weights - learning_rate * gradient

    return weights, cost_history


# Train WITHOUT normalization
print("=" * 60)
print("GRADIENT DESCENT WITHOUT NORMALIZATION")
print("=" * 60)

learning_rate_unnorm = 0.0001  # Need very small LR!
n_iterations = 1000

weights_unnorm, cost_unnorm = gradient_descent(
    X, y, learning_rate_unnorm, n_iterations
)

print(f"\nLearning rate: {learning_rate_unnorm}")
print(f"Final weights: {weights_unnorm}")
print(f"Initial cost: {cost_unnorm[0]:.2f}")
print(f"Final cost: {cost_unnorm[-1]:.2f}")


# Normalize features (except bias term)
X_normalized = X.copy()
X_normalized[:, 1:] = (X[:, 1:] - X[:, 1:].mean(axis=0)) / X[:, 1:].std(axis=0)

print("\n" + "=" * 60)
print("GRADIENT DESCENT WITH NORMALIZATION")
print("=" * 60)

learning_rate_norm = 0.1  # Can use much larger LR!
weights_norm, cost_norm = gradient_descent(
    X_normalized, y, learning_rate_norm, n_iterations
)

print(f"\nLearning rate: {learning_rate_norm}")
print(f"Final weights: {weights_norm}")
print(f"Initial cost: {cost_norm[0]:.2f}")
print(f"Final cost: {cost_norm[-1]:.2f}")


# Visualization
fig, axes = plt.subplots(1, 2, figsize=(14, 5))

# Plot 1: Cost convergence
axes[0].plot(cost_unnorm, 'r-', linewidth=2, label='Without normalization')
axes[0].plot(cost_norm, 'g-', linewidth=2, label='With normalization')
axes[0].set_xlabel('Iteration', fontsize=12)
axes[0].set_ylabel('Cost (MSE)', fontsize=12)
axes[0].set_title('Cost Function Convergence', fontsize=14, fontweight='bold')
axes[0].legend()
axes[0].grid(True, alpha=0.3)

# Plot 2: Cost convergence (log scale)
axes[1].semilogy(cost_unnorm, 'r-', linewidth=2, label='Without normalization')
axes[1].semilogy(cost_norm, 'g-', linewidth=2, label='With normalization')
axes[1].set_xlabel('Iteration', fontsize=12)
axes[1].set_ylabel('Cost (MSE) - Log Scale', fontsize=12)
axes[1].set_title('Cost Convergence (Log Scale)', fontsize=14, fontweight='bold')
axes[1].legend()
axes[1].grid(True, alpha=0.3)

plt.tight_layout()
plt.show()

print("\n" + "=" * 60)
print("KEY OBSERVATIONS")
print("=" * 60)
print(f"Without normalization - needed LR={learning_rate_unnorm}")
print(f"With normalization - could use LR={learning_rate_norm} (1000x larger!)")
print(f"\nCost reduction without normalization: {cost_unnorm[0]:.2f} → {cost_unnorm[-1]:.2f}")
print(f"Cost reduction with normalization: {cost_norm[0]:.2f} → {cost_norm[-1]:.2f}")
print("\nConclusion: Normalization allows faster convergence and larger learning rates!")
```

<div align="center">
<img src="./assets/img32.webp" alt="" width="900" loading="lazy"/><br/>
<span>Gradient descent with and without normalization.</span>
</div>


### What This Code Shows
{: .no_toc }

The code demonstrates three critical points:

1. **Without normalization**, we need a tiny learning rate (0.0001) because the gradients are on wildly different scales. Use a larger learning rate and the algorithm explodes.

2. **With normalization**, we can use a learning rate 1000x larger (0.1) because all gradients are on the same scale.

3. **Convergence is much faster** with normalization. The cost drops smoothly and quickly, while without normalization it struggles to find the right direction.

Notice how in the log-scale plot, the green line (normalized) drops like a stone, while the red line (unnormalized) barely moves.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## The Zigzag Problem

Let me show you another visualization that really drives home why normalization matters.

Imagine you have just two features, but one ranges from 0 to 100 and the other from 0 to 10. Your cost function forms a long, narrow valley:


### Let's Play with Python

```python
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Ellipse

# Set random seed for reproducibility
np.random.seed(42)

# Define a simple quadratic cost function with different scales
# C(x, y) = 0.5 * (a*x^2 + b*y^2)
# where a << b creates an elongated valley

def cost_function(x, y, a=1, b=100):
    """
    Cost function with different scales for x and y.

    Args:
        x, y: coordinates
        a, b: scaling factors (b >> a creates elongated valley)

    Returns:
        cost value
    """
    return 0.5 * (a * x**2 + b * y**2)

def gradient(x, y, a=1, b=100):
    """
    Gradient of the cost function.

    Returns:
        (grad_x, grad_y): gradient components
    """
    grad_x = a * x
    grad_y = b * y
    return grad_x, grad_y

def gradient_descent(x_start, y_start, learning_rate, n_iterations, a=1, b=100):
    """
    Perform gradient descent and track the path.

    Args:
        x_start, y_start: starting position
        learning_rate: step size
        n_iterations: number of steps
        a, b: cost function parameters

    Returns:
        x_history, y_history: trajectory of gradient descent
    """
    x = x_start
    y = y_start

    x_history = [x]
    y_history = [y]

    for i in range(n_iterations):
        # Compute gradient
        grad_x, grad_y = gradient(x, y, a, b)

        # Update position
        x = x - learning_rate * grad_x
        y = y - learning_rate * grad_y

        # Store history
        x_history.append(x)
        y_history.append(y)

    return x_history, y_history

# Parameters for unnormalized case (creates zigzag)
a_unnorm = 1     # Small coefficient for x
b_unnorm = 100   # Large coefficient for y (creates elongated valley)

# Starting point
x_start = 8.0
y_start = 0.8

# Gradient descent WITHOUT normalization
learning_rate_unnorm = 0.01  # Small learning rate required
n_iterations = 100

x_unnorm, y_unnorm = gradient_descent(
    x_start, y_start, learning_rate_unnorm, n_iterations, a_unnorm, b_unnorm
)

# Gradient descent WITH normalization (balanced scales)
a_norm = 1
b_norm = 1  # Same scale as a

learning_rate_norm = 0.1  # Can use larger learning rate
x_norm, y_norm = gradient_descent(
    x_start, y_start, learning_rate_norm, n_iterations, a_norm, b_norm
)

# Create visualization
fig = plt.figure(figsize=(16, 6))

# ============================================================================
# Plot 1: WITHOUT normalization (zigzag pattern)
# ============================================================================
ax1 = fig.add_subplot(1, 3, 1)

# Create contour plot
x_range = np.linspace(-10, 10, 400)
y_range = np.linspace(-1.5, 1.5, 400)
X, Y = np.meshgrid(x_range, y_range)
Z_unnorm = cost_function(X, Y, a_unnorm, b_unnorm)

# Plot contours
levels = np.logspace(0, 3, 20)
contour = ax1.contour(X, Y, Z_unnorm, levels=levels, cmap='viridis', alpha=0.6)
ax1.clabel(contour, inline=True, fontsize=8, fmt='%1.0f')

# Plot gradient descent path
ax1.plot(x_unnorm, y_unnorm, 'r-o', linewidth=2, markersize=4,
         label='Gradient descent path', zorder=10)

# Mark start and end
ax1.plot(x_unnorm[0], y_unnorm[0], 'go', markersize=12,
         label='Start', zorder=11)
ax1.plot(x_unnorm[-1], y_unnorm[-1], 'mo', markersize=12,
         label='End', zorder=11)
ax1.plot(0, 0, 'r*', markersize=15, label='Minimum', zorder=11)

# Add arrows to show zigzag pattern
for i in range(0, min(10, len(x_unnorm)-1), 2):
    ax1.annotate('', xy=(x_unnorm[i+1], y_unnorm[i+1]),
                xytext=(x_unnorm[i], y_unnorm[i]),
                arrowprops=dict(arrowstyle='->', color='red', lw=1.5))

ax1.set_xlabel('Feature 1 (range: 0-100)', fontsize=11)
ax1.set_ylabel('Feature 2 (range: 0-10)', fontsize=11)
ax1.set_title('WITHOUT Normalization\n(Zigzag pattern in elongated valley)',
              fontsize=12, fontweight='bold')
ax1.legend(loc='upper right', fontsize=9)
ax1.grid(True, alpha=0.3)
ax1.set_xlim(-10, 10)
ax1.set_ylim(-1.5, 1.5)

# ============================================================================
# Plot 2: WITH normalization (direct path)
# ============================================================================
ax2 = fig.add_subplot(1, 3, 2)

# Create contour plot
Z_norm = cost_function(X, Y, a_norm, b_norm)

# Plot contours
levels = np.logspace(0, 2, 20)
contour = ax2.contour(X, Y, Z_norm, levels=levels, cmap='viridis', alpha=0.6)
ax2.clabel(contour, inline=True, fontsize=8, fmt='%1.0f')

# Plot gradient descent path
ax2.plot(x_norm, y_norm, 'g-o', linewidth=2, markersize=4,
         label='Gradient descent path', zorder=10)

# Mark start and end
ax2.plot(x_norm[0], y_norm[0], 'go', markersize=12,
         label='Start', zorder=11)
ax2.plot(x_norm[-1], y_norm[-1], 'mo', markersize=12,
         label='End', zorder=11)
ax2.plot(0, 0, 'r*', markersize=15, label='Minimum', zorder=11)

# Add arrows to show direct path
for i in range(0, min(5, len(x_norm)-1)):
    ax2.annotate('', xy=(x_norm[i+1], y_norm[i+1]),
                xytext=(x_norm[i], y_norm[i]),
                arrowprops=dict(arrowstyle='->', color='green', lw=1.5))

ax2.set_xlabel('Feature 1 (normalized)', fontsize=11)
ax2.set_ylabel('Feature 2 (normalized)', fontsize=11)
ax2.set_title('WITH Normalization\n(Direct path in circular valley)',
              fontsize=12, fontweight='bold')
ax2.legend(loc='upper right', fontsize=9)
ax2.grid(True, alpha=0.3)
ax2.set_xlim(-10, 10)
ax2.set_ylim(-1.5, 1.5)

# ============================================================================
# Plot 3: Convergence comparison
# ============================================================================
ax3 = fig.add_subplot(1, 3, 3)

# Calculate distance from minimum over iterations
dist_unnorm = [np.sqrt(x**2 + y**2) for x, y in zip(x_unnorm, y_unnorm)]
dist_norm = [np.sqrt(x**2 + y**2) for x, y in zip(x_norm, y_norm)]

# Calculate cost over iterations
cost_unnorm = [cost_function(x, y, a_unnorm, b_unnorm)
               for x, y in zip(x_unnorm, y_unnorm)]
cost_norm = [cost_function(x, y, a_norm, b_norm)
             for x, y in zip(x_norm, y_norm)]

# Plot distance to minimum
iterations = range(len(dist_unnorm))
ax3.semilogy(iterations, dist_unnorm, 'r-o', linewidth=2, markersize=3,
            label='Without normalization', alpha=0.8)
ax3.semilogy(iterations[:len(dist_norm)], dist_norm, 'g-o', linewidth=2,
            markersize=3, label='With normalization', alpha=0.8)

ax3.set_xlabel('Iteration', fontsize=11)
ax3.set_ylabel('Distance to Minimum (log scale)', fontsize=11)
ax3.set_title('Convergence Speed Comparison', fontsize=12, fontweight='bold')
ax3.legend(fontsize=10)
ax3.grid(True, alpha=0.3, which='both')

plt.tight_layout()
plt.show()

# ============================================================================
# Print statistics
# ============================================================================
print("=" * 70)
print("ZIGZAG PROBLEM DEMONSTRATION")
print("=" * 70)
print("\nWITHOUT NORMALIZATION:")
print(f"  Feature scales: x has coefficient {a_unnorm}, y has coefficient {b_unnorm}")
print(f"  Learning rate: {learning_rate_unnorm}")
print(f"  Starting position: ({x_start:.2f}, {y_start:.2f})")
print(f"  Distance traveled: {sum([np.sqrt((x_unnorm[i+1]-x_unnorm[i])**2 + (y_unnorm[i+1]-y_unnorm[i])**2) for i in range(len(x_unnorm)-1)]):.2f}")
print(f"  Final position: ({x_unnorm[-1]:.6f}, {y_unnorm[-1]:.6f})")
print(f"  Final distance to minimum: {dist_unnorm[-1]:.6f}")
print(f"  Iterations to converge: {len(x_unnorm)}")

print("\nWITH NORMALIZATION:")
print(f"  Feature scales: x has coefficient {a_norm}, y has coefficient {b_norm}")
print(f"  Learning rate: {learning_rate_norm}")
print(f"  Starting position: ({x_start:.2f}, {y_start:.2f})")
print(f"  Distance traveled: {sum([np.sqrt((x_norm[i+1]-x_norm[i])**2 + (y_norm[i+1]-y_norm[i])**2) for i in range(len(x_norm)-1)]):.2f}")
print(f"  Final position: ({x_norm[-1]:.6f}, {y_norm[-1]:.6f})")
print(f"  Final distance to minimum: {dist_norm[-1]:.6f}")
print(f"  Iterations to converge: {len(x_norm)}")
```

<div align="center">
<img src="./assets/img33.webp" alt="" width="800" loading="lazy"/><br/>
<span>Without normalization: zigzag descent. With normalization: straight path.</span>
</div>


**Without normalization**, gradient descent takes tiny steps along the narrow dimension and huge steps along the wide dimension. It zigzags down the valley, wasting time and iterations.

**With normalization**, the valley becomes circular (or at least more balanced), and gradient descent can take the direct path to the minimum.

<!-- This is exactly what happens in N dimensions—except you can't visualize it anymore! But the math is the same. -->














<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Few Words About the Vector Notation

The Elegant Way. Now that we understand the concept, let's write it in clean vector notation. This is how you'll see gradient descent written in textbooks and papers.

**Weight vector**:

$$\mathbf{w} = \begin{bmatrix} w\_0 \\ w\_1 \\ w\_2 \\ \vdots \\ w\_N \end{bmatrix}$$

**Feature vector** (for one sample):

$$\mathbf{x} = \begin{bmatrix} 1 \\ x\_1 \\ x\_2 \\ \vdots \\ x\_N \end{bmatrix}$$

**Prediction**:

$$\hat{y} = \mathbf{w}^T \mathbf{x} = \sum_{i=0}^{N} w\_i \cdot x\_i$$

**Cost function** (for all M samples):

$$C(\mathbf{w}) = \frac{1}{2M} \sum_{i=1}^{M} (\mathbf{w}^T \mathbf{x}^{(i)} - y^{(i)})^2$$

**Gradient**:

$$\nabla C(\mathbf{w}) = \frac{1}{M} \mathbf{X}^T (\mathbf{X}\mathbf{w} - \mathbf{y})$$

Where $$\mathbf{X}$$ is the matrix of all M samples (M rows, N+1 columns).

**Update rule**:

$$\mathbf{w} := \mathbf{w} - \alpha \cdot \nabla C(\mathbf{w})$$

That's it. One line. Beautiful.

This vectorized form is not only elegant—it's also **much faster** to compute because modern libraries (NumPy, PyTorch, TensorFlow) are optimized for matrix operations.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## A Word About High Dimensions

***Does this really work in 1_000 dimensions? Or 10_000?***

Yes, with a caveat however... The principle is exactly the same. But a few things change:

1. **It gets slower**: More dimensions mean more computations per iteration. But the algorithm still converges.

2. **You need more data**: The "[curse of dimensionality](https://en.wikipedia.org/wiki/Curse_of_dimensionality)" means that high-dimensional spaces are sparse. You need more training examples to learn reliable patterns.

3. **Local minima become common**: In high dimensions, especially with non-linear models (neural networks), there are many valleys. Gradient descent might get stuck in a local minimum instead of finding the global one. But modern techniques (momentum, adaptive learning rates) help a lot.

4. **You might use variants**: For very large datasets, "batch" gradient descent (what we've been doing) is slow. Stochastic Gradient Descent (SGD) uses one example at a time, and mini-batch SGD uses small batches. These are faster and often work better.

But the core idea—compute the gradient, take a step downhill—remains unchanged.

**Side Note:** More dimentions (more features) is not means better model. Here like in many other places "Perfect is the enemy of good". Next rainy Saturday morning look for **PCA**. Basically the idea is that most of the time Paretto is right and 20% of the dimensions explains 80% of the model and it is most of the time better to work with smaller number of dimmensions. Then, if it is still raining Sunday morning look for **features engineering**. Here the idea is that, quite often, it is smart to compose a new dimension with 3 dimensions and to get ride of the latters.



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Summary

* **Gradient descent generalizes beautifully**: The same principle from 1D and 2D extends to N dimensions. Compute partial derivatives for each parameter, update each one independently.

* **The gradient is a vector**: In N dimensions, $$\nabla C$$ has N components, each telling us how to adjust one parameter. It points uphill; we go the opposite direction.

* **Feature scaling is critical**: When features are on different scales, one dimension dominates the gradients. This causes slow convergence and requires tiny learning rates.

* **Normalization fixes the scale problem**: Standardization (z-score) or min-max scaling puts all features on equal footing. This allows faster convergence and larger learning rates.

* **Vectorized notation is elegant and fast**: Writing gradient descent in matrix form ($$\mathbf{w} := \mathbf{w} - \alpha \cdot \nabla C(\mathbf{w})$$) is not only cleaner but also much faster to compute.

* **The zigzag problem**: Without normalization, gradient descent zigzags through long, narrow valleys. With normalization, it takes the direct path.

* **It works in high dimensions**: The same algorithm works for 10 features or 10,000 features. The principle doesn't change, though practical considerations (speed, local minima, data requirements) become more important.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Q&A - Checking for Understanding

* Why is feature normalization so important in N-dimensional gradient descent? What happens if you forget to normalize your features?

* If you have three features with ranges [0-5], [0-10], and [0-1000], which feature will dominate the gradients? Why?

* In the student exam score example, we used standardization. What would happen if we used min-max scaling instead? Would the algorithm still work?

* Explain in your own words what the gradient vector $$\nabla C$$ represents in N-dimensional space. Why do we move in the direction of $$-\nabla C$$?

* If you have 50 features and use a learning rate $$\alpha = 0.1$$ with normalized data, why can you use such a large learning rate compared to unnormalized data?

* Look at the vectorized update rule: $$\mathbf{w} := \mathbf{w} - \alpha \cdot \nabla C(\mathbf{w})$$. This single line updates all N weights simultaneously. How does this compare to writing N separate update equations?

* In the code example, why do we add a column of ones to the feature matrix $$X$$? What does the corresponding weight $$w\_0$$ represent?

* The cost function for M samples is $$C(\mathbf{w}) = \frac{1}{2M} \sum_{i=1}^{M} (\mathbf{w}^T \mathbf{x}^{(i)} - y^{(i)})^2$$. Why do we divide by M? Why do we divide by 2?




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## What Is Next?

* [Part 1]({%link docs/09_machine_learning/002_gradient_descent/gradient_descent.md%}): where we discuss the gradient descent in 1D
* [Part 2]({%link docs/09_machine_learning/002_gradient_descent/gradient_descent_2d.md%}): where we discuss the gradient descent in 2D
* [Part 3]({%link docs/09_machine_learning/002_gradient_descent/gradient_descent_nd.md%}): where we generalize the gradient descent to N dimensions

**Ready to go deeper?**

Now that you understand the fundamentals of gradient descent, you're ready to explore more advanced topics:
- Stochastic Gradient Descent (SGD) and mini-batch variants
- Adaptive learning rates (Adam, RMSprop, Adagrad)
- Momentum and Nesterov acceleration
- Regularization techniques (L1, L2)
- Gradient descent in neural networks (backpropagation)

I recommend this book for your next steps:

<div align="center">
<img src="./assets/img39.webp" alt="" width="225" loading="lazy"/><br/>
<span>Hands-On Machine Learning with Scikit-Learn, Keras, and TensorFlow</span>
</div>

Aurélien Géron's book is the perfect continuation. It covers all the topics above with clear explanations and practical code examples. You now have the foundations to understand everything in that book. Pay attention to the Edition number. Personally I prefer the English version. There other books but then the cost might be a problem.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Webliography

* [JupyterLab](https://jupyter.org/try-jupyter/lab/). Where you can paste the scripts above.
* [Hands-On Machine Learning with Scikit-Learn, Keras, and TensorFlow (3rd Edition)](https://www.oreilly.com/library/view/hands-on-machine-learning/9781098125967/) by Aurélien Géron
