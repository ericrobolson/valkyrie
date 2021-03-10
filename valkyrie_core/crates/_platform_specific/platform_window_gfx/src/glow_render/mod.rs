use core_data_structures::queue::Queue;
use core_math::{Mat4, Vec3};
use core_renderer::{BackendRenderer, Renderer};
use glow::*;
use glutin::{ContextWrapper, PossiblyCurrent};

const UNIFORM_SCREEN_SIZE: &'static str = "u_screen_size";

const UNIFORM_VIEW_EYE: &'static str = "u_view_eye";
const UNIFORM_VIEW_TARGET: &'static str = "u_view_target";
const UNIFORM_VIEW_UP: &'static str = "u_view_up";
const UNIFORM_VIEW_MATRIX: &'static str = "u_view_matrix";

pub fn make(
    w: u32,
    h: u32,
    windowed_context: &ContextWrapper<PossiblyCurrent, glutin::window::Window>,
) -> impl BackendRenderer {
    // Verts are x,y,z
    let fullscreen_verts: Vec<f32> = vec![
        // first triangle
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, 0.5, 0.0, // top let
        // second triangle
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom let
        -0.5, 0.5, 0.0, // top let
    ];
    let fullscreen_verts: Vec<f32> = fullscreen_verts.iter().map(|v| v * 2.0).collect();

    let num_fullscreen_vert_attr = 3; // x,y,z
    let num_fullscreen_verts = fullscreen_verts.len() / num_fullscreen_vert_attr;

    // TODO: make safe
    let (gl, fullscreen_vertex_array, fullscreen_vbo, program) = unsafe {
        // Create context
        let gl = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });

        gl.enable(glow::FRAMEBUFFER_SRGB);

        let shader_version = "#version 330";

        // Create fullscreen triangles
        let fullscreen_vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(fullscreen_vertex_array));

        let fullscreen_vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(fullscreen_vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            core_conversions::slice_f32_to_u8(&fullscreen_verts),
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(
            0,
            num_fullscreen_vert_attr as i32,
            glow::FLOAT,
            false,
            (num_fullscreen_vert_attr * std::mem::size_of::<f32>()) as i32,
            0,
        );

        gl.enable_vertex_attrib_array(0);

        // Create program + link shaders
        let program = gl.create_program().expect("Cannot create program");

        let vertex_shader_source = std::str::from_utf8(include_bytes!("shader.vert")).unwrap();
        let fragment_shader_source = std::str::from_utf8(include_bytes!("shader.frag")).unwrap();

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

        //cleanup
        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program)); // Need to call before setting uniforms

        // Update UBOs
        resize_screen(program, &gl, w, h);
        set_camera(program, &gl, &core_renderer::Camera::default());

        // Return
        (gl, fullscreen_vertex_array, fullscreen_vbo, program)
    };

    GlowRenderer {
        gl,
        program,
        fullscreen_vertex_array,
        fullscreen_vbo,
        num_fullscreen_verts,
    }
}

/// Updates the given uniform
fn uniform<F>(gl: &Context, program: u32, name: &'static str, op: F)
where
    F: Fn(u32) -> (),
{
    unsafe {
        let u = gl.get_uniform_location(program, name);
        match u {
            Some(u) => {
                op(u);
            }
            None => {
                println!(
                    "Unable to find uniform {:?}. Likely it is unbound or unused.",
                    name
                );
            }
        }
    }
}

fn set_camera(program: u32, gl: &Context, camera: &core_renderer::Camera) {
    // TODO: cacheing for camera changes? E.g. if it's not different, don't change state

    unsafe {
        gl.use_program(Some(program)); // Need to call before setting uniforms

        // Update eye
        let (x, y, z) = camera.eye.into();
        uniform(gl, program, UNIFORM_VIEW_EYE, |u| {
            gl.uniform_3_f32(Some(&u), x, y, z)
        });

        // Update target
        let (x, y, z) = camera.target.into();
        uniform(gl, program, UNIFORM_VIEW_TARGET, |u| {
            gl.uniform_3_f32(Some(&u), x, y, z)
        });

        // Update up
        let (x, y, z) = camera.up.unwrap_or(Vec3::unit_y()).into();
        uniform(gl, program, UNIFORM_VIEW_UP, |u| {
            gl.uniform_3_f32(Some(&u), x, y, z)
        });

        // Update view matrix
        uniform(gl, program, UNIFORM_VIEW_MATRIX, |u| {
            gl.uniform_matrix_4_f32_slice(Some(&u), false, camera.to_mat4().as_slice())
        });
    }
}

fn resize_screen(program: u32, gl: &Context, w: u32, h: u32) {
    unsafe {
        gl.use_program(Some(program)); // Need to call before setting uniforms

        // Create screensize ubo
        uniform(gl, program, UNIFORM_SCREEN_SIZE, |u| {
            gl.uniform_2_f32(Some(&u), w as f32, h as f32);
        });

        // Resize viewport
        gl.viewport(0, 0, w as i32, h as i32);
    }
}

struct GlowRenderer {
    gl: Context,
    program: u32,
    fullscreen_vertex_array: u32,
    fullscreen_vbo: u32,
    num_fullscreen_verts: usize,
}
impl BackendRenderer for GlowRenderer {
    fn dispatch(&mut self) {
        unsafe {
            self.gl.use_program(Some(self.program));
            self.gl.clear_color(0.1, 0.2, 0.3, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);

            self.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.fullscreen_vbo));
            self.gl
                .bind_vertex_array(Some(self.fullscreen_vertex_array));

            self.gl
                .draw_arrays(glow::TRIANGLES, 0, self.num_fullscreen_verts as i32);
        }
    }

    fn set_render_pass(&mut self, commands: &Queue<core_renderer::RenderCommand>) {
        for command in commands.items() {
            match command {
                core_renderer::RenderCommand::UpdateCamera(camera) => {
                    set_camera(self.program, &self.gl, camera);
                }
            }
        }
    }

    fn resize(&mut self, w: u32, h: u32) {
        resize_screen(self.program, &self.gl, w, h);
    }
}

impl Drop for GlowRenderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            self.gl.delete_vertex_array(self.fullscreen_vertex_array);
            self.gl.delete_buffer(self.fullscreen_vbo);
        }
    }
}
