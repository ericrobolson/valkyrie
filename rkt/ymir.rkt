#lang racket

; Data types
(define (data-types)
  (list
   'flag
   'char
   'n
   '+n
   'u
   'uORn
   'x
   'xt
   'addr
   'a-addr
   'c-addr
   'ior
   'd
   '+d
   'ud
   'dORud
   'xd
   'colon-sys
   'do-sys
   'case-sys
   'of-sys
   'orig
   'dest
   'loop-sys
   'nest-sys
   'i*x
   'j*x
   'k*x
   )
  )

(define (split-whitespace text)
  (string-split text))

(define (search-dict word) '('Some definition))

(define (modes) '('interpreting 'compiling))
(define (mode) 'interpreting)

(define (perform definition) (displayln 'TODO-Perform-Interpration-Of-Definition)) 


;; This is the guts. Builds up an AST to allow parsing!
(define (program) '
  (define (interpret text)
  (for ([word (split-whitespace text)])
    (define (definition)(search-dict word))
    (cond
      [(null? (definition)) (displayln "NOT FOUND")]
      [else (displayln "FOUND")]
    )
  )
 )
  )


(program)