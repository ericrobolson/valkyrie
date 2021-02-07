#lang racket
;; This is a reader for Ymir

(provide read read-syntax)

(define (read in)
  (~a "HI" in))
(define (read-syntax src in) src in)