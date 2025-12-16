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
* Create a Zip with code
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
* In the confusion matrix we indicate whether the prediction was correct (T/F) + the kind of prediction (P/N)

* "Which mistake would hurt me the most?"
    * If **missing a real positive** is catastrophic → **Recall** (bottom line)
    * If **accusing something innocent** is catastrophic → **Precision** (right col)

* Recall (bottom line) cares about missed positives (we want FN=0)
* **Pre**cision (right col) cares about mistaken **po**sitives (we want FP=0)

* Confusion matrix concept extends to multi-class problems when we need to choose among more than 2 classes


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

One day, a great Machine Learning philosopher once whispered to me: "Listen, kid. A Machine Learning project is just like a dish in a fine restaurant. Every step matters, especially the first ones. You can plate it beautifully, serve it with elegance, even impress the critics… but if the recipe is bad, the dish will never be good. And trust me, no amount of fancy deployment can save a rotten model. Capiche?"

<div align="center">
<img src="./assets/img01.webp" alt="" width="600" loading="lazy"/><br/>
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


At one of the very early steps of the process, before jumping into modeling, optimization, and all that fun stuff with Scikit-Learn, it’s absolutely crucial to choose a metric, to be able to explain *why* you chose it, to set yourself a goal, and to stick to it. And honestly, that’s usually the hardest part. Because when we don’t get the results we want, we all have a tendency to "bend the data" until it says what we want to hear, and that is a **very, very bad idea**.

When I say "choose a metric," right away you start hearing words like *Recall*, *Precision*, *F1-score*, *Accuracy*… On top of that, people start talking about the confusion matrix. And that’s usually where I completely lose my footing.

Let’s be clear: I have no problem with the F1-score itself, or with formulas in general. No, no, it is even worst than that. The real issue was that for a very long time, I just couldn’t wrap my head around how the *labels* in the confusion matrix were written: `TP`, `FP`, `TN`, and `FN`.

Which made it… somewhat awkward to properly explain my choices. But that was before. Since then, I went to Lourdes, I [saw the light](https://en.wikipedia.org/wiki/Lourdes_apparitions), and now I *almost* understand everything.

So yeah, that’s exactly what we’re going to talk about in this post. As usual, I’ll start very slowly, without assuming anything about your math background (okay, you still need to know how to add and divide), but by the end, pinky swear, your ideas will be crystal clear. You’ll be able to choose and explain the metric for your ML project… and also to legitimately worry if a test tells you that you may have caught this or that disease.

Alright. Let’s get started.








<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Drawing The Matrix

To kick things off, I want to finally put to rest this whole "how do I draw a confusion matrix?" question.

Let’s imagine we have some "thing" that makes predictions. It could be a ML model, a pregnancy test, a fortune teller… whatever you want, it’s your story.

Now, this binary predictor will sometimes get things right and sometimes get things wrong. If you look closer, you can actually split its predictions into four categories:

1. I said, before going into the club, that I was going to leave with a girl, and sure enough, I left with the one who became my wife (poor thing, for better or for worse, as they say…)
1. I said, before going into the club, that I was going to leave with a girl, but no luck, I went home alone.
1. I said, before going into the club, that I wasn’t going to leave with a girl, and… I went home alone.
1. I said, before going into the club, that I wasn’t going to leave with a girl, but the way I danced to those wild beats… I ended up leaving with the most beautiful girl of the night.

Yeah, I know, the example is silly, but that’s the point—it sticks in your mind. And trust me, when it comes to ridiculous examples, you haven’t seen anything yet. The worst is yet to come…

So, we can sum all this up in a table to evaluate how good the predictions are. If you go clubbing twice a week on average, by the end of the year you’ve made 104 predictions… and now it’s starting to look credible.

Anyway, in the previous paragraph, the key word is "**evaluate the accuracy of the predictions**". Yeah, I know, that’s more than one word.

What we’re going to do now is make a two-way table: on one side, you put the predictions, and on the other, you put reality. So it’s a "Reality vs. Predictions" matrix, and for now, don’t worry about which is the row and which is the column.

Now, **THE REALLY IMPORTANT THING** is that in each cell of the table, we will indicate:
1. whether **the prediction was correct**
2. *AND* what **kind of prediction** it was.

Let’s clarify the vocabulary:
* The prediction is **NEGATIVE** or **POSITIVE**. Here, a **POSITIVE** prediction could mean "I left with a girl".
* Reality is **NEGATIVE** or **POSITIVE**. These are in the same "units" as the predictions so we can compare them.
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

Which we will fill in together by announcing what we do "out loud."

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

Before we dive into a bit of algebra, let's step back and appreciate the matrix we just built. First off, we can be proud of ourselves. More importantly, tomorrow morning you should be able to read it "out loud", I really mean it, without any trouble. Then, as a bonus challenge, try swapping the rows and columns, or flipping the two rows and/or the two columns around. The results stay the same, only the layout changes. Here, I'm using the format that the excellent Scikit-Learn library uses, but we don't want our understanding to depend on any particular arrangement.

Speaking of understanding... Let's try replacing TP and friends with everyday words. Starting from our original matrix:

```
              ┌──────────┬──────────┐
   Negative   │    TN    │    FP    │
REALITY       ├──────────┼──────────┤
   Positive   │    FN    │    TP    │
              └──────────┴──────────┘
                Negative   Positive
                    PREDICTION
```

And I propose the following matrix:

```
              ┌────────────────────┬───────────────┐
   Negative   │ Correct Rejections │  False Alarm  │
REALITY       ├────────────────────┼───────────────┤
   Positive   │      Misses        │      Hits     │
              └────────────────────┴───────────────┘
                      Negative          Positive
                              PREDICTION
```

1. **Hits:** Whether it's at a nightclub, detecting a wildfire on a satellite image, or spotting a fraudulent credit card. You get it. I announced that my natural charisma was going to work its magic once again, and sure enough, I left with a girl. Same thing for wildfire detection and fake cards: we correctly detected what needed to be detected.
1. **False Alarm:** Okay, the nightclub pickup example doesn't work great here, but with the wildfire detector, you understand that it screamed "fire!" when there wasn't one, and we scrambled the water bombers for nothing. Way to go, AI...
1. **Correct Rejection:** This one's simple. The fake credit card detector says the card is *NOT* fraudulent, and that's indeed the case. As for me, I managed to resist the advances of my many groupies and went home alone (such strength of character, truly inspiring...).
1. **Misses:** Here, the wildfire detector saw nothing, the credit card detector caught nothing, and I didn't keep my word. I ended up leaving with a gorgeous young woman even though I'd said I wouldn't (the willpower of a panda...).

There's really nothing new here compared to what we've already covered, but I think it's **important** to be able to put words to math formulas, lines of code, or confusion matrices. It lets you verify that you've actually understood, and it confirms that you can explain to a friend what the matrix, formula, or line of code is trying to tell us. Plus, I'm convinced it helps anchor these concepts in our heads.


Alright, let's do some math. Don't panic—it's going to be fine, you'll see.



### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**






































<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Metrics

<!-- ### Ideas to explore
{: .no_toc }

- A concrete example using the clubbing scenario (consistency)
- When to prioritize this metric (which you partially cover later)
- Common pitfalls or misinterpretations -->


We've mastered the confusion matrix and we understand the "story" it tells us. That's great, but there's a small problem. There's still too much information. How are you going to walk up to your favorite CFO and ask for an extra 2 million dollars in new GPUs because the numbers in your matrix aren't looking great? That's not going to fly. Plus, as I mentioned earlier, at the start of any ML project you need to pick one metric and stick with it. A confusion matrix isn't a metric, it's a table with 4 numbers. So yeah, that's not going to work...

In what follows, I'll use **our** confusion matrix, which looks like this:

```
              ┌──────────┬──────────┐
   Negative   │    TN    │    FP    │
REALITY       ├──────────┼──────────┤
   Positive   │    FN    │    TP    │
              └──────────┴──────────┘
                Negative   Positive
                    PREDICTION
```





<!-- ###################################################################### -->
### Precision
{: .no_toc }

$$\text{Precision} = \frac{\text{TP}}{\text{TP} + \text{FP}}$$

* What matters to us is the number of hits (`TP`)
* We start with `TP`
* In the last column, we compare `TP` to the sum of that column
* **Storytelling:** "Among everything I predicted as Positive, how many were actually Positive?"

<!-- ###################################################################### -->
### Recall
{: .no_toc }

$$\text{Recall} = \frac{\text{TP}}{\text{TP} + \text{FN}}$$

* What matters to us is the number of hits (`TP`)
* We start with `TP`
* In the bottom row, we compare `TP` to the sum of that row
* **Storytelling:** "Among all actual Positives, how many did I find?"
* The term `Recall` comes from the field of information retrieval. Out of all the relevant documents available in the database, how many did I retrieve?
* For example, we might want to extract all documents of interest, even if that means pulling a few irrelevant ones along the way.









<!-- ###################################################################### -->
### F1-score
{: .no_toc }

$$\text{F1-score} = \frac{2}{\frac{1}{\text{Precision}} + \frac{1}{\text{Recall}}}$$

* The `F1-score` is the harmonic mean of `Precision` and `Recall`
* The `F1-score` looks for a compromise between `Precision` and `Recall`
* **Storytelling:** "How well am I balancing finding all the positives with not crying wolf?"

Let's go back to the nightclub. I could adopt two extreme strategies:

1. **The overconfident guy:** Every single night, I announce "Tonight, I'm leaving with someone!" This way, I never miss an opportunity (if there's a chance). I've predicted it (Recall = 100%). But my hit rate is abysmal because most nights I go home alone despite my bold claims (Precision in the gutter).

2. **The overcautious guy:** I only predict success when I'm absolutely certain (say, when a girl has already shared her phone number while we were queuing outside the nightclub). Sure, when I make a prediction, I'm almost always right (Precision ≈ 100%). But I miss tons of opportunities I didn't dare call (Recall in the gutter).

The F1-score tells us "Pick a lane, buddy, but not an extreme one." It forces us to find a balance. And here's the key property of the harmonic mean: it punishes imbalance harshly. If one metric is great but the other is terrible, the F1-score stays low. You can't hide a weakness by excelling elsewhere.

If you remember your physics classes, the harmonic mean shows up in two classic situations:

1. **Two resistors in parallel**


<div align="center">
<img src="./assets/img04.webp" alt="" width="600" loading="lazy"/><br/>
<!-- <span><b>Tadaa!</b></span> -->
</div>

2. **A car traveling the same distance at two different speeds**

<div align="center">
<img src="./assets/img05.webp" alt="" width="600" loading="lazy"/><br/>
<!-- <span><b>Tadaa!</b></span> -->
</div>

In both cases, we can't just average the values. Indeed, the smaller one "drags down" the result. Same thing here: a fantastic Precision can't compensate for a catastrophic Recall, and vice versa.

<!-- * Python script to show the compromise -->



<!-- ###################################################################### -->
### Accuracy
{: .no_toc }


$$\text{Accuracy} = \frac{\text{TP} + \text{TN}}{\text{Total}}$$

* What "matters" for us, is the number of good predictions (`TN+TP`)
* First Diagonal over the `Total`
* **Storytelling:** "The Accuracy of a predictor is the percentage of correct predictions among all predictions made."

At this point, it is important to make the distinction between [`Accuracy` and `Precision`](https://en.wikipedia.org/wiki/Accuracy_and_precision)

<div align="center">
<img src="./assets/img07.webp" alt="" width="600" loading="lazy"/><br/>
<span><b>Accuracy & Precision</b></span>
</div>

Indeed, you can have low accuracy even with a high Precision. See the top-left example below:

<div align="center">
<img src="./assets/img06.webp" alt="" width="600" loading="lazy"/><br/>
<!-- <span><b>Tadaa!</b></span> -->
</div>






Finally (*CeCe Peniston, '91*) you may want to keep in mind the figure below:

<div align="center">
<img src="./assets/img03.webp" alt="" width="600" loading="lazy"/><br/>
<span><b>Tadaa!</b></span>
</div>




<!-- ###################################################################### -->
### Confusion Matrix Labels in a Tree
{: .no_toc }

<!-- #### Ideas to explore
{: .no_toc }

* Prevalence
* Sensitivity
* Specificity
* Example and Numerical Application
* One word about Bayes? Intro a new post ?

-->

Do you remember when you were young and innocent. You start learning probabilities and you used to draw trees to simulate, for example, that you toss a coin twice and get 4 possible results (HH, HT, TH, TT).

You know what? We can find the Confusion Matrix labels in the tree you used to draw. See below:

<div align="center">
<img src="./assets/img11.webp" alt="" width="600" loading="lazy"/><br/>
<span><b>Confusion Matrix metrics in a Tree</b></span>
</div>

We assume that the probability of being sick, called the **Prevalence**, is a certain percentage of the population. Each person is therefore either sick or healthy.

Then, everyone takes a medical test that has two key characteristics:
* **Sensitivity**, which is the percentage of sick people who are correctly identified as sick by the test.
* **Specificity**, which is the percentage of healthy people who are correctly identified as not sick by the test.

Look, at the end of the top branch of the tree for example. We found our friend `TP`. Indeed, the guy is affected, so the reality is POSITIVE. Then the prediction is "he is affected", "he is `POSITIVE`". So, in this case, the prediction is correct, it is `TRUE` and so the leaf is labeled `TRUE-POSITIVE`, aka `TP`.

Check for yourself but the same reasoning applied to the three other branches and you get: `FN`, `FP` and `TN`.

Now, if we stay focused on the top branch, you may remember that our teacher was talking about the "probability to be tested sick knowing that the patient is sick". Does the word conditional probability resonate? No? Have you ever heard about Bayes statistics, the statistic of the causes? This is not the topic of this post but believe or not TP its friends not only appears in the kind of binary tree but also in Bayes statistics.



Now, if we stay focused on the top branch, you may recall that our teacher mentioned the "probability of testing positive given that the patient is sick". Does the term "conditional probability" ring a bell? No? What about "Bayesian statistics", often described as the statistics of causes?

While this is not the main topic here, it is worth noting that `TP` and its friends appear not only in this type of binary tree, but also play a central role in Bayesian statistics.



<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00:** Draw the same kind of binary tree but in the case of me and my friends clubbing. Calculate Prevalence, Sensitivity and Specificity in percentage using the numbers of the table below.

```
              ┌──────────┬──────────┐
   Negative   │  TN 18   │  FP 70   │
REALITY       ├──────────┼──────────┤
   Positive   │  FN 06   │  TP 10   │
              └──────────┴──────────┘
                Negative   Positive
                    PREDICTION
```

**Exercice 01:**

**Exercice 02:**










<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Imbalance Problem

One of the most critical and often overlooked issues in machine learning is **class imbalance**. This problem appears whenever one class is much rarer than the other. [Fraud detection](https://github.com/40tude/fraud_detection_2) is a textbook example.

### How rare is fraud, really?
{: .no_toc }

To build intuition, let’s look at real-world orders of magnitude.

* In France, credit card fraud represents about [0.015%](https://www.banque-france.fr/en/press-release/ecb-and-eba-publish-joint-report-payment-fraud) of transactions. This means 15 frauds for 100_000 transactions, let's say roughly 1 for 1_000_000.
* On the other hand, the probability of being struck by lightning in a given year is often quoted around [1 in 1_000_000](https://www.cchst.ca/oshanswers/safety_haz/weather/lightning.html).

So fraud is exceptionally rare and this rarity is the root cause of many evaluation mistakes.


### A simple thought experiment
{: .no_toc }

Assume we have 100_000 transactions.

**Fraudulent transactions:**
  $$
  100{\_}000 \times 0.015\% = 15
  $$
**Legitimate transactions:**
  $$
  100{\_}000 - 15 = 99{\_}985
  $$

So the dataset looks like this:

| Class        | Count   |
|--------------|--------:|
| Legitimate   | 99_985  |
| Fraud        | 15      |
| **Total**    | 100_000 |


### The "dummy" predictor
{: .no_toc }

Now consider a very naïve model which predicts **"Fraud" 99%** of the time, no matter what. This sounds terrible… but let’s play the game and let's compute its accuracy.

**Step 1: Predictions made**

Out of 100_000 transactions
* Predicted **Fraud**:
  $$
  99\% \times 100{\_}000 = 99{\_}000
  $$
* Predicted **Legitimate**:
  $$
  1\% \times 100{\_}000 = 1{\_}000
  $$

**Step 2: Correct predictions**

Since only 15 transactions are actually fraud, at most 15 fraud predictions can be correct. All other fraud predictions are false alarms.

Let’s assume the best-case scenario for the dummy model:

- True Positives (Fraud correctly detected): 15
- False Positives (Legitimate flagged as fraud):
  $$
  99{\_}000 - 15 = 98{\_}985
  $$

Now for legitimate predictions:

* True Negatives: at most 1_000 (since there are many legitimate transactions)

The corresponding confusion matrix looks like this:

```
              ┌────────────┬─────────────┐
   Negative   │  TN 1_000  │  FP 98_985  │
REALITY       ├────────────┼─────────────┤
   Positive   │  FN     0  │  TP     15  │
              └────────────┴─────────────┘
                 Negative      Positive
                       PREDICTION
```

**Step 3 — Accuracy calculation**
{: .no_toc }

Accuracy is defined as:

$$
\text{Accuracy} = \frac{\text{Correct predictions}}{\text{Total predictions}}
$$

Correct predictions:

$$
15 \text{ (fraud)} + 1{\_}000 \text{ (legitimate)} = 1{\_}015
$$

So:

$$
\text{Accuracy} = \frac{1{\_}015}{100{\_}000} = 1.015\%
$$

This model is **almost always wrong**, despite predicting fraud constantly.


### **The Real Trap (the opposite dummy)**
{: .no_toc }

Let’s flip the strategy and predict "Legitimate" 100% of the time.

* Correct legitimate predictions: 99_985
* Missed frauds: 15

The corresponding confusion matrix looks like this:

```
              ┌────────────┬─────────────┐
   Negative   │  TN     0  │  FP      0  │
REALITY       ├────────────┼─────────────┤
   Positive   │  FN    15  │  TP 99_985  │
              └────────────┴─────────────┘
                 Negative      Positive
                       PREDICTION
```

Accuracy becomes:

$$
\text{Accuracy} = \frac{99{\_}985}{100{\_}000} = 99.985\%
$$

99.985% accuracy without detecting a single fraud. This is the famous "99% accuracy fraud detector" trap.


### Why Accuracy is misleading?
{: .no_toc }

Accuracy answers the question "How often is the model correct overall?". However with imbalanced datasets, this question is almost meaningless, because:

* The majority class dominates the metric
* A model can ignore the minority class completely and still look "excellent"

In fraud detection, missing fraud is far more costly than mislabeling a legitimate transaction but Accuracy treats all errors equally.


### Why do we need Precision and Recall?
{: .no_toc }

To properly evaluate models under imbalance, we need metrics that focus on the rare class:

* **Recall (Sensitivity):** "Out of all real frauds, how many did we catch?" (do you see the bottom line in your mind?)
* **Precision:** "Out of all transactions flagged as fraud, how many were actually fraud?" (do you see the right column in your mind?)

These metrics force us to confront the real trade-offs:
* Catching more fraud vs. triggering too many false alarms
* Business cost vs. customer friction


### Things to keep in mind
{: .no_toc }

* Is the dataset imbalanced? If yes => Blinking red LED 🔴
* In highly imbalanced problems, accuracy can lie

Understanding this helps us to:
- Choose the right metrics to monitor
- Design meaningful models
- Avoiding dangerously misleading conclusions






<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**






































































<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Confusion Matrix in Code


You thought we were in the Matrix? Nah, instead the confusion matrix is in the code. Below you'll find two complete sample code because I hate partial code you can find in [Medium](https://medium.com/@philippe.baucour/yet-another-linear-regression-introduction-0835e333508b) that never works. One is in Python, the other is in Rust. Both use the Titanic dataset.


<!-- ###################################################################### -->
### Python
{: .no_toc }

```python
import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.impute import SimpleImputer
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import confusion_matrix, ConfusionMatrixDisplay

# Mandatory when running in a terminal
import matplotlib.pyplot as plt

df = pd.read_csv("data/titanic.csv")
# print("df datatype & shape :", type(df), df.shape)

# In a script one must use print to see the head of dataframe
print("\n\nDataset head:")
print(df.head())

# Remove `PassengerId`, `Name`, `Ticket`, `Cabin` columns from the dataset
df = df.drop(columns=["PassengerId", "Name", "Ticket", "Cabin"])

print("\n\nDataset head (useless col removed) :")
print(df.head())

# ## Preprocessing
# No EDA? What a shame!
# Split the dataset by $X$ and
# $y$ = Survived
# $X$ = Pclass	Sex	Age	SibSp	Parch	Fare	Embarked

y = df.loc[:, "Survived"]

features_list = ["Pclass", "Sex", "Age", "SibSp", "Parch", "Fare", "Embarked"]
X = df.loc[:, features_list]

# Split the data in `train` and `test` sets
X_train, X_test, y_train, y_test = train_test_split(
    # `stratify=y` allows to stratify our sample.
    # Meaning, we will have the same proportion of categories in test and train set
    X,
    y,
    test_size=0.2,
    random_state=0,
    stratify=y,
)

# Deal with missing values with `SimpleImputer`
# Create an imputer for numerical columns
numerical_imputer = SimpleImputer(strategy="mean")

# Apply it on "Age" column.
# ! See the X[["Age"]] to get a 2D array rather than 1D
X_train[["Age"]] = numerical_imputer.fit_transform(X_train[["Age"]])

# In col `Embarked` replace missing val with "Unknown"
categorical_imputer = SimpleImputer(strategy="constant", fill_value="Unknown")
X_train[["Embarked"]] = categorical_imputer.fit_transform(X_train[["Embarked"]])

# Make all the required preprocessing on the train set
print("\n\nX_train head:")
print(X_train.head())

# Reminder, we have : ['Pclass', 'Sex', 'Age', "SibSp", "Parch", "Fare", "Embarked"]
numeric_features = [0, 2, 3, 4, 5]
numeric_transformer = StandardScaler()

categorical_features = [1, 6]
categorical_transformer = OneHotEncoder()

# No change in score with or without drop=first
# TODO: I think it's better without because in `LogisticRegression` there is an l2-type regulation/penalty
# categorical_transformer = OneHotEncoder(drop="first")

# Apply ColumnTransformer to create a pipeline that will apply the above preprocessing
feature_encoder = ColumnTransformer(
    transformers=[
        ("cat", categorical_transformer, categorical_features),
        ("num", numeric_transformer, numeric_features),
    ]
)

X_train = feature_encoder.fit_transform(X_train)
print("\n\nX_train fit_transformed head:")
print(X_train[0:5, :].round(2))  # print first 5 rows (not using iloc since now X_train became a numpy array)

# Build the Logistic Regression model
classifier = LogisticRegression(random_state=0)  # Instantiate model
classifier.fit(X_train, y_train)  # Fit model. Ajustement

y_train_pred = classifier.predict(X_train)
print(f"\n\ny_train predictions head: {y_train_pred[0:5]}")

# Evaluate the model but preprocess `X_test` first
X_test[["Age"]] = numerical_imputer.transform(X_test[["Age"]])
X_test[["Embarked"]] = categorical_imputer.transform(X_test[["Embarked"]])
X_test = feature_encoder.transform(X_test)

y_test_pred = classifier.predict(X_test)
print(f"y_test predictions head: {y_test_pred[0:5]}")

# Create the confusion matrix with `plot_confusion_matrix`
cm = confusion_matrix(y_train, y_train_pred, labels=classifier.classes_)
cm_display = ConfusionMatrixDisplay.from_predictions(y_train, y_train_pred)
cm_display.ax_.set_title("Confusion matrix on train set ")

# Mandatory when running in a terminal
plt.show()

print(f"\n\nAccuracy-score on train set : {classifier.score(X_train, y_train):.3f}")

cm = confusion_matrix(y_test, y_test_pred, labels=classifier.classes_)
cm_display = ConfusionMatrixDisplay.from_predictions(y_test, y_test_pred)
cm_display.ax_.set_title("Confusion matrix on test set ")

# Mandatory when running in a terminal
plt.show()

print(f"Accuracy-score on test set : {classifier.score(X_test, y_test):.3f}")
```

<div align="center">
<img src="./assets/img08.webp" alt="" width="900" loading="lazy"/><br/>
<span><b>Running the code in the terminal</b></span>
</div>




There is also a Jupyter notebook. The code is the same at 99% and we get the same results.

<div align="center">
<img src="./assets/img09.webp" alt="" width="900" loading="lazy"/><br/>
<span><b>Running the code in VSCode in a Jupyter notebook</b></span>
</div>


So the accuracy is 0.803 on the train set and 0.788 on the test set. Then what?

Good news first. The two values are close which suggests our model isn't overfitting. It generalizes reasonably well when used with unseen data.

But wait... Should we really be celebrating an 80% accuracy? Well, it depends. Remember what we said earlier: accuracy alone can be misleading. What if 80% of passengers actually died? A dumb model that always predicts "died" would score 80% accuracy without learning anything useful.

Let's dig deeper and look at the full confusion matrix of the test set (the unseen data).

<div align="center">
<img src="./assets/img10.webp" alt="" width="600" loading="lazy"/><br/>
<span><b>Confusion matrix of the test set</b></span>
</div>


On the screen capture above we can read:
- **TN = 96:** We correctly predicted 96 passengers would not survive (Correct Rejections)
- **FP = 14:** We predicted 14 passengers would survive, but they didn't (False Alarms)
- **FN = 24:** We predicted 24 passengers wouldn't survive, but they actually did (Misses)
- **TP = 45:** We correctly predicted 45 survivors (Hits)

Total: 96 + 14 + 24 + 45 = 179 passengers in the test set (20% of the data set).

Now let's compute "by hands" our metrics:

- **Accuracy** = (96 + 45) / 179 = 141 / 179 ≈ **0.788** This matches what sklearn reported
- **Precision** = 45 / (45 + 14) = 45 / 59 ≈ **0.763** "When I predict survival, I'm right 76% of the time"
- **Recall** = 45 / (45 + 24) = 45 / 69 ≈ **0.652** "I found 65% of the actual survivors"
- **F1-score** = 2 × (0.763 × 0.652) / (0.763 + 0.652) ≈ **0.703**

What does this tell us? Our model is more cautious than aggressive. It's better at not crying wolf (decent `Precision`) than at finding all survivors (lower `Recall`). In other words, when it predicts someone will survive, it's fairly reliable. But it misses about a third of the actual survivors.

Is that a problem? It depends on the context and this brings us to our next sections...










<!-- ###################################################################### -->
### Rust
{: .no_toc }

The code below is much longer because
1. I wanted to make sure it "looks like" the code written in Python: imputer, one hot encoding, split test and train datasets...
1. In Python, Scikit-Learn is doing all the hard work for us


```rust
// Rust guideline compliant 2025-12-15

use csv::Reader;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{s, Array1, Array2, Axis};
use serde::Deserialize;

/// Passenger record from Titanic dataset
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Passenger {
    #[serde(rename = "PassengerId")]
    passenger_id: u32,

    #[serde(rename = "Survived")]
    survived: u8,

    #[serde(rename = "Pclass")]
    pclass: u8,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Sex")]
    sex: String,

    #[serde(rename = "Age")]
    age: Option<f64>,

    #[serde(rename = "SibSp")]
    sibsp: Option<u8>,

    #[serde(rename = "Parch")]
    parch: Option<u8>,

    #[serde(rename = "Ticket")]
    ticket: String,

    #[serde(rename = "Fare")]
    fare: f64,

    #[serde(rename = "Cabin")]
    cabin: Option<String>,

    #[serde(rename = "Embarked")]
    embarked: Option<String>,
}

/// Loads Titanic dataset and returns features matrix X and labels y.
///
/// Features: Pclass, Sex, Age, SibSp, Parch, Fare, Embarked (encoded)
///
/// This function mimics the Python preprocessing:
/// - Drops PassengerId, Name, Ticket, Cabin columns
/// - Keeps rows with missing values (imputation done separately)
fn load_titanic(path: &str) -> (Vec<Passenger>, Array1<u8>) {
    let mut rdr = Reader::from_path(path).expect("Failed to read CSV file");

    let mut passengers = Vec::new();
    let mut labels = Vec::new();

    for result in rdr.deserialize::<Passenger>() {
        let p = result.expect("Failed to deserialize passenger");
        labels.push(p.survived);
        passengers.push(p);
    }

    let y = Array1::from(labels);
    (passengers, y)
}

/// Computes mean of non-missing Age values.
fn compute_age_mean(passengers: &[Passenger]) -> f64 {
    let ages: Vec<f64> = passengers.iter().filter_map(|p| p.age).collect();
    let sum: f64 = ages.iter().sum();
    sum / ages.len() as f64
}

/// Imputes missing Age values with mean and missing Embarked with "Unknown".
fn impute_missing(passengers: &mut [Passenger], age_mean: f64) {
    for p in passengers.iter_mut() {
        if p.age.is_none() {
            p.age = Some(age_mean);
        }
        if p.sibsp.is_none() {
            p.sibsp = Some(0);
        }
        if p.parch.is_none() {
            p.parch = Some(0);
        }
        if p.embarked.is_none() {
            p.embarked = Some(String::from("Unknown"));
        }
    }
}

/// One-hot encodes categorical variable.
///
/// Returns (encoded_features, categories) where categories maps index to category name.
fn one_hot_encode(values: &[String]) -> (Array2<f64>, Vec<String>) {
    let mut categories: Vec<String> = values.iter().cloned().collect();
    categories.sort();
    categories.dedup();

    let n_samples = values.len();
    let n_categories = categories.len();
    let mut encoded = Array2::zeros((n_samples, n_categories));

    for (i, val) in values.iter().enumerate() {
        let cat_idx = categories.iter().position(|c| c == val).unwrap();
        encoded[[i, cat_idx]] = 1.0;
    }

    (encoded, categories)
}

/// Converts passengers to feature matrix with preprocessing.
///
/// Features order: Pclass, Sex (one-hot), Age, SibSp, Parch, Fare, Embarked (one-hot)
fn passengers_to_features(passengers: &[Passenger]) -> Array2<f64> {
    // Extract categorical features
    let sex_values: Vec<String> = passengers.iter().map(|p| p.sex.clone()).collect();
    let embarked_values: Vec<String> = passengers
        .iter()
        .map(|p| {
            p.embarked
                .clone()
                .unwrap_or_else(|| String::from("Unknown"))
        })
        .collect();

    // One-hot encode
    let (sex_encoded, _) = one_hot_encode(&sex_values);
    let (embarked_encoded, _) = one_hot_encode(&embarked_values);

    // Extract numeric features
    let n_samples = passengers.len();
    let mut numeric_features = Array2::zeros((n_samples, 5)); // Pclass, Age, SibSp, Parch, Fare

    for (i, p) in passengers.iter().enumerate() {
        numeric_features[[i, 0]] = f64::from(p.pclass);
        numeric_features[[i, 1]] = p.age.unwrap_or(0.0);
        numeric_features[[i, 2]] = f64::from(p.sibsp.unwrap_or(0));
        numeric_features[[i, 3]] = f64::from(p.parch.unwrap_or(0));
        numeric_features[[i, 4]] = p.fare;
    }

    // Concatenate: Sex (one-hot) + numeric + Embarked (one-hot)
    // This matches Python's ColumnTransformer order: cat, num
    let mut features = Array2::zeros((
        n_samples,
        sex_encoded.ncols() + numeric_features.ncols() + embarked_encoded.ncols(),
    ));

    let mut col_idx = 0;

    // Add sex one-hot
    for col in 0..sex_encoded.ncols() {
        features
            .slice_mut(s![.., col_idx])
            .assign(&sex_encoded.column(col));
        col_idx += 1;
    }

    // Add embarked one-hot
    for col in 0..embarked_encoded.ncols() {
        features
            .slice_mut(s![.., col_idx])
            .assign(&embarked_encoded.column(col));
        col_idx += 1;
    }

    // Add numeric features
    for col in 0..numeric_features.ncols() {
        features
            .slice_mut(s![.., col_idx])
            .assign(&numeric_features.column(col));
        col_idx += 1;
    }

    features
}

/// Standardizes features (zero mean, unit variance).
///
/// Returns (standardized_features, mean, std) for later use on test set.
fn standardize_features(x: &Array2<f64>) -> (Array2<f64>, Array1<f64>, Array1<f64>) {
    let mean = x.mean_axis(Axis(0)).unwrap();
    let std = x.std_axis(Axis(0), 0.0);

    let mut x_scaled = x.clone();
    for i in 0..x.ncols() {
        let std_val = if std[i] > 1e-10 { std[i] } else { 1.0 };
        x_scaled
            .column_mut(i)
            .mapv_inplace(|v| (v - mean[i]) / std_val);
    }

    (x_scaled, mean, std)
}

/// Applies standardization using pre-computed mean and std.
fn apply_standardization(x: &Array2<f64>, mean: &Array1<f64>, std: &Array1<f64>) -> Array2<f64> {
    let mut x_scaled = x.clone();
    for i in 0..x.ncols() {
        let std_val = if std[i] > 1e-10 { std[i] } else { 1.0 };
        x_scaled
            .column_mut(i)
            .mapv_inplace(|v| (v - mean[i]) / std_val);
    }
    x_scaled
}

/// Splits data into train and test sets.
///
/// Uses simple 80/20 split (random_state=0 equivalent: first 80% train, last 20% test).
fn train_test_split<T: Clone>(
    x: Array2<f64>,
    y: Array1<T>,
    test_size: f64,
) -> (Array2<f64>, Array2<f64>, Array1<T>, Array1<T>) {
    let n_samples = x.nrows();
    let n_test = (n_samples as f64 * test_size).round() as usize;
    let n_train = n_samples - n_test;

    let x_train = x.slice(s![0..n_train, ..]).to_owned();
    let x_test = x.slice(s![n_train.., ..]).to_owned();
    let y_train = y.slice(s![0..n_train]).to_owned();
    let y_test = y.slice(s![n_train..]).to_owned();

    (x_train, x_test, y_train, y_test)
}

/// Computes confusion matrix.
///
/// Returns [[TN, FP], [FN, TP]]
fn confusion_matrix(y_true: &Array1<u8>, y_pred: &Array1<u8>) -> [[usize; 2]; 2] {
    let mut cm = [[0usize; 2]; 2];

    for (t, p) in y_true.iter().zip(y_pred.iter()) {
        cm[*t as usize][*p as usize] += 1;
    }

    cm
}

/// Computes accuracy score.
fn accuracy_score(y_true: &Array1<u8>, y_pred: &Array1<u8>) -> f64 {
    let correct = y_true
        .iter()
        .zip(y_pred.iter())
        .filter(|(t, p)| t == p)
        .count();
    correct as f64 / y_true.len() as f64
}

fn main() {
    println!("Loading Titanic dataset...\n");

    // Load dataset
    let (mut passengers, y) = load_titanic("data/titanic.csv");

    println!("Dataset shape: {} passengers\n", passengers.len());
    println!("First 5 passengers (before preprocessing):");
    for (i, p) in passengers.iter().take(5).enumerate() {
        println!("{}: {:?}", i, p);
    }

    // Remove useless columns (already done by not including them in features)
    // Columns removed: PassengerId, Name, Ticket, Cabin

    // Preprocessing: impute missing values
    let age_mean = compute_age_mean(&passengers);
    println!("\n\nAge mean for imputation: {:.2}", age_mean);

    impute_missing(&mut passengers, age_mean);

    // Convert to feature matrix
    let x = passengers_to_features(&passengers);
    println!("\nFeature matrix shape: {:?}", x.dim());
    println!("First 5 rows (after encoding):");
    println!("{:.2}", x.slice(s![0..5, ..]));

    // Split into train and test sets (80/20)
    let (x_train, x_test, y_train, y_test) = train_test_split(x, y, 0.2);

    println!(
        "\n\nTrain set: {} samples, Test set: {} samples",
        x_train.nrows(),
        x_test.nrows()
    );

    // Standardize features (fit on train, apply on test)
    let (x_train_scaled, mean, std) = standardize_features(&x_train);
    let x_test_scaled = apply_standardization(&x_test, &mean, &std);

    println!("\nStandardized train set (first 5 rows):");
    println!("{:.2}", x_train_scaled.slice(s![0..5, ..]));

    // Build and train Logistic Regression model
    println!("\n\nTraining Logistic Regression model...");
    let dataset = Dataset::new(x_train_scaled.clone(), y_train.clone());

    let model = LogisticRegression::default()
        .max_iterations(100)
        .fit(&dataset)
        .expect("Failed to fit model");

    // Predictions on train set
    let y_train_pred = model.predict(&x_train_scaled);
    println!(
        "\nTrain predictions (first 5): {:?}",
        y_train_pred.slice(s![0..5])
    );

    // Confusion matrix on train set
    let cm_train = confusion_matrix(&y_train, &y_train_pred);
    println!("\n\nConfusion Matrix on train set:");
    println!("TN: {}  FP: {}", cm_train[0][0], cm_train[0][1]);
    println!("FN: {}  TP: {}", cm_train[1][0], cm_train[1][1]);

    let acc_train = accuracy_score(&y_train, &y_train_pred);
    println!("Accuracy on train set: {:.3}", acc_train);

    // Predictions on test set
    let y_test_pred = model.predict(&x_test_scaled);
    println!(
        "\n\nTest predictions (first 5): {:?}",
        y_test_pred.slice(s![0..5])
    );

    // Confusion matrix on test set
    let cm_test = confusion_matrix(&y_test, &y_test_pred);
    println!("\nConfusion Matrix on test set:");
    println!("TN: {}  FP: {}", cm_test[0][0], cm_test[0][1]);
    println!("FN: {}  TP: {}", cm_test[1][0], cm_test[1][1]);

    let acc_test = accuracy_score(&y_test, &y_test_pred);
    println!("Accuracy on test set: {:.3}", acc_test);
}


```


<div align="center">
<img src="./assets/img12.webp" alt="" width="600" loading="lazy"/><br/>
<span><b>Confusion matrix of the test set</b></span>
</div>

With the Python version we had 0.803 on the train set and 0.788 on the test set. Now we have 0.792 and 0.843. Not so different and to tell the truth I did'nt spend too much time on the differences.



<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**












<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Threshold Concept

Let’s take a two-minute breather before moving on…

At this point in the story, we know how to draw a confusion matrix and we understand the nature of the labels it uses (TP, FN, and friends). We’ve also picked up some new vocabulary along the way: Precision, Recall, F1-score, and Accuracy. And the cherry on top? We know what they actually mean. We understand them, they tell a story, and, pure bliss, we even know how to compute them. Finally, we’ve confirmed that when dealing with imbalanced datasets, Accuracy alone isn’t going to be much help. Fantastic! If we look back for a moment, we can be justifiably proud of how far we’ve come.

That said, remember what I explained right at the beginning of this post: *before* diving head-first into optimizing our machine learning model, we need to choose a metric and stick with it. Which means we must give ourselves the tools to choose, for example, between Recall, Precision, F1-score, and Accuracy. And for that, we need to take the time to truly understand how Recall and Precision work and understand the thing that actually causes the fight: the Threshold

So I suggest we leave nightclubs behind for a moment and do a bit of politics instead and imagine that these two metrics are actually two political parties.


### The Threshold is a continuous value
{: .no_toc }

Most classifiers don’t say *"this is positive"* or *"this is negative"*. Instead they say something closer to "Hmm… this looks 73% suspicious to me". That number is continuous. It lives somewhere between 0% and 100%. The Threshold is simply the rule we apply afterward:

```
If the score is above this value Then
    predict Positive
Else
    predict Negative
```

That’s it. No learning. No AI wisdom. Just a decision boundary we choose. By moving this Threshold up and down, we decide how brave (or paranoid) we want to be.



### Watching Precision and Recall move when the Threshold moves
{: .no_toc }

Let’s make this concrete. Same model. Same scores. Only the Threshold will change.

```python
import numpy as np
import matplotlib.pyplot as plt

# Ground truth: 1 = positive, 0 = negative
y_true = np.array([1, 1, 1, 1, 0, 0, 0, 0, 0, 0])

# Continuous scores produced by the model
y_score = np.array([0.95, 0.85, 0.70, 0.40, 0.60, 0.50, 0.30, 0.20, 0.10, 0.05])

def precision_recall(threshold):
    # Convert scores into binary predictions
    y_pred = (y_score >= threshold).astype(int)

    tp = np.sum((y_pred == 1) & (y_true == 1))
    fp = np.sum((y_pred == 1) & (y_true == 0))
    fn = np.sum((y_pred == 0) & (y_true == 1))

    precision = tp / (tp + fp) if tp + fp > 0 else 0.0
    recall = tp / (tp + fn) if tp + fn > 0 else 0.0

    return precision, recall

# Sweep thresholds from 0% to 100%
thresholds = np.linspace(0, 1, 101)
precisions = []
recalls = []

for t in thresholds:
    p, r = precision_recall(t)
    precisions.append(p)
    recalls.append(r)

# Plot
plt.figure()
plt.plot(thresholds, precisions, label="Precision")
plt.plot(thresholds, recalls, label="Recall")
plt.xlabel("Threshold")
plt.ylabel("Metric value")
plt.title("Precision and Recall vs Threshold")
plt.legend()
plt.show()

```

<div align="center">
<img src="./assets/img13.webp" alt="" width="600" loading="lazy"/><br/>
<span><b>Precision and Recall vs Treshold</b></span>
</div>

#### **Why the curves look like "stairs"?**
{: .no_toc }

You might expect smooth curves but, this is not what we get and the curves are step-shaped because:
1. the Threshold varies continuously
1. but the model’s decisions only change when the Threshold crosses an actual score produced by the model.

Between two consecutive score values:
* `TP`, `FP`, and `FN` do not change
* therefore Precision and Recall remain constant

Remember the Alamo but remember that "*The model outputs a finite set of scores; the Threshold only reacts to those.*"

#### **What the plot tells us**
{: .no_toc }

* **Recall** starts high and decreases as the Threshold increases
    * We miss more and more true positives
* **Precision** starts low and increases
    * We progressively filter out false positives

Same model, same data, just one slider moving from 0 to 1.


#### **What about the "weird" point when `Threshold == 1`**
{: .no_toc }

* The model predicts no positive samples at all
* Recall is 0 (we miss everything)
* Precision is undefined and is displayed here as 0 by convention


#### **How to use this kind of plot?**
{: .no_toc }

This figure allows us to make a strong statement: "Choosing a Threshold is not about improving the model. It’s about deciding where WE want to sit on this curve."

Or, even more bluntly: "Asking for a Recall of 90% means accepting a certain Precision, whether we like it or not".

So next time the Marketing manager ask for a Precision of 90% you can answer "No problemo but are YOU OK with a Recall of 30%?".
* Draw a vertical line on the plot
* Explain what happen at **this point**
* Educate people, even marketing manager can learn something


























### Explaining the Threshold "with hands"
{: .no_toc }

Think of the Threshold as a **gate**.
* Lower the gate → more people get through
* Raise the gate → fewer people get through

Now let’s translate that into metrics.



#### **Recall cares about missed positives**
{: .no_toc }

$$\text{Recall} = \frac{\text{TP}}{\text{TP} + \text{FN}}$$

When we raise the Threshold:

* fewer samples are predicted positive
* some true positives fall below the gate
* FN increases
* Recall goes down



#### **Precision cares about mistaken positives**
{: .no_toc }

$$\text{Precision} = \frac{\text{TP}}{\text{TP} + \text{FP}}$$

When we raise the Threshold:

* Fewer negatives sneak through
* FP decreases
* Precision goes up ⬆️



#### **Same action, opposite effects**
{: .no_toc }

```
Threshold ⬆️
  FP ⬇️  → Precision ⬆️
  TP ⬇️  → Recall ⬇️
```

This is why Precision and Recall feel like they fighting political parties. But in fact they are not true enemies, they are more like brothers and sisters fighting for the same toys. They are "negotiating" over the same decision rule.



#### **Why this is fundamentally a trade-off**
{: .no_toc }

Here is the part worth remembering: "Asking for a given Recall value implicitly fixes the best Precision we can hope for (and vice versa)."

If someone says "I want at least 95% Recall", what they are really saying is "I accept more false positives". Now if someone says "I want 99% Precision" they are also saying "I accept missing real positives."

We can't optimize both independently. Wou choose where to stand on the curve.




### And no, we cannot have both at 100%
{: .no_toc }

* Precision = 100% means zero false positives
* Recall = 100% means zero false negatives

That would require:
* perfect separation
* zero overlap between classes
* a world without noise, ambiguity, or measurement error

In other words a toy dataset or a miracle. For real-world problems, Precision and Recall are not targets to maximize simultaneously, they are constraints to balance.







### Things to keep in mind
{: .no_toc }

* Precision vs Recall is NOT a technical issue or a limitation of models.
* It is a decision problem.
* Move the Threshold, and you choose which mistakes you are willing to live with.














<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**












<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ## ROC Curve and AUC


### Ideas to explore
{: .no_toc }

* A standard topic
* Encountered by beginners after confusion matrices -->




<!-- ###################################################################### -->
<!-- ### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02** -->





















<!-- ###################################################################### -->
<!-- ###################################################################### -->
## How to select a Metric of the Confusion Matrix?


<!-- ###################################################################### -->
### When Precision is King: "The Guillotine of Innocence"
{: .no_toc }

**Context:** A system that predicts who is a secret vampire to be executed.
*   **Scenario:** The town's ancient law states: "If the predictor says `VAMPIRE` execution is immediate and irreversible." A false positive means killing an innocent human.
*   **Ridiculous Outcome:** The predictor, to be "safe" only labels 1 person as a vampire every decade. It's almost always the eccentric old historian who collects garlic. **Precision is 100%** (when it predicts vampire, it's correct), but **Recall is terrible** (99% of vampires roam free). The town feels "accurate" but is actually overrun by vampires.
*   **Why Precision Matters:** The cost of a false positive (executing an innocent) is catastrophically high and unethical. We must be *certain* when we trigger the positive action.

**Other Examples:**
*   **Spam Filter for Your Wedding RSVPs:** If it flags a genuine guest's "Yes" as spam, they might not get a seat. You'd rather let some spam through (lower Recall) than miss Aunt Martha's reply.
*   **Targeted Gold Bar Delivery:** A drone system identifies houses for gold bar delivery. Sending a bar to the wrong house (false positive) is a massive financial loss. Better to deliver to fewer, absolutely sure houses.
*   **Buck Rogers’ auto-fire laser:** Buck Rogers’ ship auto-fires when it detects "enemy spacecraft". False Positive = vaporizing Princess Ardala’s limousine. False Negative = letting enemy fighters escape. Buck would rather miss than accidentally start an interplanetary war. Extreme Precision required.




<!-- ###################################################################### -->
### When Recall is Critical: "The Missed Meteor"
{: .no_toc }

**Context:** A global early-warning system for civilization-ending asteroid impacts.
*   **Scenario:** The AI scans the sky. A false negative means the system says "all clear" when a deadly asteroid is inbound (doom for humanity). A false positive means a costly global alarm and panic for no reason.
*   **Ridiculous Outcome:** The system is calibrated to avoid false alarms. **Accuracy is 99.99%** because impacts are rare. But it has **poor Recall**. It misses 1 in 10 real killer asteroids. Humanity is wiped out because we optimized for a quiet, "accurate" system. Too bad...
*   **Why Recall Matters:** The cost of missing a positive instance (a real threat) is existential. You must catch *almost all* threats, even if it means frequent false alarms.

**Other Examples:**
*   **HIV Blood Test:** Telling someone they are HIV-negative when they are positive (false negative) prevents life-saving treatment and leads to further transmissions. You want the test to catch *every single* positive case, even if it means some false scares.
*   **Search & Rescue Drone:** Looking for a lost child in a forest. Flagging a log as the child (false positive) wastes time. *Not* flagging the actual child (false negative) is an unthinkable tragedy. Maximize finding the child at all costs.
*   **Predator cloaking detection:** A human AI tries to detect "invisible Predators". False Negative = Predator eats you. False Positive = shooting at trees. Shooting at trees is acceptable while being eaten is not. => Maximize Recall


<!-- ###################################################################### -->
### Side note
{: .no_toc }

When you have to decide between Precision and Recall try this. **"Which mistake would hurt me the most?"**
* If **missing a real positive** is catastrophic → **Recall**
* If **accusing something innocent** is catastrophic → **Precision**

That’s it. No metrics. No formulas. Just pain.




<!-- ###################################################################### -->
### When the F1-Score is Essential: "The Psychic Friend Recommender"
{: .no_toc }

**Context:** An algorithm for a social app that identifies and recommends potential "best friends" to users.
*   **Scenario:** If it's too strict (high Precision), you get very few, maybe perfect matches, but you miss out on many other great friends (low Recall). If it's too lax (high Recall), it recommends everyone, including terrible matches, flooding you with spam "friendship" requests (low Precision).
*   **Ridiculous Outcome:** **Optimizing only for Accuracy** leads to a useless app: it's easy to be "accurate" by just saying "NO" to everyone (since true friends are rare). You need the **F1-Score** to balance between being a lonely hermit (high Precision) and a desperate networker flooded with incompatible pals (high Recall).
*   **Why F1-Score Matters:** Both false positives (annoying, irrelevant recommendations) and false negatives (missed opportunities) carry significant but *balanced* costs. You need a harmonic mean of both concerns.

**Other Examples:**
*   **Factory QA for Expensive Gadgets:** Flagging too many good gadgets as defective (false positives) wastes money. Letting too many defective gadgets through (false negatives) ruins your brand. Both are costly; you need a balance.
*   **Document Triage for Legal Discovery:** Finding "smoking gun" emails in a million documents. Missing a key email (false negative) loses the case. Flagging too many irrelevant emails (false positive) buries lawyers in wasted time. Balance is key.





<!-- ###################################################################### -->
### When Accuracy is Meaningful: "The Potato-Chip Sorting Oracle"
{: .no_toc }

**Context:** An automated optical sorter in a chip factory separating perfect chips from broken bits (burned, too small, etc.).
*   **Scenario:** The process is binary: keep (good) or reject (bad). The pieces look very different. A false positive (sending a broken bit to the bag) angers one customer. A false negative (throwing away a perfect chip) costs a tiny fraction of a cent.
*   **Ridiculous Outcome:** Focusing only on **Recall** (catch all broken bits) might lead to rejecting 30% of perfect chips, destroying profit. Focusing only on **Precision** (ensure every rejected item is truly bad) might let too many broken bits through. Here, **Overall Accuracy** is a fantastic simple metric because the classes are roughly balanced and the costs of both errors are similar and *low*.
*   **Why Accuracy Matters:** When the dataset is balanced and the cost of both types of errors is roughly symmetrical and acceptable, accuracy gives a clear, intuitive measure of total correctness.

**Other Examples:**
*   **Broadcast Weather (Rain/No Rain):** For most people, the cost of carrying an umbrella on a sunny day is similar to the cost of getting wet. A false positive and a false negative are equally annoying. Being correct most of the time (accuracy) is what builds trust.
*   **Trivia Bot Answer Classification (Right/Wrong):** You're gauging its general knowledge. Neither type of error (calling a right answer wrong or vice versa) is more costly than the other. You just want it to be correct as often as possible.
*   **Predator's "Should I Attack Earth?" Decision Model** Training data: 999,999 days of "Earth not worth attacking", 1 day of "Schwarzenegger is here!". Model: "Never attack Earth" → 99.9999% accuracy! Predator elders: "Our model has six nines of accuracy!". Predator warriors: "But we never get to hunt anything...". Dies of boredom with perfect accuracy scores.





<!-- ###################################################################### -->
### Things to keep in mind
{: .no_toc }

* Choosing a metric isn't a technical afterthought—it's a **value judgment** about what kind of mistakes you're willing to make. These shocking examples force that point home.
* Think in terms of cost
    * What is the cost of missing a positive instance => Recall high => Precision low
* Think in term of strictness
    * Strict = Precision high = Recall low











<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**









<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Multi-class Confusion Matrices


The concept of a confusion matrix naturally extends beyond binary classification to **multi-class problems**, where the model must choose between more than two classes.

In a multi-class confusion matrix
* **Rows** represent the true classes
* **Columns** represent the predicted classes
* Each cell indicates how many samples of a given true class were predicted as another class

The first diagonal still represents **correct predictions**, while anti-diagonal values highlight **confusions between classes**. This makes the matrix especially useful to understand which classes the model tends to mix up.

### Example
{: .no_toc }

Imagine a classifier that recognizes handwritten digits (`0` to `9`).
A multi-class confusion matrix can quickly show that the model often confuses **3 and 5**, but almost never mistakes **1 for 8**.


### Minimal Python example
{: .no_toc }

Thanks to Scikit-Learn we call the same functions: `confusion_matrix()` and `ConfusionMatrixDisplay()`.

```python
import numpy as np
import matplotlib.pyplot as plt
from sklearn.metrics import ConfusionMatrixDisplay, confusion_matrix

# Example ground truth and predictions for a 3-class problem
y_true = np.array([0, 1, 2, 2, 1, 0])
y_pred = np.array([0, 2, 1, 2, 1, 0])

# Compute confusion matrix
cm = confusion_matrix(y_true, y_pred)

# Display confusion matrix
disp = ConfusionMatrixDisplay(confusion_matrix=cm)
disp.plot()
plt.show()
```


<div align="center">
<img src="./assets/img14.webp" alt="" width="600" loading="lazy"/><br/>
<span><b>Multi-class Confusion Matrix</b></span>
</div>

This provides an intuitive overview of model performance across the classes.




<!-- ###################################################################### -->
### Exercices
{: .no_toc }


**Exercice 00**

**Exercice 01**

**Exercice 02**

















<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Last Dance (*Donna Summer, ’79*)

So here we are. The lights are coming back on, the DJ is packing up, and it’s time to face reality, both in machine learning *and* on the dance floor.

If there’s one thing this long detour through confusion matrices, metrics, thresholds, vampires, asteroids, and Titanic passengers should have taught us, it’s this: **being confident is not the same thing as being right**. And sadly, my nightclub career was a textbook example of that distinction.

Looking back at my personal confusion matrix, the verdict is brutal but fair:

* My Recall was phenomenal. I *never* missed an opportunity to predict success.
* My Precision, on the other hand… Let’s say it was more "optimistic" than "scientific".
* My Accuracy? Technically measurable, emotionally questionable.
* And the F1-score politely suggests I should have spent more time calibrating my Threshold instead of rehearsing dance moves.

In other words, I was an overconfident classifier operating with a catastrophically low decision Threshold. Every weak signal was interpreted as a strong positive. A smile? Positive. Eye contact? Definitely positive. Standing within a 3-meter radius? Highly positive. The model was clearly overfitting on noise.

And that’s exactly the point. The confusion matrix is not about math for the sake of math. It’s a mirror. It forces us to confront *how* we are wrong, not just *how often*. It tells we whether we cry wolf too much, miss real signals, or proudly achieve 99% accuracy while being completely useless.

Once we understand that:
* Precision and Recall stop being abstract formulas
* Thresholds stop being "technical details"
* And metric selection stops being an afterthought

They become explicit choices about the mistakes we are willing to make.

So whether we are building a fraud detector, a medical test, a recommender system, or just trying to predict how our Friday night will end, we should remember this:
**the world doesn’t reward confidence, it rewards calibrated confidence**.

As for me? I eventually adjusted my model, raised my threshold, improved my precision… and married my best True Positive. Not bad for a guy who started with 70 false alarms.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

* [Fraud Detection 2](https://github.com/40tude/fraud_detection_2)