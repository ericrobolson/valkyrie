use core::wingfx;

const TITLE: &'static str = "SDF Walking Sim";

fn main() -> Result<(), String> {
    // TODO: move this to the engine. Would make it simpler IMO.
    let (mut window, mut renderer) = wingfx::WinGfxBuilder::new(TITLE, wingfx::BackendType::Opengl)
        .with_min_size(1080, 1920)
        .build()
        .unwrap();

    let main_loop = move |input: Option<wingfx::WindowInput>| loop {
        println!("Executing main LOOPIN!");
    };

    window.execute(main_loop);

    Ok(())
}
