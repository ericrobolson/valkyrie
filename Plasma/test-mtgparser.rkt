#lang racket


(define baleful-strix
  '(
    (name "Baleful Strix")
    (cost 1u 1b)
    (type artifact creature)
    (race bird)
    (modifiers flying deathtouch)
    (triggers
     (when this enters-battlefield draw-card 1))
    (health 1)
    (attack 1)
    ))


(define (validate-card card)
  (define (err e) (raise (~a "Error: '" e "' for card '" card "'.") #t))
  (define (required-attr attr)
    (cond
      [(not (member attr (map (lambda (l) (car l)) card))) (err (~a "required '" attr "'!"))]
      [(eq? (length (member attr card))) (err "UH")]
      ))

  (cond
    [(not (list? card)) (err "Not a list of attributes!")]
    )
  ;; Required attributes
  (required-attr 'name)
  (required-attr 'type)
  )