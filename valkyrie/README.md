A simple demo inspired by Project ILL (https://www.youtube.com/watch?v=FVZuF5WkVG0&ab_channel=OlegVdovenko)
Used to stub out an engine.



# Project Structure
`src/lib.rs` - Glue for building a lib to be consumed by Android (https://github.com/rust-windowing/android-ndk-rs)
`src/main.rs` - Main file for binary applications.
`src/program.rs` - The actual definitions for the engine. 


# Building
## Windows
* Should work out of the box
* Use `cargo run` to launch

## Android 
* `cargo install cargo-apk` to build, run, debug android apps. 
* `cargo apk run` to execute on real or connected devices.

## WASM
* TODO: add in support

## OpenXR
* TODO: add in support
