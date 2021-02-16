pub use renderer::Renderer;
pub use window::{Window, WindowControl, WindowInput};

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
    title: &'static str,
    size: Size,
}

impl WinGfxBuilder {
    pub fn new(title: &'static str, backend: BackendType) -> Self {
        Self {
            title,
            size: Size { w: 640, h: 480 },
        }
    }

    pub fn with_min_size<'a>(&'a mut self, min_w: u32, min_h: u32) -> &'a mut Self {
        self.size.h = min_h;
        self.size.w = min_w;

        self
    }

    pub fn build<MainFunc>(&self) -> Result<Box<dyn Window<MainFunc>>, WinGfxBuildErr>
    where
        MainFunc:
            FnMut(Option<WindowInput>, &mut dyn renderer::Renderer) -> WindowControl + 'static,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
