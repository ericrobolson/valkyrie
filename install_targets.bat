rustup target add arm-linux-androideabi
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android
rustup target add x86_64-pc-windows-msvc
rustup target add wasm32-unknown-unknown

cargo install cargo-apk
cargo install wasm-bindgen-cli