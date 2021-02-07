#lang racket




(provide preprocess)
(provide cinclude)
(provide const)
(provide main)
(provide def)

(define (preprocess val)
  (~a "#define " val))

(define (cinclude val)
   (~a "#include " val))

;; Creates a constant value with the given id
(define-syntax-rule
  (const id val)
  ; use `begin` to define a value to be used for later. This enables type checking before compilation.
  (begin
   (define (id) val)
   ; Now output the c code
   (~a "const " (get-type val) " " 'id " = " val ";"))   )
  
;; Creates a new value
(define-syntax-rule
  (def (type id) val)
   ; Now output the c code
  (cond
    ((null? val) (~a 'type " *" 'id " = NULL;"))
    (else (~a 'type " " 'id " = " val ";"))))


;; Define a function
(define-syntax-rule
  (main body)
  
  (~a
   "int main(int argc, char* args[]){
"
      body
      "
return 0;
}"
      ))


(define (get-type val)
  (cond
      ([integer? val] "int")
    (else (raise "TODO: Get type for " val))))