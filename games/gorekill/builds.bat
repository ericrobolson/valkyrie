REM Windows 64 bit
set target=x86_64-pc-windows-msvc
cd gore_kill_lib
cargo build --target %target%
cd ..
xcopy /f /y ".\gore_kill_lib\target\%target%\debug\gore_kill_lib.dll" ".\GORE_KILL\native_libs\gore_kill_%target%.dll"