use glam::{Mat3, Vec2};

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

struct Batch {
    indices: usize,
    texture: Option<Texture>,
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

            gfx.draw(RenderPass {
                target,
                mesh: &self.mesh,
                texture,
                shader: &self.default_shader,
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
        });
    }

    pub fn rect(&mut self, rect: Rectangle, color: Color) {
        self.push_sprite(None, Rectangle::ZERO, rect, color, Vec2::ZERO, 0.0);
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
        let transform = transform.into();

        self.push_sprite(
            Some(texture),
            Rectangle::new(
                region.x / texture.width() as f32,
                region.y / texture.height() as f32,
                region.width / texture.width() as f32,
                region.height / texture.height() as f32,
            ),
            Rectangle::new(
                transform.position.x,
                transform.position.y,
                region.width * transform.scale.x,
                region.height * transform.scale.y,
            ),
            Color::WHITE,
            transform.origin,
            transform.rotation,
        );
    }

    // TODO: This API kinda sucks and duplicates a load of code, figure out a nicer one
    pub fn subtexture_rect(
        &mut self,
        texture: &Texture,
        src: Rectangle,
        dest: Rectangle,
        color: Color,
    ) {
        self.push_sprite(
            Some(texture),
            Rectangle::new(
                src.x / texture.width() as f32,
                src.y / texture.height() as f32,
                src.width / texture.width() as f32,
                src.height / texture.height() as f32,
            ),
            dest,
            color,
            Vec2::ZERO,
            0.0,
        );
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

    fn push_sprite(
        &mut self,
        texture: Option<&Texture>,
        source: Rectangle,
        dest: Rectangle,
        color: Color,
        origin: Vec2,
        rotation: f32,
    ) {
        // TODO: We could do this with trig if we wanted to optimize
        let transform = Mat3::from_translation(dest.top_left())
            * Mat3::from_rotation_z(rotation)
            * Mat3::from_translation(-origin);

        let tl = transform.transform_point2(Vec2::ZERO);
        let bl = transform.transform_point2(Vec2::new(0.0, dest.height));
        let br = transform.transform_point2(Vec2::new(dest.width, dest.height));
        let tr = transform.transform_point2(Vec2::new(dest.width, 0.0));

        self.vertices.extend_from_slice(&[
            Vertex::new(tl, source.top_left(), color),
            Vertex::new(bl, source.bottom_left(), color),
            Vertex::new(br, source.bottom_right(), color),
            Vertex::new(tr, source.top_right(), color),
        ]);

        let batch = self.batches.last_mut().expect("should always exist");

        if batch.indices == 0 {
            batch.indices = 6;
            batch.texture = texture.cloned();
        } else if batch.texture.as_ref() != texture {
            self.batches.push(Batch {
                indices: 6,
                texture: texture.cloned(),
            });
        } else {
            batch.indices += 6;
        }
    }
}
