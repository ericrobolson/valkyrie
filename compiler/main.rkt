#lang racket
(require "c_funcs.rkt")
(require "sdl2.rkt")

  ;; TODO: ensure that set! or assign are only applied to things with mut, unless it's done in an unsafe() method (e.g. read only ECS copy needs to be swapped)


(define (program) '(


(preprocess "SDL_MAIN_HANDLED")
(cinclude "<stdio.h>")
(cinclude "\"SDL.H\"")

(const SCREEN_W 640)
(const SCREEN_H 480)

(main 
  (
   list
   (def-mut (SDL_Window window) null)
   (def-mut (SDL_Surface screenSurface) null)
   (if (SDL_Init)
       ;; Success!
       "TODO"
       ;; Crashed!
       (display "Could not initialize! SDL_Error: " (SDL_GetError)))
   (SDL_DestroyWindow 'window)
   (SDL_Quit)
   ))
))


;; This is where it gets nuts. Make a namespace from the current program.
;; Then, you can eval the program.
(define-namespace-anchor a)
(define ns (namespace-anchor->namespace a))

(define (compile prog)
  ;; How to eval a single statement
  (define (eval-stmt s)
    (cond
      [(string? s) s]
      [(procedure? s) (eval-stmt (eval s ns))]
      [else (eval s ns)]))
  ;; Eval everything
  (for ([stmt prog])
    (define (result)
      (flatten
       (if (list? stmt)
           (eval-stmt stmt)
           stmt
           )))
    (pretty-print (result))
    ))

(compile (program))