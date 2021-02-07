#lang racket
#reader "ymir.rkt"
(require "c_funcs.rkt")


(preprocess "SDL_MAIN_HANDLED")
(cinclude "<stdio.h>")
(cinclude "\"SDL.H\"")

(const SCREEN_W 640)
(const SCREEN_H 480)

(main
 (list
       (def (SDL_Window window) null)
       (def (SDL_Surface screenSurface) null)
       ))