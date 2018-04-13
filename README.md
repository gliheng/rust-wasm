This repo contains examples on using rust with webassembly.

Most examples contains Makefile for compilation. Run `make` to build.
Run `make server` to run a local server, and open `http://localhost:8000`

These examples are built using emscripten *v1.37.28*.

### hello-world
Basic hello wolrd example. Output through console.

### asmjs
Use asmjs as compilation target, run in nodejs.

### dom
Use `stdweb` crate to interact with the dom.

### ffi
js and rust interop.

### emscripten-api
Showing how to use emscripten api in rust, using C extern.

### file-read
Read a local file, count occurcy of each words.

### sdl2-basic
Draw some basic shapes, images using sdl2. Make it run both as native and web.

### sdl2-drag
A draggable box using sdl2. This demo works on both web and pc.
On web, it accepts both touch events and mouse events.
Native app can be run with `cargo run`.
[link](https://gliheng.github.io/rust-wasm/sdl2-drag/)

### sdl2-gallery
A web gallery app using sdl2 & sdl2_ttf.
[link](https://gliheng.github.io/rust-wasm/sdl2-gallery/)

### sdl2-mandelbrot
A mandelbrot example to compare perf of js and wasm. This demo works on both web and pc.
[link](https://gliheng.github.io/rust-wasm/sdl2-mandelbrot/)
