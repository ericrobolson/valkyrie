// RE4magined
use valkyrie_core::application::*;
use valkyrie_core::math::*;
use valkyrie_core::renderer::*;

struct Re4magined {}

struct Cfg {}

impl Cfg {
    pub fn new() -> SimulationOptions<Self> {
        SimulationOptions {
            use_fixed_timestep: true,
            sim_hz: Some(60),
            cfg: Self {},
        }
    }
}

enum Msg {}

impl Simulation<Cfg, Msg> for Re4magined {
    fn new(config: Cfg) -> Self {
        Self {}
    }

    fn tick(&mut self, delta_t: std::time::Duration, messages: &[Input<Msg>]) -> ControlMessage {
        ControlMessage::Ok
    }
}

impl Renderable for Re4magined {
    fn render(&self, renderer: &mut Renderer) {
        renderer
            .create_render_pass()
            .add(RenderCommand::UpdateCamera(Camera {
                eye: Vec3::default(),
                target: Vec3::default(),
                up: None,
            }));
    }
}

fn main() {
    run_client::<Re4magined, Cfg, Msg>("Title", 1920, 1080, Cfg::new(), BackendType::Opengl)
        .unwrap();
    return;
    run_server::<Re4magined, Cfg, Msg>(SimulationOptions {
        use_fixed_timestep: true,
        sim_hz: Some(60),
        cfg: Cfg {},
    })
    .unwrap();
}
