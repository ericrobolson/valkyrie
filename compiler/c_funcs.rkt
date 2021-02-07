#lang racket




(provide preprocess)
(provide cinclude)
(provide const)
(provide main)
(provide def)
(provide def-mut)

(define (preprocess val)
  (~a "#define " val))

(define (cinclude val)
   (~a "#include " val))

;; Creates a constant value with the given id
(define-syntax-rule
  (const id val)
  ; use `begin` to define a value to be used for later. This enables type checking before compilation.
  
   ; Now output the c code
   (~a "const " (get-type val) " " 'id " = " val ";"))   
  
;; Creates a new value
(define-syntax-rule
  (def (type id) val)
   ; Now output the c code
  (cond
    ((null? val) (~a 'type " *" 'id " = NULL;"))
    (else (~a 'type " " 'id " = " val ";"))))

(define-syntax-rule
  (def-mut (type id) val)
    ; Now output the c code
  (cond
    ((null? val) (~a 'type " *" 'id " = NULL;"))
    (else (~a 'type " " 'id " = " val ";"))))


;; Define a function
(define 
  (main body)
  
  (list
   
   "int main(int argc, char* args[]){\n"
      body
      "\nreturn 0\n;}\n"
      ))


(define (get-type val)
  (cond
      ([integer? val] "int")
    (else (raise "TODO: Get type for " val))))