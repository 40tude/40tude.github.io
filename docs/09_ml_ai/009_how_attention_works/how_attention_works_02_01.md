---
published: true
author: 40tude
# lang: fr
lang: en-US
layout: default
title: "How Attention Works - 02.01"
description: "From scores to weights to a new post-it: softmax, why the tanh cannot be removed, and the context vector as a weighted average of the encoded sentence."
image: docs/09_ml_ai/009_how_attention_works/assets/img00_02.webp
twitter:
  card: summary_large_image
parent: "ML & AI"
# nav_order: 36
math: mathjax
date:               2026-11-23 12:30:28
# last_modified_date is updated by tools/hooks/pre-commit
last_modified_date: 2026-11-27 17:54:44
---



# {{ page.title }}
{: .no_toc }

{{ page.description }}
{: .lead }





<h2 align="center"><span style="color:#F87115"><b> 🚧 This post is under construction 🚧</b></span></h2>




### This is Episode 02, Part 01
{: .no_toc }



{: .note }
> This post is a series in **4 episodes**, some of them in two or three parts. Alice and Bob need two evenings to rebuild the slide, one box at a time.
>
> * [Episode 00]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_00.md%}): Day 1. Leaving class with the slide. Why word-by-word translation fails, from words to tokens, from tokens to vectors.
> * Episode 01: Day 1, later that evening.
>   * [Part 00]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_01_00.md%}): Reading in order with a GRU, encoder and decoder.
>   * [Part 01]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_01_01.md%}): The bottleneck and the idea of attention.
> * Episode 02: Day 2. The attention layer step by step.
>   * [Part 00]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_02_00.md%}): The question attention asks, and the scores.
>   * [Part 01]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_02_01.md%}) **(you are here)**: Softmax and the context vector.
>   * [Part 02]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_02_02.md%}): One iteration of the decoder.
> * Episode 03: Day 2, later that evening.
>   * [Part 00]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_03_00.md%}): Training with teacher forcing, and inference.
>   * [Part 01]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_03_01.md%}): A real translation, what we simplified, and a peek at Transformers.






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* **Softmax** turns the `seq_len` scores into `seq_len` **attention weights**: all positive, sum equal to 1. Without the `tanh`, the memory would vanish from the weights: softmax ignores what is added to every score.
* **Context vector**: $$c_t = \sum_j \alpha_{tj} h_j$$, a weighted average of the rows of the book, `n_units` numbers. Not "all rows are zero but one": a mix, dosed by the weights. The new post-it.
* Still open: what the decoder does with the context vector. That's the right half of the red box, in Part 02.



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00_02.webp"
    alt="Alice and Bob in front of a laptop. Alice holds a yellow highlighter, Bob a cup. On the slide pinned behind them, the attention part of the red box is highlighted in yellow. On the blackboard: 'Où regarder ?' and the chain Scores, Softmax, Poids, Contexte"
    style="max-width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Day 2, 6 pm. Where to look? Scores, softmax, weights, context.</figcaption>
</figure>










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}









{: .note-title }
> How to run the code of this episode
>
> Go to [jupyter.org](https://jupyter.org/){:target="_blank"}, click `Try` then `JupyterLab`, open a new notebook and copy'n paste the cells.
>
> * The **first cell of each section** is self-contained. It starts with `import numpy as np` and rebuilds everything from scratch: the embedding, the encoder, the book and the post-it, with the same random numbers as in Episodes 00 and 01. You can start reading (and running) at any section.
> * The **other cells of a section** continue from the cells above them, in the same section. Run them in order, without restarting the kernel.
> * The setup lines repeat from one section to the next. That's on purpose: no cell depends on a section you may have skipped.

*Same table, same evening. The scores are on the screen, and so is the hole: nobody knows yet why the `tanh` is there.*




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Softmax: From Scores to Weights

**Bob:** Next box. `softmax`. The one the teacher was talking about when everybody nodded.

**Alice:** "And then softmax highlights the interesting word". I remember. Let's not nod this time. What does it do?

**Bob:** It takes the scores and turns them into weights. Yesterday we said we wanted weights to make a mix: big weights for the rows that matter, tiny weights for the others.

**Alice:** We already have numbers. Why not use the scores as weights directly?

**Bob:** Because they're ugly. Some are negative. What's a negative weight in a mix? "I'll take minus 0.62 of `rust`"?

**Alice:** Okay. Then we divide each score by the sum of the scores, and they add up to 1.

**Bob:** With negative numbers, the sum can be zero. Or tiny. Then you divide by almost nothing and everything explodes.

**Alice:** *(sighs)* Fine. So we want: all weights positive, sum equal to 1. And the higher the score, the higher the weight.

**Bob:** That's softmax. Step one: take the exponential of each score. The exponential of anything is positive, and it keeps the order: a bigger score gives a bigger exponential. Step two: divide by the sum. Now they add up to 1.

$$
\alpha_{tj} = \frac{\exp(e_{tj})}{\sum_{k=1}^{T} \exp(e_{tk})}
$$

**Alice:** $$\alpha_{tj}$$: the weight of row $$j$$ at step $$t$$. And the sum at the bottom goes over all the French tokens, $$k$$ from 1 to $$T$$. So the weights depend on the scores of *all* the rows, not just on their own. If one row goes up, the others go down.

**Bob:** Like slices of a cake. If someone takes a bigger slice, there's less for the others. The cake is always 1.

```python
import numpy as np
np.set_printoptions(precision=2, suppress=True)   # print 2 decimals

# Self-contained cell. Setup from episodes 00 and 01: the book and the post-it
vocab_size, emb_dim, n_units = 100, 4, 3
rng = np.random.default_rng(42)
E = rng.normal(size=(vocab_size, emb_dim))  # blue embedding (French)
tokens = ["<start>", "j'", "aime", "écrire", "du", "code", "propre", "en", "rust", ".", "<end>"]
ids = [0, 18, 5, 27, 12, 33, 61, 8, 42, 3, 1]
X = E[ids]                                  # seq_len x emb_dim = (11, 4)
W_x = rng.normal(size=(n_units, emb_dim))
W_h = rng.normal(size=(n_units, n_units))
h = np.zeros(n_units)
outputs = []
for x in X:                                 # plain RNN encoder of episode 01
    h = np.tanh(W_x @ x + W_h @ h)
    outputs.append(h)
output = np.stack(outputs)                  # the book: seq_len x n_units = (11, 3)
state = h                                   # the post-it: n_units = (3,)

W1 = rng.normal(size=(n_units, n_units))
W2 = rng.normal(size=(n_units, n_units))
v = rng.normal(size=n_units)

def softmax(z):
    return np.exp(z) / np.exp(z).sum()

def scores_for(s_prev):
    return np.tanh(output @ W1.T + W2 @ s_prev) @ v

scores = scores_for(state)
weights = softmax(scores)                   # the attention weights
for token, score, w in zip(tokens, scores, weights):
    print(f"{token:>8} {score:5.2f} -> {w:.2f}")
print("sum:", round(weights.sum(), 6))
```

```text
 <start>  0.18 -> 0.11
      j'  0.05 -> 0.10
    aime  0.17 -> 0.11
  écrire  0.13 -> 0.11
      du  0.26 -> 0.12
    code -0.34 -> 0.07
  propre  0.10 -> 0.11
      en -0.01 -> 0.09
    rust -0.62 -> 0.05
       . -0.18 -> 0.08
   <end> -0.80 -> 0.04
sum: 1.0
```

**Alice:** All positive. Sum: 1. The order is kept: `du` is still first, `<end>` still last. But... they're all around 0.1. That's not "highlighting the interesting word". That's spreading butter evenly on the toast.

**Bob:** 11 tokens, so a perfectly flat mix would be 1/11, about 0.09. We're close to that. Our scores are all between -0.8 and 0.3: they're close to each other, so the weights are close to each other.

**Alice:** So softmax doesn't "highlight" anything by itself. It only highlights if the scores are far apart.

**Bob:** Let me check that with a tiny example. Three scores, and I play with them.

```python
toy = np.array([2.0, 1.0, -1.0])
print(softmax(toy))
print(softmax(toy + 100))                   # same shift for everyone
print(softmax(toy * 5))                     # bigger gaps
```

```text
[0.71 0.26 0.04]
[0.71 0.26 0.04]
[0.99 0.01 0.  ]
```

**Alice:** First line: the biggest score gets 0.71. Not 1, not "the" word. A clear favorite, and the others still get something. Third line: I multiply the scores by 5, the gaps get bigger, and now the favorite takes almost everything. 0.99.

**Bob:** So "softmax highlights the interesting word" is the extreme case. When the trained network is very sure, the scores are far apart and one weight is close to 1. When it hesitates, the weights are shared.

**Alice:** And the second line is weird. I added 100 to every score, and *nothing* changed.

**Bob:** Huh. *(thinks)* Because of the exponential. $$\exp(e + 100) = \exp(e) \times \exp(100)$$. Every term on top gets multiplied by the same $$\exp(100)$$, and the sum at the bottom too. It cancels out.

**Alice:** So softmax doesn't care about the scores themselves, only about the *differences* between them. If I add the same number to all the scores, the weights don't move.

**Bob:** Right. Only who's ahead of whom, and by how much.

*Alice stops writing. She looks at the formula of the scores, then at the second line of the output, then at the formula again.*

**Alice:** Bob. I think I know why there's a `tanh`.

**Bob:** Go.

**Alice:** Imagine there's no `tanh`. The score is $$v^\top (W_1 h_j + W_2 s_{t-1})$$. No `tanh`, so I can distribute $$v^\top$$:

$$
e_{tj} = v^\top W_1 h_j + v^\top W_2 s_{t-1}
$$

**Alice:** Look at the second term. $$v^\top W_2 s_{t-1}$$. There's no $$j$$ in it. It's the same number for every row. It's exactly my "+100".

**Bob:** And softmax ignores it...

**Alice:** ...so the memory disappears from the weights! The weights would only depend on the book, never on $$s_{t-1}$$. The decoder would look at the same places at every step, whether it's about to write `i` or `rust`. The whole point of the machine is gone.

**Bob:** Okay, wait. That's a strong claim. Let me test it. Two very different memories, without `tanh`, then with `tanh`.

```python
def scores_without_tanh(s_prev):
    return (output @ W1.T + W2 @ s_prev) @ v

s_a = state                                 # one decoder memory
s_b = np.array([0.9, -0.5, 0.1])            # another one, very different

print(softmax(scores_without_tanh(s_a)))
print(softmax(scores_without_tanh(s_b)))
print(softmax(scores_for(s_a)))
print(softmax(scores_for(s_b)))
```

```text
[0.1  0.12 0.14 0.12 0.17 0.06 0.06 0.11 0.03 0.07 0.03]
[0.1  0.12 0.14 0.12 0.17 0.06 0.06 0.11 0.03 0.07 0.03]
[0.11 0.1  0.11 0.11 0.12 0.07 0.11 0.09 0.05 0.08 0.04]
[0.08 0.11 0.1  0.09 0.12 0.08 0.1  0.1  0.05 0.1  0.06]
```

**Bob:** *(leans back)* The first two lines are identical. Different memories, same weights.

**Alice:** And with `tanh`, the last two lines are different. Not by much, the weights are random, but different. The memory has a say.

**Bob:** So the `tanh` is not decoration. It tangles the row and the memory together. With it, the effect of the memory on the score depends on the row. Without it, the memory just adds the same constant to every row, and softmax throws it away.

**Alice:** That's the honest answer I wanted. Not "networks need non-linearities". "Without this one, the decoder can't change where it looks." Hole filled.

**Bob:** I'm writing that down. *(writes)* And for Rule 3, if you want to see softmax and attention weights with good animations, there's a 3Blue1Brown video, [Attention in transformers, step-by-step](https://www.youtube.com/watch?v=eMlx5fFNoYc){:target="_blank"}. It's about another architecture, but the "scores, then softmax, then weights" part is the same idea.

**Alice:** Feynman check, mine. Softmax turns the `seq_len` scores into `seq_len` weights: all positive, sum equal to 1, bigger score, bigger weight. It only looks at the differences between scores: adding the same number to all of them changes nothing. Far apart scores give one big weight, close scores give a flat mix. And that's why we need the `tanh` before: without it, the memory would only add a constant, and softmax would erase it.

**Bob:** Approved. And the weights go out of the box, at the top. `Attention weights`, `seq_len`.

**Alice:** Where do they go, up there?

**Bob:** Nowhere, I think. Nothing on the slide uses them after that exit. It's a window, so that *we* can look at where the network looks. The real use of the weights is on the other wire, the one that goes left, into the `x`.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## The Context Vector: a Weighted Average

**Alice:** The `x`. A multiplication. Two wires go in: the weights, from softmax, `seq_len`. And... *(follows the wire with her finger)* ...a big `seq_len x n_units` matrix, coming from the left. That's the book.

**Bob:** The translated book, then. The output of `W1`, the `keys`. We computed them, we have them, it's the obvious thing to multiply. *(starts typing)* `weights @ keys`...

**Alice:** Wait. Why "obvious"?

**Bob:** Because... it's the last version of the book we've seen? Everything after `W1` uses the translated rows. The `+`, the `tanh`, `V`...

**Alice:** *(follows the wire with her finger, all the way down)* Hmm. No. Look. This wire doesn't come out of `W1`. It comes straight from `output`, and it splits *before* `W1`. One branch goes into `W1`, the other one goes up to the `x`.

**Bob:** *(leans over the slide)* Oh. You're right. I'd have bet my cookies on `keys`. *(deletes the line)*

**Alice:** Good thing you didn't. *(pause)* And it makes sense, actually. The translated rows were in the "common language", only to compare them with the memory. To decide *where* to look. But once we know where to look, what do we want to read?

**Bob:** The book itself. The content, not the comparison language. Okay, I'm convinced. And a bit vexed.

**Bob:** Now, the multiplication. A vector of 11 weights times a matrix of 11 rows.

**Alice:** Broadcasting again?

**Bob:** Yes, but the other way around. Each weight multiplies its own row: row 1 times $$\alpha_{t1}$$, row 2 times $$\alpha_{t2}$$... Row of `propre` times the weight of `propre`. We get a matrix of the same shape, `seq_len x n_units`, with each row scaled by its weight.

**Alice:** And then the $$\Sigma$$. On the original slide, next to it, there's a little yellow label: "$$\Sigma$$ colonnes". Sum of the columns.

**Bob:** Sum *along* the columns. We add the 11 rows together, column by column. We get one row of `n_units` numbers.

**Alice:** The context vector. `n_units`. The new post-it.

```python
import numpy as np
np.set_printoptions(precision=2, suppress=True)   # print 2 decimals

# Self-contained cell. Setup from episodes 00 and 01: the book and the post-it
vocab_size, emb_dim, n_units = 100, 4, 3
rng = np.random.default_rng(42)
E = rng.normal(size=(vocab_size, emb_dim))  # blue embedding (French)
tokens = ["<start>", "j'", "aime", "écrire", "du", "code", "propre", "en", "rust", ".", "<end>"]
ids = [0, 18, 5, 27, 12, 33, 61, 8, 42, 3, 1]
X = E[ids]                                  # seq_len x emb_dim = (11, 4)
W_x = rng.normal(size=(n_units, emb_dim))
W_h = rng.normal(size=(n_units, n_units))
h = np.zeros(n_units)
outputs = []
for x in X:                                 # plain RNN encoder of episode 01
    h = np.tanh(W_x @ x + W_h @ h)
    outputs.append(h)
output = np.stack(outputs)                  # the book: seq_len x n_units = (11, 3)
state = h                                   # the post-it: n_units = (3,)

W1 = rng.normal(size=(n_units, n_units))
W2 = rng.normal(size=(n_units, n_units))
v = rng.normal(size=n_units)

def softmax(z):
    return np.exp(z) / np.exp(z).sum()

scores = np.tanh(output @ W1.T + W2 @ state) @ v
weights = softmax(scores)                   # seq_len = (11,)

weighted = weights[:, None] * output        # row j multiplied by weight j
print(weighted.shape)
context = weighted.sum(axis=0)              # sum over seq_len, column by column
print(context.shape)
print(context)
print(np.allclose(context, weights @ output))
```

```text
(11, 3)
(3,)
[ 0.05  0.2  -0.28]
True
```

**Alice:** `weights[:, None]`?

**Bob:** It turns the 11 weights into a column, 11 rows of 1 number. Then broadcasting stretches each weight along its row. Without it, NumPy would try to match the 11 weights with the 3 columns, and complain.

**Alice:** Okay. `(11, 3)`, then `(3,)`. And the last line?

**Bob:** The shortcut. The vector of weights times the book, `weights @ output`, gives exactly the same thing. Multiply, then sum: that's what a vector-matrix product does anyway.

**Alice:** So the "multiplication then sum" of the slide is one product. That was on my list of questions. *(crosses it out)* Let me write it.

$$
c_t = \sum_{j=1}^{T} \alpha_{tj} \, h_j
$$

**Alice:** The context at step $$t$$: each row of the book, times its weight, all added up. And since the weights are positive and add up to 1... *(pauses)* That's an average. A **weighted average** of the rows.

**Bob:** Like your grade at the end of the year. Each exam has a coefficient, the coefficients add up to 1, and your final grade is a mix of all your grades, closer to the ones with a big coefficient.

**Alice:** Now I remember what bothered me in class. The teacher said: "after the multiplication, all the rows are zero except one". And then "okay, I'm exaggerating".

**Bob:** That's the extreme case. If softmax gives 1 to one row and 0 to all the others, then yes: after the multiplication, all the rows are zero except one, and the sum just gives back that row. Attention picks one row of the book. Like a lookup.

**Alice:** But we just saw that softmax almost never gives exactly 1. So in real life, nothing is zero. Every row contributes a little, and the rows with big weights contribute a lot.

**Bob:** Let me fake it. I pretend the network is trained and is about to write `clean`. I invent scores, with a big one for `propre` and a medium one for `code`:

```python
# Pretend the network is trained and is about to write "clean"
fake_scores = np.array([-2, -1, 0, 0.5, -2, 1.5, 3.5, -1, 0, -2, -2])
fake_weights = softmax(fake_scores)
for token, w in zip(tokens, fake_weights):
    print(f"{token:>8} {w:.2f}")

context = fake_weights @ output
print("context:", context)
print("propre: ", output[6])
```

```text
 <start> 0.00
      j' 0.01
    aime 0.02
  écrire 0.04
      du 0.00
    code 0.11
  propre 0.78
      en 0.01
    rust 0.02
       . 0.00
   <end> 0.00
context: [-0.84  0.7  -0.7 ]
propre:  [-0.99  1.   -1.  ]
```

**Alice:** `propre` takes 0.78, `code` 0.11, the rest gets crumbs. And the context is *close* to the row of `propre`, but not equal. Same signs, a bit smaller. A post-it that says "mostly `propre`, a bit of `code`".

**Bob:** That's your new post-it from yesterday, in numbers.

**Alice:** One more question, because I know you'll ask it in the Feynman check. Why a mix at all? Why not just take the row with the biggest weight? An `argmax`, and we're done. Cleaner.

**Bob:** Hmm. Two reasons, I think. One, yesterday's: sometimes several rows matter. For `like`, `aime` *and* `écrire`. A mix can keep both, a single row can't. Two... training. To learn, the network nudges its weights a tiny bit and looks at whether the result gets better. With a mix, a tiny nudge on a score gives a tiny change in the context. With an `argmax`, a tiny nudge changes nothing at all, until suddenly it jumps to another row. There's nothing to learn from "nothing, nothing, nothing, jump".

**Alice:** So the mix is a *soft* lookup. You can turn the knob smoothly. That's why it's called softmax and not max.

**Bob:** I'd never thought of the name like that. I like it.

**Alice:** Feynman check, yours.

**Bob:** The weights multiply the rows of the *original* book, row by row, then we add the rows: that's one vector-matrix product. The result is the context vector, `n_units` numbers. Since the weights are positive and add up to 1, it's a weighted average of the rows. If one weight is 1, it's the row of that token. Usually it's a mix, closer to the rows with big weights. And a mix, unlike a hard choice, can be trained.

**Alice:** Approved. *(looks at the slide)* That's the whole attention box. From the book and the memory to the post-it.

**Bob:** Cookie break?

**Alice:** Cookie break. Then the right half of the red box.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## To Be Continued

The attention box is done: from the book and the memory, the decoder has its new post-it. What it does with it, the right half of the red box, is in [Part 02]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_02_02.md%}).




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

**Papers**
* Bahdanau, Cho, Bengio (2014), [Neural Machine Translation by Jointly Learning to Align and Translate](https://arxiv.org/abs/1409.0473){:target="_blank"}. Section 3.1 for the weights and the context vector.

**Videos**
* 3Blue1Brown, another architecture, but the same "scores, softmax, weights" idea, animated.

<figure style="max-width: 560px; margin: auto;">
<div style="position: relative; padding-bottom: 56.25%; height: 0;">
    <iframe
    src="https://www.youtube.com/embed/eMlx5fFNoYc?start=0"
    title="Attention in transformers, step-by-step - 3Blue1Brown"
    style="position: absolute; inset: 0; width: 100%; height: 100%;"
    allowfullscreen>
    </iframe>
</div>
<figcaption style="text-align: center;">
    Attention in transformers, step-by-step - 3Blue1Brown
</figcaption>
</figure>



