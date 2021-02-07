use gl::types::*;
use std::{
    convert::TryInto,
    ffi::{c_void, CString},
    mem, ptr, str,
};

pub struct GlContext {
    program: u32,
    fs: u32,
    vs: u32,
    vbo: u32,
    vao: u32,
}

impl GlContext {
    pub fn draw(&mut self) {
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        // gl_window.swap_buffers().unwrap();
    }

    pub fn new() -> Self {
        /*
        // Bind
        unsafe {
            let ptr = match window.raw_window_handle() {
                RawWindowHandle::Windows(handle) => handle.hwnd,
                raw_window_handle::RawWindowHandle::__NonExhaustiveDoNotUse(_) => {
                    todo!()
                }
            };
            gl::load_with(|symbol| ptr);
            println!("HERE!");

            println!("{:?}", gl::GetString(gl::VERSION));

            println!("HERE!");
        }
        */
        println!("HERE!");
        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        println!("HERE!");

        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);

        println!("HERE!");

        let program = link_program(vs, fs);

        let mut vao = 0;
        let mut vbo = 0;
        println!("HERE!");

        unsafe {
            // VAO

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // VBO
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&VERTEX_DATA[0]),
                gl::STATIC_DRAW,
            );

            // shader program
            gl::UseProgram(program);
            gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

            // specify layout of vertex data
            let pos_attr =
                gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            //
        }

        println!("HERE!");
        Self {
            vs,
            fs,
            program,
            vao,
            vbo,
        }
    }
}

impl Drop for GlContext {
    fn drop(&mut self) {
        // Cleanup
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

// Shader sources
static VS_SRC: &'static str = "
#version 150
in vec2 position;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";

static FS_SRC: &'static str = "
#version 150
out vec4 out_color;
void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";
fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // sub 1 to skip trailing null char
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );

            panic!(
                "{:?}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }

        program
    }
}
