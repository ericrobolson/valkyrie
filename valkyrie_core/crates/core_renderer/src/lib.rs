// Top level renderer.
// Does things at the scene level, to allow individual backends to handle what's going on.

use core_data_structures::queue::Queue;
use core_math::{Mat4, Vec3};

const RENDER_COMMAND_CAPACITY: usize = 256;

#[derive(Default)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Option<Vec3>,
}

impl Camera {
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::view_matrix(self.eye, self.target, self.up.unwrap_or(Vec3::unit_z()))
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub enum RenderCommand {
    UpdateCamera(Camera),
    DrawAabb { color: Color },
}

/// Top level renderer. Functionality based.
pub struct Renderer {
    /// The actual platform specific rendering backend.
    backend: Box<dyn BackendRenderer>,
    dirty: bool,
    render_pass: RenderPass,
}

pub struct RenderPass {
    commands: Queue<RenderCommand>,
}

impl RenderPass {
    /// Adds a new command to the render pass
    pub fn add(&mut self, command: RenderCommand) -> &mut Self {
        self.commands.push(command);
        self
    }
}

impl Renderer {
    fn new(backend: Box<dyn BackendRenderer>) -> Self {
        Self {
            dirty: true,
            backend,
            render_pass: RenderPass {
                commands: Queue::new(RENDER_COMMAND_CAPACITY),
            },
        }
    }

    /// The render pass to execute. Operates in a functional approach.
    pub fn create_render_pass(&mut self) -> &mut RenderPass {
        self.render_pass.commands.clear();
        self.dirty = true;

        &mut self.render_pass
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        let (w, h) = {
            let mut w = w;
            let mut h = h;
            if w == 0 {
                w = 1;
            }

            if h == 0 {
                h = 1;
            }

            (w, h)
        };

        self.backend.resize(w, h);
    }

    /// Dispatches the render pass.
    pub fn dispatch(&mut self) {
        if self.dirty {
            self.dirty = false;
            self.backend.set_render_pass(&self.render_pass.commands);
        }

        self.backend.dispatch();
    }
}

/// The platform specific backend renderer. Such as OpenGL, Vulkan, WGPU, etc.
pub trait BackendRenderer {
    /// Dispatches all queued commands and draws to the screen
    fn dispatch(&mut self);

    /// Resizes the window
    fn resize(&mut self, w: u32, h: u32);

    /// Updates the render commands to execute. Takes a functional approach, where this is the new frame.
    fn set_render_pass(&mut self, commands: &Queue<RenderCommand>);
}

/// Creates a new renderer
pub fn make_renderer(backend: Box<dyn BackendRenderer>) -> Renderer {
    Renderer::new(backend)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
