use crate::vec::Vec3;
use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct Quat {
    data: [f32; 4],
}

impl Quat {
    pub fn from_parts(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { data: [x, y, z, w] }
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half_angle = angle / 2.0;
        let sin_half_angle = half_angle.sin();
        let cos_half_angle = half_angle.cos();

        let x = axis.x() * sin_half_angle;
        let y = axis.y() * sin_half_angle;
        let z = axis.z() * sin_half_angle;
        let w = cos_half_angle;

        Self { data: [x, y, z, w] }
    }

    pub fn rotate_vector(&self, vec: Vec3) -> Vec3 {
        let as_quat = Self::from_parts(vec.x(), vec.y(), vec.z(), 0.0);
        let rotated_as_quat = (*self * as_quat) * self.inverse();
        debug_assert!(rotated_as_quat.w() == 0.0);
        Vec3::new(
            rotated_as_quat.x(),
            rotated_as_quat.y(),
            rotated_as_quat.z(),
        )
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

    pub fn inverse(self) -> Self {
        Self {
            data: [-self.x(), -self.y(), -self.z(), self.w()],
        }
    }
}

impl From<[f32; 4]> for Quat {
    fn from(data: [f32; 4]) -> Self {
        Self { data }
    }
}

impl std::ops::Mul<Quat> for Quat {
    type Output = Self;

    fn mul(self, rhs: Quat) -> Self::Output {
        let q1 = self;
        let q2 = rhs;
        Self {
            data: [
                q1.x() * q2.w() + q1.y() * q2.z() - q1.z() * q2.y() + q1.w() * q2.x(),
                -q1.x() * q2.z() + q1.y() * q2.w() + q1.z() * q2.x() + q1.w() * q2.y(),
                q1.x() * q2.y() - q1.y() * q2.x() - q1.z() * q2.w() + q1.w() * q2.z(),
                -q1.x() * q2.x() - q1.y() * q2.y() - q1.z() * q2.z() + q1.w() * q2.w(),
            ],
        }
    }
}
