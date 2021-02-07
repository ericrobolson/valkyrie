use core::wingfx;

const TITLE: &'static str = "SDF Walking Sim";

fn main() -> Result<(), String> {
    let (mut window, mut renderer) = wingfx::WinGfxBuilder::new(TITLE, wingfx::BackendType::Opengl)
        .with_min_size(1080, 1920)
        .build()
        .unwrap();

    Ok(())
}
