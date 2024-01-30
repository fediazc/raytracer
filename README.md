A ray tracer made by following along to [Ray Tracing in One Weekend](https://raytracing.github.io/)

## Building and Running

Prerequisites: a working installation of Rust and Cargo

While in the project directory, run the following commands:

```
cargo build --release
./target/release/rtow > image.ppm
```

(Might take a couple minutes to finish executing)

This will create an `image.ppm` file containing a ray traced render. If you don't have one, you need an image viewer that supports PPM files (like [ImageMagick](https://imagemagick.org/)) to be able to open the file.

The rendered scene in `image.ppm` is the one the defined in `src/main.rs`.
