#lang racket

(define (file) "../main.valk")

; Loads a file into a list
(define (load-file f)
  (with-input-from-file f
    (lambda () (let loop ((lines '())
                          (next-line (read-line)))
                 (if (eof-object? next-line)
                     (reverse lines)
                     (loop (cons next-line lines)
                           (read-line)
                           )
                     )
                 )
      )
    )
  )       

; processing functions

; add space around all characters in a string
(define (spaced str character)
  (string-replace str character (~a " " character " ")))

; replace whitespace characters with a space
(define (consolidate-whitespace str)
  (define (ws) (list "\t" "\r" "\n"))
  (define (rp s c)
    (string-replace s c " "))
  (rp (rp (rp str "\t") "\r") "\n")
  )

; preprocess all contents of a string so it's ready to tokenize
(define (prepare contents)
  (define (appended-contents)(~a contents))
  ; remove leading + trailing paren
  (define (cleaned)(substring (appended-contents) 1 (- (string-length (appended-contents)) 1)))
  ; add spaces around given character
 
  ; final 
  (consolidate-whitespace (spaced (spaced (cleaned) "{") "}"))
  )


; lex a file
(define (lex file)
  (define (contents) (load-file file))
  (prepare (contents))
  )


; testing


(lex "../main.valk")