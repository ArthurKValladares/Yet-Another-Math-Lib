#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size2D<N: num::Num + Copy> {
    width: N,
    height: N,
}

impl<N> Size2D<N>
where
    N: num::Num + Copy,
{
    pub fn new(width: N, height: N) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> N {
        self.width
    }

    pub fn height(&self) -> N {
        self.height
    }
}
