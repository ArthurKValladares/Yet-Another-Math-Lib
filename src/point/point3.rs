#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point3D {
    data: [f32; 3],
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
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
}
