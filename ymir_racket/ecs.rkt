#lang racket

; Creates a new type
(define-syntax make-type
  (syntax-rules ()
    ((_ type params ...)
     (define (type params ...)
        (list 'type ( list 'params params) ... )
       )
     )
    )
  )

; Creates a typed property list
(define (properties . property-list)
  (define (reduction a)
    (cond ((null? a) '())
          ((null? (cdr a))(raise "requires a list of type/properties"))
          (else (append (list (list (list-ref a 0)(list-ref a 1)))(reduction (list-tail a 2))))))
   
  (define (mapped-list) (reduction property-list))
  *(mapped-list)
  )


    

(display (properties 'Num '(pen) 'Vec3 '(normal)))

; Definition macros
(make-type struct id properties)
(make-type component id properties)
(make-type system id required-components)


;;;;;;;;;;;;;
; Definitions
;;;;;;;;;;;;;

; Structs
(define (num) (struct 'Num '()))
(define (vec3) (struct 'Vec3 (properties 'Num '(x y z))))
(define (quat) (struct 'Quaternion (properties 'Num '(x y z w))))

(define (c-manifold) (struct 'CManifold (properties 'Num '(penetration) 'Vec3 '(normal))))

; Components
(define (comp-phys) (component 'Physics (properties 'Vec3 '(position velocity))))

; Systems
(define (sys-cd) (system 'collision-detection '('Physics)))



(display (sys-cd))
(display "\ncomponents\n")
(display (comp-phys))

(define (test-list) '())




