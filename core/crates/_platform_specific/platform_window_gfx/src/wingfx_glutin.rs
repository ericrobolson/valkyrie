use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, ContextWrapper};

use window::{Window, WindowInput};

pub struct GlutinWindow {}

impl GlutinWindow {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_event<T>(event: Event<T>, control_flow: &mut ControlFlow) -> Option<WindowInput> {
        match event {
            Event::LoopDestroyed => Some(WindowInput::Shutdown),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => Some(WindowInput::Resize {
                    w: physical_size.width,
                    h: physical_size.height,
                }),
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    Some(WindowInput::Shutdown)
                }
                _ => None,
            },
            Event::RedrawRequested(_) => Some(WindowInput::RedrawRequested),
            _ => None,
        }
    }
}

impl<MainFunc> Window<MainFunc> for GlutinWindow
where
    MainFunc: FnMut(Option<WindowInput>) + 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, mut main_loop_function: MainFunc) {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        let context = unsafe {
            glow::Context::from_loader_function(|s| {
                windowed_context.get_proc_address(s) as *const _
            })
        };

        el.run(move |event, _, control_flow| {
            println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            let ev = Self::handle_event(event, control_flow);

            match ev {
                Some(ev) => match ev {
                    WindowInput::Shutdown => {}
                    WindowInput::RedrawRequested => {
                        //gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                        windowed_context.swap_buffers().unwrap();
                    }
                    WindowInput::Resize { w, h } => {
                        windowed_context.resize(glutin::dpi::PhysicalSize::new(w, h));
                    }
                },
                None => {}
            }

            main_loop_function(ev);
        });
    }
}