use rkyv::{Archive, Deserialize, Serialize};

#[repr(C)]
// Column-major
// TODO: Use Vec3
#[derive(Debug, Default, Copy, Clone, PartialEq, Archive, Deserialize, Serialize)]
pub struct Mat3 {
    n: [f32; 9],
}
