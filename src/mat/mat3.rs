use serde::Deserialize;

#[repr(C)]
// Column-major
// TODO: Use Vec3
#[derive(Debug, Deserialize, Default, Copy, Clone, PartialEq)]
pub struct Mat3 {
    n: [f32; 9],
}
