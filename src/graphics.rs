mod batch;
mod canvas;
mod color;
mod mesh;
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

impl State {
    fn bind_vertex_buffer(&self, buffer: Option<glow::Buffer>) {
        unsafe {
            if self.current_vertex_buffer.get() != buffer {
                self.gl.bind_buffer(glow::ARRAY_BUFFER, buffer);
                self.current_vertex_buffer.set(buffer);
            }
        }
    }

    fn bind_index_buffer(&self, buffer: Option<glow::Buffer>) {
        unsafe {
            if self.current_index_buffer.get() != buffer {
                self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, buffer);
                self.current_index_buffer.set(buffer);
            }
        }
    }

    fn bind_shader(&self, shader: Option<glow::Program>) {
        unsafe {
            if self.current_shader.get() != shader {
                self.gl.use_program(shader);
                self.current_shader.set(shader);
            }
        }
    }

    fn bind_texture(&self, texture: Option<glow::Texture>) {
        unsafe {
            if self.current_texture.get() != texture {
                self.gl.active_texture(glow::TEXTURE0);
                self.gl.bind_texture(glow::TEXTURE_2D, texture);
                self.current_texture.set(texture);
            }
        }
    }

    fn bind_canvas(&self, canvas: Option<glow::Framebuffer>) {
        unsafe {
            if self.current_canvas.get() != canvas {
                self.gl.bind_framebuffer(glow::FRAMEBUFFER, canvas);
                self.current_canvas.set(canvas);
            }
        }
    }
}

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

            // gl.enable(glow::DEBUG_OUTPUT);
            // gl.enable(glow::DEBUG_OUTPUT_SYNCHRONOUS);
            // gl.debug_message_callback(|_source, ty, _id, severity, msg| {
            //     if severity == glow::DEBUG_SEVERITY_NOTIFICATION && ty == glow::DEBUG_TYPE_OTHER {
            //         return;
            //     }

            //     println!("{}", msg);
            // });

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

    pub fn clear<'a>(&self, target: impl Into<Target<'a>>, color: Color) {
        unsafe {
            match target.into() {
                Target::Window(_) => {
                    self.state.bind_canvas(None);
                }
                Target::Canvas(canvas) => {
                    self.state.bind_canvas(Some(canvas.id));
                }
            }

            self.state.gl.disable(glow::SCISSOR_TEST);
            self.state
                .gl
                .clear_color(color.r, color.g, color.b, color.a);
            self.state.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&self, pass: RenderPass<'_>) {
        unsafe {
            self.state
                .bind_vertex_buffer(Some(pass.mesh.inner.vertex_buffer));
            self.state
                .bind_index_buffer(Some(pass.mesh.inner.index_buffer));
            self.state.bind_shader(Some(pass.shader.inner.id));
            self.state.bind_texture(Some(pass.texture.inner.id));

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

            let proj = self
                .state
                .gl
                .get_uniform_location(pass.shader.inner.id, "u_projection")
                .unwrap();

            let (target_width, target_height, flipped) = match pass.target {
                Target::Window(window) => {
                    self.state.bind_canvas(None);
                    self.state.gl.front_face(glow::CCW);

                    let size = window.size();
                    (size.0 as i32, size.1 as i32, false)
                }

                Target::Canvas(canvas) => {
                    self.state.bind_canvas(Some(canvas.id));
                    self.state.gl.front_face(glow::CW);

                    let size = canvas.texture().size();
                    (size.0, size.1, true)
                }
            };

            self.state.gl.uniform_matrix_4_f32_slice(
                Some(&proj),
                false,
                Mat4::orthographic_rh_gl(
                    0.0,
                    target_width as f32,
                    if flipped { 0.0 } else { target_height as f32 },
                    if flipped { target_height as f32 } else { 0.0 },
                    -1.0,
                    1.0,
                )
                .as_ref(),
            );

            self.state
                .gl
                .viewport(0, 0, target_width as i32, target_height as i32);

            self.state.gl.draw_elements(
                glow::TRIANGLES,
                pass.index_count as i32,
                glow::UNSIGNED_INT,
                (pass.index_start * std::mem::size_of::<u32>()) as i32,
            );
        }
    }
}

#[derive(Copy, Clone)]
pub enum Target<'a> {
    Window(&'a Window),
    Canvas(&'a Canvas),
}

impl<'a> From<&'a Window> for Target<'a> {
    fn from(window: &'a Window) -> Target<'a> {
        Target::Window(window)
    }
}

impl<'a> From<&'a Canvas> for Target<'a> {
    fn from(canvas: &'a Canvas) -> Target<'a> {
        Target::Canvas(canvas)
    }
}

pub struct RenderPass<'a> {
    pub target: Target<'a>,

    pub mesh: &'a Mesh,
    pub texture: &'a Texture,
    pub shader: &'a Shader,

    pub index_start: usize,
    pub index_count: usize,
}
