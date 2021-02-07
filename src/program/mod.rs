mod gfx;

// TODO: eventually refactor this after you have something working

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

pub fn program() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for ev in event_pump.poll_iter() {
            // handle user input here
            println!("EVENT: {:?}", ev);

            match ev {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // render window contents here
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }

    // Device is automatically closed when dropped

    Ok(())
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
