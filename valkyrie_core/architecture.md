# Guiding Principles
* Use references
* Do all allocation at startup
* Dependencies (should) always flow down and are one directional
* Platform specific stuff should be put into a subcrate, then exposed through a common api

# Project Layout
* `crates/` contains specific functionality implementations
* `src/` ties together all crates into one engine


# OUT OF SCOPE FOR NOW:
* Multi threaded engine
* Netcode
* 3d