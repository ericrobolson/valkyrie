#lang racket
(require rackunit)


;; An experiment in creating a platform agnostic GUI + programming language
;; Works on Racket, JavaScript, Rust and possibly Elixir
;; Tail call optimized



;; Is it a valid function?
(define (func? f)
  (define (defs)(car (cdr f)))
  (define (body)(cdr (cdr f)))
  
  (cond
    ;; Validate it's a valid form
    [(not (list? f)) #f]
    [(not (eq? (length f) 3)) #f]
    [(not (eq? (car f) 'fn)) #f]
    ;; Validate first arg of func definition 
    [(not (list? (defs))) #f]
    [(eq? (defs) '()) #f]

    ;; It's valid
    [else #t]))

(check-eq? (func? 'test) #f "Not a list not func")
(check-eq? (func? '()) #f "Empty list not func")
(check-eq? (func? '(1 '(asdf) '(bsdf))) #f "Requires 'fn'")

(check-eq? (func? '(fn (main) (println (bar (foo))))) #t)
(check-eq? (func? '(fn (foo - i32) (13))) #t)
(check-eq?(func? '(fn (bar a:i32 - i32) (* a a))) #t)
(check-eq?(func? '(fn () (* a a))) #f)

(define (compile program)
  (map (lambda (f) (make-func f)) program))

(define (execute-func f)
  (write f))

(define (make-func f)
  (define (name) (car (car (cdr f))))
  (define (body) (compile (cdr (cdr f))))
  ;; compile it
  (define (write-func f)
    (write (list "pub fn" (name) "()" "{" (body) "}" "\n"))
    )
  
  (cond
    ;; valid length
    [(eq? (length f) 3) (write-func f)]
    ;; invalid length
    [else (raise
           (~a 
           "invalid length for function! expected the form '(fn (def name args* (return type)?) (body))'. Passed '"
           f
           "'")
           #t)]
    ))




;; testing


(define test-program '(
  (fn (main) (println (bar (foo))))
  (fn (foo - i32) (13))
  (fn (bar a:i32 - i32) (* a a))
  ))



;;(compile test-program)