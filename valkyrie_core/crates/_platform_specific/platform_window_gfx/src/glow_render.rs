use core_data_structures::queue::Queue;
use core_renderer::{BackendRenderer, Renderer};
use glow::*;
use glutin::{ContextWrapper, PossiblyCurrent};

const VERT_SHADER: &'static str = r#"
            layout (location = 0) in vec3 aPos;

            void main()
            {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
            "#;

const FRAG_SHADER: &'static str = r#"
            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            } "#;

pub fn make(
    windowed_context: &ContextWrapper<PossiblyCurrent, glutin::window::Window>,
) -> impl BackendRenderer {
    // TODO: make safe
    let (gl, vertex_array, vbo, program) = unsafe {
        let gl = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });

        let shader_version = "#version 330";

        let verts: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            core_conversions::slice_f32_to_u8(&verts),
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            3 * std::mem::size_of::<f32>() as i32,
            0,
        );

        gl.enable_vertex_attrib_array(0);

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (VERT_SHADER, FRAG_SHADER);

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        (gl, vertex_array, vbo, program)
    };

    GlowRenderer {
        gl,
        program,
        vertex_array,
        vbo,
    }
}

struct GlowRenderer {
    gl: Context,
    program: u32,
    vertex_array: u32,
    vbo: u32,
}
impl BackendRenderer for GlowRenderer {
    fn dispatch(&mut self) {
        unsafe {
            self.gl.use_program(Some(self.program));
            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);

            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.bind_vertex_array(Some(self.vertex_array));

            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }

    fn set_render_pass(&mut self, commands: &Queue<core_renderer::RenderCommand>) {
        for command in commands.items() {
            match command {
                core_renderer::RenderCommand::UpdateCamera => {
                    println!("update camera");
                }
            }
        }
    }
}

impl Drop for GlowRenderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            self.gl.delete_vertex_array(self.vertex_array);
            self.gl.delete_buffer(self.vbo);
        }
    }
}
