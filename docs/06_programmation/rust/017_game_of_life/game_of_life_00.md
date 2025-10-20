---
published: true
lang: en-US
layout: default
title: Building Conway’s Game of Life in Rust with Pixels and Winit 
description: A beginner-friendly guide to using the Pixels and Winit crates to create a graphical version of Conway’s Game of Life in Rust.
parent: Rust
#math: mathjax
date               : 2025-10-20 07:00:00
last_modified_date : 2025-10-23 10:00:00
---

<!-- 
TODO:
* 
-->


# Building Conway’s Game of Life in Rust with Pixels and Winit
{: .no_toc }

A beginner-friendly guide to using the Pixels and Winit crates to create a graphical version of Conway’s Game of Life in Rust.
{: .lead }




<h2 align="center">
<span style="color:orange"><b> 🚧 This post is under construction 🚧</b></span>    
</h2>




<!-- ### This is Episode 00
{: .no_toc } -->


## TL;DR
{: .no_toc }

* For beginners
* The Rust workspace is on GitHub
* Use VSCode + Win11 (not tested elsewhere)

<div align="center">
<img src="./assets/img00.webp" alt="" width="900" loading="lazy"/><br/>
<span>Game of Life in Rust using Winit and Pixels crates.</span>
</div>






<!-- 
#### Posts 
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/016_errors/errors_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/016_errors/errors_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/016_errors/errors_02.md%}) 
-->




## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}






<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->

## Introduction
* The project consist of a Rust workspace with 25 different packages

## Objectives
* 


## A gentle start

When I discussed the idea with ChatGPT and explained that, for teaching purposes, I wanted to build a Game of Life application in Rust and was looking for a helpful library, it kindly pointed me to the Winit and Pixels crates. I then checked out the links below (this is a lie but this what I should have done):   

### winit
* crate: https://crates.io/crates/winit
* doc: https://docs.rs/winit/0.30.12/winit/
* examples: https://github.com/rust-windowing/winit/tree/master/winit/examples

### Pixels
* crate: https://crates.io/crates/pixels
* doc: https://docs.rs/pixels/0.15.0/pixels/
* examples: https://github.com/parasyte/pixels/tree/main/examples


**What I learnt:** Generally speaking, there's quite a bit of documentation, the code is available, but there are no tutorials. Furthermore—and it took me a while to figure this out—if you're looking for an example, you need to go to the GitHub site and check out the `examples/` directory.

There has been, and this is really great, a huge effort made in terms of documentation. However, I don't understand why, unlike the canonical `README.md` file, there isn't a `GETTING_STARTED.md` file that would explain the key concepts behind the API, the important points to keep in mind, the mindset needed when approaching the crate... We could, of course, also have some diagrams and additional explanations around the code samples in the `examples/` directory.

1. Get the code from GitHub 
1. Open the workspace with VSCode
1. Open a terminal (CTRL + ù)
1. `cargo run -p step_00_winit_029`


<div align="center">
<img src="./assets/img01.webp" alt="" width="450" loading="lazy"/><br/>
<span>A first window using Rust, Winit and Pixels</span>
</div>

Now open the package (project) named `step_00_winit_029`. It consists of a `Cargo.toml` and a `main.rs` files

<div align="center">
<img src="./assets/img02.webp" alt="" width="450" loading="lazy"/><br/>
<span>The first package in the Rust Workspace</span>
</div>

Here is `Cargo.toml`
```toml
[package]
name = "step_00_winit_029"
version = "0.1.0"
edition = "2024"

[dependencies]
pixels = "0.15.0"
winit = "0.29"
```

The issue I have is that it seems `Pixels` requires `winit` 0.29 but this is not the last version. See below what I see: 

<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

This is an issue because when you go to [the WinIt GitHub page](https://github.com/rust-windowing/winit/tree/master) you must make sure you pick the version 0.29 **before** looking at the examples source code.  

<div align="center">
<img src="./assets/img04.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

Indeed, but I discovered it much later, between 0.29 and 0.30 there are breaking changes in the API but we will address them with the next sample code. For now, let's study the code below:

```rust
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 150;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut window: Option<&'static Window> = None;
    let mut pixels: Option<Pixels> = None;

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll); 

        match event {
            Event::Resumed => {
                if window.is_none() {
                    let built_window = WindowBuilder::new().with_title("Step_00_winit_029: First try").build(elwt).unwrap();

                    let scale_factor = built_window.scale_factor();
                    println!("Scale factor : {}", scale_factor);

                    let size = built_window.inner_size();

                    let window_ref: &'static Window = Box::leak(Box::new(built_window));

                    let surface = SurfaceTexture::new(size.width, size.height, window_ref);
                    let built_pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

                    window = Some(window_ref);
                    pixels = Some(built_pixels);
                }
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested, ..
            } => {
                if let Some(pixels) = &mut pixels {
                    let frame = pixels.frame_mut();

                    for pixel in frame.chunks_exact_mut(4) {
                        pixel[0] = 0x20; // R
                        pixel[1] = 0x40; // G
                        pixel[2] = 0xFF; // B
                        pixel[3] = 0xFF; // A
                    }

                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        elwt.exit();
                    }
                }
            }

            Event::AboutToWait => {
                window.expect("Bug - Window should exist").request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => {
                elwt.exit();
            }

            _ => {}
        }
    })?;

    Ok(())
}
```


### Comments
This is NOT the code I had at the beginning but since I try to tell you a gentle story I'm lying... In fact I spent too much time mixing winit 0.30 with 0.29 API. Nothing was working. I did'nt read the documentation nor checked the dependencies... A nightmare...

The code is 80 lines...





## Conclusion







<!-- ### Exercises

1. **Exo 1:** 
* ... -->












<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


<!-- 
#### Posts 
{: .no_toc }
* [Episode 00]({%link docs/06_programmation/rust/016_errors/errors_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/016_errors/errors_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/016_errors/errors_02.md%}) 
-->
