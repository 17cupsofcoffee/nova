use bytemuck::{Pod, Zeroable};
use glam::{Mat3, Vec2};

use crate::graphics::{
    Color, Graphics, Mesh, Rectangle, RenderPass, Shader, SpriteFont, Target, TextSegment, Texture,
    Vertex,
};

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

pub struct DrawParams {
    color: Color,
    origin: Vec2,
    scale: Vec2,
    rotation: f32,
}

impl DrawParams {
    pub fn new() -> DrawParams {
        DrawParams {
            color: Color::WHITE,
            origin: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn origin(mut self, origin: Vec2) -> Self {
        self.origin = origin;
        self
    }

    pub fn scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }
}

#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(C)]
struct Sprite {
    // The order of these fields matters, as it'll determine the
    // winding order of the quad.
    top_left: Vertex,
    bottom_left: Vertex,
    bottom_right: Vertex,
    top_right: Vertex,
}

struct Batch {
    sprites: usize,
    texture: Option<Texture>,
}

pub struct Batcher {
    mesh: Mesh,
    default_texture: Texture,
    default_shader: Shader,

    sprites: Vec<Sprite>,
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

        let default_shader = Shader::from_str(gfx, VERTEX_SHADER, FRAGMENT_SHADER);
        let default_texture = Texture::from_data(gfx, 1, 1, &[255, 255, 255, 255]);

        Batcher {
            mesh,

            sprites: Vec::new(),
            batches: vec![Batch {
                sprites: 0,
                texture: None,
            }],

            default_texture,
            default_shader,
        }
    }

    pub fn draw(&mut self, gfx: &Graphics, target: &impl Target) {
        let mut index = 0;

        for mut batch in self.batches.drain(..) {
            let texture = batch.texture.as_ref().unwrap_or(&self.default_texture);

            while batch.sprites > 0 {
                let num_sprites = usize::min(batch.sprites, MAX_SPRITES);

                let sprites = &self.sprites[index..index + num_sprites];
                let vertices = bytemuck::cast_slice(sprites);

                self.mesh.set_vertices(vertices);

                gfx.draw(RenderPass {
                    target,
                    mesh: &self.mesh,
                    texture,
                    shader: &self.default_shader,
                    index_start: 0,
                    index_count: num_sprites * 6,
                });

                index += num_sprites;
                batch.sprites -= num_sprites;
            }
        }

        self.sprites.clear();

        self.batches.push(Batch {
            sprites: 0,
            texture: None,
        });
    }

    pub fn rect(&mut self, rect: Rectangle, params: DrawParams) {
        self.push_sprite(None, Rectangle::ZERO, rect, params);
    }

    pub fn texture(&mut self, texture: &Texture, position: Vec2, params: DrawParams) {
        self.push_sprite(
            Some(texture),
            Rectangle::new(0.0, 0.0, 1.0, 1.0),
            Rectangle::new(
                position.x,
                position.y,
                texture.width() as f32,
                texture.height() as f32,
            ),
            params,
        );
    }

    pub fn texture_region(
        &mut self,
        texture: &Texture,
        position: Vec2,
        src: Rectangle,
        params: DrawParams,
    ) {
        self.push_sprite(
            Some(texture),
            Rectangle::new(
                src.x / texture.width() as f32,
                src.y / texture.height() as f32,
                src.width / texture.width() as f32,
                src.height / texture.height() as f32,
            ),
            Rectangle::new(position.x, position.y, src.width, src.height),
            params,
        );
    }

    pub fn texture_dest(
        &mut self,
        texture: &Texture,
        src: Rectangle,
        dest: Rectangle,
        params: DrawParams,
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
            params,
        );
    }

    pub fn text_segments(
        &mut self,
        font: &SpriteFont,
        position: Vec2,
        text: &[TextSegment<'_>],
        max_chars: Option<usize>,
    ) {
        let mut cursor = Vec2::new(0.0, font.ascent.floor());
        let mut last_char = None;
        let mut chars = 0;

        for segment in text {
            for ch in segment.content.chars() {
                chars += 1;

                if max_chars.map(|m| chars > m) == Some(true) {
                    return;
                }

                if ch.is_control() {
                    if ch == '\n' {
                        cursor.x = 0.0;
                        cursor.y += font.line_height().floor();
                    }

                    continue;
                }

                if let Some(glyph) = font.glyph(ch) {
                    if let Some(kerning) = last_char.and_then(|l| font.kerning(l, ch)) {
                        cursor.x += kerning;
                    }

                    if let Some(image) = &glyph.image {
                        self.texture_region(
                            font.texture(),
                            (position + cursor + image.offset).floor(),
                            image.uv,
                            DrawParams::new().color(segment.color),
                        );
                    }

                    cursor.x += glyph.advance;

                    last_char = Some(ch);
                }
            }
        }
    }

    pub fn text(
        &mut self,
        font: &SpriteFont,
        position: Vec2,
        text: &str,
        max_chars: Option<usize>,
    ) {
        self.text_segments(font, position, &[TextSegment::new(text)], max_chars)
    }

    fn push_sprite(
        &mut self,
        texture: Option<&Texture>,
        source: Rectangle,
        dest: Rectangle,
        params: DrawParams,
    ) {
        // TODO: We could do this with trig if we wanted to optimize
        let transform = Mat3::from_translation(dest.top_left())
            * Mat3::from_rotation_z(params.rotation)
            * Mat3::from_translation(-params.origin);

        let tl = transform.transform_point2(Vec2::ZERO);
        let bl = transform.transform_point2(Vec2::new(0.0, dest.height * params.scale.y));
        let br = transform.transform_point2(Vec2::new(
            dest.width * params.scale.x,
            dest.height * params.scale.y,
        ));
        let tr = transform.transform_point2(Vec2::new(dest.width * params.scale.x, 0.0));

        self.sprites.push(Sprite {
            top_left: Vertex::new(tl, source.top_left(), params.color),
            bottom_left: Vertex::new(bl, source.bottom_left(), params.color),
            bottom_right: Vertex::new(br, source.bottom_right(), params.color),
            top_right: Vertex::new(tr, source.top_right(), params.color),
        });

        let batch = self.batches.last_mut().expect("should always exist");

        if batch.sprites == 0 {
            batch.sprites = 1;
            batch.texture = texture.cloned();
        } else if batch.texture.as_ref() != texture {
            self.batches.push(Batch {
                sprites: 1,
                texture: texture.cloned(),
            });
        } else {
            batch.sprites += 1;
        }
    }
}
