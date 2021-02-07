#lang racket
;; A simple 'compiles to c' program

;; Flattens a list
(define (flatten l)
  (cond ((null? l) '())
        ((pair? l) (append (flatten (car l)) (flatten (cdr l))))
        (else (list l))))

(define (to-code-str l)
  (cond ((null? l) "")
        (else (apply ~a (map (lambda (i)(~a "" i))(flatten l))))))


(define (tab l)
  (define t "    ")
  (cond ((null? l) (list t))
        ((pair? l) (append (tab (car l)) (tab (cdr l))))
        (else (list t l))))


;; Creates an item that is initialized, used, then finally cleaned up
(define (scoped initialize do-work cleanup)
  (list initialize do-work cleanup))

;; Creates a null type
(define-syntax-rule
  (ptr type name)
    (statement (~a 'type " *" 'name " = NULL")))

;; Defines a return statement
(define (return value)
  (statement (~a 'return " " value)))

;; Defines an assign
(define (assign id value)
  (list id '= value))

(define (default type)
  (case type
    ['void '()]
    ['int (return 0)]
    [else (raise (~a "Unhandled default for type '" type "'"))]))



;; Creates a new function
(define-syntax-rule
  (fn type id args body)
  (list
   type " " id " " args
   (block (list body
   ; Default return statement
        (if (eq? type 'void) '()
        (default type)
        )))
  ))


(define (block stmt)
  (list "\n{\n" stmt "\n}"))


(define (statement s)
  (~a "\n" s ";"))

(define (SDL_Quit) (statement "SDL_Quit()"))
(define (SDL_DestroyWindow) (statement "SDL_DestroyWindow()"))

;; Definition of SDL2's lifecycle
(define (cSDL2)
  (scoped (ptr SDL_Window window)
          (scoped
           (ptr SDL_Surface screenSurface)
           ;;'() ;; TODO: add in main functinos
           (list (block 'SDL_Init-failed) (block 'SDL_Init-success))
           '())
          (list (SDL_Quit)(SDL_DestroyWindow))))


;; main function
(define main (fn 'int 'main 'todo-params
  (list
   (cSDL2)
   (return 0)
   )
  ))

;; Compile it
(define (generated-code)
  (to-code-str main))

;; DEBUG
(display (generated-code))

;; WRITE IT TO A FILE!
(define out (open-output-file "output.c" #:mode 'binary #:exists 'replace ))
 (display (to-code-str main) out)
 (close-output-port out)