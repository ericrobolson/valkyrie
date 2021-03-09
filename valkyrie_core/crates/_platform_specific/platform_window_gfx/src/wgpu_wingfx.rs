use crate::wgpu_render;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use core_renderer::{BackendRenderer, Renderer};
use core_simulation::{ControlMessage, Input, Simulation, SimulationExecutor, WindowMsg};
use core_window::{Renderable, Window};

pub struct WgpuWindow {
    title: &'static str,
    w: u32,
    h: u32,
}

impl WgpuWindow {
    pub fn new(title: &'static str, w: u32, h: u32) -> Self {
        Self { title, w, h }
    }

    fn handle_event<T>(event: Event<T>, control_flow: &mut ControlFlow) -> Option<WindowMsg> {
        match event {
            Event::LoopDestroyed => None,
            Event::MainEventsCleared => Some(WindowMsg::RedrawRequested),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => Some(WindowMsg::Resize {
                    w: physical_size.width,
                    h: physical_size.height,
                }),
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    Some(WindowMsg::Shutdown)
                }
                _ => None,
            },
            Event::RedrawRequested(_) => Some(WindowMsg::RedrawRequested),
            _ => None,
        }
    }
}

impl<Sim, Cfg, Msg> Window<Sim, Cfg, Msg> for WgpuWindow
where
    Sim: Simulation<Cfg, Msg> + Renderable + 'static,
    Cfg: 'static,
    Msg: 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, mut executor: SimulationExecutor<Sim, Cfg, Msg>) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(self.title)
            .build(&event_loop)
            .unwrap();

        let mut renderer = core_renderer::make_renderer(Box::new(wgpu_render::make()));

        let mut last_frame = u64::MAX;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            let ev = Self::handle_event(event, control_flow);

            let ev = match &ev {
                Some(ev) => {
                    match ev {
                        WindowMsg::RedrawRequested => {
                            renderer.dispatch();
                            window.request_redraw();
                        }
                        WindowMsg::Shutdown => *control_flow = ControlFlow::Exit,
                        WindowMsg::Resize { w, h } => {
                            //windowed_context.resize(glutin::dpi::PhysicalSize::new(*w, *h));
                            //windowed_context.window().request_redraw();
                        }
                    }

                    Some(Input::WindowMsg(*ev))
                }
                None => None,
            };

            match executor.tick(ev) {
                ControlMessage::Ok => {}
                ControlMessage::ExitSim => {
                    *control_flow = ControlFlow::Exit;
                }
            }

            // If state was changed update render state + request redraw
            if executor.last_updated_frame() != last_frame {
                last_frame = executor.last_updated_frame();
                executor.sim().render(&mut renderer);
            }
        });
    }
}
