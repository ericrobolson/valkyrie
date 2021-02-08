There's to layers to this primarily.
A `_platform_specific/` folder which contains all functionality that is can change on various platforms or is hardware dependant.
Everything else, which is platform agnostic and is built off the platform specific stuff. The interfaces + traits are defined in the platform agnostic crates, and implemented in `_platform_specific` crates.

# Platform Agnostic 
`wingfx/` contains the method to build out windows + renderers. Primarily focused around WGPU + GLow for now.
`renderer/` contains a platform agnostic rendering implementation
`threading` provides threading + job queue mechanisms
`file_io` provides an async file i/o library built off of `threading`
`audio` provides audio mechanisms