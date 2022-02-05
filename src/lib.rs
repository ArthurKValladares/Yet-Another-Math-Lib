#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(&self, rhs: Vec2) -> f32 {
        (*self - rhs).magnitude()
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

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn distance(&self, rhs: Vec3) -> f32 {
        (*self - rhs).magnitude()
    }
}

impl From<Point2D> for Vec3 {
    fn from(point: Point2D) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: 0.0,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl std::ops::Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl std::ops::Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::Sub<Vec4> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

#[repr(C)]
// Column-major
// TODO: Use Vec3
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mat3 {
    n: [[f32; 3]; 3],
}

#[repr(C)]
// Column-major
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mat4 {
    d: [Vec4; 4],
}

impl Mat4 {
    pub fn from_cols_array(d: &[f32; 16]) -> Self {
        Self {
            d: [
                Vec4::new(d[0], d[1], d[2], d[3]),
                Vec4::new(d[4], d[5], d[6], d[7]),
                Vec4::new(d[8], d[9], d[10], d[11]),
                Vec4::new(d[12], d[13], d[14], d[15]),
            ],
        }
    }

    pub fn from_rows_array(d: &[f32; 16]) -> Self {
        Self {
            d: [
                Vec4::new(d[0], d[4], d[8], d[12]),
                Vec4::new(d[1], d[5], d[9], d[13]),
                Vec4::new(d[2], d[6], d[10], d[14]),
                Vec4::new(d[3], d[7], d[11], d[15]),
            ],
        }
    }

    pub fn col(&self, idx: usize) -> Vec4 {
        self.d[idx]
    }

    #[must_use]
    pub fn transpose(&self) -> Mat4 {
        Self {
            d: [
                Vec4::new(self.d[0].x, self.d[1].x, self.d[2].x, self.d[3].x),
                Vec4::new(self.d[0].y, self.d[1].y, self.d[2].y, self.d[3].y),
                Vec4::new(self.d[0].z, self.d[1].z, self.d[2].z, self.d[3].z),
                Vec4::new(self.d[0].w, self.d[1].w, self.d[2].w, self.d[3].w),
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quat {
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let s = (angle / 2.0).sin();
        Self {
            x: axis.x * s,
            y: axis.x * s,
            z: axis.x * s,
            w: (angle / 2.0).cos(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size2D {
    width: f32,
    height: f32,
}

impl Size2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rect2D {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl Rect2D {
    pub fn from_top_right_bottom_left(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn from_top_left(top_left: Point2D, size: Size2D) -> Self {
        Self::from_top_right_bottom_left(
            top_left.y,
            top_left.x + size.width,
            top_left.y + size.height,
            top_left.x,
        )
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn top_left(&self) -> Point2D {
        Point2D::new(self.left, self.top)
    }

    pub fn top_right(&self) -> Point2D {
        Point2D::new(self.right, self.top)
    }

    pub fn bottom_left(&self) -> Point2D {
        Point2D::new(self.left, self.bottom)
    }

    pub fn bottom_right(&self) -> Point2D {
        Point2D::new(self.right, self.bottom)
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rect3D {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
    depth: f32,
}

impl Rect3D {
    pub fn from_top_right_bottom_left_depth(
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        depth: f32,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
            depth,
        }
    }

    pub fn from_top_left(top_left: Point3D, size: Size2D) -> Self {
        Self::from_top_right_bottom_left_depth(
            top_left.y,
            top_left.x + size.width,
            top_left.y + size.height,
            top_left.x,
            top_left.z,
        )
    }
}
