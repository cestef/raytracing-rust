# Raytracing in Rust

This is a raytracer written in Rust. It is based on the book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.

## Features

- [x] Spheres
- [x] Planes
- [x] Antialiasing
- [x] Lambertian materials
- [x] Metal materials
- [x] Dielectric materials
- [x] Positionable camera
- [x] Depth of field
- [x] FOV
- [x] Gamma correction
- [x] Multithreading
- [x] Triangle mesh support
- [ ] Rendering to a window
- [ ] Textures
- [ ] Importing models
- [ ] Lights

## Running

The help page can be accessed by running the program with the `--help` flag.

```bash
cargo run --release -- --help
```

This will print the following:

```
$ cargo run --release -- --help

A raytracer written in Rust.

Usage: ray-tracing-rust [OPTIONS]

Options:
  -w, --width <WIDTH>                [default: 2560]
  -h, --height <HEIGHT>              
  -a, --aspect-ratio <ASPECT_RATIO>  [default: 16/9]
  -s, --samples <SAMPLES>            [default: 100]
  -t, --threads <THREADS>            [default: 8]
  -o, --output <OUTPUT>              [default: output.ppm]
  -f, --fov <FOV>                    [default: 90]
      --open                         
  -h, --help                         Print help
  -V, --version                      Print version
```

Example:

```bash
cargo run --release -- -w 1920 -h 1080 -s 1000 -t 8 -o output.ppm
```
