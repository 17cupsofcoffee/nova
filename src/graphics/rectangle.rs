use std::ops::{Add, Sub};

use glam::Vec2;

#[derive(Copy, Clone)]
pub struct Rectangle<T = f32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl Rectangle<f32> {
    pub const ZERO: Rectangle = Rectangle::new(0.0, 0.0, 0.0, 0.0);
}

impl<T> Rectangle<T> {
    pub const fn new(x: T, y: T, width: T, height: T) -> Rectangle<T> {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

impl<T> Rectangle<T>
where
    T: Copy,
{
    pub fn intersects(&self, other: &Rectangle<T>) -> bool
    where
        T: Add<Output = T> + PartialOrd,
    {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn contains(&self, other: &Rectangle<T>) -> bool
    where
        T: Add<Output = T> + PartialOrd,
    {
        self.x <= other.x
            && other.x + other.width <= self.x + self.width
            && self.y <= other.y
            && other.y + other.height <= self.y + self.height
    }

    pub fn combine(&self, other: &Rectangle<T>) -> Rectangle<T>
    where
        T: Add<Output = T> + Sub<Output = T> + PartialOrd,
    {
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

        Rectangle {
            x,
            y,
            width: right - x,
            height: bottom - y,
        }
    }

    pub fn left(&self) -> T {
        self.x
    }

    pub fn right(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.x + self.width
    }

    pub fn top(&self) -> T {
        self.y
    }

    pub fn bottom(&self) -> T
    where
        T: Add<Output = T>,
    {
        self.y + self.height
    }
}

impl Rectangle<f32> {
    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.x + self.width, self.y)
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.x, self.y + self.height)
    }

    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new(self.x + self.width, self.y + self.height)
    }
}
