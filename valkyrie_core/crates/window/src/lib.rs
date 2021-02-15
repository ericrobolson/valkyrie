/// Input from a window
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WindowInput {
    Shutdown,
    RedrawRequested,
    Resize { w: u32, h: u32 },
}

/// Control messages provided to the window
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WindowControl {
    /// Do nothing, keep processing
    Ok,
    /// Shut down
    Shutdown,
}

///  Implementation of a window
pub trait Window<MainFunc>
where
    MainFunc: FnMut(Option<WindowInput>, &mut dyn renderer::Renderer) -> WindowControl + 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, main_loop_function: MainFunc);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
