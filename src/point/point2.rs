#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point2D {
    data: [f32; 2],
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { data: [x, y] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }
}
