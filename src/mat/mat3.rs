#[repr(C)]
// Column-major
// TODO: Use Vec3
#[derive(Debug, Default, Copy, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Mat3 {
    n: [f32; 9],
}
