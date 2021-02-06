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

; List of operations that do things to the stack
(define (glossary built-in)
  (match built-in
    ['+ (list 'nORu 'nORu '-- 'nORu)]
    [else 'UNDEFINED]
  ))

; List of predefined commands for FORTH.
(define (built-in-commands)
  (list
   'BYE
   'HELP
   '+
   ))



; Let's add some helper functions and their descriptions.
(define (built-in-help built-in)
  (define (stack-op-help msg) (~a msg " Stack OP: '" (glossary built-in) "'"))
  (~a "'" (symbol->string built-in) "': "
  (match built-in
    ['BYE "Quit the program."]
    ['HELP "Retrieve a list of all built-in commands and their information."]
    ['+ (stack-op-help "Adds two numbers together, pushing the result on the stack.")]
    [else (~a "The built-in '" built-in "' needs to be documented.")])))

(define (input-loop)
  (display "valk->")
  (define command (read-line))
  (cond [(string=? command "BYE") (displayln "quitting loop")]
        [(string=? command "HELP")
                   (for ([cmd (built-in-commands)]) (displayln (~a (built-in-help cmd))))
                   (input-loop)
                   ]                                 
                                   
        [else (displayln (~a "Unknown command: '" command "'. Type 'HELP' for some info on commands.")) (input-loop)]))


;; Now start FORTH!
(input-loop)

