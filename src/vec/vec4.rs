use super::{Vec2, Vec3};
use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Default, Copy, Clone, PartialEq)]
pub struct Vec4 {
    data: [f32; 4],
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { data: [x, y, z, w] }
    }

    pub fn idx(&self, idx: usize) -> f32 {
        self.data[idx]
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

    pub fn r(&self) -> f32 {
        self.data[0]
    }

    pub fn g(&self) -> f32 {
        self.data[1]
    }

    pub fn b(&self) -> f32 {
        self.data[2]
    }

    pub fn a(&self) -> f32 {
        self.data[3]
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn negate(&self) -> Self {
        *self * -1.0
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn is_normal(&self) -> bool {
        self.magnitude() == 1.0
    }

    pub fn distance(&self, rhs: Self) -> f32 {
        (*self - rhs).magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z() + self.w() * rhs.w()
    }
}

impl std::ops::Mul<f32> for Vec4 {
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

impl std::ops::Div<f32> for Vec4 {
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

impl std::ops::Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self {
            data: [
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z(),
                self.w() + rhs.w(),
            ],
        }
    }
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Self {
            data: [
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z(),
                self.w() - rhs.w(),
            ],
        }
    }
}

impl std::ops::AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
        self.data[3] += rhs.data[3];
    }
}

impl std::ops::SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
        self.data[3] -= rhs.data[3];
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(data: [f32; 4]) -> Self {
        Self { data }
    }
}

impl From<Vec4> for [f32; 4] {
    fn from(vec: Vec4) -> Self {
        vec.data
    }
}

impl From<Vec2> for Vec4 {
    fn from(vec: Vec2) -> Self {
        Self {
            data: [vec.x, vec.y, 0.0, 0.0],
        }
    }
}

impl From<Vec3> for Vec4 {
    fn from(vec: Vec3) -> Self {
        Self {
            data: [vec.x(), vec.y(), vec.z(), 1.0],
        }
    }
}

impl From<[f32; 3]> for Vec4 {
    fn from(data: [f32; 3]) -> Self {
        Self {
            data: [data[0], data[1], data[2], 1.0],
        }
    }
}
