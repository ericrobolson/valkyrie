pub enum BackendType {
    /// Utilizes OpenGL as the backend
    Opengl,
    /// Uses WGPU Vulkan as the backend
    WgpuVulkan,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub h: u32,
    pub w: u32,
}

pub struct WindowBuilder {
    title: &'static str,
    size: Size,
}

impl WindowBuilder {
    pub fn new(title: &'static str, backend: BackendType) -> Self {
        Self {
            title,
            size: Size { w: 640, h: 480 },
        }
    }

    pub fn with_min_size<'a>(&'a mut self, min_h: u32, min_w: u32) -> &'a mut Self {
        self.size.h = min_h;
        self.size.w = min_w;

        self
    }

    pub fn build(&self) -> impl Window {
        GlutinWindow::new(self.title, self.size)
    }
}

pub struct GlutinWindow {}
impl GlutinWindow {
    pub fn new(title: &'static str, size: Size) -> Self {
        let el = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(size.w, size.h));

        let context = glutin::ContextBuilder::new()
            .build_windowed(wb, &el)
            .unwrap();
        loop {}
        Self {}
    }
}
impl Window for GlutinWindow {
    fn build_renderer(&mut self) -> std::result::Result<Box<dyn Renderer>, RenderBuildError> {
        todo!()
    }
}

pub struct WinitWindow {}
impl Window for WinitWindow {
    fn build_renderer(&mut self) -> std::result::Result<Box<dyn Renderer>, RenderBuildError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
