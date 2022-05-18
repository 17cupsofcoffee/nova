use crate::graphics::{Graphics, IRectangle, Texture};

/// An individual shelf within the packed atlas, tracking how much space
/// is currently taken up.
#[derive(Copy, Clone, Debug)]
struct Shelf {
    current_x: i32,
    start_y: i32,
    height: i32,
}

/// Packs texture data into an atlas using a naive shelf-packing algorithm.
pub struct ShelfPacker {
    texture: Texture,
    shelves: Vec<Shelf>,
    next_y: i32,
}

impl ShelfPacker {
    /// Creates a new `ShelfPacker`.
    pub fn new(gfx: &Graphics, texture_width: i32, texture_height: i32) -> ShelfPacker {
        ShelfPacker {
            texture: Texture::empty(gfx, texture_width, texture_height),
            shelves: Vec::new(),
            next_y: 0,
        }
    }

    /// Consumes the packer, returning the generated texture.
    pub fn into_texture(self) -> Texture {
        self.texture
    }

    /// Tries to insert RGBA data into the atlas, and returns the position.
    ///
    /// If the data will not fit into the remaining space, `None` will be returned.
    pub fn insert(
        &mut self,
        data: &[u8],
        width: i32,
        height: i32,
        padding: i32,
    ) -> Option<IRectangle> {
        let padded_width = width + padding * 2;
        let padded_height = height + padding * 2;

        let space = self.find_space(padded_width, padded_height);

        if let Some(s) = space {
            self.texture
                .set_region(s.x + padding, s.y + padding, width, height, data);
        }

        space
    }

    /// Finds a space in the atlas that can fit a sprite of the specified width and height,
    /// and returns the position.
    ///
    /// If it would not fit into the remaining space, `None` will be returned.
    fn find_space(&mut self, source_width: i32, source_height: i32) -> Option<IRectangle> {
        let texture_width = self.texture.width();
        let texture_height = self.texture.height();

        self.shelves
            .iter_mut()
            .find(|shelf| {
                shelf.height >= source_height && texture_width - shelf.current_x >= source_width
            })
            .map(|shelf| {
                // Use existing shelf:
                let position = (shelf.current_x, shelf.start_y);
                shelf.current_x += source_width;

                IRectangle::new(position.0, position.1, source_width, source_height)
            })
            .or_else(|| {
                if self.next_y + source_height < texture_height {
                    // Create new shelf:
                    let position = (0, self.next_y);

                    self.shelves.push(Shelf {
                        current_x: source_width,
                        start_y: self.next_y,
                        height: source_height,
                    });

                    self.next_y += source_height;

                    Some(IRectangle::new(
                        position.0,
                        position.1,
                        source_width,
                        source_height,
                    ))
                } else {
                    // Won't fit:
                    None
                }
            })
    }
}
