/// Module for creating client and server applications.
pub mod application;

/// Entity-Component-System module.
pub mod ecs {
    pub use core_ecs::*;
}

/// Common, generic data structures.
pub mod data_structures {
    pub use core_data_structures::*;
}

/// File input-output
pub mod file_io {
    pub use core_file_io::*;
}

/// Threading.
pub mod threading {
    pub use core_threading::*;
}

pub mod timing {
    pub use core_timing::*;
}

pub mod renderer {
    pub use core_renderer::Renderer;
}

mod window {
    pub use core_window::*;
}

mod sim_managers;
mod windowing;
