#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WindowInput {
    Shutdown,
    RedrawRequested,
    Resize { w: u32, h: u32 },
}

///  Implementation of a window
pub trait Window<MainFunc>
where
    MainFunc: FnMut(Option<WindowInput>) + 'static,
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
