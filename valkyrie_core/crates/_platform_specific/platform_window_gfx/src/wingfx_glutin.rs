use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, ContextWrapper};

use core_renderer::Renderer;
use core_window::{Renderable, Simulation, Window, WindowControl, WindowInput};

pub struct GlutinWindow {
    title: &'static str,
    w: u32,
    h: u32,
}

impl GlutinWindow {
    pub fn new(title: &'static str, w: u32, h: u32) -> Self {
        Self { title, w, h }
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

impl<Sim> Window<Sim> for GlutinWindow
where
    Sim: Simulation + Renderable + 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, mut simulation: Sim) {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(self.title)
            .with_inner_size(glutin::dpi::LogicalSize::new(self.w as f32, self.h as f32));
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

        let mut glow_renderer = make_glow_renderer();

        el.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            let ev = Self::handle_event(event, control_flow);

            match ev {
                Some(ev) => match ev {
                    WindowInput::Shutdown => *control_flow = ControlFlow::Exit,
                    WindowInput::RedrawRequested => {
                        windowed_context.swap_buffers().unwrap();
                    }
                    WindowInput::Resize { w, h } => {
                        windowed_context.resize(glutin::dpi::PhysicalSize::new(w, h));
                    }
                },
                None => {}
            }

            let mut queue_render = false;

            match simulation.tick(ev) {
                WindowControl::Ok => {
                    // Do nothing
                }
                WindowControl::Shutdown => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowControl::Render => {
                    queue_render = true;
                }
                WindowControl::UpdateRenderState => {
                    simulation.render(&mut glow_renderer);
                    queue_render = true;
                }
            }

            if queue_render {
                windowed_context.window().request_redraw();
            }
        });
    }
}

fn make_glow_renderer() -> impl Renderer {
    GlowRenderer {}
}

struct GlowRenderer {}
impl Renderer for GlowRenderer {}
