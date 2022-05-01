use crate::vec::Vec3;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quat {
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let s = (angle / 2.0).sin();
        Self {
            x: axis.x() * s,
            y: axis.y() * s,
            z: axis.z() * s,
            w: (angle / 2.0).cos(),
        }
    }
}
