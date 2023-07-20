use serde::Deserialize;

#[repr(C)]
#[derive(Debug, Deserialize, Default, Copy, Clone, PartialEq)]
#[serde(transparent)]
pub struct Size2D<N: num::Num + Copy> {
    data: [N; 2],
}

impl<N> Size2D<N>
where
    N: num::Num + Copy,
{
    pub fn new(width: N, height: N) -> Self {
        Self {
            data: [width, height],
        }
    }

    pub fn width(&self) -> N {
        self.data[0]
    }

    pub fn height(&self) -> N {
        self.data[1]
    }
}
