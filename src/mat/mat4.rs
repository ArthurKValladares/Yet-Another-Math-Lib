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

    pub fn row(&self, idx: usize) -> Vec4 {
        self.data[idx]
    }

    pub fn col(&self, idx: usize) -> Vec4 {
        Vec4::new(self.data[0].idx(idx), self.data[1].idx(idx), self.data[2].idx(idx), self.data[3].idx(idx))
    }

    pub fn rotate_x(t: f32) -> Self {
        let c = t.cos();
        let s = t.sin();

        Self::from_data(
            1.0, 0.0, 0.0, 0.0, 0.0, c, -s, 0.0, 0.0, s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate_y(t: f32) -> Self {
        let c = t.cos();
        let s = t.sin();

        Self::from_data(
            c, 0.0, s, 0.0, 0.0, 1.0, 0.0, 0.0, -s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate_z(t: f32) -> Self {
        let c = t.cos();
        let s = t.sin();

        Self::from_data(
            c, -s, 0.0, 0.0, s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
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

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Mat4) -> Self::Output {
        Self {
            data: [
                Vec4::new(self.data[0].dot(&rhs.col(0)), self.data[0].dot(&rhs.col(1)), self.data[0].dot(&rhs.col(2)), self.data[0].dot(&rhs.col(3))),
                Vec4::new(self.data[1].dot(&rhs.col(0)), self.data[1].dot(&rhs.col(1)), self.data[1].dot(&rhs.col(2)), self.data[1].dot(&rhs.col(3))),
                Vec4::new(self.data[2].dot(&rhs.col(0)), self.data[2].dot(&rhs.col(1)), self.data[2].dot(&rhs.col(2)), self.data[2].dot(&rhs.col(3))),
                Vec4::new(self.data[3].dot(&rhs.col(0)), self.data[3].dot(&rhs.col(1)), self.data[3].dot(&rhs.col(2)), self.data[3].dot(&rhs.col(3))),
            ],
        }
    }
}