use std::rc::Rc;

use glow::HasContext;

use crate::graphics::{Graphics, State};

pub const DEFAULT_VERTEX_SHADER: &str = "
#version 150

in vec2 a_pos;
in vec2 a_uv;
in vec4 a_color;

uniform mat4 u_projection;

out vec2 v_uv;
out vec4 v_color;

void main() {
    v_uv = a_uv;
    v_color = a_color;

    gl_Position = u_projection * vec4(a_pos, 0.0, 1.0);
}
";

pub const DEFAULT_FRAGMENT_SHADER: &str = "
#version 150

in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_texture;

out vec4 o_color;

void main() {
    o_color = texture(u_texture, v_uv) * v_color;
}
";

#[derive(Clone)]
pub struct Shader {
    pub(crate) raw: Rc<RawShader>,
}

impl Shader {
    pub fn from_str(gfx: &Graphics, vertex_src: &str, fragment_src: &str) -> Shader {
        let raw = RawShader::new(gfx, vertex_src, fragment_src);

        Shader { raw: Rc::new(raw) }
    }
}

pub struct RawShader {
    state: Rc<State>,
    pub(crate) id: glow::Program,
}

impl RawShader {
    pub fn new(gfx: &Graphics, vertex_src: &str, fragment_src: &str) -> RawShader {
        unsafe {
            let program = gfx.state.gl.create_program().unwrap();

            gfx.state.gl.bind_attrib_location(program, 0, "a_pos");
            gfx.state.gl.bind_attrib_location(program, 1, "a_uv");

            let vertex_shader = gfx.state.gl.create_shader(glow::VERTEX_SHADER).unwrap();

            gfx.state.gl.shader_source(vertex_shader, vertex_src);
            gfx.state.gl.compile_shader(vertex_shader);
            gfx.state.gl.attach_shader(program, vertex_shader);

            if !gfx.state.gl.get_shader_compile_status(vertex_shader) {
                panic!("{}", gfx.state.gl.get_shader_info_log(vertex_shader));
            }

            let fragment_shader = gfx.state.gl.create_shader(glow::FRAGMENT_SHADER).unwrap();

            gfx.state.gl.shader_source(fragment_shader, fragment_src);
            gfx.state.gl.compile_shader(fragment_shader);
            gfx.state.gl.attach_shader(program, fragment_shader);

            if !gfx.state.gl.get_shader_compile_status(fragment_shader) {
                panic!("{}", gfx.state.gl.get_shader_info_log(fragment_shader));
            }

            gfx.state.gl.link_program(program);

            if !gfx.state.gl.get_program_link_status(program) {
                panic!("{}", gfx.state.gl.get_program_info_log(program));
            }

            gfx.state.gl.delete_shader(vertex_shader);
            gfx.state.gl.delete_shader(fragment_shader);

            gfx.bind_shader(Some(program));

            let sampler = gfx
                .state
                .gl
                .get_uniform_location(program, "u_texture")
                .unwrap();

            gfx.state.gl.uniform_1_i32(Some(&sampler), 0);

            RawShader {
                state: Rc::clone(&gfx.state),
                id: program,
            }
        }
    }
}

impl Drop for RawShader {
    fn drop(&mut self) {
        unsafe {
            self.state.gl.delete_program(self.id);

            if self.state.current_shader.get() == Some(self.id) {
                self.state.current_shader.set(None);
            }
        }
    }
}
