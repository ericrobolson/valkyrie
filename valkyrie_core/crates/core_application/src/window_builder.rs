use core_simulation::Simulation;
use core_window::{Renderable, Window};

pub enum BackendType {
    /// Utilizes OpenGL as the backend
    Opengl,
    /// Uses WGPU Vulkan as the backend
    Wgpu,
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
    pub fn build<Sim, Cfg, Msg>(
        &self,
        backend: BackendType,
    ) -> Result<Box<dyn Window<Sim, Cfg, Msg>>, WinGfxBuildErr>
    where
        Sim: Simulation<Cfg, Msg> + Renderable + 'static,
        Cfg: 'static,
        Msg: 'static,
    {
        match backend {
            BackendType::Opengl => Ok(Box::new(platform_window_gfx::OpenGlWindow::new(
                self.title,
                self.size.w,
                self.size.h,
            ))),
            BackendType::Wgpu => Ok(Box::new(platform_window_gfx::WgpuWindow::new(
                self.title,
                self.size.w,
                self.size.h,
            ))),
        }
    }
}
