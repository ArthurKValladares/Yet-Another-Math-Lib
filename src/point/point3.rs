use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Default, Copy, Clone, PartialEq)]
pub struct Point3D<N: num::Num + Copy> {
    data: [N; 3],
}

impl<N> Point3D<N>
where
    N: num::Num + Copy,
{
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn x(&self) -> N {
        self.data[0]
    }

    pub fn y(&self) -> N {
        self.data[1]
    }

    pub fn z(&self) -> N {
        self.data[2]
    }
}
