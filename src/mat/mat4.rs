use crate::vec::Vec3;

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

    pub fn rotate(angle: f32, axis: Vec3) -> Self {
        let sin_t = angle.sin();
        let cos_t = angle.cos();

        let x = axis.x();
        let y = axis.y();
        let z = axis.z();

        let xx = axis.x() * axis.x();
        let yy = axis.y() * axis.y();
        let zz = axis.z() * axis.z();

        let xy = axis.x() * axis.y();
        let xz = axis.x() * axis.z();
        let yz = axis.y() * axis.z();

        Self::from_rows_array([
            //
            cos_t + xx * (1.0 - cos_t),
            xy * (1.0 - cos_t) - z * sin_t,
            xz * (1.0 - cos_t) + y * sin_t,
            0.0,
            //
            xy * (1.0 - cos_t) + z * sin_t,
            cos_t + yy * (1.0 - cos_t),
            yz * (1.0 - cos_t) - x * sin_t,
            0.0,
            //
            xz * (1.0 - cos_t) - y * sin_t,
            yz * (1.0 - cos_t) + x * sin_t,
            cos_t + zz * (1.0 - cos_t),
            0.0,
            //
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
}
