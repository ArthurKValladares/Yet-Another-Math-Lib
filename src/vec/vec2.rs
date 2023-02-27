use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Default, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn negate(&self) -> Self {
        *self * -1.0
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
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
        self.x * rhs.x + self.y * rhs.y
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(data: [f32; 2]) -> Self {
        Self {
            x: data[0],
            y: data[1],
        }
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(vec: Vec2) -> Self {
        [vec.x, vec.y]
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
