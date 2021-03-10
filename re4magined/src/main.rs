// RE4magined
use valkyrie_core::application::*;
use valkyrie_core::math::*;
use valkyrie_core::renderer::*;

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

struct Re4magined {
    camera_pos: Vec3,
    up_held: bool,
    down_held: bool,
    left_held: bool,
    right_held: bool,
}

impl Simulation<Cfg, Msg> for Re4magined {
    fn new(config: Cfg) -> Self {
        Self {
            camera_pos: Vec3::new(0., 0., 5.),
            up_held: false,
            down_held: false,
            left_held: false,
            right_held: false,
        }
    }

    fn tick(&mut self, delta_t: std::time::Duration, messages: &[Input<Msg>]) -> ControlMessage {
        for msg in messages {
            match msg {
                Input::UserMsg(_) => {}
                Input::WindowMsg(msg) => match msg {
                    valkyrie_core::simulation::WindowMsg::Shutdown => {}
                    valkyrie_core::simulation::WindowMsg::RedrawRequested => {}
                    valkyrie_core::simulation::WindowMsg::Resize { w, h } => {}
                    valkyrie_core::simulation::WindowMsg::KeyPress(key) => match key {
                        valkyrie_core::simulation::KeyboardMsg::W => self.up_held = true,
                        valkyrie_core::simulation::KeyboardMsg::A => self.left_held = true,
                        valkyrie_core::simulation::KeyboardMsg::S => self.down_held = true,
                        valkyrie_core::simulation::KeyboardMsg::D => self.right_held = true,
                    },
                    valkyrie_core::simulation::WindowMsg::KeyRelease(key) => match key {
                        valkyrie_core::simulation::KeyboardMsg::W => self.up_held = false,
                        valkyrie_core::simulation::KeyboardMsg::A => self.left_held = false,
                        valkyrie_core::simulation::KeyboardMsg::S => self.down_held = false,
                        valkyrie_core::simulation::KeyboardMsg::D => self.right_held = false,
                    },
                },
            }
        }

        let velocity = {
            let mut v = Vec3::default();
            let speed = 0.1;

            if self.up_held {
                v.y += speed;
            }

            if self.down_held {
                v.y -= speed;
            }

            if self.right_held {
                v.x += speed;
            }

            if self.left_held {
                v.x -= speed;
            }

            v
        };

        self.camera_pos += velocity;

        ControlMessage::Ok
    }
}

impl Renderable for Re4magined {
    fn render(&self, renderer: &mut Renderer) {
        renderer
            .create_render_pass()
            .add(RenderCommand::UpdateCamera(Camera {
                eye: self.camera_pos,
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
