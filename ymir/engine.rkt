#lang racket

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Definition of the Valkyrie Engine ;
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Systems - defined in dependency order from lowest level to highest levels
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;


; NOTE: this is based off Game Engine Architecture, so it may be overkill

; Platform independence layer, defined in startup order
(define (pil-systems)
  (list
   'pil-data-structures ; common data structures + iterators
   'pil-threading ; threading library + job queue
   'pil-time ; system time
   'pil-file-io ; synchronous file io
   'pil-network ; low tcp/udp networking layer
   'pil-gfx ; low level gfx wrappers
   )
  )

; Core systems, defined in startup order. Depends on pil-* systems.
(define (core-systems)
  (list
   'core-alloc ; memory allocation
   'core-math ; math operations
   'core-string ; strings
   'core-debug-log ; debug/logging
   'core-parsers ; file parsers (GLTF, CSV, etc.)
   'core-profile ; engine profiling tools
   'core-engine-config ; engine configurations
   'core-rng ; random number generators
   'core-object-id-handles ; object handles/ids
   'core-async-file-io ; async-file threading using pil-threading + pil-file-io
   'core-network-transport-layer ; high level networking protocols such as GGPO, Tribes2, Halo Reach, etc.
   )
  )

; Resource/game assets
(define (ass-systems)
  (list
   'ass-3d-gltf
   'ass-3d-obj
   'ass-textures
   'ass-materials
   'ass-fonts
   'ass-physics-definitions
   'ass-levels
   )
  )

; Human interaction devices
(define (hid-systems)
  (list
   'hid-physical-device
   )
  )

; Profiling + debugging + playback systems
(define (pdp-systems)
  (list
   'pdp-recording
   'pdp-perf-stats
   'pdp-in-game-settings
   )
  )


; Physics/Collision/Skeletal
(define (phys-systems)
  (list
   'phys-positional
   'phys-forces
   'phys-constraints
   'phys-shapes
   'phys-collision-detection
   'phys-collision-resolution
   )
  )

; Spatial/Scene graphs
(define (spat-systems)
  (list
   'spatial-scene-graph
   'spatial-occlusion
   'spatial-lod
   )
  )

; Online multiplayer
(define (mp-systems)
  (list
   'mp-state-replication
   'mp-server
   'mp-client
   )
  )


; Audio Renderer
(define (aud-systems)
  (list
   'aud-dsp
   'aud-spatial-model
   'aud-playback
   )
  )

; Low level GFX renderer
(define (gfx-systems)
  (list
   'gfx-materials
   'gfx-shaders
   'gfx-lighting
   'gfx-cameras
   'gfx-text
   'gfx-primitive
   'gfx-viewports
   'gfx-textures
   'gfx-surface
   'gfx-debug
   )
  )

; Visual Effects
(define (vfx-systems)
  (list
   'vfx-light-maps
   'vfx-shadows
   'vfx-hdr-lighting
   'vfx-pbr
   'vfx-particles
   'vfx-post-fx
   'vfx-environmental
   )
  )


; Front End
(define (fe-systems)
  (list
   'fe-hud
   'fe-in-game-cinematics
   'fe-in-game-gui
   'fe-menus
   )
  )


; Scripting (game specific)
(define (script-systems)
  (list
   'script-eventing
   'script-language-loader
   'script-language-parser
   'script-language-executor
   )
  )

; Define all systems in their dependency order
(define (systems)
  (append
   (pil-systems)
   (core-systems)
   (ass-systems)
   (hid-systems)
   (pdp-systems)
   (phys-systems)
   (spat-systems)
   (mp-systems)
   (aud-systems)
   (gfx-systems)
   (vfx-systems)
   (fe-systems)
   (script-systems)
   )
  )

; Define the engine to run
(define (engine systems) 
  ; build up the main loop of the program
  (append
   ; start up all systems in dependency order
   (list 'start-systems (systems))

   ; TODO: main loop
   (list '('main-loop '('main-loop-executions)))

   ; shut down all systems in reverse dependency order
   (list 'shutdown-systems (reverse (systems)))
   )
  )


; execute the build of the engine
(pretty-print (engine systems))
