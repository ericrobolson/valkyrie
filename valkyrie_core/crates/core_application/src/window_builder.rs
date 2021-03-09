use core_simulation::Simulation;
use core_window::{Renderable, Window};

pub enum BackendType {
    /// Utilizes OpenGL as the backend
    Opengl,
    /// Uses WGPU Vulkan as the backend
    WgpuVulkan,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WinGfxBuildErr {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub h: u32,
    pub w: u32,
}

pub struct WinGfxBuilder {
    pub title: &'static str,
    pub size: Size,
}

impl WinGfxBuilder {
    pub fn build<Sim, Cfg, Msg>(&self) -> Result<Box<dyn Window<Sim, Cfg, Msg>>, WinGfxBuildErr>
    where
        Sim: Simulation<Cfg, Msg> + Renderable + 'static,
        Cfg: 'static,
        Msg: 'static,
    {
        Ok(Box::new(
            platform_window_gfx::wingfx_glutin::GlutinWindow::new(
                self.title,
                self.size.w,
                self.size.h,
            ),
        ))
    }
}
