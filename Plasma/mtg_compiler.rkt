#lang racket
(require rackunit)

;;; Card Game Bytecode
;;; Stack based
;;; Maybe for GFX, do some sort of procedural generation with fractals based on the card?


;; TODO: define modifiers macro which adds to script
;; TODO: 

;; TODO: modifiers adds script text
(define (modifier mod)
  (display "TODO"))

(define (deathtouch) '(deals-damage this destroy other creature))
(define (flying) '(blockers this can't be blocked except by creatures with flying or reach))

(define test-card
  '(
    (name "Baleful Strix")
    (cost 1u 1b)
    (types artifact creature)
    (races bird)
    (modifiers (flying) (deathtouch))
    (
    ))

(define (valid-card? card)
  (
