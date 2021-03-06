#lang racket

;; An experiment in creating a platform agnostic GUI + programming language
;; Works on Racket, JavaScript, Rust and possibly Elixir
;; Tail call optimized


; Prints a line to STD out
(define (print-ln data) (display data))

; Reads a line from STD in
(define (read-ln) (read-line))


(define (targets) (list 'rust 'js 'elixir 'racket))

(define (compile language source-dir)
  (print-ln (list "Beginning compilation for " language)))


(define (make-fn data)
  (display data))



(define prog
  '(
    (fn (hello_world) (
                       (print-ln "HELLO")))
    )
  )