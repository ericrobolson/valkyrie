/// This is the lib that pulls in all things together. Done this way to support Android builds.
mod program;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    program::program();
}
