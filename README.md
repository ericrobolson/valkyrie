A simple demo inspired by Project ILL (https://www.youtube.com/watch?v=FVZuF5WkVG0&ab_channel=OlegVdovenko)
Used to stub out an engine.

## Main Project Structure
* `core/` Contains engine specific code.
* `pil/` Contains platform specific implementations of traits. (platform independence layer)
* `src/` Contains game specific code.
* * `src/lib.rs` - Glue for building a lib to be consumed by Android (https://github.com/rust-windowing/android-ndk-rs)
* * `src/main.rs` - Main file for binary applications.
* * `src/program.rs` - The actual definitions for the engine. 


# Building
## Windows
* Should work out of the box
* Use `cargo run` to launch

## Android 
* Ensure Android Studio is set up. Ensure `ANDROID_SDK_ROOT`, `ANDROID_NDK_ROOT` env vars are set. 
* Ensure you have a env var for `keytool` set to `\Android Studio\jre\bin\keytool.exe` if you're getting `Error: Command keytool not found.`.
* Reference `https://crates.io/crates/cargo-apk` for issues + build targets.
* `cargo install cargo-apk` to build, run, debug android apps. 
* `cargo apk run` to execute on real or connected devices.
* Right now only `i686-linux-android` is supported, but more will be added later on.

## WASM
* TODO: add in support

## OpenXR
* TODO: add in support

# Verification
* Run `verify.bat` to build on all expected platforms.
