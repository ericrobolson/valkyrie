#lang racket

(provide SDL_Init)
(provide SDL_DestroyWindow)
(provide SDL_Quit)

(define (SDL_Init) "SDL_Init(SDL_INIT_VIDEO) < 0")

(define (SDL_DestroyWindow window)
  (~a "SDL_DestroyWindow(" window ")"))

(define (SDL_Quit)
  "SDL_Quit();")