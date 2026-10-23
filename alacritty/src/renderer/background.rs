use std::mem;
use std::time::Instant;

use crate::display::SizeInfo;
use crate::gl;
use crate::gl::types::*;
use crate::renderer::shader::{ShaderError, ShaderProgram, ShaderVersion};

const BACKGROUND_SHADER_V: &str = include_str!("../../res/background.v.glsl");

const GLSL3_PRELUDE: &str = "out vec4 glacrittyFragColor;\n";
const GLES2_PRELUDE: &str = "#ifdef GL_FRAGMENT_PRECISION_HIGH\nprecision highp \
                             float;\n#else\nprecision mediump float;\n#endif\n#define \
                             glacrittyFragColor gl_FragColor\n";

const UNIFORMS: &str =
    "uniform vec3 iResolution;\nuniform float iTime;\nuniform float glacrittyOpacity;\n#line 1\n";

const MAIN: &str = "\nvoid main() {\n    vec4 color;\n    mainImage(color, gl_FragCoord.xy);\n    \
                    glacrittyFragColor = vec4(color.rgb * glacrittyOpacity, \
                    glacrittyOpacity);\n}\n";

#[derive(Debug)]
pub struct BackgroundRenderer {
    program: ShaderProgram,
    vao: GLuint,
    vbo: GLuint,
    u_resolution: GLint,
    u_time: GLint,
    u_opacity: GLint,
    start: Instant,
}

impl BackgroundRenderer {
    pub fn new(
        shader_version: ShaderVersion,
        source: &str,
        start: Instant,
    ) -> Result<Self, ShaderError> {
        let prelude = match shader_version {
            ShaderVersion::Glsl3 => GLSL3_PRELUDE,
            ShaderVersion::Gles2 => GLES2_PRELUDE,
        };
        let fragment = [prelude, UNIFORMS, source, MAIN].concat();
        let program = ShaderProgram::new(shader_version, None, BACKGROUND_SHADER_V, &fragment)?;

        let uniform = |name: &[u8]| unsafe {
            gl::GetUniformLocation(program.id(), name.as_ptr() as *const _)
        };
        let u_resolution = uniform(b"iResolution\0");
        let u_time = uniform(b"iTime\0");
        let u_opacity = uniform(b"glacrittyOpacity\0");

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let vertices: [f32; 8] = [-1., -1., 1., -1., -1., 1., 1., 1.];
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                mem::size_of_val(&vertices) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Ok(Self { program, vao, vbo, u_resolution, u_time, u_opacity, start })
    }

    pub fn start(&self) -> Instant {
        self.start
    }

    /// The GLSL compiler strips uniforms the shader never reads, so this is exact.
    pub fn uses_time(&self) -> bool {
        self.u_time != -1
    }

    pub fn draw(&self, size_info: &SizeInfo, opacity: f32) {
        unsafe {
            gl::Viewport(0, 0, size_info.width() as i32, size_info.height() as i32);
            gl::Disable(gl::BLEND);
            gl::UseProgram(self.program.id());
            gl::Uniform3f(self.u_resolution, size_info.width(), size_info.height(), 1.);
            gl::Uniform1f(self.u_time, self.start.elapsed().as_secs_f32());
            gl::Uniform1f(self.u_opacity, opacity);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
            gl::Enable(gl::BLEND);
        }
    }
}

impl Drop for BackgroundRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
