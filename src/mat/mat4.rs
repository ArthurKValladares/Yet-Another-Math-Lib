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
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotate(t: f32, a: Vec3) -> Self {
        let s = t.sin();
        let c = t.cos();
        let d = 1.0 - c;

        let x = a.x() * d;
        let y = a.y() * d;
        let z = a.z() * d;

        let axay = x * a.y();
        let axaz = x * a.z();
        let ayaz = y * a.z();

        Self::from_rows_array([
            c + x * a.x(),
            axay + s * a.z(),
            axaz - s * a.y(),
            0.0,
            //
            axay - s * a.z(),
            c + y * a.y(),
            ayaz + s * a.x(),
            0.0,
            //
            axaz + s * a.y(),
            ayaz - s * a.x(),
            c + z * a.z(),
            0.0,
            //
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
}
