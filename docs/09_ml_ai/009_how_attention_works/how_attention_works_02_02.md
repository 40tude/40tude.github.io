---
published: true
author: 40tude
# lang: fr
lang: en-US
layout: default
title: "How Attention Works - 02.02"
description: "The right half of the red box: embedding, concat, GRU, fc Dense and softmax. One full iteration of the decoder, in code, and a Feynman check of the evening."
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




### This is Episode 02, Part 02
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
>   * [Part 01]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_02_01.md%}): Softmax and the context vector.
>   * [Part 02]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_02_02.md%}) **(you are here)**: One iteration of the decoder.
> * Episode 03: Day 2, later that evening.
>   * [Part 00]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_03_00.md%}): Training with teacher forcing, and inference.
>   * [Part 01]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_03_01.md%}): A real translation, what we simplified, and a peek at Transformers.






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* **One iteration of the decoder**: embedding of the previous English token, `concat` with the context, GRU with $$s_{t-1}$$ gives $$s_t$$, `fc Dense` and softmax give one probability per English token, `argmax()` picks one. At the next step, the book stays the same, the memory changes, so the weights change.
* Still open: which English token feeds the next step, the predicted one or the right one? That's switch 3, teacher forcing, in Episode 03.



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

*The attention box is done: from the book and the memory, the decoder now has its new post-it, the context vector. What it does with it is the right half of the red box.*




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## One Iteration of the Decoder

*A few cookies later.*

**Alice:** Okay. We have the context vector, `n_units`. It leaves the attention part on the wire labeled `Context vector` and goes right, into `concat`. Let me redraw the whole red box, with everything.

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img15.webp"
    alt="The full slide redrawn with our sentence: the encoder on the left; in the red decoder, the attention part produces the context vector (n_units), concat glues it with the red embedding of the previous English token (emb_dim), the GRU (n_units) also receives s(t-1) from the side, then fc Dense and softmax (vocab_size) give pred; argmax sends the predicted token back to switch 3 through the green wire; the new memory goes out through State and back down to switch 2"
    style="max-width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>The whole red box, redrawn by Alice. One iteration of the decoder, from the bottom (previous English token) to the top (pred).</figcaption>
</figure>

**Bob:** Yesterday we did the decoder without attention: `Embedding`, `GRU`, `fc Dense`, `pred`, the green wire. What changes now?

**Alice:** One box: `concat`. Before, the embedding went straight into the GRU. Now, it goes into `concat` first, and the context vector joins it there.

**Bob:** Let's do one step, from the bottom. Step $$t = 1$$. Switch 3 gives the decoder the index of the previous English token. At step 1, there's no previous token, so...

**Alice:** `<start>`. Index `0`. It goes through the red embedding, the English one, `vocab_size x emb_dim`. A lookup, like the blue one yesterday. We get a vector of `emb_dim` numbers. 4 in the toy code.

**Bob:** Careful, though. It's easy to get this wrong. That embedding is *not* the word we're about to write. It's the last word we *wrote*. At step 1, `<start>`. At step 2, `i`. At step 3, `like`. The word to predict is the next one.

**Alice:** Always one step behind, like $$s_{t-1}$$. Okay. Then `concat`. Two vectors go in, the embedding, `emb_dim`, and the context, `n_units`. And the label above `concat` says `n_units + emb_dim`.

**Bob:** Concat just puts them end to end. 4 numbers, then 3 numbers: one vector of 7.

$$
\left[\, E\,y_{t-1} \,;\, c_t \,\right]
$$

**Alice:** $$y_{t-1}$$ is the previous English token, $$E\,y_{t-1}$$ its embedding, and the semicolon means "glued". The label says `n_units + emb_dim` and you put the embedding first.

**Bob:** The PyTorch code puts the embedding first. The slide draws the context coming from the left. It doesn't matter, as long as we always glue them in the same order: the GRU learns which of its 7 inputs is which.

**Alice:** Fine. Now *why* glue them? What does each piece bring?

**Bob:** The context says: "here is the part of the French sentence that matters right now". The embedding says: "here is the word I just wrote". Both are useful. To write `writing` after `like`, you want to know that the source talks about `écrire`, and that you just wrote `like`, which calls for an `-ing`.

**Alice:** And the third piece of information? The memory, "where am I in the sentence"?

**Bob:** That's $$s_{t-1}$$. It doesn't go through `concat`. It goes into the GRU from the side.

**Alice:** *(taps the drawing)* My wire. The one I added yesterday, because the original slide forgot it. The GRU has its two inputs: the glued vector from below, and the old memory from the side. And it gives the new memory.

$$
s_t = \text{GRU}\left(\left[\, E\,y_{t-1} \,;\, c_t \,\right],\ s_{t-1}\right) \quad (n)
$$

**Bob:** `n_units` again. And look: $$s_{t-1}$$ is used twice. Once in the attention, through `W2`, to decide where to look. Once in the GRU, to remember where we are.

**Alice:** Same memory, two jobs. Okay, and the top is like yesterday. `fc Dense`, from `n_units` to `vocab_size`: one score per English token.

**Bob:** Those raw scores have a name, by the way: **logits**. Then a softmax turns them into probabilities.

**Alice:** Wait. Yesterday you told me this softmax was "hidden in `pred`". On my drawing, I gave it its own box, at the top. *(points)* So there are two softmaxes in the red box.

**Bob:** Two softmaxes, two jobs. The one in the attention works over `seq_len`: it shares the attention between the French tokens. This one works over `vocab_size`: it shares the probability between all the English tokens. Same function, different cakes.

$$
p_t = \text{softmax}\left(W_o\, s_t\right)
$$

**Alice:** $$W_o$$ is the matrix of `fc Dense`, `vocab_size x n_units`. And $$p_t$$ is `pred`: one probability per English token. Then `argmax()`, the most likely one, and the green wire.

{: .note }
> On the original slide, there is no softmax box after `fc Dense`: `pred (x)` comes straight out of it, with `vocab_size` written next to it. The output softmax is implied. In our redrawings, from the figure above on, it gets its own box and `pred` is labeled "probabilities", so that every operation is visible. In real code, `fc Dense` often outputs the logits only, and the softmax is merged into the loss function. More on that in Episode 03, with the training.

**Bob:** Code. Everything in one cell. The GRU is replaced by my plain RNN cell from yesterday, same shapes, fewer lines. And for the toy, the English vocabulary also has 100 tokens.

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

# Attention layer (random weights, not trained)
W1 = rng.normal(size=(n_units, n_units))
W2 = rng.normal(size=(n_units, n_units))
v = rng.normal(size=n_units)

# Decoder (random weights, not trained)
E_en = rng.normal(size=(vocab_size, emb_dim))           # red embedding (English)
Wd_x = rng.normal(size=(n_units, emb_dim + n_units))    # reads the concat
Wd_h = rng.normal(size=(n_units, n_units))              # reads the decoder memory
W_o = rng.normal(size=(vocab_size, n_units))            # fc Dense

def softmax(z):
    return np.exp(z) / np.exp(z).sum()

def attention(s_prev):
    scores = np.tanh(output @ W1.T + W2 @ s_prev) @ v
    weights = softmax(scores)
    context = weights @ output
    return weights, context

# One iteration of the decoder, t = 1
s_prev = state                              # s_0: the post-it
y_prev = 0                                  # index of <start> in the English vocabulary

weights, context = attention(s_prev)        # (11,) and (3,)
y_emb = E_en[y_prev]                        # red embedding: (4,)
gru_in = np.concatenate([y_emb, context])   # concat: (7,)
s = np.tanh(Wd_x @ gru_in + Wd_h @ s_prev)  # new decoder memory s_1: (3,)
logits = W_o @ s                            # fc Dense: (100,)
probs = softmax(logits)                     # pred: (100,)
y_hat = int(np.argmax(probs))               # most likely English token

print(y_emb.shape, context.shape, gru_in.shape, s.shape, logits.shape, probs.shape)
print("sum of probs:", probs.sum())
print("predicted index:", y_hat, "with probability", round(probs[y_hat], 2))
```

```text
(4,) (3,) (7,) (3,) (100,) (100,)
sum of probs: 1.0
predicted index: 95 with probability 0.3
```

**Alice:** *(goes through the shapes one by one)* Embedding, 4: `emb_dim`. Context, 3: `n_units`. Concat, 7: `n_units + emb_dim`. New memory, 3: `n_units`. Logits, 100: `vocab_size`. Probabilities, 100, and they add up to 1. Every label of the red box, checked.

**Bob:** And the decoder says: index 95. We wanted 9, `i`.

**Alice:** Random weights. It has never seen a single English sentence. Index 95 is as good as any.

**Bob:** Still, look at the probability: 0.3. Out of 100 tokens. For a network that knows nothing, it's oddly sure of itself.

**Alice:** Random weights give random confidence. A trained network can be wrong *and* sure of itself too, by the way. Something to remember when we talk about using it for real.

**Bob:** Noted. Now, the new memory $$s_1$$. Where does it go?

**Alice:** *(follows the wire on the drawing)* It leaves the GRU on the right, goes up to `State`, and down again, all the way to the bottom... to switch 2. And from there, at the next step, it would feed `W2` and the GRU. It becomes $$s_{t-1}$$ for step 2.

**Bob:** So at step 2, the attention gets a new memory. The book doesn't change: same 11 rows, the French sentence hasn't moved. But the question changes. So...

**Alice:** ...the scores change, so the weights change. Let me guess how. At step 1, it should look at the beginning of the sentence. At step 2, one row further. At step 3, one more. The attention slides along the French sentence, like a finger under the words.

**Bob:** Sounds reasonable. Let's see if the weights agree.

```python
weights_2, context_2 = attention(s)         # step t = 2: same book, new memory s_1
for token, w1, w2 in zip(tokens, weights, weights_2):
    print(f"{token:>8} {w1:.2f}  {w2:.2f}")
```

```text
 <start> 0.11  0.08
      j' 0.10  0.09
    aime 0.11  0.12
  écrire 0.11  0.11
      du 0.12  0.13
    code 0.07  0.08
  propre 0.11  0.09
      en 0.09  0.09
    rust 0.05  0.05
       . 0.08  0.09
   <end> 0.04  0.07
```

**Alice:** Small changes, because nothing is trained. But they move. `<start>` goes from 0.11 to 0.08, `<end>` from 0.04 to 0.07. *(frowns)* But not the way I said. `j'` goes *down*. `<end>`, at the other end of the sentence, goes *up*. No finger sliding to the next row.

**Bob:** Random weights, maybe? A trained one would slide?

**Alice:** *(thinks, then shakes her head)* No, it's not about the training. Look at the code. Where is the "next row" in there? There's no counter, no position, nothing that says "row 2 now". There's only the book, always all of it, and the memory. My finger doesn't exist.

**Bob:** So at every step, it looks at *all* the rows again and recomputes all the weights. What changes is only the memory, so the question.

**Alice:** Same book, new memory, new weights. New post-it. And if a trained network looks at the rows in order, it's because it *learned* to, not because the machine forces it.

**Bob:** And that's a big deal. Because sometimes the order should *not* be followed.

**Alice:** `code propre` -> `clean code`. To write `clean`, at step 4, it can look at `propre`, row 7, even though it hasn't "used" row 6, `code`, yet. With my finger sliding left to right, it would have looked at `code` first, and written `code clean`. Without the finger, the decoder can jump anywhere at any step.

**Bob:** Feynman check? I'll do this one, it's the biggest.

**Alice:** Go.

**Bob:** One iteration of the decoder. The previous English token goes through the red embedding: `emb_dim` numbers. It's the word we just wrote, not the one we're about to write. The attention uses $$s_{t-1}$$ and the book to build the context vector, `n_units`. `concat` glues the embedding and the context: `emb_dim + n_units`. The GRU takes that vector and the old memory $$s_{t-1}$$, and gives the new memory $$s_t$$, `n_units`. `fc Dense` turns $$s_t$$ into one score per English token, softmax turns the scores into probabilities, `argmax()` picks one. $$s_t$$ goes back down to switch 2 and becomes the memory of the next step.

**Alice:** Objection. You said "the previous English token". At step 2, which one? The one the decoder *predicted*, index 95, garbage? Or the one it *should* have predicted, `i`, index 9?

**Bob:** *(looks at switch 3, then at the green wire, then at the column `<start> i like writing...` under the red box)* That's... exactly what switch 3 is about. "Teacher forcing" on one side, "inference" on the other.

**Alice:** So, objection not rejected. Just postponed.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Feynman Check: The Red Box, Halfway

*7:30 pm. The bag of cookies is lighter. Alice flips the slide over, to the list of questions she wrote yesterday.*

**Alice:** Let's go through yesterday's list. Honestly.

**Bob:** First one. "How `W1`, `W2`, `+`, `tanh`, `V` and `softmax` turn the memory `s` and the book into weights."

**Alice:** Done. `W1` and `W2` translate the book and the memory into a common language. `+` asks the same question to every row. `tanh` tangles the row and the memory, otherwise softmax would erase the memory. `V` gives one score per row. Softmax turns the scores into positive weights that add up to 1. *(crosses it out)*

**Bob:** Second. "Why the mix is a multiplication followed by a sum."

**Alice:** Because the mix is a weighted average: each row times its weight, then everything added up. It's one vector-matrix product. And it's a mix and not a choice because several rows can matter, and because a mix can be trained. *(crosses it out)*

**Bob:** Third. "What `concat` glues together, and why."

**Alice:** The embedding of the last English token we wrote, and the context vector. "What I just wrote" and "what matters in the source right now". The third piece, "where I am", comes through the GRU's side door: $$s_{t-1}$$. *(crosses it out)*

**Bob:** Fourth. "Switches 2 and 3, teacher forcing versus inference, the green wire."

**Alice:** Half. Switch 2: at step 1 it gives $$s_0$$, the post-it, and after that, the decoder's own memory. That part I get. Switch 3 and the green wire... that's your objection. Not crossed out.

**Bob:** Fifth. "How the whole thing is trained."

**Alice:** Not at all. We have a machine full of random weights that says "95" instead of "i". *(leaves it)*

**Bob:** Three and a half out of five.

**Alice:** And the table. Yesterday, two lines were "guessed". Let me fix them and add the new boxes.

| What | Math | Shape | Where on the slide |
|---|---|---|---|
| `output`: the book | $$H$$, rows $$h_j$$ | `seq_len x n_units` | top left, goes to `W1` and to `x` |
| `state`: the post-it | $$s_0 = h_T$$ | `n_units` | goes to switch 2 |
| decoder memory | $$s_{t-1}$$ | `n_units` | out of switch 2, into `W2` and the red `GRU` |
| translated book | $$W_1 h_j$$ | `seq_len x n_units` | out of `W1 Dense` |
| translated memory | $$W_2 s_{t-1}$$ | `n_units`, copied on every row | out of `W2 Dense` |
| scores | $$e_{tj} = v^\top \tanh(W_1 h_j + W_2 s_{t-1})$$ | `seq_len` | out of `V Dense` (1 neuron) |
| attention weights | $$\alpha_{tj}$$, positive, sum 1 | `seq_len` | out of the attention `softmax` |
| context vector: the new post-it | $$c_t = \sum_j \alpha_{tj} h_j$$ | `n_units` | out of `x` and $$\Sigma$$ |
| previous English token | $$y_{t-1}$$ | 1 index | switch 3, then red `Embedding` |
| its embedding | $$E\,y_{t-1}$$ | `emb_dim` | out of the red `Embedding` |
| glued vector | $$[E\,y_{t-1} ; c_t]$$ | `emb_dim + n_units` | out of `concat` |
| new decoder memory | $$s_t$$ | `n_units` | out of the red `GRU`, back to switch 2 |
| `pred` | $$p_t = \text{softmax}(W_o s_t)$$ | `vocab_size` | top of the red box |

**Bob:** That's the whole red box. Every wire.

**Alice:** Every wire of *one* step. *(taps switch 3)* What we don't know yet is how the steps follow each other, and how this thing learns anything at all. That's the bottom of the slide: the switches, "teacher forcing", "inference". And the loss.

**Bob:** Pizza first?

**Alice:** Pizza first. Then switches.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

Let's step out of the library again.

In this episode, Alice and Bob opened the attention box and computed every number inside. At each step, the decoder compares its memory with every row of the encoded sentence. `W1` and `W2` put both in a common space, broadcasting asks the same question to every row, `tanh` keeps the memory in the game, and `V` turns each row into a score. Softmax turns the scores into weights, and the context vector is the weighted average of the rows: a soft lookup, not a hard choice. One iteration of the decoder then glues this context with the previous English token, updates the memory with the GRU, and produces a probability for every English token.

Two ideas deserve to be kept. First, softmax only cares about differences between scores, which is exactly why the `tanh` cannot be removed. Second, the decoder does not move along the source sentence: at every step it looks at all the rows again, and only its memory changes. That is what lets "code propre" become "clean code".

What remains is the part where steps follow one another and where the random weights become useful: training with teacher forcing, the switches, inference, and what we simplified. That will be the topic of [Episode 03]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_03_00.md%}), later the same evening, after the pizza.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

**Papers**
* Bahdanau, Cho, Bengio (2014), [Neural Machine Translation by Jointly Learning to Align and Translate](https://arxiv.org/abs/1409.0473){:target="_blank"}.

**Code**
* PyTorch tutorial, [NLP From Scratch: Translation with a Sequence to Sequence Network and Attention](https://pytorch.org/tutorials/intermediate/seq2seq_translation_tutorial.html){:target="_blank"}. Classes `BahdanauAttention` and `AttnDecoderRNN`.



