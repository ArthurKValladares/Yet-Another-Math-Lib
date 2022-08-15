#[repr(C)]
// Column-major
// TODO: Use Vec3
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Mat3 {
    n: [f32; 9],
}
