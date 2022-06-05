#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2 {
    data: [f32; 2],
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { data: [x, y] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn u(&self) -> f32 {
        self.data[0]
    }

    pub fn v(&self) -> f32 {
        self.data[1]
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn negate(&self) -> Self {
        *self * -1.0
    }

    pub fn magnitude(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn distance(&self, rhs: Self) -> f32 {
        (*self - rhs).magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(data: [f32; 2]) -> Self {
        Self { data }
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(vec: Vec2) -> Self {
        vec.data
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            data: [self.x() + rhs.x(), self.y() + rhs.y()],
        }
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            data: [self.x() - rhs.x(), self.y() - rhs.y()],
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            data: [
                self.x() * rhs,
                self.y() * rhs,
            ],
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            data: [self.x() / rhs, self.y() / rhs],
        }
    }
}
