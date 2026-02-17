---
published: true
author: 40tude
lang: en-US
layout: default
title: "Half of All Developers Have Less Than 5 Years of Experience: Myth, Reality, and Why It Matters"
description: "A data-driven investigation into one of the most repeated claims in software engineering, and what it means for technical education in 2026."
image: docs/06_programmation/005_50_percent_dev/assets/img03.webp
twitter:
  card: summary_large_image
parent: "Programmation"
# nav_order: 35
date:               2026-02-15 22:00:00
last_modified_date: 2026-02-17 11:30:00
---


# Half of All Developers Have Less Than 5 Years of Experience: Myth, Reality, and Why It Matters
{: .no_toc }

A data-driven investigation into one of the most repeated claims in software engineering, and what it means for technical education in 2026.




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>
</h2>



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## TL;DR
{: .no_toc }

* In 2017, the data was remarkably close to Uncle Bob’s prediction.
* For decades, the observation remained broadly accurate due to sustained industry growth.
* Starting in 2024, the growth rate of the developer population began to decelerate.
* The developer population is aging, changing the experience distribution.
* Software engineering continues to expand globally, creating entirely new audiences that must relearn established knowledge.
* The core insight remains profoundly relevant: software engineering is still a young field with constant influx and insufficient knowledge transfer.
* The rise of AI does not reduce the need for education. It increases the need for fundamentals, validation, and critical thinking.
* We are no longer just training developers. We are training engineers who can guide, constrain, and validate machines that build.



<div align="center">
<img src="./assets/img00.webp" alt="" width="600" loading="lazy"/><br/>
<!-- <span>A step-by-step guide to leveraging Claude and Microsoft's Rust Guidelines in VSCode.</span> -->
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
## 01 - Where Does the Claim Come From?

<!-- *Tracing the source to Uncle Bob Martin's keynotes* -->

You've probably heard it before, in a conference talk, a blog post, or a podcast: *"Half of all developers have less than five years of experience."* The statement is striking, almost alarming. It suggests an industry in perpetual adolescence, where the majority of practitioners are still figuring things out. But where does this claim actually originate, and is it true?

The primary source is **Robert C. Martin**, universally known as "Uncle Bob." Co-author of the Agile Manifesto, author of *Clean Code* and *Clean Architecture*. One of the most influential voices in software craftsmanship, Martin has been making this argument in his famous "Future of Programming" talk since at least 2014. His reasoning is elegantly simple: if the number of programmers worldwide doubles every five years, then at any given moment, half of them must have been hired within the last five years.

If the reasoning is not crystal clear, read this page which explain  [what is a percentage]({%link docs/03_maths/002_pourcentages/pourcentages.md%}) and where I take the time to explain exponential growth plus many other things like the rule of the 70 etc.

<figure style="text-align: center;">
<iframe width="560" height="315" src="https://www.youtube.com/embed/ecIWPzGEbFc?si=ursHMV8_PrXlskjb&amp;start=3059" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
<figcaption></figcaption>
</figure>

> *"The number of programmers in the world doubles every five years, which means that half the programmers in the world have less than five years of experience. And this leaves our industry in a state of perpetual inexperience."*
>
> Robert C. Martin, "The Future of Programming" (2016, repeated in every edition since)

Martin's reasoning begins in 1946 with Alan Turing writing the first lines of code, and traces exponential growth through to the present day. In his 2014 blog post "My Lawn," he estimated roughly 22 million programmers worldwide and calculated a growth rate of about 14.5% per year, a doubling time of approximately five years (because [70/15 ≃ 5]({%link docs/03_maths/002_pourcentages/pourcentages.md%}#la-règle-du-70)). The mathematical consequence is inescapable: if the population doubles every five years, the newest half of that population necessarily has less than five years of tenure.

This idea has been enormously influential. It's cited in podcasts (The Changelog #367), Hacker News threads, LinkedIn articles, conference talks, and countless blog posts. But does the data actually support it?





<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 02 - What the Survey Data Actually Says

*Stack Overflow, Developer Nation, and JetBrains paint a nuanced picture*

Let's start with the most authoritative source of developer demographics: the Stack Overflow Developer Survey. With over 65,000 respondents in 2024 and 49,000+ in 2025, it's the largest annual snapshot of the developer community.

> **~50%** of developers had ≤5 years professional experience (SO 2017)
> *Stack Overflow Developer Survey 2017, 64_000+ respondents*

In 2017, when the claim was most frequently repeated, the data was remarkably close to Uncle Bob's prediction. Stack Overflow found that 50.1% of respondents had been coding professionally for five years or less. The number was even higher for mobile developers at software companies (60%). The claim, at that moment, was essentially true.


### Professional Coding Experience: Stack Overflow 2017
{: .no_toc }


<!--
| Experience range | % of developers |
|:-----------------|:-:|
| 0–2 years | **26.3%** 🔴 |
| 3–5 years | **23.8%** 🔴 |
| 6–10 years | 21.4% |
| 11–15 years | 12.6% |
| 16–20 years | 8.4% |
| 20+ years | 7.5% |

*🔴 = ≤5 years professional experience, totaling ~50.1%. Source: Stack Overflow Developer Survey 2017.*
-->



<figure style="text-align: center;">
<img src="./assets/img01.webp" alt="≤5 years professional experience, totaling ~50.1%. Source: Stack Overflow Developer Survey 2017" width="900" loading="lazy"/><br/>
<figcaption>≤5 years professional experience, totaling ~50.1%. Source: Stack Overflow Developer Survey 2017</figcaption>
</figure>



But fast-forward to 2024, and the picture has shifted. The 2024 survey reports that 25% of professional developers have 1 to 4 years of work experience, while the majority of respondents are described as "early-to-mid career professionals" with nine or fewer years of experience. Crucially, the survey notes that respondents have started to "skew more experienced" compared to previous years.

### Professional Coding Experience: Stack Overflow 2024
{: .no_toc }

<!--
| Experience range | % of developers |
|:-----------------|:-:|
| 1–4 years | **25.0%** 🔴 |
| 5–9 years | 26.2% |
| 10–14 years | 20.3% |
| 15–19 years | 12.5% |
| 20–24 years | 8.4% |
| 25+ years | 7.6% |

🔴 *By 2024, only ~25% of professional developers report 1–4 years of experience. The population has aged. Source: Stack Overflow Developer Survey 2024.*
-->

<figure style="text-align: center;">
<img src="./assets/img02.webp" alt="By 2024, only ~25% of professional developers report 1–4 years of experience. The population has aged. Source: Stack Overflow Developer Survey 2024." width="900" loading="lazy"/><br/>
<figcaption>By 2024, only ~25% of professional developers report 1–4 years of experience. The population has aged. Source: Stack Overflow Developer Survey 2024.</figcaption>
</figure>

The 2025 survey reinforces this trend: 38% of respondents have been coding for 15+ years, and the age bracket 35+ has been growing steadily (from 31% in 2022 to 39% in 2024). The Developer Nation survey (SlashData, 2022) reported that close to 60% of developers had less than five years of experience, higher than Stack Overflow's figure, but using a broader definition that includes hobbyists and self-taught learners.







<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 03 - The Doubling Hypothesis in 2025

*Has the exponential growth continued?*

Uncle Bob's claim hinges on one key assumption: that the developer population doubles every five years. For decades, this was roughly accurate. But the latest data tells a more complex story.

### Global Developer Population (2018–2025)
{: .no_toc }


<!--
| Year | Developers (millions) | YoY Growth |
|:-----|:---------------------:|:----------:|
| 2018 | 23.0M | - |
| 2019 | 23.9M | +3.9% |
| 2020 | 24.5M | +2.5% |
| 2021 | 26.8M | +9.4% |
| 2022 | 31.0M | +15.7% |
| 2023 | 35.7M | +15.2% |
| 2024 | 43.0M | +20.4% |
| 2025 | 47.2M | +9.8% |

*Sources: Statista/Evans Data Corp (2018–2024), SlashData (2022–2025). Note: Different research firms use different methodologies. SlashData's 47M figure includes professionals + amateurs. Evans Data estimates 27M professionals in 2024. JetBrains estimates 20.8M professionals in 2025.*
 -->


<figure style="text-align: center;">
<img src="./assets/img03.webp" alt="Sources: Statista/Evans Data Corp (2018–2024), SlashData (2022–2025). Note: Different research firms use different methodologies. SlashData's 47M figure includes professionals + amateurs. Evans Data estimates 27M professionals in 2024. JetBrains estimates 20.8M professionals in 2025." width="900" loading="lazy"/><br/>
<figcaption>Sources: Statista/Evans Data Corp (2018–2024), SlashData (2022–2025). Note: Different research firms use different methodologies. SlashData's 47M figure includes professionals + amateurs. Evans Data estimates 27M professionals in 2024. JetBrains estimates 20.8M professionals in 2025.</figcaption>
</figure>



According to SlashData's 2025 report, the global developer population has reached 47.2 million (including both professionals and amateurs). Professional developers alone grew 70% in three years, from 21.8 million in 2022 to 36.5 million in 2025. But here's the key finding: **the growth rate is decelerating**. After a 21% spike in 2023–2024, growth dropped to just 10% in the last 12 months. SlashData warns this may mark the beginning of a plateau.

> **10%** annual growth rate (2024–2025), down from 21% the year before
> *SlashData, Global Developer Population Trends 2025*

This deceleration means the "doubling every five years" assumption is no longer holding. At a 10% annual growth rate, the doubling time stretches to about seven years. And the composition of that growth matters too: the amateur developer segment is actually shrinking (down over 1 million in the past year), while professionals are staying in the field longer.










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 04 - The Developer Population Is Aging

*Fewer juniors entering, more seniors staying*

### Developer Age Distribution Shift (2022 → 2025)
{: .no_toc }


<!--
| Age group | Q1 2022 | Q1 2025 | Change |
|:----------|:-------:|:-------:|:------:|
| 18–24 | 33% | 23% | **−10 pts** 📉 |
| 25–34 | 30% | 29% | −1 pt |
| 35–44 | 22% | 26% | **+4 pts** 📈 |
| 45–54 | 10% | 14% | +4 pts |
| 55+ | 5% | 8% | +3 pts |

*Source: SlashData, Developer Nation surveys. The 18–24 age group dropped by 10 percentage points in just three years.*
 -->


<figure style="text-align: center;">
<img src="./assets/img04.webp" alt="Source: SlashData, Developer Nation surveys. The 18–24 age group dropped by 10 percentage points in just three years." width="900" loading="lazy"/><br/>
<figcaption>Source: SlashData, Developer Nation surveys. The 18–24 age group dropped by 10 percentage points in just three years.</figcaption>
</figure>



This is perhaps the most striking finding. The developer community is getting *older*, not younger. The share of developers aged 18–24 fell from 33% in early 2022 to just 23% by early 2025, a dramatic 10-point decline in three years. Meanwhile, the 35–44 age bracket climbed from 22% to 26%.

The Stack Overflow 2025 survey corroborates this: 66% of professional developers are now between 25 and 44. The 35+ age group has been growing for three consecutive years.

Several forces explain this shift. First, the tech industry's 2023–2024 layoffs (347,600 workers over two years, according to industry trackers) disproportionately affected junior roles. Second, AI tools are enabling smaller, more senior teams to do more with less, reducing the need for entry-level hires. Third, software development has matured as a career path, people are staying longer instead of transitioning to management or leaving the field.










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 05 - The Verdict: Mostly True, But Evolving

*The claim was accurate and the underlying logic still matters*

**The original claim (Uncle Bob Martin, ~2014–2016):** Mathematically derived from the observation that the developer population doubles every five years. If true, half of all developers necessarily have <5 years of experience.

**Empirical support (2017):** Stack Overflow's 2017 survey confirmed 50.1% of professional developers had ≤5 years of professional experience. Developer Nation (2021–2022) reported ~60%.

**Current situation (2024–2025):** The figure has declined. Stack Overflow 2024 puts the <5 year group at roughly 25%. Growth is decelerating (10%/year), the population is aging, and junior entry has slowed.

**Verdict:** The claim was **substantially true** for two decades (roughly 2000–2020). It is now **becoming less accurate** as growth slows and the profession matures. However, the *core insight*, that software engineering is a young field with massive influx and insufficient knowledge transfer, remains profoundly relevant.










<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 06 - Why This Still Matters. The Case for Continuous Education

*Perpetual inexperience requires perpetual teaching*

Even if the exact "50%" figure is softening, the structural problem Uncle Bob identified hasn't gone away,if anything, it's become more complex. Let's consider why technical education, vulgarization, and returning to "already covered" topics is more critical than ever.

| Key Stat                                     |  Value  | Source              |
| :------------------------------------------- | :-----: | :------------------ |
| Developers who learned a new skill last year | **69%** | Stack Overflow 2025 |
| Developers using or planning to use AI tools | **84%** | Stack Overflow 2025 |
| Developers who trust AI output               | **33%** | Stack Overflow 2025 |
| Developers who actively distrust AI output   | **46%** | Stack Overflow 2025 |



### How Developers Learn (2025)
{: .no_toc }


<!--
| Learning Method                             | % of Respondents |
| :------------------------------------------ | :--------------: |
| Technical documentation                     |      **68%**     |
| Online resources (tutorials, blogs, forums) |      **62%**     |
| AI-powered tools                            |      **52%**     |
| Online courses / certifications             |      **42%**     |
| Stack Overflow                              |      **38%**     |
| Books / physical media                      |      **30%**     |
| Traditional school                          |      **18%**     |

*Source: Stack Overflow Developer Survey 2025 (49,000+ respondents). Documentation and online content dominate formal education.*

 -->


<figure style="text-align: center;">
<img src="./assets/img05.webp" alt="Source: Stack Overflow Developer Survey 2025 (49,000+ respondents). Online resources and documentation dominate over formal education." width="900" loading="lazy"/><br/>
<figcaption>Source: Stack Overflow Developer Survey 2025 (49,000+ respondents). Online resources and documentation dominate over formal education.</figcaption>
</figure>


### 1. The renewal is constant
{: .no_toc }

Even in a slower-growth industry, **69% of developers learned a new language or technique last year**. That’s an astonishing level of churn in professional knowledge.

The tooling landscape keeps shifting:

* Docker → Kubernetes
* REST → gRPC / event-driven systems
* Manual testing → CI/CD pipelines
* jQuery → React → Server Components
* Monoliths → distributed systems → modular monoliths (again)

A developer with 15 years of experience may still be a beginner in Rust, WebAssembly, or LLM integration. Every generation of tools resets part of the learning curve. Remember when C++11 was "modern"? Entire mental models had to be rebuilt.

Experience accumulates but relevance must be continuously renewed.





### 2. The Developer Population Is Expanding Geographically
{: .no_toc }

The global developer base is not static. It is growing rapidly in regions that are encountering many software engineering concepts for the first time:

* South Asia nearly doubled its developer population (≈4M → 7.5M since 2022)
* Greater China more than doubled (≈2.4M → 5.8M)
* South America roughly doubled (≈1.7M → 3.4M)

For many of these developers, ideas like clean architecture, testing discipline, or domain modeling are not "old news." They are new discoveries.

Content that feels repetitive to a Western audience can be foundational elsewhere. What seems like rehashing is often **knowledge diffusion at planetary scale**.




### 3. The Profession Is Aging But Mentorship Hasn’t Scaled
{: .no_toc }

Developers are staying in the field longer, while fewer newcomers enter through traditional academic pipelines.

That creates a paradox:

* The experience pool is deeper than ever
* But structured mentorship has not scaled with it

As the survey shows, developers primarily learn through:

* Documentation (68%)
* Online content (62%)

Formal education barely registers (18%).

In practice, blog posts, talks, tutorials, and open-source examples have become the industry’s informal university. Writing educational material is no longer "giving back." It is part of maintaining the profession itself.




### 4. Tools Change. Fundamentals Don’t.
{: .no_toc }

Uncle Bob emphasized this years ago: the major programming paradigms (structured, object-oriented, and functional) were all established before 1970.

What we call "new" ideas are often rediscoveries:

* Hexagonal architecture
* CQRS
* Event sourcing
* Data locality optimization
* Concurrency patterns

A Rust tutorial on [hexagonal architecture]({%link docs/06_programmation/rust/024_hexagonal/hexagonal_lite.md%}) reaches a completely different audience than a Java one written in 2012, even if the concept is identical.

Each generation must re-learn the same principles through the lens of new tools.






### 5. The AI Era Increases the Need for Understanding
{: .no_toc }

This already changed in early 2026 but the 2025 survey revealed a striking paradox:

* **84%** of developers use or plan to use AI tools
* Only **33%** trust the output
* **46%** actively distrust it
* The #1 frustration: solutions that are *"almost right, but not quite"* (66%)

> *"Three quarters of developers said they would still want to ask a human rather than AI because they don't trust AI answers. 61% cited ethical or security concerns. 61% said they want to fully understand their code."*
>
> Stack Overflow 2025 survey: free-form responses


AI doesn’t remove the need for fundamentals. It raises the bar.

To validate AI-generated code, developers must:

* Understand architecture
* Detect subtle correctness issues
* Recognize security risks
* Debug systems they didn’t directly write

Blindly accepting generated code is exactly the kind of failure mode Uncle Bob warned about: software that works until it catastrophically doesn’t.


Again, it is clear that perceptions shifted dramatically in Q1 2026 with the widespread adoption of AI agents, systems like Opus 4.6, and coordinated teams of agents. Who would have imagined, just six months earlier, that we would be comfortable letting AI agents work for two weeks and produce, with little to no supervision, something as substantial as a nearly [complete C compiler](https://www.anthropic.com/engineering/building-c-compiler)?

Trust in these systems will absolutely continue to grow. But we cannot lose sight of the fundamentals. If we do not want to find ourselves five years from now with an entire segment of developers missing, we must keep hiring junior engineers and take the time to train them properly.



<figure style="text-align: center;">
<iframe width="560" height="315" src="https://www.youtube.com/embed/Nd2pavAegx4?si=ihCyKo9lq2nLl5iA" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
<figcaption>The English audio track is available</figcaption>
</figure>





### 6. Early 2026: From Assistants to Autonomous Teams
{: .no_toc }

We are living through an extraordinary transition.

In less than a year, we’ve moved from a **"ping-pong" model** (ask → answer) to a **"team sport" model** where multiple agents collaborate, iterate, retry, and deliver substantial systems with minimal supervision.

This is powerful and disorienting. The pace of change is now faster than most individuals can comfortably track.

The result is not less need for education, but more:

We must now learn how to *direct*, *constrain*, and *verify* these systems.




### 7. Like Electricity, AI Will Eventually Become Invisible
{: .no_toc }

AI will likely fade into the background and become infrastructure. People who are not programmers will increasingly describe what they want, and software will be generated for them.

They may never realize they are "programming."

But someone must still understand:

* How to validate results
* How to test behavior
* How to define specifications clearly
* How to detect failure modes

Trusting generated software is fundamentally different from trusting a human-written system you can inspect.

If an AI designs the recipe, chooses the ingredients, cooks the meal, and serves it... Do you trust it without tasting?

The same question applies to:

* Medical analysis
* Financial software
* Business automation
* Data interpretation

As AI becomes more seamless, critical thinking must become sharper, not weaker.

Disciplines like TDD, validation pipelines, and explicit specification become even more important because they are how we *constrain* automation.






### 8. The Economic Case for Continuous Education
{: .no_toc }

Training developers is often seen as a cost center. In reality, it is a form of risk management.

Replacing a developer is expensive. Rebuilding shared understanding is even more expensive. A team that does not continuously learn ends up paying in different ways. They accumulate technical debt, misuse new tools, and ship systems that nobody fully understands. The price is paid later through outages, rewrites, and slow delivery.

Investing in education is cheaper than recovering from misunderstanding. Up skilling an existing team preserves context, domain knowledge, and ownership. Education is not charity. It is operational resilience.




### 9. The AI Second-System Effect
{: .no_toc }

Fred Brooks described the ["Second-System Effect"](https://en.wikipedia.org/wiki/Second-system_effect) as the tendency to overbuild the second system because confidence grows faster than judgment.

AI amplifies this effect.

Developers can now produce large systems before they have internalized the constraints that usually come from experience. The tooling removes friction, but it does not replace architectural maturity. This makes it easier to create software that looks sophisticated yet lacks coherence, observability, or long-term maintainability.

Faster construction without deeper understanding increases the probability of fragile systems. Education is what keeps speed aligned with sound engineering decisions.




### 10. Creation Is Faster. Maintenance Is Still Human
{: .no_toc }

AI dramatically accelerates the act of writing code. It does not eliminate the need to maintain it.

Most software effort has always been maintenance (write once, read hundred). Debugging, adapting to new requirements, improving performance, and understanding side effects remain human-driven tasks. Generated code still has to live inside real systems with real constraints.

When teams skip learning because "the AI can write it," they create artifacts that nobody can safely evolve. The bottleneck simply moves from creation to comprehension.

Understanding remains the scarce resource.





### 11. The Risk of Losing Tacit Knowledge
{: .no_toc }

Not all knowledge is written down. Some of the most important engineering insights exist only in experience.

Senior engineers carry mental models about tradeoffs, failure patterns, and system behavior that rarely appear in documentation. When this knowledge is not actively transmitted, it disappears. No tool can regenerate it automatically.

Documentation scales. Tribal knowledge does not.

Continuous teaching is how that invisible layer of expertise becomes shared capability instead of institutional memory loss.




### 12. Teaching Is Part of the Engineering Job
{: .no_toc }

In many professions, transmitting knowledge is considered part of the work itself. Software engineering is no different, even if we sometimes pretend otherwise.

Code reviews, design discussions, internal talks, blog posts, and mentoring are not side activities. They are mechanisms that keep the discipline functional across time. A team that does not teach eventually stops improving because each person must rediscover the same lessons alone.

Writing educational material is not just giving back to the community. It is maintaining the profession’s collective competence.

Engineers do not only build systems. They build the people who will maintain them.









<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 07 - Conclusion: Keep Teaching, Keep Writing, Keep Explaining

Uncle Bob Martin’s observation that half of all programmers had less than five years of experience was originally a mathematical consequence of rapid exponential growth. For nearly two decades, the data supported that claim. In 2025 and beyond, growth is slowing and the developer population is maturing, so the exact ratio is changing.

But the deeper truth has not changed.

Our industry continuously absorbs new practitioners. Tools, platforms, and practices evolve faster than institutional knowledge can stabilize. Fundamentals must be rediscovered, relearned, and re-explained in every technological generation.

### The Responsibility of Technical Educators and Content Creators
{: .no_toc }

Do not hesitate to revisit so-called basic topics. Do not assume your audience already understands the foundations.

An article about [SOLID principles written for Rust]({%link docs/06_programmation/rust/022_solid/solid_00.md%}) does not reach the same audience as one written for C# in 2009, even if the principles are identical. A [modern tutorial on gradient descent]({%link docs/09_machine_learning/002_gradient_descent/gradient_descent.md%}) using today’s Python ecosystem connects with readers who would never encounter the 2015 version. A guide to [hexagonal architecture]({%link docs/06_programmation/rust/024_hexagonal/hexagonal_lite.md%}) using [NATS and distributed services]({%link docs/06_programmation/rust/026_monolith_to_distributed/monolith_to_distributed.md%}) addresses realities that did not exist ten years ago.

Every shift in technology, every new wave of developers, and every geographic expansion of the profession creates a new audience that deserves clear and thoughtful explanations.

Technical education is not just about exploring new territory. It is about making established knowledge accessible and relevant again and again.

### A New Educational Challenge: Leading AI Systems
{: .no_toc }

Education must now prepare developers for roles that did not previously exist.

We need to train team leads of AI agent teams, architects, and managers who understand how to create environments where AI can produce reliable outcomes. Their job is not simply to write code. Their job is to define constraints, validation strategies, and feedback mechanisms so that automated systems deliver results that meet expectations.

This raises an important question.

**How do we teach young developers to become architects and leaders of AI-driven engineering?**

A young architect is not placed in charge of the Jeddah Tower.
A young architect is not assigned to design a nuclear submarine.
A young architect is not made responsible for an A380.

They begin with smaller structures. They learn materials, constraints, safety margins, and verification methods. They work under supervision. They study failures. They develop judgment before they are trusted with scale.

The same progression must exist in the age of AI.

We cannot expect someone who has only experimented with prompts or assembled small applications to suddenly orchestrate autonomous agent systems that generate production software. Leading AI requires architectural thinking, testing discipline, traceability, and risk awareness. The complexity has not vanished. It has moved to a higher level of abstraction.

### Designing the System That Builds the System
{: .no_toc }

The modern architect must design:

* The clarity of the specifications that guide the agents
* The guardrails that constrain their behavior
* The automated tests that define what "done" means
* The observability that reveals drift and failure modes
* The review structures that keep humans meaningfully involved

They are no longer just designing software, they are designing the system that builds the software.

### From Teaching Programming to Teaching Supervision of Automation
{: .no_toc }

Education must evolve from teaching only how to code to teaching how to supervise automated creation.

Developers need to learn how to decompose problems, express verifiable requirements, and construct rigorous validation suites. Practices like test driven development, incremental delivery, and formal review become even more critical because they are how we align fast AI generation with real world correctness.

We are no longer just training developers.

We are training engineers who can guide, constrain, and validate machines that build.











<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
## 08 - Webliography

01. [Robert C. Martin, "My Lawn", Clean Coder Blog, June 2014.](https://blog.cleancoder.com/uncle-bob/2014/06/20/MyLawn.html)
02. [Robert C. Martin, "The Future of Programming" talk (2016)](https://www.youtube.com/watch?v=ecIWPzGEbFc). YouTube
03. [Robert C. Martin, interview on The Changelog Podcast #367 (2019).](https://changelog.com/podcast/367)
04. [Robert C. Martin, interview on Tech Lead Journal #90 (2022).](https://techleadjournal.dev/episodes/90/)
05. [Stack Overflow Developer Survey 2017.](https://insights.stackoverflow.com/survey/2017)
06. [Stack Overflow Developer Survey 2023.](https://survey.stackoverflow.co/2023/)
07. [Stack Overflow Developer Survey 2024.](https://survey.stackoverflow.co/2024/)
08. [Stack Overflow Developer Survey 2025.](https://survey.stackoverflow.co/2025/)
09. [SlashData, "Global Developer Population Trends 2025", May 2025.](https://www.slashdata.co/post/global-developer-population-trends-2025-how-many-developers-are-there)
10. [Developer Nation (SlashData), Pulse Report DN22.](https://www.developernation.net/developer-reports/dn22/)
11. [JetBrains, "Global Developer Population Reaches 19.6 Million in 2024".](https://blog.jetbrains.com/research/2025/01/global-developer-population-2024/)
12. [Evans Data Corporation, Worldwide Developer Population Report 2024.](https://evansdata.com/press/viewRelease.php?pressID=365)
13. [Dice.com, "Most Devs Have Less Than 6 Years Experience", March 2017.](https://www.dice.com/career-advice/devs-less-experience-survey)
14. [Global Nerdy, "Robert C. Uncle Bob Martin: The Future of Programming, 2019 edition".](https://www.globalnerdy.com/2019/08/22/robert-c-uncle-bob-martin-the-future-of-programming-2019-edition/)
15. [What is a percentage?]({%link docs/03_maths/002_pourcentages/pourcentages.md%})
16. [Building a C compiler with a team of parallel Claudes](https://www.anthropic.com/engineering/building-c-compiler)
17. [Mythical Man-Month, The: Essays on Software Engineering (Fred Brooks)](https://www.amazon.com/Mythical-Man-Month-Software-Engineering-Anniversary/dp/0201835959)


*Article researched and written with data collected February 2026. All survey data cited is from publicly available reports.*