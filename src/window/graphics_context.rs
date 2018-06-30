use super::gl;
use super::gl::types::*;

use super::cgmath::prelude::*;
use super::cgmath::Matrix4;
use super::glutin::{GlContext, GlWindow};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    uniform mat4 scale;
    uniform mat4 translate;
    void main() {
       gl_Position = translate * scale * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

// TODO: uniform colour.
const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub struct GraphicsContext {
    shader_program: GLuint,
    vao: GLuint,
}

impl GraphicsContext {
    pub fn new() -> Self {
        GraphicsContext {
            shader_program: 0,
            vao: 0,
        }
    }

    pub fn init(&mut self, gl_window: &GlWindow) -> Result<(), String> {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        unsafe {
            // NOTE: these will be used a number of times.
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            info_log.set_len(512 - 1);
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(format!(
                    "vertex shader compilation failed: {}",
                    str::from_utf8(&info_log).unwrap()
                ));
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(format!(
                    "fragment shader compilation failed: {}",
                    str::from_utf8(&info_log).unwrap()
                ));
            }

            self.shader_program = gl::CreateProgram();
            gl::AttachShader(self.shader_program, vertex_shader);
            gl::AttachShader(self.shader_program, fragment_shader);
            gl::LinkProgram(self.shader_program);
            gl::GetProgramiv(self.shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    self.shader_program,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                return Err(format!(
                    "shader program compilation failed: {}",
                    str::from_utf8(&info_log).unwrap()
                ));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            // Using vertices and indices a square is drawn that covers the entire screen.
            let vertices: [f32; 12] = [
                // top left
                -1.0,
                -1.0,
                0.0,
                // top right
                1.0,
                -1.0,
                0.0,
                // bottom right
                1.0,
                1.0,
                0.0,
                // bottom left
                -1.0,
                1.0,
                0.0,
            ];
            let indices = [
                // first triangle
                0,
                1,
                2,
                // second triangle
                2,
                3,
                0,
            ];
            let (mut vbo, mut ebo) = (0, 0);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);
        }

        Ok(())
    }

    pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    // TODO: color.
    pub fn draw_square_with_scale_translation(&self, scale: Matrix4<f32>, translate: Matrix4<f32>) {
        unsafe {
            gl::UseProgram(self.shader_program);

            let scale_square = gl::GetUniformLocation(
                self.shader_program,
                CString::new("scale").unwrap().as_ptr(),
            );
            gl::UniformMatrix4fv(scale_square, 1, gl::FALSE, scale.as_ptr());
            let translate_square = gl::GetUniformLocation(
                self.shader_program,
                CString::new("translate").unwrap().as_ptr(),
            );
            gl::UniformMatrix4fv(translate_square, 1, gl::FALSE, translate.as_ptr());

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
