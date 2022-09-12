#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point2D<N: num::Num + Copy> {
    data: [N; 2],
}

impl<N> Point2D<N>
where
    N: num::Num + Copy,
{
    pub fn new(x: N, y: N) -> Self {
        Self { data: [x, y] }
    }

    pub fn x(&self) -> N {
        self.data[0]
    }

    pub fn y(&self) -> N {
        self.data[1]
    }
}
