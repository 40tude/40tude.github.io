---
published: true
lang: en-US
layout: default
title: "CI/CD for Rust: Automating Tests and Heroku Deployment of Your API"
parent: "Rust"
#math: mathjax
date               : 2025-08-02 16:00:00
last_modified_date : 2025-08-02 16:00:00
---

# CI/CD for Rust: Automating Tests and Heroku Deployment of Your API
{: .no_toc }

## TL;DR
{: .no_toc }
* A step by step recipe where I explain how to automate the testing and the deployment on Heroku of first Rust API (BMI calculator)
* This is a continuation of [this post]({%link docs/06_programmation/rust/011_api_heroku/api_heroku.md%}). Make sure to read it first.
* The current project is on [GitHub](https://github.com/40tude/bmi_api_2)


<div align="center">
<img src="./assets/img_00.webp" alt="" width="900" loading="lazy"/>
</div>


<!-- 
git push heroku main 
-->

<!-- Find the [blog post here](https://www.40tude.fr/docs/06_programmation/rust/011_api_heroku/api_heroku.html). -->

<!-- [![Test and Deploy](https://github.com/40tude/bmi_api_2/actions/workflows/test-and-deploy.yml/badge.svg)](https://github.com/40tude/bmi_api_2/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.88+-blue.svg)](https://www.rust-lang.org)
[![Heroku App](https://img.shields.io/badge/heroku-live-purple?logo=heroku)](https://rust-bmi-api-2.herokuapp.com) -->


## Table of Contents
{: .no_toc .text-delta}
- TOC
{:toc}


## Step 1 : Make sure the duplicated project works
Since I want to keep both projects from episode 1 and 2 side by side I need to copy/paste the previous project, rename it and double check everything still works as expected. In a real life I would modify files from episode 1 and keep track of the history with Git.

- Copy paste the previous directory
- Remove the `./.git` directory
- Remove the `./target` directory
- Update `Cargo.toml` to name the project `bmi_api_2`. See below :

```
[package]
name = "bmi_api_2"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.4"
axum-server = "0.7.2"
reqwest = { version = "0.12.22", features = ["json"] }
serde = { version = "1.0.219", features = ["derive"] }
tokio = { version = "1.47.1", features = ["full"] }

[profile.release]
strip = "symbols"
```

- Publish the new project on GitHub
- Make a test locally

```
cargo run
```

```
curl -X POST http://localhost:8080/bmi  -H "Content-Type: application/json" -d '{"height": 1.69, "weight": 71.0}'
```

<div align="center">
<img src="./assets/img_01.webp" alt="" width="900" loading="lazy"/>
</div>





- Update the `Procfile` file to build and run `bmi_api_2` on Heroku

```
web: ./target/release/bmi_api_2

```

- Create a project on Heroku

```
heroku create rust-bmi-api-2 --buildpack emk/rust
git remote -v
heroku  https://git.heroku.com/rust-bmi-api-2.git (fetch)
heroku  https://git.heroku.com/rust-bmi-api-2.git (push)
origin  https://github.com/40tude/bmi_api_2.git (fetch)
origin  https://github.com/40tude/bmi_api_2.git (push)
```

- There is no need to get a new token (`heroku auth:token`)
- We can directly deploy on Heroku

```
git push heroku main
```

<div align="center">
<img src="./assets/img_02.webp" alt="" width="900" loading="lazy"/>
</div>




- Copy the URL (see above, `https://rust-bmi-api-2-ddcca46c7003.herokuapp.com/`)
- Open `./examples/client.rs` and paste the copied URL. See the line below :

```rust
    let url = "https://rust-bmi-api-2-ddcca46c7003.herokuapp.com/bmi";
```

- Make a test with the client

```
cargo run --example client
```

<div align="center">
<img src="./assets/img_03.webp" alt="" width="900" loading="lazy"/>
</div>


At this stage we are in good shape :
- The previous project has been duplicated
- The application has been renamed
- The new project is on GitHub
- The code runs locally
- The API Server can be deployed on Heroku with a simple "push"
- Clients (curl, web, Rust apps) can use the API
















## The Plan


Now the aim of the game is :
1. To add testings
1. To run the tests on GitHub each time we push on GitHub
1. Push from GitHub to Heroku only when the tests are OK

The good news is that it is possible. The bad news is that it requires to reorganize the files.

At this point, the project directory looks like :

```
.
├── examples/
│   └── client.rs      
└── src/
    └── main.rs       
```

Everything is in the `main.rs` file and this does not facilitate testing. Below is the new organization

```
.
├── examples/
│   └── client.rs      
├── src/
│   ├── main.rs      ⟵ code to launch the app
│   ├── lib.rs       ⟵ where we declare the modules (ex: api)
│   └── api.rs       ⟵ BMI + struct request/response
└── tests/
    └── api.rs       ⟵ test which use /src/api.rs

```








## Step 2 : Create `src/lib.rs`

- Create a file `src/lib.rs` and copy/paste the code below

```rust
// src/lib.rs
pub mod api;
```













## Step 3 : Simplify `src/main.rs`

Modify `main.rs` as below so that it calls functions (here, `api::calculate_bmi()`) from the `api.rs` module

```rust
use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use bmi_api::api::calculate_bmi;
use std::net::SocketAddr;

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "BMI API is up and running")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/bmi", post(calculate_bmi)).route("/", axum::routing::get(health_check));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Listening on {addr}");

    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
```











## Step 4 : Create a lib section in `Cargo.toml`

Modify `Cargo.toml` and add a `[lib]` section

```
[package]
name = "bmi_api_2"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.4"
axum-server = "0.7.2"
reqwest = { version = "0.12.22", features = ["json"] }
serde = { version = "1.0.219", features = ["derive"] }
tokio = { version = "1.47.1", features = ["full"] }

[lib]
name = "bmi_api"
path = "src/lib.rs"

[profile.release]
strip = "symbols"
```











## Step 5 : Create `src/api.rs`

Create a file `src/api.rs` and copy/paste the code below

```rust
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

// Structure for request body
#[derive(Deserialize)]
pub struct BmiRequest {
    pub height: f32,
    pub weight: f32,
}

// Structure for response body
// Now it derive Deserialize. This is for the tests
#[derive(Serialize, Deserialize)] 
pub struct BmiResponse {
    pub bmi: f32,
}

// Handler for the /bmi endpoint
pub async fn calculate_bmi(Json(payload): Json<BmiRequest>) -> Result<Json<BmiResponse>, (StatusCode, String)> {
    if payload.height <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Height must be > 0".into()));
    }

    if payload.weight <= 0.0 {
        return Err((StatusCode::BAD_REQUEST, "Weight must be > 0".into()));
    }

    let bmi = payload.weight / (payload.height * payload.height);
    Ok(Json(BmiResponse { bmi: (bmi * 100.0).round() / 100.0 }))
}
```















## Step 6 : Create `tests/api.rs`

Run the command below to extend `Cargo.toml` 

```
cargo add tower
cargo add serde-json
```

Create a `tests/api.rs` file with the following content :

```rust
use axum::body::to_bytes;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::post,
};
// use bmi_api::api::{BmiRequest, BmiResponse, calculate_bmi};
use bmi_api::api::{BmiResponse, calculate_bmi};
use tower::ServiceExt;

fn app() -> Router {
    Router::new().route("/bmi", post(calculate_bmi))
}

#[tokio::test]
async fn test_zero_weight_should_fail() {
    let app = app();

    let payload = r#"{ "height": 1.75, "weight": 0.0 }"#;
    let request = Request::builder()
        .method("POST")
        .uri("/bmi")
        .header("Content-Type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_valid_bmi_should_succeed() {
    let app = app();

    let payload = r#"{ "height": 1.75, "weight": 70.0 }"#;
    let request = Request::builder()
        .method("POST")
        .uri("/bmi")
        .header("Content-Type", "application/json")
        .body(Body::from(payload))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Check HTTP 200
    assert_eq!(response.status(), StatusCode::OK);

    // Extract JSON body
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let parsed: BmiResponse = serde_json::from_slice(&body).unwrap();

    // Check BMI value (rounded to 2 decimals)
    assert!((parsed.bmi - 22.86).abs() < 0.01, "Expected ≈22.86, got {}", parsed.bmi);
}
```

## Step 7 : Check that everything works locally

- Publish on GitHub
- Run the app locally, just to make sure...

```
cargo run
```

<div align="center">
<img src="./assets/img_04.webp" alt="" width="900" loading="lazy"/>
</div>


```
curl -X POST http://localhost:8080/bmi  -H "Content-Type: application/json" -d '{"height": 1.69, "weight": 71.0}'

```

<div align="center">
<img src="./assets/img_05.webp" alt="" width="900" loading="lazy"/>
</div>


Now run the tests manually and locally. Again, just to make sure...

```
cargo test  

```


<div align="center">
<img src="./assets/img_06.webp" alt="" width="900" loading="lazy"/>
</div>








## Step 8 : Setup GitHub CI/CD

Create `.github/workflows/test-and-deploy.yml` and copy the code below

```yml
name: Test and Deploy

on:
  push:
    branches: [ main ]

jobs:
  test-and-deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0 # Mandatory for Heroku push

      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Run tests
        run: cargo test

      - name: Install Heroku CLI
        run: |
          curl https://cli-assets.heroku.com/install.sh | sh

      - name: Deploy to Heroku
        env:
          HEROKU_API_KEY: ${{ secrets.HEROKU_API_KEY }}
        run: |
          git remote add heroku https://heroku:${HEROKU_API_KEY}@git.heroku.com/rust-bmi-api-2.git
          git push heroku HEAD:main --force
```

In the script above, make sure to update the `heroku git:remote -a rust-bmi-api-2` with the name of you project (find the name with either `git remote -v` or `heroku apps`)

This is obvious but, if we want GitHub to talk to Heroku, we need to add the Heroku token to GitHub. Let's get back the Heroku token :

```
heroku auth:token
»   Warning: token will expire 08/30/2025
»   To generate a token that expires in one year, use heroku authorizations:create.
HRKU-AAYLneWaZt-hqJ7cqwH3Ou_...

```

- Copy the token
- Open the GitHub repo
- GitHub → Settings → Secrets and variables → Actions → New repository secret :
    * Name: HEROKU_API_KEY
    * Value: Past the token value you got with `heroku auth:token`


<div align="center">
<img src="./assets/img_07.webp" alt="" width="900" loading="lazy"/>
</div>











## Step 9 : Check that its works when we push on GitHub 

Commit on GitHub and check `Actions` in the repo


<div align="center">
<img src="./assets/img_08.webp" alt="" width="900" loading="lazy"/>
</div>

At the end, let's double check what is available on Heroku

```
heroku run bash
```

<div align="center">
<img src="./assets/img_09.webp" alt="" width="900" loading="lazy"/>
</div>

Above the `date` command helps to confirms that bmi_api_2 was created 4 minutes ago.

### Note
{: .no_toc }

- Forget the file `bmi_api`. This is my fault.
- It should not be visible on your side.



















## Step 10 : Add a test logo to `README.md`

- Create a `README.md` file
- Add the line below at the top of the `README.md` file
- It displays the status of the tests

```
[![Test and Deploy](https://github.com/40tude/bmi_api_2/actions/workflows/test-and-deploy.yml/badge.svg)](https://github.com/40tude/bmi_api_2/actions)
```






## Step 11 : Optional - Add more logos to `README.md`

Additionally one can add a `LICENSE` file and a sticker
- Go to the GitHub repo
- Click on "Add file" > "Create new file"
- Name it `LICENSE` (with no extension)
- At the top, GitHub offers a button "Choose a license template"
- Click on it, choose MIT for example, and validate
- Now that you have a `LICENSE` file you can add the line below at the top of the `README.md` file

```
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
```

Similarly, we can also add the minimal Rust version required

```
[![Rust Version](https://img.shields.io/badge/rust-1.88+-blue.svg)](https://www.rust-lang.org)
```
In the line above adjust the `1.88+` to your needs




Finally, let's add an Heroku logo with a link to the app

```
[![Heroku App](https://img.shields.io/badge/heroku-live-purple?logo=heroku)](https://rust-bmi-api-2-ddcca46c7003.herokuapp.com/)
```

If one day we have a release we can add the line below

```
[![GitHub release](https://img.shields.io/github/v/release/40tude/bmi_api_2)](https://github.com/40tude/bmi_api_2/releases)
```

This requires at least one “release” created in GitHub, via the repo's “Releases” tab.


Now on GitHub the `README.md` should be displayed as below 
<div align="center">
<img src="./assets/img_10.webp" alt="" width="900" loading="lazy"/>
</div>





## Conclusion

At the end of [Episode 1/2]({%link docs/06_programmation/rust/011_api_heroku/api_heroku.md%}) and Episode 2/2 (this file) we have :
* A first API Server which can be checked locally
* We also have a Rust client
* More importantly, the files are organized so that tests can be written and executed locally
* Cherry on the cake, when we push on GitHub, the tests are executed remotely and if they are successful the API server is deployed from GitHub to Heroku
* On GitHub, the `README.md` file clearly indicates the status of the tests... With some logos

Not bad. I hope this can help.






























