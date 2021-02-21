/// Common, generic data structures.
#[cfg(feature = "data_structures")]
pub mod data_structures {
    pub use core_data_structures::*;
}

/// Entity-Component-System module.
#[cfg(feature = "ecs")]
pub mod ecs {
    pub use core_ecs::*;
}

/// Module for creating client and server applications.
#[cfg(feature = "application")]
pub mod application {
    pub use crate::core_application::*;
}
#[cfg(feature = "application")]
mod core_application;
#[cfg(feature = "application")]
mod core_sim_renderable;

/// Fixed timestep simulation
#[cfg(feature = "simulation")]
pub mod simulation {
    pub use crate::core_sim::*;
}
#[cfg(feature = "simulation")]
mod core_sim;

/// File input-output
#[cfg(feature = "file_io")]
pub mod file_io {
    pub use core_file_io::*;
}

/// Threading.
#[cfg(feature = "threading")]
pub mod threading {
    pub use core_threading::*;
}

/// Timing functionality
#[cfg(feature = "timing")]
pub mod timing {
    pub use core_timing::*;
}

#[cfg(feature = "renderer")]
pub mod renderer {
    pub use core_renderer::Renderer;
}

#[cfg(feature = "window")]
mod window {
    pub use core_window::*;
}
#[cfg(feature = "window")]
mod windowing;
