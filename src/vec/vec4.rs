use super::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec4 {
    data: [f32; 4],
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
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
            data: [vec.x(), vec.y(), 0.0, 0.0],
        }
    }
}

impl From<Vec3> for Vec4 {
    fn from(vec: Vec3) -> Self {
        Self {
            data: [vec.x(), vec.y(), vec.z(), 0.0],
        }
    }
}
