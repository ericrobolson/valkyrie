#lang racket
(define out (open-output-file "../main.rs"))
(display "hello" out)
(close-output-port out)