use glam::{Mat3, Vec2};

pub struct Transform {
    pub position: Vec2,
    pub origin: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vec2::ZERO,
            origin: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }

    pub fn position(mut self, position: Vec2) -> Transform {
        // self.matrix *= Mat3::from_translation(translation);
        self.position = position;
        self
    }

    pub fn origin(mut self, origin: Vec2) -> Transform {
        self.origin = origin;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Transform {
        // self.matrix *= Mat3::from_rotation_z(rotation);
        self.rotation = rotation;
        self
    }

    pub fn scale(mut self, scale: Vec2) -> Transform {
        // self.matrix *= Mat3::from_scale(scale);
        self.scale = scale;
        self
    }

    pub fn to_matrix(&self) -> Mat3 {
        Mat3::from_translation(self.position)
            * Mat3::from_scale(self.scale)
            * Mat3::from_rotation_z(self.rotation)
            * Mat3::from_translation(-self.origin)
    }
}

impl From<Vec2> for Transform {
    fn from(position: Vec2) -> Transform {
        Transform {
            position,
            origin: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}
