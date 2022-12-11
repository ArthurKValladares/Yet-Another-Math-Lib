use super::Vec2;
use crate::point::{Point2D, Point3D};
use rkyv::{Archive, Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Archive, Deserialize, Serialize)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
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

    pub fn r(&self) -> f32 {
        self.data[0]
    }

    pub fn g(&self) -> f32 {
        self.data[1]
    }

    pub fn b(&self) -> f32 {
        self.data[2]
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn negate(&self) -> Self {
        *self * -1.0
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn distance(&self, rhs: Self) -> f32 {
        (*self - rhs).magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(), //
            self.z() * rhs.x() - self.x() * rhs.z(), //
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(data: [f32; 3]) -> Self {
        Self { data }
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(vec: Vec3) -> Self {
        vec.data
    }
}

impl From<Vec2> for Vec3 {
    fn from(vec: Vec2) -> Self {
        Self {
            data: [vec.x, vec.y, 0.0],
        }
    }
}

impl From<Point2D<f32>> for Vec3 {
    fn from(point: Point2D<f32>) -> Self {
        Self {
            data: [point.x(), point.y(), 0.0],
        }
    }
}

impl From<Point3D<f32>> for Vec3 {
    fn from(point: Point3D<f32>) -> Self {
        Self {
            data: [point.x(), point.y(), point.z()],
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            data: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.data[0] -= rhs.data[0];
        self.data[1] -= rhs.data[1];
        self.data[2] -= rhs.data[2];
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            data: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            data: [self.x() / rhs, self.y() / rhs, self.z() / rhs],
        }
    }
}
