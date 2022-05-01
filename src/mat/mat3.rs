#[repr(C)]
// Column-major
// TODO: Use Vec3
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mat3 {
    n: [f32; 9],
}
