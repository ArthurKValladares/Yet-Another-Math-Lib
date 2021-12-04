#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct Mat3 {
    n: [[f32; 3]; 3],
}

#[repr(C)]
// Column-major
#[derive(Copy, Clone)]
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

    pub fn col(&self, idx: usize) -> Vec4 {
        self.d[idx]
    }

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
#[derive(Copy, Clone)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
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
