The guts of the engine. Written in a platform agnostic way to ensure maximum compatibility.
As such, it's marked `#![no_std]`. 

Things such as file i/o, rendering, networking, etc. must be handled in a platform specific layer.
This crate simply provides traits that must be implemented to enable those.