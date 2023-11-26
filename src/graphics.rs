mod batch;
mod canvas;
mod color;
mod mesh;
mod packer;
mod rectangle;
mod scaling;
mod shader;
mod text;
mod texture;

use std::cell::Cell;
use std::rc::Rc;

use glam::Mat4;
use glow::{Context, HasContext};

pub use batch::*;
pub use canvas::*;
pub use color::*;
pub use mesh::*;
pub use rectangle::*;
pub use scaling::*;
pub use shader::*;
pub use text::*;
pub use texture::*;

use crate::window::Window;

struct State {
    gl: Context,

    current_vertex_buffer: Cell<Option<glow::Buffer>>,
    current_index_buffer: Cell<Option<glow::Buffer>>,
    current_shader: Cell<Option<glow::Program>>,
    current_texture: Cell<Option<glow::Texture>>,
    current_canvas: Cell<Option<glow::Framebuffer>>,
}

#[derive(Clone)]
pub struct Graphics {
    state: Rc<State>,
}

impl Graphics {
    pub fn new(window: &mut Window) -> Graphics {
        let gl = window.load_gl();

        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            gl.enable(glow::CULL_FACE);
            gl.enable(glow::BLEND);
            gl.blend_func_separate(
                glow::ONE,
                glow::ONE_MINUS_SRC_ALPHA,
                glow::ONE,
                glow::ONE_MINUS_SRC_ALPHA,
            );

            println!("OpenGL Version: {}", gl.get_parameter_string(glow::VERSION));
            println!("Renderer: {}", gl.get_parameter_string(glow::RENDERER));
        }

        Graphics {
            state: Rc::new(State {
                gl,
                current_vertex_buffer: Cell::new(None),
                current_index_buffer: Cell::new(None),
                current_shader: Cell::new(None),
                current_texture: Cell::new(None),
                current_canvas: Cell::new(None),
            }),
        }
    }

    pub fn draw<T>(&self, pass: RenderPass<'_, T>)
    where
        T: Target,
    {
        unsafe {
            pass.target.bind(self);

            if let Some(color) = pass.clear_color {
                self.state
                    .gl
                    .clear_color(color.r, color.g, color.b, color.a);

                self.state.gl.clear(glow::COLOR_BUFFER_BIT);
            }

            self.bind_vertex_buffer(Some(pass.mesh.raw.vertex_buffer));
            self.bind_index_buffer(Some(pass.mesh.raw.index_buffer));
            self.bind_shader(Some(pass.shader.raw.id));
            self.bind_texture(Some(pass.texture.raw.id));

            let proj = self
                .state
                .gl
                .get_uniform_location(pass.shader.raw.id, "u_projection")
                .unwrap();

            let (target_width, target_height) = pass.target.size();

            self.state.gl.uniform_matrix_4_f32_slice(
                Some(&proj),
                false,
                Mat4::orthographic_rh_gl(
                    0.0,
                    target_width as f32,
                    if T::FLIPPED {
                        0.0
                    } else {
                        target_height as f32
                    },
                    if T::FLIPPED {
                        target_height as f32
                    } else {
                        0.0
                    },
                    -1.0,
                    1.0,
                )
                .as_ref(),
            );

            self.state.gl.viewport(0, 0, target_width, target_height);

            self.state.gl.draw_elements(
                glow::TRIANGLES,
                pass.index_count as i32,
                glow::UNSIGNED_INT,
                (pass.index_start * std::mem::size_of::<u32>()) as i32,
            );
        }
    }

    pub fn bind_vertex_buffer(&self, buffer: Option<glow::Buffer>) {
        unsafe {
            if self.state.current_vertex_buffer.get() != buffer {
                self.state.gl.bind_buffer(glow::ARRAY_BUFFER, buffer);

                if buffer.is_some() {
                    // TODO: If I ever want to use something other than `Vertex` in a buffer
                    // I'll need to rethink this code, but it's fine for now.

                    self.state.gl.vertex_attrib_pointer_f32(
                        0,
                        2,
                        glow::FLOAT,
                        false,
                        std::mem::size_of::<Vertex>() as i32,
                        0,
                    );

                    self.state.gl.vertex_attrib_pointer_f32(
                        1,
                        2,
                        glow::FLOAT,
                        false,
                        std::mem::size_of::<Vertex>() as i32,
                        8,
                    );

                    self.state.gl.vertex_attrib_pointer_f32(
                        2,
                        4,
                        glow::FLOAT,
                        false,
                        std::mem::size_of::<Vertex>() as i32,
                        16,
                    );

                    self.state.gl.enable_vertex_attrib_array(0);
                    self.state.gl.enable_vertex_attrib_array(1);
                    self.state.gl.enable_vertex_attrib_array(2);
                } else {
                    self.state.gl.disable_vertex_attrib_array(0);
                    self.state.gl.disable_vertex_attrib_array(1);
                    self.state.gl.disable_vertex_attrib_array(2);
                }

                self.state.current_vertex_buffer.set(buffer);
            }
        }
    }

    pub fn bind_index_buffer(&self, buffer: Option<glow::Buffer>) {
        unsafe {
            if self.state.current_index_buffer.get() != buffer {
                self.state
                    .gl
                    .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, buffer);
                self.state.current_index_buffer.set(buffer);
            }
        }
    }

    pub fn bind_shader(&self, shader: Option<glow::Program>) {
        unsafe {
            if self.state.current_shader.get() != shader {
                self.state.gl.use_program(shader);
                self.state.current_shader.set(shader);
            }
        }
    }

    pub fn bind_texture(&self, texture: Option<glow::Texture>) {
        unsafe {
            if self.state.current_texture.get() != texture {
                self.state.gl.active_texture(glow::TEXTURE0);
                self.state.gl.bind_texture(glow::TEXTURE_2D, texture);
                self.state.current_texture.set(texture);
            }
        }
    }

    pub fn bind_canvas(&self, canvas: Option<glow::Framebuffer>) {
        unsafe {
            if self.state.current_canvas.get() != canvas {
                self.state.gl.bind_framebuffer(glow::FRAMEBUFFER, canvas);
                self.state.current_canvas.set(canvas);
            }
        }
    }
}

pub trait Target {
    const FLIPPED: bool;

    fn bind(&self, gfx: &Graphics);
    fn size(&self) -> (i32, i32);
}

impl Target for Window {
    const FLIPPED: bool = false;

    fn bind(&self, gfx: &Graphics) {
        gfx.bind_canvas(None);

        unsafe {
            gfx.state.gl.front_face(glow::CCW);
        }
    }

    fn size(&self) -> (i32, i32) {
        let (width, height) = self.size();
        (width as i32, height as i32)
    }
}

impl Target for Canvas {
    const FLIPPED: bool = true;

    fn bind(&self, gfx: &Graphics) {
        gfx.bind_canvas(Some(self.raw.id));

        unsafe {
            gfx.state.gl.front_face(glow::CW);
        }
    }

    fn size(&self) -> (i32, i32) {
        self.size()
    }
}

pub struct RenderPass<'a, T> {
    pub target: &'a T,

    pub mesh: &'a Mesh,
    pub texture: &'a Texture,
    pub shader: &'a Shader,

    pub index_start: usize,
    pub index_count: usize,

    pub clear_color: Option<Color>,
}
