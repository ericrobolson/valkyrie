echo !! BEGIN NATIVE BUILD !!
cargo build

echo !! BEGIN WINDOWS BUILDS !!
cargo build --target=x86_64-pc-windows-msvc		

echo !! BEGIN ANDROID BUILDS !!
cargo apk build

echo !! BEGIN WEB BUILDS - done last due to flags !!
cargo build --target wasm32-unknown-unknown

echo !! !! !! !!
set /p Input=Press any key to exit


