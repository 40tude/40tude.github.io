---
published: true
lang: en-US
layout: default
title: "Understanding the Confusion Matrix"
description: "A gentle, story-driven introduction so you’ll never be confused again."
parent: "Machine Learning"
math: mathjax
date:               2025-12-13 18:00:00 +0000
last_modified_date: 2025-12-13 18:00:00 +0000
---



<!--
TODO
* Do this
* Do that
-->


# Understanding the Confusion Matrix
{: .no_toc }

A gentle, story-driven introduction so you’ll never be confused again.
{: .lead }


<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }
* For beginners
* In a ML context but applicable elsewhere
* In the confusion matrix we indicate whether the prediction is correct (T/F) + the kind of prediction (P/N)
* For binary classifiers and beyond


<div align="center">
<img src="./assets/img00.webp" alt="" width="300" loading="lazy"/><br/>
<span>Click the images to zoom in.</span>
</div>


<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}







<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Introduction

One day, a great Machine Learning philosopher once whispered to me: “Listen, kid. A Machine Learning project is just like a dish in a fine restaurant. Every step matters, especially the first ones. You can plate it beautifully, serve it with elegance, even impress the critics… but if the recipe is bad, the dish will never be good. And trust me — no amount of fancy deployment can save a rotten model. Capiche?”

<div align="center">
<img src="./assets/img01.webp" alt="" width="300" loading="lazy"/><br/>
<span>Rémy, the ML philosopher</span>
</div>

| Step                      | Analogy                        |
|---------------------------|--------------------------------|
| EDA                       | The recipe                     |
| Features Engineering      | The secret sauce               |
| Baseline model            | The first taste                |
| Metrics Analysis          | The critics' score             |
| API & App                 | Sharing with friends           |
| Deployment & Monitoring   | Serve the dish, maintain quality |


At one of the very early steps of the process — before jumping into modeling, optimization, and all that fun stuff with Scikit-Learn — it’s absolutely crucial to choose a metric, to be able to explain *why* you chose it, to set yourself a goal, and to stick to it. And honestly, that’s usually the hardest part. Because when we don’t get the results we want, we all have a tendency to “bend the data” until it says what we want to hear — and that is a **very, very bad idea**.

When I say “choose a metric,” right away you start hearing words like *Recall*, *Precision*, *F1 score*, *Accuracy*… On top of that, people start talking about the confusion matrix. And that’s usually where I completely lose my footing.

Let’s be clear: I have no problem with the F1 score itself, or with formulas in general. No, no, it is even worst than that. The real issue was that for a very long time, I just couldn’t wrap my head around how the *labels* in the confusion matrix were written: `TP`, `FP`, `TN`, and `FN`.

Which made it… somewhat awkward to properly explain my choices. But that was before. Since then, I went to Lourdes, I [saw the light](https://en.wikipedia.org/wiki/Lourdes_apparitions), and now I *almost* understand everything.

So yeah — that’s exactly what we’re going to talk about in this post. As usual, I’ll start very slowly, without assuming anything about your math background (okay, you still need to know how to add and divide), but by the end — pinky swear — your ideas will be crystal clear. You’ll be able to choose and explain the metric for your ML project… and also to legitimately worry if a test tells you that you may have caught this or that disease.

Alright. Let’s get started.








<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Drawing The Matrix

To kick things off, I want to finally put to rest this whole “how do I draw a confusion matrix?” question.

Let’s imagine we have some “thing” that makes predictions. It could be a ML model, a pregnancy test, a fortune teller… whatever you want, it’s your story.

Now, this binary predictor will sometimes get things right and sometimes get things wrong. If you look closer, you can actually split its predictions into four categories:

1. I said before going into the club that I was going to leave with a girl, and sure enough, I left with the one who became my wife (poor thing, for better or for worse, as they say…)
1. I said before going into the club that I was going to leave with a girl, but no luck, I went home alone.
1. I said before going into the club that I wasn’t going to leave with a girl, and… I went home alone.
1. I said before going into the club that I wasn’t going to leave with a girl, but the way I danced to those wild beats… I ended up leaving with the most beautiful girl of the night.

Yeah, I know, the example is silly, but that’s the point—it sticks in your mind. And trust me, when it comes to ridiculous examples, you haven’t seen anything yet. The worst is yet to come…

So, we can sum all this up in a table to evaluate how good the predictions are. If you go clubbing twice a week on average, by the end of the year you’ve made 104 predictions… and now it’s starting to look legit.

Anyway, in the previous paragraph, the key phrase is “**evaluate the accuracy of the predictions**”. Yeah, I know, that’s more than one word.

What we’re going to do now is make a two-way table: on one side, you put the predictions, and on the other, you put reality. So it’s a “Reality vs. Predictions” matrix, and for now, don’t worry about which is the row and which is the column.

Now, **THE REALLY IMPORTANT THING** is that in each cell of the table, we’ll indicate whether **the prediction was correct** and what **kind of prediction** it was.

Let’s clarify the vocabulary:
* The prediction is **NEGATIVE** or **POSITIVE**. Here, a **POSITIVE** prediction could mean "I left with a girl".
* Reality is **NEGATIVE** or **POSITIVE**. These are the same "units" as the predictions so we can compare them.
* The **correctness of the prediction** compared to reality is **TRUE** or **FALSE**.

So I suggest this first empty matrix:

```
              ┌──────────┬──────────┐
   Negative   │          │          │
REALITY       ├──────────┼──────────┤
   Positive   │          │          │
              └──────────┴──────────┘
                Negative   Positive
                    PREDICTION
```

Which we will fill in together by announcing what we do “out loud.”

**The data:**
At the end of the year, out of 104 outings, I said I was going to go out with a girl 80 times, but in fact I came home alone 70 times. On the other 24 outings where I said I was going to be serious, I only kept my word 18 times.

Let's continue, and now I say:

1. Prediction **P** and Reality **P**: bottom right. The prediction is correct. I said that I would meet a girl and that’s what happened (what a charmer!). I write **T** (the prediction was true) and then **P** (because the prediction was **P**). The value is 10 (80–70).

2. Prediction **P** and Reality **N**: top right. The prediction is incorrect. I said that I would meet a girl, but I went home alone. I write **F** (the prediction was false) and then **P** (because the prediction was **P**). The value is 70.

3. Prediction **N** and Reality **N**: top left. The prediction is correct. I said that I would behave seriously and go home alone, and that is indeed what happened. I write **T** (the prediction was true) and then **N** (because the prediction was **N**). The value is 18.

4. Prediction **N** and Reality **P**: bottom left. The prediction is incorrect. I said that I would behave seriously and go home alone, but on those nights I met the girl of my life (at least that’s what I thought at the time). I write **F** (the prediction was false) and then **N** (because the prediction was **N**). The value is 06 (24–18).


<div align="center">
<img src="./assets/img02.webp" alt="" width="300" loading="lazy"/><br/>
<span><b>Tadaa!</b></span>
</div>



```
              ┌──────────┬──────────┐
   Negative   │  TN 18   │  FP 70   │
REALITY       ├──────────┼──────────┤
   Positive   │  FN 06   │  TP 10   │
              └──────────┴──────────┘
                Negative   Positive
                    PREDICTION
```

**Notes:**
* You can see that it doesn’t really matter what is shown in rows or columns. Here I followed what Scikit-Learn (a library used with Python) displays, but that’s really not the most important part. In this case we have:
  * **X-axis (columns)**: what the model predicted (Negative then Positive)
  * **Y-axis (rows)**: the ground truth (Positive at the bottom, Negative at the top)
* Obviously, the sum of all the cells is 104, the total number of nights out.
* In the same way, with this matrix, the sums of the different columns correspond to my predictions (going home alone 24 times, being a charmer 80 times).
* The sum along the main diagonal (top-left, bottom-right) corresponds to the number of correct predictions (with either positive or negative outcomes, but the predictions were correct: 28).
* The sum along the anti-diagonal (bottom-left, top-right) is the number of times the predictions were wrong (76).



**Summary:**
Each cell contains a two-letter code:
- **First letter**: whether the prediction was correct (**T** for True) or wrong (**F** for False)
- **Second letter**: the prediction itself (**P** or **N**)

Building the Matrix Step by Step

| Prediction | Reality | Correct? | Label |
|------------|---------|----------|-------|
| P | P | Yes → T | TP |
| P | N | No → F | FP |
| N | P | No → F | FN |
| N | N | Yes → T | TN |





<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**









<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Intuitive Understanding

* A Visual/Intuitive Section Before the Formulas
* Explain the four quadrants with intuitive names:
    * "Hits,"
    * "False Alarms,"
    * "Misses,"
    * "Correct Rejections."
* To bridge the gap before diving into formulas



<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**





















<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Class Imbalance Problem

### Ideas to explore
{: .no_toc }

* To motivate the metrics
* Critical for beginners, often overlooked.
* Explain why accuracy can be misleading with imbalanced datasets (the classic "99% accuracy on fraud detection" trap).
* This explains why we need Precision/Recall.






<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**




















<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Metrics

### Ideas to explore
{: .no_toc }

- A concrete example using the clubbing scenario (consistency)
- When to prioritize this metric (which you partially cover later)
- Common pitfalls or misinterpretations






<!-- ###################################################################### -->
### Precision
{: .no_toc }

**Precision** = TP / (TP + FP)
"Among everything I predicted as Positive, how many were actually Positive?"

<!-- ###################################################################### -->
### Recall
{: .no_toc }

* **Recall** (sensitivity) = TP / (TP + FN)
* "Among all actual Positives, how many did I find?"
* Come from communications

<!-- ###################################################################### -->
### F1 Score
{: .no_toc }

* Harmonic mean
* Resistor in //
* Look for compromise
* Python script to show the compromise

<!-- ###################################################################### -->
### Accuracy
{: .no_toc }

* Diag over Total




<!-- ###################################################################### -->
### Metrics in a Tree

#### Ideas to explore
{: .no_toc }

* Prevalence
* Sensitivity
* Specificity
* Example and Numerical Application
* One word about Bayes? Intro a new post ?




<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**








































<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Threshold Concept

### Ideas to explore
{: .no_toc }

* Before "How to Select a Metric,"
* Explains that changing the threshold shifts the trade-off between FP and FN.
* Finish with the ROC curve.



<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**












<!-- ###################################################################### -->
<!-- ###################################################################### -->
## ROC Curve and AUC


### Ideas to explore
{: .no_toc }

* A standard topic
* Encountered by beginners after confusion matrices




<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**































<!-- ###################################################################### -->
<!-- ###################################################################### -->
## How to select a Metric?

### Ideas to explore
{: .no_toc }

* Sex films among cartoons for kids
* CB
* Spam
* Find more weird examples 😁



<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**

















<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Confusion Matrix in code




<!-- ###################################################################### -->
### Python
{: .no_toc }

- Don' forget to interpret the results in context



<!-- ###################################################################### -->
### Rust
{: .no_toc }

- Don' forget to interpret the results in context



<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**





















<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Multi-class Confusion Matrices

* One word
* the concept extends beyond binary classification.
* don't need to go deep, show an example with some explanation/interpretation




<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**

















<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

* With a Bayes teaser




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

