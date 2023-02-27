use crate::vec::Vec3;
use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct Quat {
    data: [f32; 4],
}

impl Quat {
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let s = (angle / 2.0).sin();
        Self {
            data: [
                axis.x() * s,
                axis.y() * s,
                axis.z() * s,
                (angle / 2.0).cos(),
            ],
        }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn w(&self) -> f32 {
        self.data[3]
    }

    pub fn rotate(&self, vec: Vec3) -> Vec3 {
        let b = Vec3::new(self.x(), self.y(), self.z());
        let b2 = b.magnitude_squared();
        vec * (self.w() * self.w() - b2)
            + b * (vec.dot(&b) * 2.0)
            + b.cross(&vec) * (self.w() * 2.0)
    }
}

impl From<[f32; 4]> for Quat {
    fn from(data: [f32; 4]) -> Self {
        Self { data }
    }
}
