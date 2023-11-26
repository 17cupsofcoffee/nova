use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use glow::HasContext;

use crate::graphics::Graphics;

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

pub struct Mesh {
    pub(crate) raw: Rc<RawMesh>,
}

impl Mesh {
    pub fn new(gfx: &Graphics, vertex_count: usize, index_count: usize) -> Mesh {
        let raw = RawMesh::new(gfx, vertex_count, index_count);

        Mesh { raw: Rc::new(raw) }
    }

    pub fn set_vertices(&self, data: &[Vertex]) {
        self.raw.set_vertices(data);
    }

    pub fn set_indices(&self, data: &[u32]) {
        self.raw.set_indices(data);
    }
}

#[derive(Clone)]
pub struct RawMesh {
    pub(crate) gfx: Graphics,

    pub(crate) vertex_buffer: glow::Buffer,
    pub(crate) index_buffer: glow::Buffer,
}

impl RawMesh {
    pub fn new(gfx: &Graphics, vertex_count: usize, index_count: usize) -> RawMesh {
        unsafe {
            let vertex_buffer = gfx.state.gl.create_buffer().unwrap();

            gfx.bind_vertex_buffer(Some(vertex_buffer));

            gfx.state.gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                (vertex_count * std::mem::size_of::<Vertex>()) as i32,
                glow::DYNAMIC_DRAW,
            );

            let index_buffer = gfx.state.gl.create_buffer().unwrap();

            gfx.bind_index_buffer(Some(index_buffer));

            gfx.state.gl.buffer_data_size(
                glow::ELEMENT_ARRAY_BUFFER,
                (index_count * std::mem::size_of::<u32>()) as i32,
                glow::STATIC_DRAW,
            );

            RawMesh {
                gfx: gfx.clone(),

                vertex_buffer,
                index_buffer,
            }
        }
    }

    pub fn set_vertices(&self, data: &[Vertex]) {
        unsafe {
            self.gfx.bind_vertex_buffer(Some(self.vertex_buffer));

            self.gfx.state.gl.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                0,
                bytemuck::cast_slice(data),
            );
        }
    }

    pub fn set_indices(&self, data: &[u32]) {
        unsafe {
            self.gfx.bind_index_buffer(Some(self.index_buffer));

            self.gfx.state.gl.buffer_sub_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                0,
                bytemuck::cast_slice(data),
            );
        }
    }
}

impl Drop for RawMesh {
    fn drop(&mut self) {
        unsafe {
            self.gfx.state.gl.delete_buffer(self.vertex_buffer);

            if self.gfx.state.current_vertex_buffer.get() == Some(self.vertex_buffer) {
                self.gfx.state.current_vertex_buffer.set(None);
            }

            self.gfx.state.gl.delete_buffer(self.index_buffer);

            if self.gfx.state.current_index_buffer.get() == Some(self.index_buffer) {
                self.gfx.state.current_index_buffer.set(None);
            }
        }
    }
}
