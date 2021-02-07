#lang racket
;; A simple 'compiles to c' program

;; Things that are 'c' code start with c, such as 'cptr' or 'cassign'



;; Creates an item that is initialized, used, then finally cleaned up
(define (scoped initialize do-work cleanup)
  (list initialize do-work cleanup))

;; Flattens a list
(define (flatten l)
  (cond ((null? l) '())
        ((pair? l) (append (flatten (car l)) (flatten (cdr l))))
        (else (list l))))

(define (flatten-append l)
  (cond ((null? l) "")
        (else (~a  (car l) " " (flatten-append (cdr (flatten l)))))))

;; Creates a null type
(define-syntax-rule
  (cptr type name)
    (statement (~a 'type " *" 'name " = NULL")))

(define (main argc args)
  (list
   (cSDL2)
   '(0)
   ))

(define (statement s)
  (~a s ";"))

(define (SDL_Quit) (statement "SDL_Quit()"))
(define (SDL_DestroyWindow) (statement "SDL_DestroyWindow()"))

;; Definition of SDL2's lifecycle
(define (cSDL2)
  (scoped (cptr SDL_Window window)
          (scoped (cptr SDL_Surface screenSurface)
                  "INIT"
                  '())
          (list (SDL_Quit)(SDL_DestroyWindow))))



(define (assign id value)
  (list id '= value))

(main 'blah 'la)




