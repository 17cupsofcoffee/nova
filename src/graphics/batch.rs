use glam::{Mat3, Vec2};

use crate::graphics::{
    Color, Graphics, Mesh, Rectangle, RenderPass, RichText, Shader, SpriteFont, Target,
    TextSection, Texture, Vertex,
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

        // TODO: This doesn't handle batches that are larger than MAX_VERTICES
        // very well
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

    pub fn text(
        &mut self,
        font: &SpriteFont,
        position: Vec2,
        text: &str,
        max_chars: Option<usize>,
    ) {
        let mut state = TextState::new(Vec2::new(0.0, font.ascent() + font.descent()));
        self.text_inner(font, position, text, max_chars, &mut state)
    }

    pub fn rich_text(
        &mut self,
        font: &SpriteFont,
        position: Vec2,
        text: &RichText,
        max_chars: Option<usize>,
    ) {
        let mut state = TextState::new(Vec2::new(0.0, font.ascent() + font.descent()));

        for section in &text.sections {
            match section {
                TextSection::String(string) => {
                    self.text_inner(font, position, string, max_chars, &mut state)
                }

                TextSection::ChangeColor(new_color) => {
                    state.color = *new_color;
                }
            }
        }
    }

    fn text_inner(
        &mut self,
        font: &SpriteFont,
        position: Vec2,
        text: &str,
        max_chars: Option<usize>,
        state: &mut TextState,
    ) {
        for ch in text.chars() {
            state.chars += 1;

            if max_chars.map(|m| state.chars > m).unwrap_or(false) {
                return;
            }

            if ch.is_control() {
                if ch == '\n' {
                    state.offset.x = 0.0;
                    state.offset.y += font.line_height();
                }

                continue;
            }

            if let Some(glyph) = font.glyph(ch) {
                if let Some(kerning) = state.last_char.and_then(|l| font.kerning(l, ch)) {
                    state.offset.x += kerning;
                }

                self.texture_region(
                    font.texture(),
                    (position + state.offset + glyph.offset).floor(),
                    glyph.uv,
                    DrawParams::new().color(state.color),
                );

                state.offset.x += glyph.advance;
                state.last_char = Some(ch);
            }
        }
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

        self.vertices.extend_from_slice(&[
            Vertex::new(tl, source.top_left(), params.color),
            Vertex::new(bl, source.bottom_left(), params.color),
            Vertex::new(br, source.bottom_right(), params.color),
            Vertex::new(tr, source.top_right(), params.color),
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

struct TextState {
    offset: Vec2,
    last_char: Option<char>,
    chars: usize,
    color: Color,
}

impl TextState {
    fn new(offset: Vec2) -> TextState {
        TextState {
            offset,
            last_char: None,
            chars: 0,
            color: Color::WHITE,
        }
    }
}
