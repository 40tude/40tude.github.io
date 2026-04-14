---
published: true
author: 40tude
lang: en-US
layout: default
title: "Ollama 101: Windows 11 Quickstart Checklist"
description: "Everything you need to install, run, and experiment with local LLMs in minutes"
image: docs/09_ml_ai/005_ollama/assets/img00.webp
twitter:
  card: summary_large_image
parent: "ML & AI"
# nav_order: 36
math: mathjax
date:               2026-04-10 15:00:00
# last_modified_date is updated by .git/hooks/pre-commit
last_modified_date: 2026-04-14 12:59:26
---



# {{ page.title }}
{: .no_toc }

{{ page.description }}
{: .lead }





<h2 align="center"><span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span></h2>






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!--
## TL;DR
{: .no_toc }

* Point 1
* Point 2
 -->


<figure style="max-width: 600px; margin: auto; text-align: center;">
<img src="./assets/img00.webp"
    alt="Under Windows 11, use Ollama to install and run LLM locally"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Under Windows 11, use Ollama to install and run LLM locally</figcaption>
</figure>










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc} -->






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ## Introduction

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam luctus blandit tincidunt. Nunc et laoreet ipsum. Fusce neque nisi, blandit vitae magna nec, sollicitudin commodo felis. Morbi a viverra lorem, ut sollicitudin lacus. Pellentesque euismod magna et enim fermentum tempor. Etiam vel sagittis mauris. Praesent dictum nisl sit amet tellus rhoncus mollis. Aenean nisi nibh, tincidunt at massa eget, luctus aliquet lectus. Mauris ac massa dolor. Sed fringilla tristique est. Suspendisse nec leo et magna tincidunt ultrices. Suspendisse lacinia leo sed congue ornare. Mauris congue eu dui posuere ultricies. Duis volutpat volutpat erat, ut consequat nisl bibendum gravida. Curabitur fringilla tincidunt auctor.

<!-- Link to a video -->
<!-- <figure style="max-width: 560px; margin: auto;">
<div style="position: relative; padding-bottom: 56.25%; height: 0;">
    <iframe
    src="https://www.youtube.com/embed/MIeYz6aMBbk"
    title="Add a title"
    style="position: absolute; inset: 0; width: 100%; height: 100%;"
    allowfullscreen>
    </iframe>
</div>
<figcaption style="text-align: center;">TODO: Add a legend</figcaption>
</figure> -->



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Install Ollama

Visit [ollama.com](https://ollama.com/)

<figure style="max-width: 450px; margin: auto; text-align: center;">
<img
    src="./assets/img06.webp"
    alt="Visit ollama.com"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Visit ollama.com</figcaption>
</figure>


```powershell
irm https://ollama.com/install.ps1 | iex
```


<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Install and Run QWEN

This is a "small" model. Perfect to check our setup.

```powershell
ollama run qwen2.5:3b
```


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img01.webp"
    alt="Install and Run QWEN in Ollama"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Install and Run QWEN in Ollama</figcaption>
</figure>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## First prompt

Once you see the `>>>>`, you can try your first prompt.

```powershell
# prompt:
Write a Rust function that reverses a string safely

# Explain what Rust ownership is in simple terms
# ...

# /exit or CTRL+D to exit
```

See below the answer:

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img02.webp"
    alt="First prompt within Ollama"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>First prompt within Ollama</figcaption>
</figure>





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Getting Help and Information

As Claude, Ollama have a set of slash commands but it is poor:

<figure style="max-width: 600px; margin: auto; text-align: center;">
<img
    src="./assets/img16.webp"
    alt="List of Ollama's slash commands"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>List of Ollama's slash commands</figcaption>
</figure>



- Exit Ollama (`/exit` or `/bye`)
- In the terminal type


```powershell
ollama --help
```


<figure style="max-width: 600px; margin: auto; text-align: center;">
<img
    src="./assets/img03.webp"
    alt="Getting Help from Ollama"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Getting Help from Ollama</figcaption>
</figure>


- Let's try `ollama list` to see the list of model available locally:

```powershell
ollama list
ollama show qwen2.5:3b
```


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img04.webp"
    alt="Getting information about QWEN"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Getting information about QWEN</figcaption>
</figure>




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Check where the models are stored

Ollama is installed in `$env:USERPROFILE/.ollama`

```powershell
Get-ChildItem $env:USERPROFILE/.ollama/models -Recurse
```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img05.webp"
    alt="Ollama: checking where the models are stored locally"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Ollama: checking where the models are stored locally</figcaption>
</figure>





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Know your hardware configuration

```powershell
& dxdiag /t "$env:TEMP\dxdiag.txt"; Start-Sleep -Seconds 5; Select-String "Display Memory|Card name" "$env:TEMP\dxdiag.txt"
Get-CimInstance Win32_Processor | Select-Object Name, NumberOfCores, NumberOfLogicalProcessors, MaxClockSpeed
Get-CimInstance Win32_PhysicalMemory | Select-Object Manufacturer, Capacity, Speed, MemoryType
(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB
```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img07.webp"
    alt="Describe the image here"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>I'm a legend</figcaption>
</figure>




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Selecting Gemma 4

Select and copy the text from the terminal into ChatGPT, Claude or other friends then add this prompt:

```
Given the hardware configuration described above, which Gemma 4 model would you recommend for my Windows 11 PC?
```



<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img08.webp"
    alt="Claude recommend 26B A4B (MoE, Mixture of Experts)"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Claude recommend 26B A4B (MoE, Mixture of Experts)</figcaption>
</figure>


After discussing with ChatGPT and double checking [this page](https://ollama.com/library/gemma4) I decided to give a try to Gemma 4:e4B




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Using Gemma 4 in the terminal

⚠️ Make sure to plug an Ethernet cable rather than using Wifi then use the following command.

```powershell
ollama run gemma4:e4b
```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img09.webp"
    alt="Ollama download Gemma 4:e4B"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Ollama download Gemma 4:e4B</figcaption>
</figure>


Once the model is succesfully downloaded, let's try a prompt:

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img10.webp"
    alt="Second prompt with gemma4:e4b"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Second prompt with gemma4:e4b</figcaption>
</figure>


Let's check the state of the GPU while the model is working. I tried different options but I was not abel to display Compute/CUDA on the discrete GPU using task manager (CTRL+SHIFT+ESC). So I propose to open a second terminal and run this command

```powershell
nvidia-smi --query-gpu=utilization.gpu --format=csv -l 1
```

Below we can see that `ollama.exe` uses the GPU. Then the monitoring shows what happens when I run a prompt in another terminal.

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img11.webp"
    alt="The GPU is working"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>The GPU is working</figcaption>
</figure>


**Side Note**
- `%LOCALAPPDATA%/Ollama` contains the logs
- `%LOCALAPPDATA%/Programs/Ollama` contains the app and the lib




<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Using Gemma 4 in the Ollama Windows app

Launch the Ollama app and select Gemma4 model in the drop down list (bottom right). Use a third prompt (we could alternatively paste an image)

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img12.webp"
    alt="Using Gemma4:e4B model in the Ollama Windows app"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Using Gemma4:e4B model in the Ollama Windows app</figcaption>
</figure>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Using Gemma 4 with Claude

* Open VSCode in an empty directory
* Open a terminal and call

```powershell
ollama launch
```

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img13.webp"
    alt="ollama launch"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>ollama launch</figcaption>
</figure>


Select `Claude Code` then use the ➡️ (**right arrow** ) to select in the list of models, at the bottom of the terminal, select `gemma4:e4b`


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img14.webp"
    alt="ollama launch"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>ollama launch</figcaption>
</figure>


Once in Claude, my status line indicates the model, the directory... We are all set:

<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img15.webp"
    alt="gemma4:e4b used by Claude Code in VSCode"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>gemma4:e4b used by Claude Code in VSCode</figcaption>
</figure>

Then I use this prompt

```
show me how to print the 10 first odd number in Python
```

I get this answer:


<figure style="max-width: 900px; margin: auto; text-align: center;">
<img
    src="./assets/img17.webp"
    alt="Answer from the model"
    style="width: 100%; height: auto;"
    loading="lazy"
/>
<figcaption>Answer from the model</figcaption>
</figure>









Then I use this prompt:

```
Can you save the script in the current directory in odd.py
```

Then things get weird and **not usable**. Indeed Claude context is too big and the context of the model is too small. The model doesn't even remember the code it generated few seconds before. It does'nt know how to save a file, it does not use Claude API to save the file...

I also did some tests with Gemma 26B. Same kind of problems/bugs... Not yet usable in my point of view.

This is unfortunate because I wanted to run the model all night to improve some code. To do that, I would need the ability to:

* generate code
* save it
* run it
* read some results (for example, execution time)
* decide whether the latest version of the code is better or not
* repeat the loop

I can’t do this with my “limited” Claude Pro subscription. That’s why I wanted to run everything locally, even if it meant it would be slow. Time wasn’t the issue for this experiment.



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ## Conclusion

Maecenas in arcu id erat interdum tristique sed fermentum tortor. Donec eget felis ornare sem dapibus tincidunt at vitae justo. Mauris feugiat tristique augue at maximus. Vivamus euismod blandit mi, ut pretium libero tempor sit amet. In tristique nisi vel mi molestie, ac ornare enim blandit. Phasellus bibendum diam massa, in tempor purus imperdiet a. Curabitur mattis mauris eget cursus molestie. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.

Aliquam blandit malesuada purus at porta. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vestibulum efficitur sapien leo, id iaculis sem sagittis ac. Praesent dolor nisl, fringilla fermentum maximus id, ornare id justo. Morbi at gravida purus, eu imperdiet risus.
 -->



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## Webliography

* [Unsloth: Gemma 4](https://unsloth.ai/docs/models/gemma-4)


