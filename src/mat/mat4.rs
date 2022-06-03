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
        let s = angle.sin();
        let c = angle.cos();
        let d = 1.0 - c;

        let x = axis.x() * d;
        let y = axis.y() * d;
        let z = axis.z() * d;

        let axay = x * axis.y();
        let axaz = x * axis.z();
        let ayaz = y * axis.z();

        Self::from_rows_array([
            c + x * axis.x(),
            axay - s * axis.z(),
            axaz + s * axis.y(),
            0.0,
            //
            axay + s * axis.z(),
            c + y * axis.y(),
            ayaz - s * axis.x(),
            0.0,
            //
            axaz - s * axis.y(),
            ayaz + s * axis.x(),
            c + z * axis.z(),
            0.0,
            //
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
}
