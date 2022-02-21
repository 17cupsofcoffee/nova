use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use glow::HasContext;

use crate::graphics::{Graphics, State};

use super::Color;

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
    pub color: Color,
}

impl Vertex {
    pub const fn new(pos: Vec2, uv: Vec2, color: Color) -> Vertex {
        Vertex { pos, uv, color }
    }
}

#[derive(Clone)]
pub struct MeshInner {
    state: Rc<State>,

    pub(crate) vertex_buffer: glow::Buffer,
    // vertex_count: usize,
    pub(crate) index_buffer: glow::Buffer,
    // index_count: usize,
}

impl Drop for MeshInner {
    fn drop(&mut self) {
        unsafe {
            self.state.gl.delete_buffer(self.vertex_buffer);

            if self.state.current_vertex_buffer.get() == Some(self.vertex_buffer) {
                self.state.current_vertex_buffer.set(None);
            }

            self.state.gl.delete_buffer(self.index_buffer);

            if self.state.current_index_buffer.get() == Some(self.index_buffer) {
                self.state.current_index_buffer.set(None);
            }
        }
    }
}

pub struct Mesh {
    pub(crate) inner: Rc<MeshInner>,
}

impl Mesh {
    pub fn new(gfx: &Graphics, vertex_count: usize, index_count: usize) -> Mesh {
        unsafe {
            let vertex_buffer = gfx.state.gl.create_buffer().unwrap();

            gfx.state.bind_vertex_buffer(Some(vertex_buffer));

            gfx.state.gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                (vertex_count * std::mem::size_of::<Vertex>()) as i32,
                glow::DYNAMIC_DRAW,
            );

            let index_buffer = gfx.state.gl.create_buffer().unwrap();

            gfx.state.bind_index_buffer(Some(index_buffer));

            gfx.state.gl.buffer_data_size(
                glow::ELEMENT_ARRAY_BUFFER,
                (index_count * std::mem::size_of::<u32>()) as i32,
                glow::STATIC_DRAW,
            );

            Mesh {
                inner: Rc::new(MeshInner {
                    state: Rc::clone(&gfx.state),

                    vertex_buffer,
                    // vertex_count,
                    index_buffer,
                    // index_count,
                }),
            }
        }
    }

    pub fn set_vertices(&self, data: &[Vertex]) {
        unsafe {
            self.inner
                .state
                .bind_vertex_buffer(Some(self.inner.vertex_buffer));

            self.inner.state.gl.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                0,
                bytemuck::cast_slice(data),
            );
        }
    }

    pub fn set_indices(&self, data: &[u32]) {
        unsafe {
            self.inner
                .state
                .bind_index_buffer(Some(self.inner.index_buffer));

            self.inner.state.gl.buffer_sub_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                0,
                bytemuck::cast_slice(data),
            );
        }
    }
}
