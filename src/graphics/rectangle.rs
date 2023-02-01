use glam::{IVec2, Vec2};

macro_rules! rect {
    ($rect:ident, $t:ty, $point:path, $zero:literal) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $rect {
            pub x: $t,
            pub y: $t,
            pub width: $t,
            pub height: $t,
        }

        impl $rect {
            pub const ZERO: $rect = $rect::new($zero, $zero, $zero, $zero);

            pub const fn new(x: $t, y: $t, width: $t, height: $t) -> $rect {
                $rect {
                    x,
                    y,
                    width,
                    height,
                }
            }

            pub const fn from_point(point: $point, width: $t, height: $t) -> $rect {
                $rect {
                    x: point.x,
                    y: point.y,
                    width,
                    height,
                }
            }

            pub fn left(&self) -> $t {
                self.x
            }

            pub fn right(&self) -> $t {
                self.x + self.width
            }

            pub fn top(&self) -> $t {
                self.y
            }

            pub fn bottom(&self) -> $t {
                self.y + self.height
            }

            pub fn top_left(&self) -> $point {
                <$point>::new(self.x, self.y)
            }

            pub fn top_right(&self) -> $point {
                <$point>::new(self.x + self.width, self.y)
            }

            pub fn bottom_left(&self) -> $point {
                <$point>::new(self.x, self.y + self.height)
            }

            pub fn bottom_right(&self) -> $point {
                <$point>::new(self.x + self.width, self.y + self.height)
            }

            pub fn intersects(&self, other: &$rect) -> bool {
                self.x < other.x + other.width
                    && self.x + self.width > other.x
                    && self.y < other.y + other.height
                    && self.y + self.height > other.y
            }

            pub fn contains(&self, other: &$rect) -> bool {
                self.x <= other.x
                    && other.x + other.width <= self.x + self.width
                    && self.y <= other.y
                    && other.y + other.height <= self.y + self.height
            }

            pub fn contains_point(&self, point: $point) -> bool {
                self.x <= point.x
                    && point.x < self.x + self.width
                    && self.y <= point.y
                    && point.y < self.y + self.height
            }

            pub fn combine(&self, other: &$rect) -> $rect {
                let x = if self.x < other.x { self.x } else { other.x };
                let y = if self.y < other.y { self.y } else { other.y };

                let right = if self.right() > other.right() {
                    self.right()
                } else {
                    other.right()
                };

                let bottom = if self.bottom() > other.bottom() {
                    self.bottom()
                } else {
                    other.bottom()
                };

                $rect {
                    x,
                    y,
                    width: right - x,
                    height: bottom - y,
                }
            }
        }
    };
}

rect!(Rectangle, f32, Vec2, 0.0);

impl Rectangle {
    pub fn as_irectangle(&self) -> IRectangle {
        IRectangle {
            x: self.x as i32,
            y: self.y as i32,
            width: self.width as i32,
            height: self.height as i32,
        }
    }
}

rect!(IRectangle, i32, IVec2, 0);

impl IRectangle {
    pub fn as_rectangle(&self) -> Rectangle {
        Rectangle {
            x: self.x as f32,
            y: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}
