echo !! BEGIN NATIVE BUILD !!
cargo build

echo !! BEGIN WINDOWS BUILDS !!
cargo build --target=x86_64-pc-windows-msvc		

echo !! BEGIN ANDROID BUILDS !!
cargo apk build

echo !! BEGIN IOS BUILDS !!

cargo build --target=aarch64-apple-ios 
cargo build --target=aarch64-apple-ios 
cargo build --target=x86_64-apple-ios 


echo !! BEGIN WEB BUILDS TODO !!
###cargo build --target wasm32-unknown-unknown

echo !! !! !! !!
set /p Input=Press any key to exit


