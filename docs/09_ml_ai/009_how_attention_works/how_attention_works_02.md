---
published: false
author: 40tude
# lang: fr
lang: en-US
layout: default
title: "How Attention Works - 02"
description: "Two lost students, one cryptic slide, and the Feynman method: let's rebuild attention, one box at a time."
image: docs/09_ml_ai/009_how_attention_works/assets/img00.webp
twitter:
  card: summary_large_image
parent: "ML & AI"
# nav_order: 36
math: mathjax
date:               2026-11-23 12:30:28
# last_modified_date is updated by tools/hooks/pre-commit
last_modified_date: 2026-11-23 18:05:56
---



# {{ page.title }}
{: .no_toc }

{{ page.description }}
{: .lead }





<h2 align="center"><span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span></h2>





### This is Episode 02
{: .no_toc }

{: .note }
> This post is a series in **4 episodes**. Alice and Bob need two evenings to rebuild the slide, one box at a time.
>
> * [Episode 00]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_00.md%}): Day 1. Leaving class with the slide. Why word-by-word translation fails, from words to tokens, from tokens to vectors.
> * [Episode 01]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_01.md%}): Day 1, later that evening. Reading in order with a GRU, encoder and decoder, the bottleneck, and the idea of attention.
> * [Episode 02]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_02.md%}) **(you are here)**: Day 2. The attention layer step by step: scores, softmax, context vector, one iteration of the decoder.
> * [Episode 03]({%link docs/09_ml_ai/009_how_attention_works/how_attention_works_03.md%}): Day 2, later that evening. Training with teacher forcing, inference, what we simplified, and a peek at Transformers.






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* Point 1
* Point 2



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img00.webp"
    alt="TODO: Add a description"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>TODO: Add a legend</figcaption>
</figure>










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

Lorem ipsum dolor sit amet, consectetur adipiscing elit.

<!-- Link to a video -->
<figure style="max-width: 560px; margin: auto;">
<div style="position: relative; padding-bottom: 56.25%; height: 0;">
    <iframe
    src="https://www.youtube.com/embed/MIeYz6aMBbk"
    title="Add a title"
    style="position: absolute; inset: 0; width: 100%; height: 100%;"
    allowfullscreen>
    </iframe>
</div>
<figcaption style="text-align: center;">TODO: Add a legend</figcaption>
</figure>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Prerequisites

* This
* That




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Level 2 Section

Lorem ipsum dolor sit amet, consectetur adipiscing elit.

<!-- ###################################################################### -->
### Level 3 Section
{: .no_toc }

Lorem ipsum dolor sit amet, consectetur adipiscing elit.


Math inline $$x(t)$$ and, below, math offline:

$$
\frac{dS}{dt} = r \cdot S
$$


<!-- ---------------------------------------------------------------------- -->
#### Comments
Lorem ipsum dolor sit amet, consectetur adipiscing elit.









<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Conclusion

Lorem ipsum dolor sit amet, consectetur adipiscing elit.




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

* [Link to a web page](https://cleonis.nl/physics/phys256/energy_position_equation.php).
* [Link to a page of the site]({%link docs/03_maths/013_gauss_forme_integrale_differentielle/gauss_forme_integrale_et_differentielle.md%})
