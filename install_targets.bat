REM ANDROID
rustup target add arm-linux-androideabi
rustup target add armv7-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android
rustup target add x86_64-linux-android
cargo install cargo-apk

REM IOS
rustup target add aarch64-apple-ios 
rustup target add aarch64-apple-ios 
rustup target add x86_64-apple-ios 
xcode-select --install
cargo install cargo-lipo

REM Windows
rustup target add x86_64-pc-windows-msvc

REM WASM
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
   

set /p Input=Press any key to exit
