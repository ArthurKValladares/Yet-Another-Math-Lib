#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[rustfmt::skip]
pub struct Mat4 {
    data: [f32; 16],
}

impl Mat4 {
    pub fn from_rows_array(d: [f32; 16]) -> Self {
        Self { data: d }
    }

    pub fn identity() -> Mat4 {
        Self::from_rows_array([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }
}
