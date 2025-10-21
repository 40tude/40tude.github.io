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


**What I learnt:** Generally speaking, there's quite a bit of documentation, the code is available, but, most of the time, there are no tutorials. Furthermore—and it took me a while to figure this out—if you're looking for an example, you need to go to the GitHub site and check out the `examples/` directory.

There has been, and this is really great, a huge effort made in terms of documentation. However, I don't understand why, unlike the canonical `README.md` file, there isn't a `GETTING_STARTED.md` file that would explain the key concepts behind the API, the important points to keep in mind, the mindset needed when approaching the crate... We could, of course, also have some diagrams and additional explanations around the code samples in the `examples/` directory.

1. Get the code from GitHub 
1. Open the workspace with VSCode
1. Open a terminal (CTRL + ù on a FR keyboard)
1. `cargo run -p step_00_winit_029`
    * You must use -p because I use a Rust workspace with multiple packages inside
    * Compare to the other packages (02, 03...) the name of this first package is long because it expresses the fact that it depends on the Winit 0.29 version. 


<div align="center">
<img src="./assets/img01.webp" alt="" width="450" loading="lazy"/><br/>
<span>Blue Screen of Birth: A first window using Rust, Winit and Pixels</span>
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

Indeed, but I discovered it much later, between 0.29 and 0.30 there are breaking changes in the API. We will address them with the next sample code. For now, let's study the code below:

```rust
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{/*ControlFlow,*/ EventLoop},
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
        match event {
            Event::Resumed => {
                if window.is_none() {
                    let built_window = WindowBuilder::new().with_title("Step_00_winit_029: First try").build(elwt).unwrap();
                    
                    let size = built_window.inner_size();
                    let window_ref: &'static Window = Box::leak(Box::new(built_window));
                    let surface = SurfaceTexture::new(size.width, size.height, window_ref);

                    let built_pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

                    window = Some(window_ref);
                    pixels = Some(built_pixels);

                    let scale_factor = window_ref.scale_factor();
                    println!("Scale factor : {}", scale_factor);
                }
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested, ..
            } => {
                if let Some(pixels) = &mut pixels {
                    let frame = pixels.frame_mut();

                    for spot in frame.chunks_exact_mut(4) {
                        spot[0] = 0x20; // R
                        spot[1] = 0x40; // G
                        spot[2] = 0xFF; // B
                        spot[3] = 0xFF; // A
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
This is **NOT** the code I had at the very beginning but since I try to tell you a "gentle" story I'm lying... In fact, at the beginning, I spent too much time mixing Winit 0.30 with its 0.29 API. Nothing was really working. I did'nt read the documentation nor checked the dependencies... A nightmare...

Anyway... The code above is around 80 lines, it consists of one `main()` function, it is partially based on [the code available here](https://github.com/rust-windowing/winit/blob/v0.29.x/examples/window.rs) and it should be working. Right? Do you confirm? Ok, let's move on. 

If you don't understand the signature of `main()` this is not a problem. Feel free to read this series of post about [errors]({%link docs/06_programmation/rust/016_errors/errors_00.md%}) during a rainy day. For the moment keep in mind that `main()` can return errors (do you see `Result` and `Error` type aliases). For example if during the call to `EventLoop::new()`, instead of panicking and stop here, the `?` operator returns the error to `main()` which propagates the error message, if any, in the terminal.

Once in the `main()` function, one of the first thing to do is to create an event loop. Then I propose to forget `window` and `pixels` for the moment and reach the `event_loop.run()` function call. This one is interesting. The closure (`|event, elwt|{...}`) is **moved** into the event loop which now, owns it. 

Then the event loop will run until `elwt.exit()` (`elwt` stands for Event Loop Window Target). The loop is mostly a `match` expression where we respond to the different type of events. In this first try we respond to the events : Resumed, RedrawRequested, AboutToWait, CloseRequested

* **WindowEvent::CloseRequested**: as explained, when this event happens, we call the `.exit()` of the event loop window target. Then Winit closes the window, release the resources, the event loops ends and the `main()` function as well.  

* **Event::AboutToWait**: This event is emitted by the winit's event loop just before it goes idle waiting for new events. It says: "I've finished processing all pending events, and I'm about to sleep until something else happens. Is there anything I can do for you before?" ([more info](https://docs.rs/winit/0.29.15/winit/event/enum.Event.html#variant.AboutToWait)). Here I decided to call `window.request_redraw()` to create a continuous rendering loop. This is not smart, this is brutal. Indeed we are constantly asking to redraw the content of the window. There are smarter options but let start with this one because it mimics somehow the behavior of a game loop where we want to draw N frames per second for example.  

* **WindowEvent::RedrawRequested**: In response to this event we must redraw the content of the window. However, in the case of a Game of Life (or game, simulation...) what I know is: which cells are dead and which ones are alive. My universe, my world is a grid of cells. I don't know anything about the number of screens, the size of the window, the resolution...This is where Pixels comes into the play providing multiple levels of indirection which help me to stay in my universe. We will dig into the details soon but for the moment imagine that our world is a Flatland that we want to paint in blue. 

<div align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/avMX-Zft7K4?si=s7n8lhAexcTKtDlG" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>

In order to paint our universe, we start by getting the `pixels` object (no panic, I did'nt talk about this guy yet). Then, with `frame`,  we get a mutable byte slice from the pixels object. Then we traverse the content of the frame, 4 bytes at a time since the spots of our universe are RGBA encoded with 3 bytes for Red, Green and Blue components while the last byte is for transparency.

Once all the spots of our universe are repainted in blue, we ask the `pixels` object to draw itself. The key point is that I don't care how this will happen. With the level of indirection provided by Pixes and Winit I can stay focus on my universe. For the rest, Pixels and Winit will discuss between them about the most efficient way to render it on the final screen(s). Not my problem and this is pretty cool. Thanks to them.

* **Event::Resumed**: This event is emitted when the application starts or wake up. It is **important** because this is where the magic happens. IOW, this is where all the pipes are connected... Here is a copy of the code fragment:

    ```rust
    if window.is_none() {
        let built_window = WindowBuilder::new().with_title("Step_00_winit_029: First try").build(elwt).unwrap();
        
        let size = built_window.inner_size();
        let window_ref: &'static Window = Box::leak(Box::new(built_window));
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);

        let built_pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

        window = Some(window_ref);
        pixels = Some(built_pixels);

        let scale_factor = window_ref.scale_factor();
        println!("Scale factor : {}", scale_factor);
    }
    ```

We first make sure the `window` has not yet been initialized. Do you see the `if window.is_none()`? This also explains why, above, `window` is an `Option<T>`. 

Now, focus your attention on the 3 function calls:
    * `WindowBuilder::new()`
    * `SurfaceTexture::new()`
    * `Pixels::new()`

At this point it is important to understand the distinctions between `elwt` and the `window`. `elwt` abstracts platform-specific details (Win64, Wayland, Browser DOM...). `elwt` can create windows (do you see the `.build(elwt)`). `elwt` can control the event loop (do you remember the `elwt.exit()`). On the other hand, `window` is a product that `elwt` creates.

This said, we pass the `elwt` to the Winit `WindowBuilder::new()` factory function and get a `built_window` in return. Then we pass a reference to the `built_window` to the Pixels `SurfaceTexture::new()` factory function and we get a `surface` object in return. Finally we pass the `surface` to the Pixels `Pixels::new()` factory function and we get a Pixels object in return.

I know, this seems over complicated especially to display a blue window... So let's draw how it works.

<div align="center">
<img src="./assets/img05.webp" alt="" width="900" loading="lazy"/><br/>
<span>Winit and Pixels rendering pipeline</span>
</div>

* In front of our eyes we have a screen where, may be, there is more than one window on the screen. Think about your cell phone with one app at a time on screen or to your 4 screens configuration at home... To fix the idea I suppose we are on a WIN11 configuration with a 800x600 pixels window on screen. The green rectangle above represents the inside of the window and does not include the border nor the title bar. Those are managed by the operating system. We use `WindowBuilder::new()` to create this window.
* Pixels proposes to use a surface texture to fill efficiently the content of the window. The size of the texture are in pixels. The texture may not have the same size than the window. It can be stretched or compressed to fit the content. Applying the texture to the window content is done by the GPU. If the ratio is 1:1 with the inside of the displayed window there is no distorsion otherwise the content may become weird. This is where we use `SurfaceTexture::new()`.
* To link the texture to our universe the Pixels crate proposes to use a Pixels buffer. Its units are the same as our universe. Each spot is encode on 4 byte (RGBA). In the example above, the Pixels RGBA buffer is 400x300. It is best to ensure that the dimensions of the surface and the RGBA buffer are in a whole number ratio. This is where we use `Pixels::new()`.   
* Finally we have to provide our universe. Its size are whatever we want but they should match the one of the Pixels RGBA buffer. In the loop, we get access to the RGB Pixels buffer with `pixels.frame_mut()` and when it is updated we call `pixels.render()`

Ok... Let's go back to the code fragment and let's pay attention to 3 points:

```rust
event_loop.run(move |event, elwt| {
match event {
    Event::Resumed => {
        if window.is_none() {
            let built_window = WindowBuilder::new().with_title("Step_00_winit_029: First try").build(elwt).unwrap();
            
            let size = built_window.inner_size();
            let window_ref: &'static Window = Box::leak(Box::new(built_window));
            let surface = SurfaceTexture::new(size.width, size.height, window_ref);

            let built_pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

            window = Some(window_ref);
            pixels = Some(built_pixels);

            let scale_factor = window_ref.scale_factor();
            println!("Scale factor : {}", scale_factor);
        }
    }
...
```

**Point 1**: If you hover `SurfaceTexture::new()` then you can see that it expects a static reference. See below:

<div align="center">
<img src="./assets/img06.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>

This is where the `Box::leak()` come into action. It converts a `Box<T>` into a `&'static T` reference intentionally creating a memory leak. It transfers ownership of the heap-allocated value (`Box::new(built_window)`) to "nobody," making it live forever (until the program ends).


**Point2**: When all the function calls are done, at the end of the `Event::Resumed` arm, we update the 2 variables `window` (the reference to the window) and `pixels` (the RGBA buffer) because we need them we we handle the other events. At this point, `built_window` has been moved into Box, then leaked. `window_ref` points to heap memory that will never be freed. I know, I'm not very proud...

**Point3**: In addition to all the level of indirection there is a last one. Indeed on high resolution screens (HiDPI) the logical size of the window may be 800x600 while the physical size is 1000x750. For example this is what happens on my system. On WIN11 you can right click on the screen and select `Display settings`. Here is what I can see:

<div align="center">
<img src="./assets/img07.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Optional comment</span> -->
</div>






<!-- More information about the ControlFlow: https://docs.rs/winit/0.29.15/winit/event_loop/enum.ControlFlow.html -->










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
