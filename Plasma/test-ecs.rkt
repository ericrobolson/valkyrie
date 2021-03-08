#lang racket



(define test-ecs
  '(
    (const MOVE_UP:u16 0x0000_0000_0000_0000)
    
    (component Velocity (x:i32 y:i32))
    (component Position (x:i32 y:i32))
    (component Health (hp:i32))
    (component Input (mask:u16))
    (system Movement (Input:r Velocity:w)
            (do-some-stuff))
    (system Position (Velocity:r Position:w)
            (add-assign Position Velocity))

    )
  )

test-ecs