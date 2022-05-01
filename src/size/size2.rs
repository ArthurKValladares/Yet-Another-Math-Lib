#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size2D {
    pub width: f32,
    pub height: f32,
}

impl Size2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
