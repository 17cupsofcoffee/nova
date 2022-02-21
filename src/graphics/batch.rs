use glam::Vec2;

use crate::graphics::{
    Graphics, Mesh, Rectangle, RenderPass, Shader, Target, Texture, Transform, Vertex,
};

use super::{Color, SpriteFont};

const MAX_SPRITES: usize = 2048;
const MAX_VERTICES: usize = MAX_SPRITES * 4; // Cannot be greater than 32767!
const MAX_INDICES: usize = MAX_SPRITES * 6;
const INDEX_ARRAY: [u32; 6] = [0, 1, 2, 2, 3, 0];

const VERTEX_SHADER: &str = "
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

const FRAGMENT_SHADER: &str = "
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
struct Batch {
    pub indices: usize,
    pub texture: Option<Texture>,
    pub shader: Option<Shader>,
}

pub struct Batcher {
    mesh: Mesh,
    default_texture: Texture,
    default_shader: Shader,

    vertices: Vec<Vertex>,
    batches: Vec<Batch>,
}

impl Batcher {
    pub fn new(gfx: &Graphics) -> Batcher {
        let mesh = Mesh::new(gfx, MAX_VERTICES, MAX_INDICES);

        let indices: Vec<u32> = INDEX_ARRAY
            .iter()
            .cycle()
            .take(MAX_INDICES)
            .enumerate()
            .map(|(i, vertex)| vertex + i as u32 / 6 * 4)
            .collect();

        mesh.set_indices(&indices);

        let default_shader = Shader::new(gfx, VERTEX_SHADER, FRAGMENT_SHADER);
        let default_texture = Texture::new(gfx, 1, 1);

        default_texture.set_data(&[255, 255, 255, 255]);

        Batcher {
            mesh,

            vertices: Vec::new(),
            batches: vec![Batch {
                indices: 0,
                texture: None,
                shader: None,
            }],
            default_texture,
            default_shader,
        }
    }

    pub fn draw<'a, T>(&mut self, gfx: &Graphics, target: T)
    where
        T: Into<Target<'a>>,
    {
        let target = target.into();

        self.mesh.set_vertices(&self.vertices);

        let mut index_start = 0;

        for batch in &self.batches {
            let texture = batch.texture.as_ref().unwrap_or(&self.default_texture);
            let shader = batch.shader.as_ref().unwrap_or(&self.default_shader);

            gfx.draw(RenderPass {
                target,
                mesh: &self.mesh,
                texture,
                shader,
                index_start,
                index_count: batch.indices,
            });

            index_start += batch.indices;
        }

        self.batches.clear();
        self.vertices.clear();

        self.batches.push(Batch {
            indices: 0,
            texture: None,
            shader: None,
        });
    }

    pub fn rect(&mut self, rect: Rectangle, color: Color) {
        let mut batch = self.batches.last_mut().expect("should always exist");

        if batch.indices != 0 && batch.texture.is_some() {
            let mut new_batch = batch.clone();
            new_batch.indices = 0;

            self.batches.push(new_batch);

            batch = self.batches.last_mut().expect("should always exist");
        }

        batch.texture = None;

        self.vertices.extend_from_slice(&[
            Vertex::new(Vec2::new(rect.x, rect.y), Vec2::ZERO, color),
            Vertex::new(Vec2::new(rect.x, rect.y + rect.height), Vec2::ZERO, color),
            Vertex::new(
                Vec2::new(rect.x + rect.width, rect.y + rect.height),
                Vec2::ZERO,
                color,
            ),
            Vertex::new(Vec2::new(rect.x + rect.width, rect.y), Vec2::ZERO, color),
        ]);

        batch.indices += 6;
    }

    pub fn texture(&mut self, texture: &Texture, transform: impl Into<Transform>) {
        self.subtexture(
            texture,
            Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
            transform,
        )
    }

    pub fn subtexture(
        &mut self,
        texture: &Texture,
        region: Rectangle,
        transform: impl Into<Transform>,
    ) {
        let mut batch = self.batches.last_mut().expect("should always exist");

        if batch.indices != 0 && batch.texture.as_ref() != Some(texture) {
            let mut new_batch = batch.clone();
            new_batch.indices = 0;

            self.batches.push(new_batch);

            batch = self.batches.last_mut().expect("should always exist");
        }

        batch.texture = Some(texture.clone());

        let transform = transform.into();
        let matrix = transform.to_matrix();

        let tl = matrix.transform_point2(Vec2::ZERO);
        let bl = matrix.transform_point2(Vec2::new(0.0, region.height));
        let br = matrix.transform_point2(Vec2::new(region.width, region.height));
        let tr = matrix.transform_point2(Vec2::new(region.width, 0.0));

        let u1 = region.x / texture.width() as f32;
        let v1 = region.y / texture.height() as f32;
        let u2 = region.right() / texture.width() as f32;
        let v2 = region.bottom() / texture.height() as f32;

        self.vertices.extend_from_slice(&[
            Vertex::new(tl, Vec2::new(u1, v1), Color::WHITE),
            Vertex::new(bl, Vec2::new(u1, v2), Color::WHITE),
            Vertex::new(br, Vec2::new(u2, v2), Color::WHITE),
            Vertex::new(tr, Vec2::new(u2, v1), Color::WHITE),
        ]);

        batch.indices += 6;
    }

    // TODO: This API kinda sucks and duplicates a load of code, figure out a nicer one
    pub fn subtexture_rect(
        &mut self,
        texture: &Texture,
        src: Rectangle,
        dest: Rectangle,
        color: Color,
    ) {
        let mut batch = self.batches.last_mut().expect("should always exist");

        if batch.indices != 0 && batch.texture.as_ref() != Some(texture) {
            let mut new_batch = batch.clone();
            new_batch.indices = 0;

            self.batches.push(new_batch);

            batch = self.batches.last_mut().expect("should always exist");
        }

        batch.texture = Some(texture.clone());

        let tl = dest.top_left();
        let bl = dest.bottom_left();
        let br = dest.bottom_right();
        let tr = dest.top_right();

        let u1 = src.x / texture.width() as f32;
        let v1 = src.y / texture.height() as f32;
        let u2 = src.right() / texture.width() as f32;
        let v2 = src.bottom() / texture.height() as f32;

        self.vertices.extend_from_slice(&[
            Vertex::new(tl, Vec2::new(u1, v1), color),
            Vertex::new(bl, Vec2::new(u1, v2), color),
            Vertex::new(br, Vec2::new(u2, v2), color),
            Vertex::new(tr, Vec2::new(u2, v1), color),
        ]);

        batch.indices += 6;
    }

    pub fn text(&mut self, font: &SpriteFont, text: &str, position: Vec2) {
        let mut offset = Vec2::new(0.0, font.ascent() + font.descent());
        let mut last_char = None;

        for ch in text.chars() {
            if ch.is_control() {
                if ch == '\n' {
                    offset.x = 0.0;
                    offset.y += font.line_height();
                }

                continue;
            }

            if let Some(glyph) = font.glyph(ch) {
                if let Some(kerning) = last_char.and_then(|l| font.kerning(l, ch)) {
                    offset.x += kerning;
                }

                self.subtexture(
                    font.texture(),
                    glyph.uv,
                    (position + offset + glyph.offset).floor(),
                );

                offset.x += glyph.advance;

                last_char = Some(ch);
            }
        }
    }
}
