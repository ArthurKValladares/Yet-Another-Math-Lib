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

    pub fn vector_component(&self) -> Vec3 {
        // TODO: mem::transmute
        Vec3::new(self.x(), self.y(), self.z())
    }

    pub fn scalar_component(&self) -> f32 {
        self.w()
    }

    pub fn rotate_vector(&self, vec: Vec3) -> Vec3 {
        let b = self.vector_component();
        let b2 = b.magnitude_squared();

        let q = self.normalized();

        vec * (q.w() * q.w() - b2) + b * (vec.dot(&b) * 2.0) + b.cross(&vec) * (q.w() * 2.0)
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(&self) -> Quat {
        *self / self.magnitude()
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
                q1.w() * q2.x() + q1.x() * q2.w() + q1.y() * q2.z() - q1.z() * q2.y(),
                q1.w() * q2.y() - q1.x() * q2.z() + q1.y() * q2.w() + q1.z() * q2.x(),
                q1.w() * q2.z() + q1.x() * q2.y() - q1.y() * q2.z() + q1.z() * q2.w(),
                q1.w() * q2.w() - q1.x() * q2.x() - q1.y() * q2.y() - q1.z() * q2.z(),
            ],
        }
    }
}

impl std::ops::Mul<f32> for Quat {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: [
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs,
                self.w() * rhs,
            ],
        }
    }
}

impl std::ops::Div<f32> for Quat {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            data: [
                self.x() / rhs,
                self.y() / rhs,
                self.z() / rhs,
                self.w() / rhs,
            ],
        }
    }
}
