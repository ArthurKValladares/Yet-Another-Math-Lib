use crate::vec::{Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[rustfmt::skip]
pub struct Mat4 {
    data: [Vec4; 4],
}

impl Mat4 {
    pub fn from_data(
        n00: f32,
        n01: f32,
        n02: f32,
        n03: f32,
        n10: f32,
        n11: f32,
        n12: f32,
        n13: f32,
        n20: f32,
        n21: f32,
        n22: f32,
        n23: f32,
        n30: f32,
        n31: f32,
        n32: f32,
        n33: f32,
    ) -> Self {
        Self {
            data: [
                Vec4::new(n00, n01, n02, n03),
                Vec4::new(n10, n11, n12, n13),
                Vec4::new(n20, n21, n22, n23),
                Vec4::new(n30, n31, n32, n33),
            ],
        }
    }

    pub fn identity() -> Mat4 {
        Self::from_data(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
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

        Self::from_data(
            c + x * a.x(),
            axay - s * a.z(),
            axaz + s * a.y(),
            0.0,
            axay + s * a.z(),
            c + y * a.y(),
            ayaz - s * a.x(),
            0.0,
            axaz - s * a.y(),
            ayaz + s * a.x(),
            c + z * a.z(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
}
