#lang racket

;; Helper functions

(provide flatten)

;; Flattens a list
(define (flatten l)
  (cond ((null? l) '())
        ((pair? l) (append (flatten (car l)) (flatten (cdr l))))
        (else (list l))))