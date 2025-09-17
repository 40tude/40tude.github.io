---
published: true
lang: en-US
layout: default
title: "Rust Traits: Defining Character - 04"
description: "From basic syntax to building plugins with once_cell and organizing Rust projects."
parent: "Rust"
#math: mathjax
date               : 2025-09-14 14:00:00
last_modified_date : 2025-09-17 07:00:00
---



# Rust Traits: Defining Character
{: .no_toc }

From basic syntax to building plugins with once_cell and organizing your Rust projects.
{: .lead }


<h2 align="center">
<span style="color:orange"><b>This post is under construction.</b></span>    
</h2>

### This is Episode 04
{: .no_toc }

## TL;DR
{: .no_toc }

* For beginners
* The code is on [GitHub](https://github.com/40tude/traits_as_plugins)

<div align="center">
<img src="./assets/img00.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>In space, no one can hear you scream.</span> -->
</div>


#### Posts 
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/015_traits/traits_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/015_traits/traits_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/015_traits/traits_02.md%})
* [Episode 03]({%link docs/06_programmation/rust/015_traits/traits_03.md%})
* [Episode 04]({%link docs/06_programmation/rust/015_traits/traits_04.md%})

## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}



<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


## Once Cell - First-time use

Where the `once_cell` crate lets us define a global list of sensors dynamically initialized at runtime.

### Running the demo code
{: .no_toc }

* Right click on `assets/11_once_cell_0`
* Select the option "Open in Integrated Terminal"
* `cargo add rand`
* `cargo add once_cell`
* `cargo run`
* `cargo run --example ex00`

<div align="center">
<img src="./assets/img30.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Comment about the picture above</span> -->
</div>



### Explanations 1/2 
{: .no_toc }

Houston, we have a problem. The POC works but I'm not sure it will work with 10_000 thermocouples, different kind of sensors, different kinds of actuators. Do you 
remember, in the previous version of the application, in `temperature_sensor.rs` we had a young and innocent `make_sensor()` function. It looked like this :

```rust
pub fn make_sensor(kind: usize) -> Box<dyn TempSensor> {
    match kind {
        1 => Box::new(my_sensor1::TempSensor01),
        2 => Box::new(your_sensor2::TempSensor02),
        other => {
            // in production return a Result
            eprintln!("Unknown SENSOR_KIND='{other}', falling back to temp1.");
            Box::new(my_sensor1::TempSensor01)
        }
    }
}
```

Pretty young thing, no? No! What will happen with 250 different kind of sensors. We will need a huge match statement. On the other hand, we need to make sure to not overload the application with many sensors while only a dozen is in use... But again, the most critical point is that the match statement above is not scalable and ideally the available sensors should register by themselves.

And this is where the Rust crate [once_cell](https://crates.io/crates/once_cell) comes to the rescue. In this first version we will keep the file organization as close as possible from the previous one (see the section [Dynamic Sensor Creation]({%link docs/06_programmation/rust/015_traits/traits_03.md%})). One thing however. Instead of `temperature_sensor1` and `temperature_sensor2` we now use the names `thermocouple` and `rtd` which are 2 different kinds of technologies for temperature sensors. Other than that the hub files are still present and the hierarchy is exactly the same. See below :


### Show me the code!
{: .no_toc }

```
.
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   
├───examples
│       ex00.rs
│       
├───src
│   │   lib.rs
│   │   main.rs
│   │   sensors.rs
│   │
│   └───sensors
│       │   temperature.rs
│       │
│       └───temperature
│           │   rtd.rs
│           │   temperature_sensor.rs
│           │   thermocouple.rs
│           │
│           ├───rtd
│           │       rtd_512.rs
│           │
│           └───thermocouple
│                   thermocouple_128.rs
│
└───target
```


### Explanations 2/2 
{: .no_toc }

Let's start with `main.rs`

```rust
use demo_registry_0::sensors::{self, temperature::temperature_sensor};

fn main() {
    sensors::register();

    let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());

    let rtd_01 = temperature_sensor::make_sensor("Rtd_type_512").expect("Unknown sensor");
    println!("RTD 01         : {:6.2}", rtd_01.get_temp());
}
```

At a high level the code should be easy to understand
1. First, the sensors (we don't really know yet what this covers nor how it works) register themselves 
1. On return, the sensors are not yet initialized, they just confirm we can instantiate the ones the app need
1. Using its name (`Thermocouple_type_128`) we create an instance of a temperature sensor and print a temperature measurement   
1. We do the same with `Rtd_type_512`

On the other hand, `ex00.rs` simulates the case where the names of the sensors come from a database and when one reference (`temp_42`) cannot be instantiated.

```rust
fn main() {
    sensors::register();

    for sensor_name in ["Thermocouple_type_128", "Rtd_type_512", "temp42"] {
        match temperature_sensor::make_sensor(sensor_name) {
            Some(sensor) => {
                let temp = sensor.get_temp();
                println!("Sensor {sensor_name}: {:6.2} °C", temp);
            }
            None => {
                println!("Sensor '{sensor_name}': not found in registry!");
            }
        }
    }
}
```

In `main.rs`, let's start with `sensors::register();`. If you use VSCode I recommend to :

* Right click on `assets/11_once_cell_0`
* Select the option "Reveal in File Explorer"

<div align="center">
<img src="./assets/img32.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Comment about the picture above</span> -->
</div>

Once in File Explorer

* Right click on `11_once_cell_0`
* Select the option "Open with VSCode"

<div align="center">
<img src="./assets/img33.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Comment about the picture above</span> -->
</div>

Once in the new instance of VSCode, open `main.rs` click on the word `register` of `sensors::register`. You can either press `F12` or right click and select the option "Go to Definition" 

<div align="center">
<img src="./assets/img34.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Comment about the picture above</span> -->
</div>

The file `sensors.rs` opens and we see :

```rust
// sensors.rs
pub mod temperature;

pub fn register() {
    temperature::register();
}
```

Let's step back and make sure we are on the same page. In `main()` function I know I have some sensors and no actuators. I don't care about the details, I just call `sensors::register()` and I do not call `actuators::register()`. I my mind, calling `sensors::register()` means I delegate to someone else the registrations of the sensors.

Now at `sensors.rs` level, I know I only have temperature sensors so I call `temperature::register()`. This may change later. If we add other kind of sensors (strain gauge, pH meter...) I will then add a call to `strain_gauge:register()`. This is my responsibility, and strain gauges registration will remain transparent to the `main()` function. 

Ok, let's press `F12` once the cursor is on `temperature::register()`. This opens `temperature.rs`.

```rust
// temperature.rs
pub mod rtd; 
pub mod temperature_sensor; 
pub mod thermocouple; 

pub fn register() {
    thermocouple::register();
    rtd::register();
}
```

At `temperature.rs` level, I know I only have only 2 types of temperature sensors : thermocouples and rtd. I call `thermocouple::register()` and `rtd::register()`. Let's press `F12` once the cursor is on `thermocouple::register()`. This opens `thermocouple.rs`.

```rust
// thermocouple.rs
pub mod thermocouple_128;

pub fn register() {
    thermocouple_128::register();
}
```

At `thermocouple.rs` level, I know I only have one kind of thermocouple so I call `thermocouple_128::register()`. Let's press F12. This opens `thermocouple_128.rs` :

```rust
// thermocouple_128.rs
use crate::sensors::temperature::temperature_sensor::{self, TemperatureSensor};

pub struct Thermocouple128; // camel case => no _

impl TemperatureSensor for Thermocouple128 {
    fn get_temp(&self) -> f64 {
        let temp: f64 = rand::random_range(0.0..128.0);
        temp
    }
}
pub fn register() {
    temperature_sensor::register_sensor("Thermocouple_type_128", || Box::new(Thermocouple128));
}
```

The upper part of the source code is pretty well known. We have a `Thermocouple128` data type and we implement the `TemperatureSensor` trait. Nothing new under the sun and as before the `TemperatureSensor` trait is defined in `temperature_sensor.rs` but, if the cursor is on the word `TemperatureSensor`, do not press F12 yet.

Obviously, the most interesting part is the `register()` function definition. However, the syntax looks a bit odd, or at least, is not that easy to grasp the first time. Click on `temperature_sensor::register_sensor` then press F12. This opens the file `temperature_sensor.rs`. The code below is not complete but it is good enough to understand what is going on :

```rust
// temperature_sensor.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait TemperatureSensor {
    fn get_temp(&self) -> f64;
}

type Constructor = fn() -> Box<dyn TemperatureSensor>;

pub static TEMPERATURE_SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_sensor(name: &'static str, constructor: Constructor) {
    let mut map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("registry mutex poisoned");
    map.insert(name, constructor);
}
```

In the upper part part of the file we found the `TemperatureSensor` trait declaration. No change compared to what we already know. Then, few lines below, there is the definition of `register_sensor()`, the function we want to study :

```rust
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    let mut map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("registry mutex poisoned");
    map.insert(name, constructor);
}
```

The `register_sensor()` function expects 2 parameters. The first one is a name that identify the kind of temperature sensor in the application (thermocouple, rtd... `Thermocouple_type_128`, `Rtd_512`...). Note that it is a `&str` with a `static` lifetime where `static` must be understood as "valid as long as the program runs". This is usually a string literal (e.g. `"Thermocouple_type_128"`). The second parameter is of type `Constructor`. 

`Constructor` is a type alias (a synonym) declared above the `register_sensor()` function : 

```rust
type Constructor = fn() -> Box<dyn TemperatureSensor>;
```

Now, when we read `Constructor`, we should read `fn() -> Box<dyn TemperatureSensor>`, meaning "a function returning a `Box<dyn TemperatureSensor>`". We already talked about `Box<dyn T>`, you know what this is, I do not have to explain it again: 

Finally  the really weird thing in `register_sensor()` definition is the `TEMPERATURE_SENSOR_REGISTRY`. It is declared this way :

```rust
pub static TEMPERATURE_SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));
```

Stay here, don't run away and don't panic... The line above just says something like :
* `TEMPERATURE_SENSOR_REGISTRY` is a global static variable (it lives as long as the application)
    * Its data type is `Lazy<Mutex<HashMap<&'static str, Constructor>>>`
    * It is initialized with the value `once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()))`

Regarding `TEMPERATURE_SENSOR_REGISTRY` we see that:
* This is a `HashMap` where the keys are the literal of the sensors and the values are the `Constructor()` of the respective sensor
* The `HashMap` is protected by a `Mutex`. 
* **Important:** A static variable cannot be `mut`, so we need **interior mutability** to modify the HashMap at runtime. `Mutex` provides that interior mutability and thread safety.
* Then the `Mutex<HashMap<...>>` is wrapped into a `Lazy` layer.
    * A `once_cell::sync::Lazy<T>` is a wrapper type
    * It says something like : "Here is a static that will be initialized the first time it’s used."
    * "It uses a closure `(|| Mutex::new(HashMap::new()))` to specify how to build the value.
    * It guarantees that initialization happens once and only once, even if multiple threads race to access it.
    * That’s why the crate is named "once_cell": it’s like a "memory cell" that can be written exactly once. 

Regarding the initialization value `once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()))`
* A Lazy value says something like : "When somebody first touches this static, run the closure to build the real value."
    * The closure `|| Mutex::new(HashMap::new())` creates an empty HashMap wrapped in a `Mutex`.
    * So the very first time we call `register_sensor()` the Lazy runs the closure and it builds the `Mutex<HashMap<&'static str, Constructor>>`

Now that we understand what is behind the `TEMPERATURE_SENSOR_REGISTRY` variable declaration we can understand what the `register_sensor()` does :

```rust
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    let mut map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("registry mutex poisoned");
    map.insert(name, constructor);
}
```


* If the `TEMPERATURE_SENSOR_REGISTRY` is not yet created, it creates the `HashMap` (lazy implementation). 
* Then we call `.lock().expect("registry mutex poisoned")`. If a panic ever occurs while the registry is being updated, the mutex becomes poisoned. In that case, `.expect()` will panic and print a custom message. Remember, a `Mutex` gets poisoned if a thread panics while holding the lock.
* To finish we insert the name of the sensor and its constructor in the HashMap.

***Could we recover instead of panicking?*** Yes we can, see below one way but I wanted to keep the POC short :

```rust
match TEMPERATURE_SENSOR_REGISTRY.lock() {
    Ok(mut map) => {
        map.insert(name, constructor);
    }
    Err(poisoned) => {
        let mut map = poisoned.into_inner();
        map.insert(name, constructor);
    }
}
```

The idea is that once all the sensors have called `temperature_sensor::register_sensor()` everyone can use the initialized `TEMPERATURE_SENSOR_REGISTRY` value and we don't have to pass the registry around as a parameter in all the functions of the application. Ok?

***No. I'm lost! Where are we? What should I keep in mind?*** Remember... Starting from `main()`, pressing F12, we followed the `register()` function calls. Finally we reach `thermocouple_128::register()` which call `temperature_sensor::register_sensor()`. On the first call, `once_cell::sync::Lazy` initializes the registry (a global static variable that holds a `HashMap` of sensor constructors). Each `temperature_sensor::register_sensor()` call locks the map and inserts `(name, constructor)`. From then on, any code can look up the name of a temperature sensor and call the stored constructor to obtain a `Box<dyn TemperatureSensor>`.

Let's see how a function could look for a sensor name. For that let's go back to `main()`. Below is a shortened version with only one sensor created and used. After the line `sensors::register();` all sensors are in the global registry with their name and constructor. No constructor have been called. No sensor have been created. Then comes the call `temperature_sensor::make_sensor()` :

```rust
// main.rs
use demo_registry_0::sensors::{self, temperature::temperature_sensor};

fn main() {
    sensors::register();

    let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());
}
```

You know what to do. Set the cursor on `temperature_sensor::make_sensor()` and press F12. Below is the definition of `make_sensor` :

```rust
pub fn make_sensor(name: &str) -> Option<Box<dyn TemperatureSensor>> {
    let map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("TEMPERATURE_SENSOR_REGISTRY mutex poisoned");
    map.get(name).map(|ctor| ctor())
}
```

First we get access to the global registry and try to lock it. As before, if the `Mutex` is poisoned `.expect()` panics and prints a custom message. Otherwise we find the sensor by its name (the key) and we call the constructor (the value). The returned value of the constructor is the returned value of `make_sensor()`. 

***What is `ctor` again ?*** Previously, we stored in the static global HashMap (`TEMPERATURE_SENSOR_REGISTRY`) the name of the sensor (`"Thermocouple_type_128"`) as a key and a closure that builds a new instance of this sensor as a value (`|| Box::new(Thermocouple128)`). Here, with `map(|ctor| ctor()`, `|| Box::new(Thermocouple128)` is called and its output becomes the returned value of `make_sensor()`. Look at the signature of `make_sensor()`. It returns an `Option` to a boxed data type which implements the `TemperatureSensor` trait (could be a `Thermocouple128`, `Rtd512`...) 


**OK... Could we recover instead of panicking?** Yes we can. See below one idea :

```rust
pub fn make_sensor(name: &str) -> Option<Box<dyn TemperatureSensor>> {
    match TEMPERATURE_SENSOR_REGISTRY.lock() {
        Ok(map) => map.get(name).map(|ctor| ctor()),
        Err(poisoned) => {
            eprintln!("[warn] TEMPERATURE_SENSOR_REGISTRY mutex poisoned — recovering.");
            let map = poisoned.into_inner(); // take the inner HashMap anyway
            map.get(name).map(|ctor| ctor())
        }
    }
}
```


Let's move on. At the end of the day from the caller point of view, in `main()`, we have : 

```rust
let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");
```

`thermo_01` is a ready to use instance of the `Thermocouple_type_128` thermocouple. It implements the `TemperatureSensor` trait so we can call `.get_temp()` on it :

```rust
println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());
```












### Exercise
{: .no_toc }

1. Starting from `main()`, follow the white rabbit, press F12 and retrieve the `register_sensor()` of the rtd.
1. Do you feel brave enough to add a new category of temperature sensor?
    * Add `optical` in addition to `rtd` and `temperature` and the kind of optical sensor could be `camera_007`.   
1. It seems the registry is written once per kind of sensor and read once per sensor. We may end up with much more read than write operations. Any idea on how we could improve performances?
    * **Solution:** 
        * Rename `temperature_sensor.rs` as `temperature_sensor.rs.mutex` 
        * Rename `temperature_sensor.rs.rwlock` as `temperature_sensor.rs`
        * `Cargo run`
        * `Mutex` is replaced by `RwLock`






### Summary
{: .no_toc }

* We wanted to avoid the potentially huge `match` statement of the previous version of the `make_sensor()` function.
* The solution is to let each sensor register itself in a global registry.
* In Rust, plain global variables must be initialized at compile time, which doesn’t work here since the registry must be built dynamically.
* We therefore need a global variable that is created at runtime.
* `once_cell::sync::Lazy` lets us define such a global variable: it ensures the registry is initialized safely on first access.
* The registry is a `HashMap` where:
    * the key is a sensor identifier (usually a string literal),
    * the value is a constructor function returning a `Box<dyn TemperatureSensor>`.
* The `HashMap` is wrapped in a `Mutex` to provide interior mutability and thread safety.
* Sensors register themselves by inserting their constructor into the registry.
* Later, sensors can be instantiated on demand by looking them up in the registry.
* Smoking!



















<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


## Once Cell - Different types of sensors

Where we have 2 registries. One for temperature sensors and another for pH sensors.

### Running the demo code
{: .no_toc }

* Right click on `assets/12_once_cell_1`
* Select the option "Open in Integrated Terminal"
* `cargo add rand`
* `cargo add once_cell`
* `cargo run`

<div align="center">
<img src="./assets/img35.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Comment about the picture above</span> -->
</div>



### Explanations 1/2 
{: .no_toc }

People were so pleased with the previous POC that they ask us a new version :  
1. with not only temperature sensors but also, other kinds of sensors : they mentioned pH meter, strain gauges, flow meter... 
1. with multiple references under a kind of sensor. The idea is to show that under `sensor/temperature/thermocouple` we can have `termocoupleX`, `termocoupleY`...      
1. with multiple instances of the same sensor reference because in a plant you can easily have 100 temperature sensors of the same reference.

No worry. This section is short and easy because we already know all we need to know : traits, hub files, once_cell... 

If you know nothing about sensors, below I added pH sensors in a dedicated directory. pH sensors help to measure the acidity of a solution. I added 2 kinds of pH meters : probe and ISFET. The idea is to answer point 1 above. 

Below, think about the `ph` directory as a copy/paste/rename of the temperature directory. Nothing more. The hub files, traits and implementation are used exactly the same way.  

Note that in the thermocouple directory we now have `thermocouple_128` adn `thermocouple_256`. Two different thermocouple references. This is to answer point 2 above.

See below the file hierarchy of the project. 


### Show me the code!
{: .no_toc }

```
.
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│
├───src
│   │   lib.rs
│   │   main.rs
│   │   sensors.rs
│   │
│   └───sensors
│       │   ph.rs
│       │   temperature.rs
│       │
│       ├───ph
│       │   │   isfet.rs
│       │   │   ph_sensor.rs
│       │   │   probe.rs
│       │   │
│       │   ├───isfet
│       │   │       isfet_1024.rs
│       │   │
│       │   └───probe
│       │           probe_2048.rs
│       │
│       └───temperature
│           │   rtd.rs
│           │   temperature_sensor.rs
│           │   thermocouple.rs
│           │
│           ├───rtd
│           │       rtd_512.rs
│           │
│           └───thermocouple
│                   thermocouple_128.rs
│                   thermocouple_256.rs
│
└───target
```


### Explanations 2/2 
{: .no_toc }



In `main.rs` we still have the `sensors::register();` function call followed by the creation of : 
* `thermo_01` and `thermo_02` to address point 2
* `thermo_02` and `thermo_03` to address point 3
* `isfet_01` and `probe_42` to address point 1  

Here is `main.rs` :

```rust
use demo_registry_1::sensors;
use demo_registry_1::sensors::ph::ph_sensor;
use demo_registry_1::sensors::temperature::temperature_sensor;

fn main() {
    sensors::register();

    let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");
    let thermo_02 = temperature_sensor::make_sensor("Thermocouple_type_256").expect("Unknown sensor");
    let thermo_03 = temperature_sensor::make_sensor("Thermocouple_type_256").expect("Unknown sensor");

    println!("\nSensors:");
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());
    println!("Thermocouple 02: {:6.2}", thermo_02.get_temp());
    println!("Thermocouple 03: {:6.2}", thermo_03.get_temp());

    let rtd_01 = temperature_sensor::make_sensor("Rtd_type_512").expect("Unknown sensor");
    println!("RTD 01         : {:6.2}", rtd_01.get_temp());

    let isfet_01 = ph_sensor::make_sensor("IsFET_type_1024").expect("Unknown sensor");
    println!("IsFET_01       : {:6.2}", isfet_01.get_ph());

    let probe_42 = ph_sensor::make_sensor("Probe_type_2048").expect("Unknown sensor");
    println!("Probe_42       : {:6.2}", probe_42.get_ph());
}
```

You should be able to navigate within the project as we did before.

One thing however. In `src/sensors/temperature/temperature_sensor.rs` we have :

```rust
// temperature_sensor.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait TemperatureSensor {
    fn get_temp(&self) -> f64;
}

// Type alias for a constructor function returning a boxed sensor
type Constructor = fn() -> Box<dyn TemperatureSensor>;

// Global registry of sensor constructors
pub static TEMPERATURE_SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Called by sensors
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    let mut map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("TEMPERATURE_SENSOR_REGISTRY mutex poisoned");
    map.insert(name, constructor);
}

// Called by binaries (main.rs, examples, tests...) to creates a sensor by name
pub fn make_sensor(name: &str) -> Option<Box<dyn TemperatureSensor>> {
    let map = TEMPERATURE_SENSOR_REGISTRY.lock().expect("TEMPERATURE_SENSOR_REGISTRY mutex poisoned");
    map.get(name).map(|ctor| ctor())
}

```
While in `src/sensors/ph/ph_sensor.rs` we have :

```rust
// ph_sensor.rs
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait PhSensor {
    fn get_ph(&self) -> f64;
}

// Type alias for a constructor function returning a boxed sensor
type Constructor = fn() -> Box<dyn PhSensor>;

// Global registry of sensor constructors
pub static PH_SENSOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Called by sensors
pub fn register_sensor(name: &'static str, constructor: Constructor) {
    let mut map = PH_SENSOR_REGISTRY.lock().expect("PH_SENSOR_REGISTRY mutex poisoned");
    map.insert(name, constructor);
}

// Called by binaries (main.rs, examples, tests...) to creates a sensor by name
pub fn make_sensor(name: &str) -> Option<Box<dyn PhSensor>> {
    let map = PH_SENSOR_REGISTRY.lock().expect("PH_SENSOR_REGISTRY mutex poisoned");
    map.get(name).map(|ctor| ctor())
}
```

Both used `Mutex` and not `RwLock` but this is not the point. No, the point I want to underline is that one create a static global `TEMPERATURE_SENSOR_REGISTRY` while the other one create `PH_SENSOR_REGISTRY`. This means that the application ends up with 2 registries in memory. This might be smart if we have lot of temperature sensors and pH sensors and if we want to shorten look up time. If the look up time is not an issue we could create a unique `SENSOR_REGISTRY`. Obviously this one should be created in `src/sensors.rs` (the common ancestor of all the sensors).

Other than that, I think we're done.



### Exercise
{: .no_toc }

1. ??? 




### Summary
{: .no_toc }

* ???






























<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


## Once Cell - Sensors and actuators

Where the application finally uses sensors and actuators. 

### Running the demo code
{: .no_toc }

* Right click on `assets/13_once_cell_2`
* Select the option "Open in Integrated Terminal"
* `cargo add rand`
* `cargo add once_cell`
* `cargo run`

<div align="center">
<img src="./assets/img36.webp" alt="" width="450" loading="lazy"/><br/>
<!-- <span>Comment about the picture above</span> -->
</div>



### Explanations 1/2 
{: .no_toc }

As you can imagine, people now want to see a POC of the complete application where sensors and actuators are used. 

This section should be even shorter because, again, we already know all we need to know : traits, hub files, once_cell, REGISTRY... 

For demo purpose I created an actuators directory (stuffs that act on the real world to lock doors, heat, cool, turn on alarms). I only created electrical category of actuators but add to kind of electrical motors : servo motor and solenoid. 

Again if you don't know or don't care about sensors and actuators this is OK. Imagine that the application must handle different kinds of file format, different kinds of communication protocols... They are organized in directories, subdirectories and files.

See below the files hierarchy of the project.    


### Show me the code!
{: .no_toc }

```
.
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│
├───src
│   │   actuators.rs
│   │   lib.rs
│   │   main.rs
│   │   sensors.rs
│   │
│   ├───actuators
│   │   │   electric.rs
│   │   │
│   │   └───electric
│   │       │   electric_actuator.rs
│   │       │   servo_motor.rs
│   │       │   solenoid.rs
│   │       │
│   │       ├───servo_motor
│   │       │       servo_motor_01.rs
│   │       │
│   │       └───solenoid
│   │               solenoid_101.rs
│   │
│   └───sensors
│       │   ph.rs
│       │   temperature.rs
│       │
│       ├───ph
│       │   │   isfet.rs
│       │   │   ph_sensor.rs
│       │   │   probe.rs
│       │   │
│       │   ├───isfet
│       │   │       isfet_1024.rs
│       │   │
│       │   └───probe
│       │           probe_2048.rs
│       │
│       └───temperature
│           │   rtd.rs
│           │   temperature_sensor.rs
│           │   thermocouple.rs
│           │
│           ├───rtd
│           │       rtd_512.rs
│           │
│           └───thermocouple
│                   thermocouple_128.rs
│                   thermocouple_256.rs
│
└───target
```


### Explanations 2/2 
{: .no_toc }

In `main.rs` we still have the `sensors::register();` function call followed by `actuators::register();`. Then, as before, we instantiate the sensors. Finally we instantiate actuators (`motor_01` and `solenoid_01`) with `electric_actuator::make_actuator`. We can act on actuators to turn the on and we can also read their status : this is done with the functions `.get_state()` and `.set_state(true)`. This explain why at the bottom of the terminal we see `motor_01   : Stopped` when it is created then `motor_01   : Running` when it is turned on.

Here is `main.rs` :

```rust
use demo_registry_2::actuators;
use demo_registry_2::actuators::electric::electric_actuator;

use demo_registry_2::sensors;
use demo_registry_2::sensors::ph::ph_sensor;
use demo_registry_2::sensors::temperature::temperature_sensor;

fn main() {
    sensors::register();
    actuators::register();

    let thermo_01 = temperature_sensor::make_sensor("Thermocouple_type_128").expect("Unknown sensor");
    let thermo_02 = temperature_sensor::make_sensor("Thermocouple_type_256").expect("Unknown sensor");

    println!("\nSensors:");
    println!("Thermocouple 01: {:6.2}", thermo_01.get_temp());
    println!("Thermocouple 02: {:6.2}", thermo_02.get_temp());

    let rtd_01 = temperature_sensor::make_sensor("Rtd_type_512").expect("Unknown sensor");
    println!("RTD 01         : {:6.2}", rtd_01.get_temp());

    let isfet_01 = ph_sensor::make_sensor("IsFET_type_1024").expect("Unknown sensor");
    println!("IsFET_01       : {:6.2}", isfet_01.get_ph());

    let probe_42 = ph_sensor::make_sensor("Probe_type_2048").expect("Unknown sensor");
    println!("Probe_42       : {:6.2}", probe_42.get_ph());

    println!("\nActuators:");
    let mut motor_01 = electric_actuator::make_actuator("Servo_Motor_type_01").expect("Unknown sensor");
    println!("motor_01   : {}", if motor_01.get_state() { "Running" } else { "Stopped" });
    motor_01.set_state(true);
    println!("motor_01   : {}", if motor_01.get_state() { "Running" } else { "Stopped" });

    let mut solenoid_01 = electric_actuator::make_actuator("Solenoid_type_101").expect("Unknown sensor");
    println!("solenoid_01: {}", if solenoid_01.get_state() { "Running" } else { "Stopped" });
    solenoid_01.set_state(true);
    println!("solenoid_01: {}", if solenoid_01.get_state() { "Running" } else { "Stopped" });
}
```

You are able to navigate into the project. For example here is the content of `electric_actuator.rs` where we can see that a `ELEC_ACTUATOR_REGISTRY` global registry is created.

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub trait ElectricActuator {
    fn set_state(&mut self, state: bool);
    fn get_state(&self) -> bool;
}

type Constructor = fn() -> Box<dyn ElectricActuator>;

pub static ELEC_ACTUATOR_REGISTRY: Lazy<Mutex<HashMap<&'static str, Constructor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_actuator(name: &'static str, constructor: Constructor) {
    ELEC_ACTUATOR_REGISTRY.lock().unwrap().insert(name, constructor);
}

pub fn make_actuator(name: &str) -> Option<Box<dyn ElectricActuator>> {
    ELEC_ACTUATOR_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}
```

And here is the content of `servo_motor_01.rs`. Without surprise, we find a `register()` function. More interestingly, an actuator needs to store its `state`. Here in a motor we store a boolean to indicate if the engine is running or not. Below we can see the implementation of the `.set_state()` and `.get_state()` methods.  

```rust
// servo_motor_01.rs
use crate::actuators::electric::electric_actuator::{self, ElectricActuator};

pub struct ServoMotor01 {
    state: bool,
}

impl ElectricActuator for ServoMotor01 {
    fn set_state(&mut self, new_state: bool) {
        self.state = new_state;
    }

    fn get_state(&self) -> bool {
        self.state
    }
}

pub fn register() {
    electric_actuator::register_actuator("Servo_Motor_type_01", || Box::new(ServoMotor01 { state: false }));
}
```



### Exercise
{: .no_toc }

1. In the actuators, add a `Heater` category and a `gaz` kind of heater. Name the reference `gaz_3852`. In `main()` create an instance `my_gaz_heater`, set it on and off. Read it status after each new setting.
1. Can you easily modify your heater actuator so that the `.set_state()` expect a `f64` parameter representing the percentage of heat to be used (from 0.0 to 100.0). Modify `.get_state()` so that the returned value represents the current percentage of heat.   




### Summary
{: .no_toc }

* ???








#### Posts 
{: .no_toc }

* [Episode 00]({%link docs/06_programmation/rust/015_traits/traits_00.md%})
* [Episode 01]({%link docs/06_programmation/rust/015_traits/traits_01.md%})
* [Episode 02]({%link docs/06_programmation/rust/015_traits/traits_02.md%})
* [Episode 03]({%link docs/06_programmation/rust/015_traits/traits_03.md%})
* [Episode 04]({%link docs/06_programmation/rust/015_traits/traits_04.md%})




































---

<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->
<!-- ###################################################################### -->


<!-- ## Template

One sentence

### Running the demo code
{: .no_toc }

* Right click on `assets/?????`
* Select the option "Open in Integrated Terminal"
* `cargo run`

<div align="center">
<img src="./assets/imgXX.webp" alt="" width="450" loading="lazy"/><br/>
<span>Comment about the picture above</span>
</div>



### Explanations 1/2 
{: .no_toc }




### Show me the code!
{: .no_toc }

```rust

```


### Explanations 2/2 
{: .no_toc }




### Exercise
{: .no_toc }




### Summary
{: .no_toc } -->
