# makepad-site
 Static web site template using Rust/WASM and Makepad.
 It uses Spin on the backend, so it's easy to deploy to services like Fermyon.
 It uses Makepad on the frontend, so it's fast on any modern browser.

# building
You need to install:
- rust (https://www.rust-lang.org/)
- spin (https://github.com/fermyon/spin)
- makepad (https://github.com/makepad/makepad)

The Makepad root directory should be in the same directory as makepad-site.

In the makepad-site directory:
To build: spin build
To run: spin run

By default makepad-site will run locally at http://127.0.0.1:3000.

# customizing
You can edit the frontend code in:
frontend/src/app.rs

You can edit the backend api code in:
src/lib.rs