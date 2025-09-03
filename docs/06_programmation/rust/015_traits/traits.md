---
published: true
lang: en-US
layout: default
title: "Rust Traits: Defining Character"
description: "From basic syntax to building plugins with once_cell and organizing Rust projects."
parent: "Rust"
#math: mathjax
date               : 2025-09-03 14:00:00
last_modified_date : 2025-09-03 14:00:00
---

# Rust Traits: Defining Character
{: .no_toc }

From basic syntax to building plugins with once_cell and organizing your Rust projects.
{: .lead }


<h2 align="center">
<span style="color:orange"><b>This post is still under construction.</b></span>    
</h2>


## TL;DR
{: .no_toc }

* For beginners
* The code is on [GitHub](https://github.com/40tude/traits_as_plugins)

<div align="center">
<img src="./assets/img00.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>In space, no one can hear you scream.</span> -->
</div>












## A gentle start - Static dispatch
With static dispatch, everything is known at compile time

### Running the demo code
{: .no_toc }

I will not explain how to run the code every time.
* Get the projet from [GitHub](https://github.com/40tude/traits_as_plugins)
* Open the folder with VSCode
* Right click on `assets/00_static_dispatch`
* Select the option "Open in Integrated Terminal"




<div align="center">
<img src="./assets/img01.webp" alt="" width="450" loading="lazy"/><br/>
<span>Open in Integrated Terminal</span>
</div>




***Why do I need to open a terminal before to run the code?*** Simply because the project include multiple projects and some of them are more than few lines of code dropped in a `/examples/demo01.rs` file. 


* Enter `cargo run`



<div align="center">
<img src="./assets/img02.webp" alt="" width="450" loading="lazy"/><br/>
<span>Results in the Integrated Terminal</span>
</div>



* If you don't want to run the code locally, until the chapter about Modules & Crates you should be able to copy and paste the code in the excellent [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d2d109f3055e1780562c8e7a97279470). 
    * For this time, and this time only, you can click the previous link and press CTRL+ENTER once in Rust Playground
    * Or... You can copy the code below 
    * Past it [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024)
    * Press CTRL + ENTER



<div align="center">
<img src="./assets/img03.webp" alt="" width="450" loading="lazy"/><br/>
<span>Running code in Rust PlayGround</span>
</div>


### Explanations 1/2
{: .no_toc }

In this first part, I suggest approaching the problem from the opposite angle. Rather than explaining what a Trait is and then looking at how it is used, we will start with a problem to be solved, see how the Traits respond to the questions, and then study their implementation in the code.

Imagine... Imagine that we work in industry. We deploy control systems at various sites around the world. Don't worry about it. Our task is simple: we install temperature sensors in the factory and we want to read them. Once we have the values, we can display them, store them...  

But we have to anticipate... Sure, we're so efficient and so good that we'll be asked to deploy other types of sensors: pressure sensors, torsion sensors, flow meters, cameras... And while we're at it, we'll be asked to install actuators to close valves, unlock doors, turn on alarms... 

First thing first, let's focus on the temprature sensors. Depending on the region of the world, we are asked to support both °C and °F (nobody's perfect...). On the other hand not all sensors are the same. Some of them may be already in place... Some of them may have different communication link (serial, Ethercat...). So we can imagine that we have different types of temperature sensors, but this should be transparent from the software stand point.

OK... Then what? 

What I just described exists in many other situations, and so there are some people who are smarter than others, who took a step back from all this and said to themselves: what you actually want is for all thermocouples to be measurable. It's a bit like describing people's characters. Some are touchy, others are cheerful, and still others are very intelligent. They are all different people, men, women, young, old... But they all have certain character traits. Well, we're going to give Rust a way to add character traits to existing types. 


For example, I create a Dog type with a struct{}. I then create a Cat type with another struct{}. Next, I describe what the Deceitful character trait is. Finally, I can then enrich the Dog and Cat types with the trait Deceitful. If I decide to say that all Cats are deceitful but not Dogs, I only add the trait Deceitful to Cats. Anyway, you get the idea.

Before we look at the first code example, there is one last point to keep in mind. Since Rust is quite strict about issues with types, we can write functions that take, as parameter, only data type we certain traits. For example, I can write a function that takes as a parameter any animal that has the trait Deceitful. It will then be able to treat Cats, Parrots, etc. in the same way. Similarly, I can create vectors that only contain animals with the trait Deceitful.

Okay, let's move on to studying the first source code and see how all this apply to our thermocouple





### Show me the code!
{: .no_toc }



```rust
pub trait Measurable {
    fn get_temp(&self) -> f64;
}

struct TempSensor01 {
    temp: f64,
}
impl Measurable for TempSensor01 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

#[allow(dead_code)]
struct TempSensor02 {
    label: String,
    temp: f64,
}
impl Measurable for TempSensor02 {
    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

// static dispatch - known at compile time
fn get_temp_from_any_sensor_static1(t: &impl Measurable) {
    println!("{}", t.get_temp());
}

// static dispatch - generic syntax
fn get_temp_from_any_sensor_static2<T: Measurable>(t: &T) {
    println!("{}", t.get_temp());
}

fn main() {
    let my_sensor = TempSensor01 { temp: 25.0 };
    println!("{}", my_sensor.get_temp());

    let sensor1 = TempSensor01 { temp: 25.0 };
    let sensor2 = TempSensor02 {
        label: "thermocouple".into(),
        temp: 25.0, // 77 °F
    };

    get_temp_from_any_sensor_static1(&sensor1);
    get_temp_from_any_sensor_static1(&sensor2);

    get_temp_from_any_sensor_static2(&sensor1);
    get_temp_from_any_sensor_static2(&sensor2);
}
```

### Explanations 2/2
{: .no_toc }









## Dynamic dispatch

datatype are discovered at runtime

```rust
trait Measurable {
    fn get_temp(&self) -> f64;
}

struct TempSensor01 {
    temp: f64,
}
impl Measurable for TempSensor01 {
    fn get_temp(&self) -> f64 {
        self.temp
    }
}

#[allow(dead_code)]
struct TempSensor02 {
    label: String,
    temp: f64,
}
impl Measurable for TempSensor02 {
    fn get_temp(&self) -> f64 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

// Factory that decides at runtime
// Returning a trait object hides the concrete type
fn make_sensor(kind: &str) -> Box<dyn Measurable> {
    match kind {
        "celsius" => Box::new(TempSensor01 { temp: 1.0 }),
        "fahrenheit" => Box::new(TempSensor02 {
            label: "thermocouple".into(),
            temp: 25.0, // 77 °F
        }),
        _ => Box::new(TempSensor01 { temp: 0.0 }),
    }
}

fn main() {
    // A vector of Measurable
    let mut sensors: Vec<Box<dyn Measurable>> = Vec::new();
    sensors.push(make_sensor("celsius"));
    sensors.push(make_sensor("fahrenheit"));

    for s in &sensors {
        // Virtual call through a vtable (dynamic dispatch, fat vector)
        println!("Reading: {}", s.get_temp());
    }
}

```