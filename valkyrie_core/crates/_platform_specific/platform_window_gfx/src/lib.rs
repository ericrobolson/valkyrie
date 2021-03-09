pub mod glow_wingfx;
pub mod wgpu_wingfx;

pub use glow_wingfx::OpenGlWindow;
pub use wgpu_wingfx::WgpuWindow;

mod glow_render;
mod wgpu_render;
